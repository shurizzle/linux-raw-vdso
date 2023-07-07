#![allow(clippy::single_match)]
#[doc = " vDSO for `x86`"]
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    pub sigreturn: *const ::core::ffi::c_void,
    pub rt_sigreturn: *const ::core::ffi::c_void,
    pub vsyscall: *const ::core::ffi::c_void,
    #[doc = " exported since Linux 3.15"]
    pub clock_gettime: *const ::core::ffi::c_void,
    #[doc = " exported since Linux 3.15"]
    pub gettimeofday: *const ::core::ffi::c_void,
    #[doc = " exported since Linux 3.15"]
    pub time: *const ::core::ffi::c_void,
    #[doc = " exported since Linux ?"]
    pub clock_getres: *const ::core::ffi::c_void,
    #[doc = " exported since Linux ?"]
    pub clock_gettime64: *const ::core::ffi::c_void,
    #[doc = " exported since Linux ?"]
    pub getcpu: *const ::core::ffi::c_void,
}
impl Vdso {
    fn from_reader(reader: crate::VdsoReader) -> ::core::option::Option<Self> {
        unsafe {
            let mut version_mandatory_0 = 0u16;
            let mut version_optional_0 = 0u16;
            let mut vdso_inst = Self {
                sigreturn: ::core::ptr::null(),
                rt_sigreturn: ::core::ptr::null(),
                vsyscall: ::core::ptr::null(),
                clock_gettime: ::core::ptr::null(),
                gettimeofday: ::core::ptr::null(),
                time: ::core::ptr::null(),
                clock_getres: ::core::ptr::null(),
                clock_gettime64: ::core::ptr::null(),
                getcpu: ::core::ptr::null(),
            };
            {
                let mut mandatory_count = 0usize;
                for version in reader.versions() {
                    match version.hash() {
                        61765109 => {
                            if crate::util::streq(
                                version.name(),
                                [76, 73, 78, 85, 88, 95, 50, 46, 53, 0u8][..].as_ptr(),
                            ) {
                                if version_mandatory_0 == 0 {
                                    mandatory_count += 1;
                                    version_mandatory_0 = version.id();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
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
                if mandatory_count != 1 {
                    return ::core::option::Option::None;
                }
            }
            {
                let mut mandatory_count = 0usize;
                for symbol in reader.symbols() {
                    match crate::util::elf_hash(symbol.name()) {
                        11538501 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 118, 100, 115, 111, 95, 103, 101, 116, 99, 112, 117,
                                    0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_optional_0) == symbol.version_id()
                            {
                                if !vdso_inst.getcpu.is_null() {
                                    return ::core::option::Option::None;
                                }
                                vdso_inst.getcpu = symbol.ptr();
                            }
                        }
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
                        69603644 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 118, 115, 121, 115,
                                    99, 97, 108, 108, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.vsyscall.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.vsyscall = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        80960643 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 118, 100, 115, 111, 95, 99, 108, 111, 99, 107, 95, 103,
                                    101, 116, 114, 101, 115, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_optional_0) == symbol.version_id()
                            {
                                if !vdso_inst.clock_getres.is_null() {
                                    return ::core::option::Option::None;
                                }
                                vdso_inst.clock_getres = symbol.ptr();
                            }
                        }
                        99382692 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 118, 100, 115, 111, 95, 99, 108, 111, 99, 107, 95, 103,
                                    101, 116, 116, 105, 109, 101, 54, 52, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_optional_0) == symbol.version_id()
                            {
                                if !vdso_inst.clock_gettime64.is_null() {
                                    return ::core::option::Option::None;
                                }
                                vdso_inst.clock_gettime64 = symbol.ptr();
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
                        171164805 => {
                            if crate::util::streq(
                                symbol.name(),
                                [95, 95, 118, 100, 115, 111, 95, 116, 105, 109, 101, 0u8][..]
                                    .as_ptr(),
                            ) && Some(version_optional_0) == symbol.version_id()
                            {
                                if !vdso_inst.time.is_null() {
                                    return ::core::option::Option::None;
                                }
                                vdso_inst.time = symbol.ptr();
                            }
                        }
                        208980462 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 115, 105, 103, 114,
                                    101, 116, 117, 114, 110, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.sigreturn.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.sigreturn = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
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
                if mandatory_count != 3 {
                    return ::core::option::Option::None;
                }
            }
            Some(vdso_inst)
        }
    }
    #[doc = r" Parse vDSO from memory"]
    #[doc = r" # Safety"]
    #[doc = r" This is unsafe because we can't validate the given pointer so"]
    #[doc = r" use it carefully"]
    pub unsafe fn from_ptr(ptr: *const ::core::ffi::c_void) -> ::core::option::Option<Self> {
        Self::from_reader(crate::VdsoReader::from_ptr(ptr)?)
    }
}
