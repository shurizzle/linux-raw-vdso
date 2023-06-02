#[cfg(target_endian = "little")]
pub const ELFDATA: u8 = 1;
#[cfg(target_endian = "big")]
pub const ELFDATA: u8 = 2;

#[cfg(target_pointer_width = "32")]
pub const ELFCLASS: u8 = 1;
#[cfg(target_pointer_width = "64")]
pub const ELFCLASS: u8 = 2;

#[cfg(target_pointer_width = "32")]
pub type Word = u32;
#[cfg(target_pointer_width = "64")]
pub type Word = u64;

/// The ELF magic number.
pub const ELFMAG: &[u8; 4] = b"\x7FELF";

/// File class byte index.
pub const EI_CLASS: usize = 4;
/// OS ABI index
pub const EI_OSABI: usize = 7;
/// ABI version index
pub const EI_ABIVERSION: usize = 8;
/// File version index
pub const EI_VERSION: usize = 6;
/// Endianness index
pub const EI_DATA: usize = 5;

#[cfg(target_pointer_width = "32")]
pub const CLASS: u8 = 1;
#[cfg(target_pointer_width = "64")]
pub const CLASS: u8 = 2;
pub const EV_CURRENT: u8 = 1;

#[cfg(target_arch = "x86")]
pub const EM_CURRENT: u16 = 3;
#[cfg(target_arch = "aarch64")]
pub const EM_CURRENT: u16 = 0xb7;
#[cfg(target_arch = "arm")]
pub const EM_CURRENT: u16 = 0x28;
#[cfg(any(target_arch = "mips", target_arch = "mips64"))]
pub const EM_CURRENT: u16 = 8;
#[cfg(target_arch = "powerpc")]
pub const EM_CURRENT: u16 = 0x14;
#[cfg(target_arch = "powerpc64")]
pub const EM_CURRENT: u16 = 0x15;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub const EM_CURRENT: u16 = 0xf3;
#[cfg(target_arch = "s390x")]
pub const EM_CURRENT: u16 = 0x16;
#[cfg(target_arch = "x86_64")]
pub const EM_CURRENT: u16 = 0x3e;

pub const ELFOSABI_SYSV: u8 = 0;
pub const ELFOSABI_LINUX: u8 = 3;

/// Shared object file.
pub const ET_DYN: u16 = 3;

/// Version symbol table.
pub const SHT_GNU_VERSYM: u32 = 0x6fff_ffff;
/// Version definition section.
pub const SHT_GNU_VERDEF: u32 = 0x6fff_fffd;
/// String table.
pub const SHT_STRTAB: u32 = 3;
/// Dynamic linker symbol table.
pub const SHT_DYNSYM: u32 = 11;

/// Number of bytes in an identifier.
pub const SIZEOF_IDENT: usize = 16;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Header {
    /// Magic number and other info
    pub e_ident: [u8; SIZEOF_IDENT],
    /// Object file type
    pub e_type: u16,
    /// Architecture
    pub e_machine: u16,
    /// Object file version
    pub e_version: u32,
    /// Entry point virtual address
    pub e_entry: Word,
    /// Program header table file offset
    pub e_phoff: Word,
    /// Section header table file offset
    pub e_shoff: Word,
    /// Processor-specific flags
    pub e_flags: u32,
    /// ELF header size in bytes
    pub e_ehsize: u16,
    /// Program header table entry size
    pub e_phentsize: u16,
    /// Program header table entry count
    pub e_phnum: u16,
    /// Section header table entry size
    pub e_shentsize: u16,
    /// Section header table entry count
    pub e_shnum: u16,
    /// Section header string table index
    pub e_shstrndx: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct Verdef {
    /// Version revision. This field shall be set to 1.
    pub vd_version: u16,
    /// Version information flag bitmask.
    pub vd_flags: u16,
    /// Version index numeric value referencing the SHT_GNU_versym section.
    pub vd_ndx: u16,
    /// Number of associated verdaux array entries.
    pub vd_cnt: u16,
    /// Version name hash value (ELF hash function).
    pub vd_hash: u32,
    /// Offset in bytes to a corresponding entry in an array of Elfxx_Verdaux structures
    pub vd_aux: u32,
    /// Offset to the next verdef entry, in bytes.
    pub vd_next: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Verdaux {
    /// Offset to the version or dependency name string in the section header, in bytes.
    pub vda_name: u32,
    /// Offset to the next verdaux entry, in bytes.
    pub vda_next: u32,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "alloc", derive(Pread, Pwrite, SizeWith))]
/// 64-bit Sym - used for both static and dynamic symbol information in a binary
pub struct Sym {
    /// Symbol name (string tbl index)
    pub st_name: u32,
    /// Symbol type and binding
    pub st_info: u8,
    /// Symbol visibility
    pub st_other: u8,
    /// Section index
    pub st_shndx: u16,
    /// Symbol value
    pub st_value: u64,
    /// Symbol size
    pub st_size: u64,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "alloc", derive(Pread, Pwrite, SizeWith))]
