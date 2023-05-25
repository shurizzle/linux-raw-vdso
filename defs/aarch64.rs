#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    pub rt_sigreturn:  __kernel_rt_sigreturn    @ "LINUX_2.6.39",
    pub gettimeofday:  __kernel_gettimeofday    @ "LINUX_2.6.39",
    pub clock_gettime: __kernel_clock_gettime   @ "LINUX_2.6.39",
    pub clock_getres:  __kernel_clock_getres    @ "LINUX_2.6.39",
}
