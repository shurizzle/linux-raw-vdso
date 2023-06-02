#![allow(clippy::single_match)]
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    pub clock_getres: *const ::core::ffi::c_void,
    pub clock_gettime: *const ::core::ffi::c_void,
    pub clock_gettime64: *const ::core::ffi::c_void,
    pub datapage_offset: *const ::core::ffi::c_void,
    pub get_syscall_map: *const ::core::ffi::c_void,
    pub get_tbfreq: *const ::core::ffi::c_void,
    pub getcpu: *const ::core::ffi::c_void,
    pub gettimeofday: *const ::core::ffi::c_void,
    pub sigtramp_rt32: *const ::core::ffi::c_void,
    pub sigtramp32: *const ::core::ffi::c_void,
    pub sync_dicache: *const ::core::ffi::c_void,
    pub sync_dicache_p5: *const ::core::ffi::c_void,
}
impl Vdso {
    fn from_reader(reader: crate::VdsoReader) -> ::core::option::Option<Self> {
        unsafe {
            let mut version_mandatory_0 = 0u16;
            let mut version_mandatory_1 = 0u16;
            let mut vdso_inst = Self {
                clock_getres: ::core::ptr::null(),
                clock_gettime: ::core::ptr::null(),
                clock_gettime64: ::core::ptr::null(),
                datapage_offset: ::core::ptr::null(),
                get_syscall_map: ::core::ptr::null(),
                get_tbfreq: ::core::ptr::null(),
                getcpu: ::core::ptr::null(),
                gettimeofday: ::core::ptr::null(),
                sigtramp_rt32: ::core::ptr::null(),
                sigtramp32: ::core::ptr::null(),
                sync_dicache: ::core::ptr::null(),
                sync_dicache_p5: ::core::ptr::null(),
            };
            {
                let mut mandatory_count = 0usize;
                for version in reader.versions() {
                    match version.hash() {
                        123718565 => {
                            if crate::util::streq(
                                version.name(),
                                [76, 73, 78, 85, 88, 95, 50, 46, 54, 46, 49, 53, 0u8][..].as_ptr(),
                            ) {
                                if version_mandatory_0 == 0 {
                                    mandatory_count += 1;
                                    version_mandatory_0 = version.id();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        182947697 => {
                            if crate::util::streq(
                                version.name(),
                                [76, 73, 78, 85, 88, 95, 53, 46, 49, 49, 0u8][..].as_ptr(),
                            ) {
                                if version_mandatory_1 == 0 {
                                    mandatory_count += 1;
                                    version_mandatory_1 = version.id();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        _ => (),
                    }
                    if mandatory_count == 2 {
                        break;
                    }
                }
                if mandatory_count != 2 {
                    return ::core::option::Option::None;
                }
            }
            {
                let mut mandatory_count = 0usize;
                for symbol in reader.symbols() {
                    match crate::util::elf_hash(symbol.name()) {
                        18059221 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 115, 121, 110, 99,
                                    95, 100, 105, 99, 97, 99, 104, 101, 95, 112, 53, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.sync_dicache_p5.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.sync_dicache_p5 = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        21682034 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 115, 105, 103, 116,
                                    114, 97, 109, 112, 95, 114, 116, 51, 50, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.sigtramp_rt32.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.sigtramp_rt32 = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
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
                        67179938 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 115, 105, 103, 116,
                                    114, 97, 109, 112, 51, 50, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.sigtramp32.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.sigtramp32 = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        68519413 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 103, 101, 116, 99,
                                    112, 117, 0u8,
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
                        153931009 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 103, 101, 116, 95,
                                    116, 98, 102, 114, 101, 113, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.get_tbfreq.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.get_tbfreq = symbol.ptr();
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
                        214117184 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 103, 101, 116, 95,
                                    115, 121, 115, 99, 97, 108, 108, 95, 109, 97, 112, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.get_syscall_map.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.get_syscall_map = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        215425940 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 99, 108, 111, 99,
                                    107, 95, 103, 101, 116, 116, 105, 109, 101, 54, 52, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_1) == symbol.version_id()
                            {
                                if vdso_inst.clock_gettime64.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.clock_gettime64 = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        228462901 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 115, 121, 110, 99,
                                    95, 100, 105, 99, 97, 99, 104, 101, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.sync_dicache.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.sync_dicache = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        237167108 => {
                            if crate::util::streq(
                                symbol.name(),
                                [
                                    95, 95, 107, 101, 114, 110, 101, 108, 95, 100, 97, 116, 97,
                                    112, 97, 103, 101, 95, 111, 102, 102, 115, 101, 116, 0u8,
                                ][..]
                                    .as_ptr(),
                            ) && Some(version_mandatory_0) == symbol.version_id()
                            {
                                if vdso_inst.datapage_offset.is_null() {
                                    mandatory_count += 1;
                                    vdso_inst.datapage_offset = symbol.ptr();
                                } else {
                                    return ::core::option::Option::None;
                                }
                            }
                        }
                        _ => (),
                    }
                    if mandatory_count == 12 {
                        break;
                    }
                }
                if mandatory_count != 12 {
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