/// 32-bit Sym - used for both static and dynamic symbol information in a binary
pub struct Sym {
    /// Symbol name (string tbl index)
    pub st_name: u32,
    /// Symbol value
    pub st_value: u32,
    /// Symbol size
    pub st_size: u32,
    /// Symbol type and binding
    pub st_info: u8,
    /// Symbol visibility
    pub st_other: u8,
    /// Section index
    pub st_shndx: u16,
}

/// Global symbol.
pub const STB_GLOBAL: u8 = 1;
/// Weak symbol.
pub const STB_WEAK: u8 = 2;

/// Symbol type is unspecified.
pub const STT_NOTYPE: u8 = 0;
/// Symbol is a code object.
pub const STT_FUNC: u8 = 2;

/// Default: Visibility is specified by the symbol's binding type
pub const STV_DEFAULT: u8 = 0;

#[repr(C)]
pub struct SectionHeader {
    /// Section name (string tbl index)
    pub sh_name: u32,
    /// Section type
    pub sh_type: u32,
    /// Section flags
    pub sh_flags: Word,
    /// Section virtual addr at execution
    pub sh_addr: Word,
    /// Section file offset
    pub sh_offset: Word,
    /// Section size in bytes
    pub sh_size: Word,
    /// Link to another section
    pub sh_link: u32,
    /// Additional section information
    pub sh_info: u32,
    /// Section alignment
    pub sh_addralign: Word,
    /// Entry size if section holds table
    pub sh_entsize: Word,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default)]
/// A 64-bit ProgramHeader typically specifies how to map executable and data segments into memory
pub struct ProgramHeader {
    /// Segment type
    pub p_type: u32,
    /// Segment flags
    pub p_flags: u32,
    /// Segment file offset
    pub p_offset: u64,
    /// Segment virtual address
    pub p_vaddr: u64,
    /// Segment physical address
    pub p_paddr: u64,
    /// Segment size in file
    pub p_filesz: u64,
    /// Segment size in memory
    pub p_memsz: u64,
    /// Segment alignment
    pub p_align: u64,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default)]
/// A 32-bit ProgramHeader typically specifies how to map executable and data segments into memory
pub struct ProgramHeader {
    /// Segment type
    pub p_type: u32,
    /// Segment file offset
    pub p_offset: u32,
    /// Segment virtual address
    pub p_vaddr: u32,
    /// Segment physical address
    pub p_paddr: u32,
    /// Segment size in file
    pub p_filesz: u32,
    /// Segment size in memory
    pub p_memsz: u32,
    /// Segment flags
    pub p_flags: u32,
    /// Segment alignment
    pub p_align: u32,
}

#[inline]
pub fn st_type(info: u8) -> u8 {
    info & 0xf
}

#[inline]
pub fn st_bind(info: u8) -> u8 {
    info >> 4
}

#[inline]
pub fn st_visibility(other: u8) -> u8 {
    other & 0x7
}
