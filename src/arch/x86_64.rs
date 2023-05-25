use linux_vdso_macros::vdso;

vdso! {
    #[derive(Debug, Copy, Clone)]
    pub struct Vdso {
        pub clock_gettime: __vdso_clock_gettime @ "LINUX_2.6",
        pub getcpu:        __vdso_getcpu        @ "LINUX_2.6",
        pub gettimeofday:  __vdso_gettimeofday  @ "LINUX_2.6",
        pub time:          __vdso_time          @ "LINUX_2.6",
    }
}
