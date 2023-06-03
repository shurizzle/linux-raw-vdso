//! A tiny library to parse the in-memory vDSO.
//! For more documentation of what's vDSO and why we need it on Linux see
//! [here](https://man7.org/linux/man-pages/man7/vdso.7.html).

#![no_std]

#[cfg_attr(
    all(target_os = "linux", target_arch = "aarch64"),
    path = "arch/aarch64.rs"
)]
#[cfg_attr(all(target_os = "linux", target_arch = "arm"), path = "arch/arm.rs")]
#[cfg_attr(
    all(target_os = "linux", target_arch = "loongarch64"),
    path = "arch/loongarch64.rs"
)]
#[cfg_attr(
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    path = "arch/mips.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "powerpc"),
    path = "arch/powerpc.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "powerpc64"),
    path = "arch/powerpc64.rs"
)]
#[cfg_attr(
    all(
        target_os = "linux",
        any(target_arch = "riscv32", target_arch = "riscv64")
    ),
    path = "arch/riscv.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "s390x"),
    path = "arch/s390x.rs"
)]
#[cfg_attr(
    all(
        target_os = "linux",
        target_arch = "x86_64",
        target_pointer_width = "32"
    ),
    path = "arch/x32.rs"
)]
#[cfg_attr(all(target_os = "linux", target_arch = "x86"), path = "arch/x86.rs")]
#[cfg_attr(
    all(
        target_os = "linux",
        target_arch = "x86_64",
        target_pointer_width = "64"
    ),
    path = "arch/x86_64.rs"
)]
mod arch;

mod elf;
pub(crate) mod util;

pub use arch::Vdso;

use core::{marker::PhantomData, ptr};

pub(crate) struct VdsoReader<'a> {
    header: &'a VdsoHeader,
    versyms: *const u16,
    verdefs: *const elf::Verdef,
    strings: *const u8,
    syms: &'a [elf::Sym],
}

impl<'a> VdsoReader<'a> {
    pub unsafe fn from_ptr(ptr: *const core::ffi::c_void) -> Option<Self> {
        Self::from_header(VdsoHeader::from_ptr(ptr)?)
    }

    unsafe fn from_header(header: &'a VdsoHeader) -> Option<VdsoReader> {
        let mut versyms: *const u16 = ptr::null();
        let mut verdefs: *const elf::Verdef = ptr::null();
        let mut strings: *const u8 = ptr::null();
        let mut syms: Option<&[elf::Sym]> = None;
        let mut filled = 0u8;

        for sh in header.shs() {
            match sh.sh_type {
                elf::SHT_GNU_VERSYM => {
                    versyms = header.offset(sh.sh_offset);
                    filled |= 1 << 0;
                    if filled == 15 {
                        break;
                    }
                }
                elf::SHT_GNU_VERDEF => {
                    verdefs = header.offset(sh.sh_offset);
                    filled |= 1 << 1;
                    if filled == 15 {
                        break;
                    }
                }
                elf::SHT_STRTAB => {
                    strings = header.offset(sh.sh_offset);
                    filled |= 1 << 2;
                    if filled == 15 {
                        break;
                    }
                }
                elf::SHT_DYNSYM => {
                    syms = Some(header.slice(sh.sh_offset, sh.sh_size));
                    filled |= 1 << 3;
                    if filled == 15 {
                        break;
                    }
                }
                _ => (),
            }
        }

        if versyms.is_null() {
            verdefs = ptr::null();
        } else if verdefs.is_null() {
            versyms = ptr::null();
        }
        let syms = syms?;
        if strings.is_null() {
            return None;
        }

        Some(Self {
            header,
            versyms,
            verdefs,
            strings,
            syms,
        })
    }

    pub fn versions(&self) -> VersionIter {
        VersionIter {
            verdefs: self.verdefs,
            reader: self,
        }
    }

    pub fn symbols(&self) -> SymbolIter {
        SymbolIter {
            versyms: self.versyms,
            iter: self.syms.iter(),
            reader: self,
        }
    }
}

