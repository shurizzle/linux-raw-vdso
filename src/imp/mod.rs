#![allow(dead_code)]

#[cfg_attr(target_arch = "arm", path = "../arch/arm.rs")]
#[cfg_attr(target_arch = "aarch64", path = "../arch/aarch64.rs")]
#[cfg_attr(
    any(target_arch = "mips", target_arch = "mips64"),
    path = "../arch/mips.rs"
)]
#[cfg_attr(target_arch = "powerpc", path = "../arch/powerpc.rs")]
#[cfg_attr(target_arch = "powerpc64", path = "../arch/powerpc64.rs")]
#[cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    path = "../arch/riscv.rs"
)]
#[cfg_attr(target_arch = "s390x", path = "../arch/s390x.rs")]
#[cfg_attr(target_arch = "x86", path = "../arch/x86.rs")]
#[cfg_attr(target_arch = "x86_64", path = "../arch/x86_64.rs")]
mod arch;

pub use arch::*;

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "s390x",
))]
impl Vdso {
    pub unsafe fn from_ptr(ptr: *const core::ffi::c_void) -> Option<Self> {
        Self::from_reader(VdsoReader::from_ptr(ptr)?)
    }
}

pub(crate) mod util;

mod elf_common {
    #[cfg(target_endian = "little")]
    pub use goblin::elf::header::ELFDATA2LSB as ELFDATA;
    #[cfg(target_endian = "big")]
    pub use goblin::elf::header::ELFDATA2MSB as ELFDATA;

    #[cfg(target_arch = "x86")]
    pub use goblin::elf::header::EM_386 as EM_CURRENT;
    #[cfg(target_arch = "aarch64")]
    pub use goblin::elf::header::EM_AARCH64 as EM_CURRENT;
    #[cfg(target_arch = "arm")]
    pub use goblin::elf::header::EM_ARM as EM_CURRENT;
    #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
    pub use goblin::elf::header::EM_MIPS as EM_CURRENT;
    #[cfg(target_arch = "powerpc")]
    pub use goblin::elf::header::EM_PPC as EM_CURRENT;
    #[cfg(target_arch = "powerpc64")]
    pub use goblin::elf::header::EM_PPC64 as EM_CURRENT;
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    pub use goblin::elf::header::EM_RISCV as EM_CURRENT;
    #[cfg(target_arch = "s390x")]
    pub use goblin::elf::header::EM_S390 as EM_CURRENT;
    #[cfg(target_arch = "x86_64")]
    pub use goblin::elf::header::EM_X86_64 as EM_CURRENT;
}

#[cfg(target_pointer_width = "64")]
mod elf {
    pub use goblin::elf64::dynamic::*;
    pub use goblin::elf64::header::*;
    pub use goblin::elf64::header::*;
    pub use goblin::elf64::program_header::*;
    pub use goblin::elf64::section_header::*;
    pub use goblin::elf64::sym::*;

    pub const CLASS: u8 = ELFCLASS64;
    pub type Word = u64;

    pub use super::elf_common::*;
}

#[cfg(target_pointer_width = "32")]
mod elf {
    pub use goblin::elf32::dynamic::*;
    pub use goblin::elf32::header::*;
    pub use goblin::elf32::header::*;
    pub use goblin::elf32::program_header::*;
    pub use goblin::elf32::section_header::*;
    pub use goblin::elf32::sym::*;

    pub const CLASS: u8 = ELFCLASS32;
    pub type Word = u32;

    pub use super::elf_common::*;
}

use core::{marker::PhantomData, ptr};

#[repr(C)]
#[derive(Debug)]
struct Verdef {
    /// Version revision. This field shall be set to 1.
    vd_version: u16,
    /// Version information flag bitmask.
    vd_flags: u16,
    /// Version index numeric value referencing the SHT_GNU_versym section.
    vd_ndx: u16,
    /// Number of associated verdaux array entries.
    vd_cnt: u16,
    /// Version name hash value (ELF hash function).
    vd_hash: u32,
    /// Offset in bytes to a corresponding entry in an array of Elfxx_Verdaux structures as defined in Figure 2-2
    vd_aux: u32,
    /// Offset to the next verdef entry, in bytes.
    vd_next: u32,
}

#[repr(C)]
#[derive(Debug)]
struct Verdaux {
    /// Offset to the version or dependency name string in the section header, in bytes.
    vda_name: u32,
    /// Offset to the next verdaux entry, in bytes.
    vda_next: u32,
}

pub(crate) struct VdsoReader<'a> {
    header: &'a VdsoHeader,
    versyms: *const u16,
    verdefs: *const Verdef,
    strings: *const u8,
    syms: &'a [elf::Sym],
}

impl<'a> VdsoReader<'a> {
    pub unsafe fn from_ptr(ptr: *const core::ffi::c_void) -> Option<Self> {
        Self::from_header(VdsoHeader::from_ptr(ptr)?)
    }

    unsafe fn from_header(header: &'a VdsoHeader) -> Option<VdsoReader> {
        let mut versyms: *const u16 = ptr::null();
        let mut verdefs: *const Verdef = ptr::null();
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

pub struct Version<'a> {
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

pub struct VersionIter<'a> {
    verdefs: *const Verdef,
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
                    let aux = &*(verdef as *const Verdef)
                        .cast::<u8>()
                        .add(verdef.vd_aux as usize)
                        .cast::<Verdaux>();
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
        if head.0.e_ident[elf::EI_CLASS] != elf::CLASS {
            return None;
        }

        // Test OS ABI
        if !matches!(
            head.0.e_ident[elf::EI_OSABI],
            elf::ELFOSABI_SYSV | elf::ELFOSABI_LINUX
        ) {
            return None;
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

        if head.0.e_phnum == u16::MAX {
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
    use linux_auxv::*;

    #[test]
    fn retrieve() {
        let ptr = Auxv::from_env().into_iter().find_map(|v| {
            if let AuxvType::SysInfoHeader(p) = v {
                Some(p)
            } else {
                None
            }
        });
        assert!(ptr.is_some());
        let ptr = ptr.unwrap();
        assert!(unsafe { super::Vdso::from_ptr(ptr).is_some() })
    }
}
