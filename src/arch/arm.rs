#![allow(clippy::single_match)]
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    #[doc = " exported since Linux 4.1"]
    pub gettimeofday: *const ::core::ffi::c_void,
    #[doc = " exported since Linux 4.1"]
    pub clock_gettime: *const ::core::ffi::c_void,
}
impl Vdso {
    fn from_reader(reader: crate::VdsoReader) -> ::core::option::Option<Self> {
        unsafe {
            let mut version_optional_0 = 0u16;
            let mut vdso_inst = Self {
                gettimeofday: ::core::ptr::null(),
                clock_gettime: ::core::ptr::null(),
            };
            {
                let mut mandatory_count = 0usize;
                for version in reader.versions() {
                    match version.hash() {
                        61765110 => {
                            if crate::util::streq(
                                version.name(),
                                [76, 73, 78, 85, 88, 95, 50, 46, 54, 0u8][..].as_ptr(),
                            ) {
                                if version_optional_0 != 0 {
                                    return ::core::option::Option::None;
                                }
                                version_optional_0 = version.id();
                            }
                        }
                        _ => (),
                    }
                }
                if mandatory_count != 0 {
                    return ::core::option::Option::None;
                }
            }
            {
                let mut mandatory_count = 0usize;
                for symbol in reader.symbols() {
                    match crate::util::elf_hash(symbol.name()) {
                        51759705 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 118, 100, 115, 111, 95, 103, 101, 116, 116, 105, 109,
                                    101, 111, 102, 100, 97, 121, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_optional_0) == symbol.version_id()
                            {
                                if !vdso_inst.gettimeofday.is_null() {
                                    return ::core::option::Option::None;
                                }
                                vdso_inst.gettimeofday = symbol.ptr();
                            }
                        }
                        221637749 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 118, 100, 115, 111, 95, 99, 108, 111, 99, 107, 95, 103,
                                    101, 116, 116, 105, 109, 101, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_optional_0) == symbol.version_id()
                            {
                                if !vdso_inst.clock_gettime.is_null() {
                                    return ::core::option::Option::None;
                                }
                                vdso_inst.clock_gettime = symbol.ptr();
                            }
                        }
                        _ => (),
                    }
                }
                if mandatory_count != 0 {
                    return ::core::option::Option::None;
                }
            }
            Some(vdso_inst)
        }
    }
    pub unsafe fn from_ptr(ptr: *const ::core::ffi::c_void) -> ::core::option::Option<Self> {
        Self::from_reader(crate::VdsoReader::from_ptr(ptr)?)
    }
}
