#![allow(clippy::single_match)]
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    pub clock_gettime: *const ::core::ffi::c_void,
    pub getcpu: *const ::core::ffi::c_void,
    pub gettimeofday: *const ::core::ffi::c_void,
    pub time: *const ::core::ffi::c_void,
}
impl Vdso {
    pub(crate) fn from_reader(reader: crate::VdsoReader) -> ::core::option::Option<Self> {
        unsafe {
            let mut version_mandatory_0 = 0u16;
            let mut vdso_inst = Self {
                clock_gettime: ::core::ptr::null(),
                getcpu: ::core::ptr::null(),
                gettimeofday: ::core::ptr::null(),
                time: ::core::ptr::null(),
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
                        11538501 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 118, 100, 115, 111, 95, 103, 101, 116, 99, 112, 117,
                                    0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.getcpu.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.getcpu = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
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
                        171164805 => {
                            if crate::util::streq(
                                symbol.name(),
                                [95, 95, 118, 100, 115, 111, 95, 116, 105, 109, 101, 0u8][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.time.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.time = symbol.ptr();
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
}
