use linux_vdso_macros::vdso;

vdso! {
    #[derive(Debug, Copy, Clone)]
    pub struct Vdso {
        pub rt_sigreturn:  __vdso_rt_sigreturn    @ "LINUX_4.15",
        pub gettimeofday:  __vdso_gettimeofday    @ "LINUX_4.15",
        pub clock_gettime: __vdso_clock_gettime   @ "LINUX_4.15",
        pub clock_getres:  __vdso_clock_getres    @ "LINUX_4.15",
        pub getcpu:        __vdso_getcpu          @ "LINUX_4.15",
        pub flush_icache:  __vdso_flush_icache    @ "LINUX_4.15",
    }
}