pub(crate) struct Version<'a> {
    hash: u32,
    id: u16,
    name: *const u8,
    _life: PhantomData<&'a ()>,
}

impl<'a> Version<'a> {
    #[inline]
    pub const fn hash(&self) -> u32 {
        self.hash
    }

    #[inline]
    pub const fn id(&self) -> u16 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> *const u8 {
        self.name
    }
}

pub(crate) struct VersionIter<'a> {
    verdefs: *const elf::Verdef,
    reader: &'a VdsoReader<'a>,
}

impl<'a> Iterator for VersionIter<'a> {
    type Item = Version<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            unsafe {
                if self.verdefs.is_null() {
                    return None;
                }

                let verdef = &*self.verdefs;
                if verdef.vd_next == 0 {
                    self.verdefs = ptr::null();
                } else {
                    self.verdefs = self
                        .verdefs
                        .cast::<u8>()
                        .add(verdef.vd_next as usize)
                        .cast();
                }

                if verdef.vd_version == 1 && verdef.vd_flags & 1 == 0 {
                    let aux = &*(verdef as *const elf::Verdef)
                        .cast::<u8>()
                        .add(verdef.vd_aux as usize)
                        .cast::<elf::Verdaux>();
                    let name = self.reader.strings.add(aux.vda_name as usize);

                    return Some(Version {
                        hash: verdef.vd_hash,
                        id: verdef.vd_ndx,
                        name,
                        _life: PhantomData,
                    });
                }
            }
        }
    }
}

pub(crate) struct Symbol<'a> {
    name: *const u8,
    ptr: *const core::ffi::c_void,
    vid: Option<u16>,
    _life: PhantomData<&'a ()>,
}

impl<'a> Symbol<'a> {
    #[inline]
    pub fn name(&self) -> *const u8 {
        self.name
    }

    #[inline]
    pub fn ptr(&self) -> *const core::ffi::c_void {
        self.ptr
    }

    #[inline]
    pub fn version_id(&self) -> Option<u16> {
        self.vid
    }
}

pub(crate) struct SymbolIter<'a> {
    versyms: *const u16,
    iter: core::slice::Iter<'a, elf::Sym>,
    reader: &'a VdsoReader<'a>,
}

impl<'a> Iterator for SymbolIter<'a> {
    type Item = Symbol<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            unsafe {
                let sym = self.iter.next()?;
                let vid = if !self.versyms.is_null() {
                    let res = *self.versyms;
                    self.versyms = self.versyms.add(1);
                    Some(res)
                } else {
                    None
                };

                let typ = elf::st_type(sym.st_info);
                let bind = elf::st_bind(sym.st_info);

                if (bind == elf::STB_GLOBAL || bind == elf::STB_WEAK)
                    && (typ == elf::STT_FUNC || typ == elf::STT_NOTYPE)
                    && elf::st_visibility(sym.st_other) == elf::STV_DEFAULT
                {
                    let name = self.reader.strings.add(sym.st_name as usize);

                    return Some(Symbol {
                        name,
                        ptr: self.reader.header.offset(sym.st_value),
                        vid,
                        _life: PhantomData,
                    });
                }
            }
        }
    }
}

#[repr(transparent)]
struct VdsoHeader(elf::Header);

