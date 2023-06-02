#![allow(clippy::single_match)]
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    pub rt_sigreturn: *const ::core::ffi::c_void,
    pub gettimeofday: *const ::core::ffi::c_void,
    pub clock_gettime: *const ::core::ffi::c_void,
    pub clock_getres: *const ::core::ffi::c_void,
}
impl Vdso {
    fn from_reader(reader: crate::VdsoReader) -> ::core::option::Option<Self> {
        unsafe {
            let mut version_mandatory_0 = 0u16;
            let mut vdso_inst = Self {
                rt_sigreturn: ::core::ptr::null(),
                gettimeofday: ::core::ptr::null(),
                clock_gettime: ::core::ptr::null(),
                clock_getres: ::core::ptr::null(),
            };
            {
                let mut mandatory_count = 0usize;
                for version in reader.versions() {
                    match version.hash() {
                        123718537 => {
                            if crate::util::streq(
                                version.name(),
                                [76, 73, 78, 85, 88, 95, 50, 46, 54, 46, 51, 57, 0u8][..].as_ptr(),
                            ) {
                                if version_mandatory_0 == 0 {
                                    mandatory_count += 1;
                                    version_mandatory_0 = version.id();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        _ => (),
                    }
                    if mandatory_count == 1 {
                        break;
                    }
                }
                if mandatory_count != 1 {
                    return ::core::option::Option::None;
                }
            }
            {
                let mut mandatory_count = 0usize;
                for symbol in reader.symbols() {
                    match crate::util::elf_hash(symbol.name()) {
                        28364627 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 99, 108, 111, 99,
                                    107, 95, 103, 101, 116, 114, 101, 115, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.clock_getres.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.clock_getres = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        128999177 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 103, 101, 116, 116,
                                    105, 109, 101, 111, 102, 100, 97, 121, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.gettimeofday.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.gettimeofday = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        165542654 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 114, 116, 95, 115,
                                    105, 103, 114, 101, 116, 117, 114, 110, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.rt_sigreturn.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.rt_sigreturn = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        185390885 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 99, 108, 111, 99,
                                    107, 95, 103, 101, 116, 116, 105, 109, 101, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.clock_gettime.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.clock_gettime = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        _ => (),
                    }
                    if mandatory_count == 4 {
                        break;
                    }
                }
                if mandatory_count != 4 {
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
