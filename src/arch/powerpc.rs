use linux_vdso_macros::vdso;

vdso! {
    #[derive(Debug, Copy, Clone)]
    pub struct Vdso {
        pub clock_getres:    __kernel_clock_getres      @ "LINUX_2.6.15",
        pub clock_gettime:   __kernel_clock_gettime     @ "LINUX_2.6.15",
        pub clock_gettime64: __kernel_clock_gettime64   @ "LINUX_5.11",
        pub datapage_offset: __kernel_datapage_offset   @ "LINUX_2.6.15",
        pub get_syscall_map: __kernel_get_syscall_map   @ "LINUX_2.6.15",
        pub get_tbfreq:      __kernel_get_tbfreq        @ "LINUX_2.6.15",
        pub gettimeofday:    __kernel_gettimeofday      @ "LINUX_2.6.15",
        pub sigtramp_rt32:   __kernel_sigtramp_rt32     @ "LINUX_2.6.15",
        pub sigtramp32:      __kernel_sigtramp32        @ "LINUX_2.6.15",
        pub sync_dicache:    __kernel_sync_dicache      @ "LINUX_2.6.15",
        pub sync_dicache_p5: __kernel_sync_dicache_p5   @ "LINUX_2.6.15",
    }
}