impl VdsoHeader {
    pub(crate) unsafe fn from_ptr<'a>(ptr: *const core::ffi::c_void) -> Option<&'a Self> {
        let head = &*(ptr as *const Self);

        // Test magic number
        if head.0.e_ident[..elf::ELFMAG.len()] != elf::ELFMAG[..] {
            return None;
        }

        // Test class
        if head.0.e_ident[elf::EI_CLASS] != elf::ELFCLASS {
            return None;
        }

        // Test OS ABI
        match head.0.e_ident[elf::EI_OSABI] {
            elf::ELFOSABI_SYSV | elf::ELFOSABI_LINUX => (),
            _ => return None,
        }

        // Test ABI version
        if head.0.e_ident[elf::EI_ABIVERSION] != 0 {
            return None;
        }

        // Test elf type, it must be dynamic
        if head.0.e_type != elf::ET_DYN {
            return None;
        }

        // Test elf version
        if head.0.e_ident[elf::EI_VERSION] != elf::EV_CURRENT {
            return None;
        }

        // Test some sizes
        if head.0.e_ehsize as usize != core::mem::size_of::<elf::Header>()
            || head.0.e_phentsize as usize != core::mem::size_of::<elf::ProgramHeader>()
        {
            return None;
        }

        if head.0.e_phnum == 0xffff {
            return None;
        }

        if (head.0.e_phoff as usize) < core::mem::size_of::<elf::Header>() {
            return None;
        }

        // Test endianness
        if head.0.e_ident[elf::EI_DATA] != elf::ELFDATA {
            return None;
        }

        // Test arch
        if head.0.e_machine != elf::EM_CURRENT {
            return None;
        }

        Some(head)
    }

    pub(crate) unsafe fn offset<T, O>(&self, off: O) -> *const T
    where
        O: Into<elf::Word>,
    {
        (self as *const Self)
            .cast::<u8>()
            .add(off.into() as usize)
            .cast::<T>()
    }

    pub(crate) unsafe fn slice<'a, T, T1, T2>(&'a self, off: T1, len: T2) -> &[T]
    where
        T1: Into<elf::Word> + 'a,
        T2: Into<elf::Word> + 'a,
    {
        core::slice::from_raw_parts::<u8>(self.offset(off), len.into() as usize)
            .align_to()
            .1
    }

    pub(crate) unsafe fn shs(&self) -> &[elf::SectionHeader] {
        self.slice(self.0.e_shoff, self.0.e_shentsize * self.0.e_shnum)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    #[cfg(all(
        target_os = "linux",
        target_arch = "x86_64",
        target_endian = "little",
        target_pointer_width = "64"
    ))]
    const FAKE: &str = "fake/x86_64.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "x86_64",
        target_endian = "little",
        target_pointer_width = "32"
    ))]
    const FAKE: &str = "fake/x32.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "x86",
        target_endian = "little",
        target_pointer_width = "32"
    ))]
    const FAKE: &str = "fake/x86.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "aarch64",
        target_endian = "little",
        target_pointer_width = "64"
    ))]
    const FAKE: &str = "fake/aarch64.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "arm",
        target_endian = "little",
        target_pointer_width = "32"
    ))]
    const FAKE: &str = "fake/arm.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "loongarch64",
        target_endian = "little",
        target_pointer_width = "64"
    ))]
    const FAKE: &str = "fake/loongarch64.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "mips",
        target_endian = "big",
        target_pointer_width = "32"
    ))]
    const FAKE: &str = "fake/mips.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "mips64",
        target_endian = "big",
        target_pointer_width = "64"
    ))]
    const FAKE: &str = "fake/mips64.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "powerpc",
        target_endian = "big",
        target_pointer_width = "32"
    ))]
    const FAKE: &str = "fake/powerpc.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "powerpc64",
        target_endian = "big",
        target_pointer_width = "64"
    ))]
    const FAKE: &str = "fake/powerpc64.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "riscv32",
        target_endian = "little",
        target_pointer_width = "32"
    ))]
    const FAKE: &str = "fake/riscv32.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "riscv64",
        target_endian = "little",
        target_pointer_width = "64"
    ))]
    const FAKE: &str = "fake/riscv64.so";
    #[cfg(all(
        target_os = "linux",
        target_arch = "s390x",
        target_endian = "big",
        target_pointer_width = "64"
    ))]
    const FAKE: &str = "fake/s390x.so";

    #[test]
    fn parse() {
        let data = std::fs::read(FAKE).unwrap();
        assert!(unsafe { super::Vdso::from_ptr(data.as_ptr().cast()).is_some() })
    }
}
