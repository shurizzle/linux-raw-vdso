/// vDSO for `x86`
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    pub sigreturn:      __kernel_sigreturn      @ "LINUX_2.5",
    pub rt_sigreturn:   __kernel_rt_sigreturn   @ "LINUX_2.5",
    pub vsyscall:       __kernel_vsyscall       @ "LINUX_2.5",
    /// exported since Linux 3.15
    pub clock_gettime?: __vdso_clock_gettime    @ "LINUX_2.6",
    /// exported since Linux 3.15
    pub gettimeofday?:  __vdso_gettimeofday     @ "LINUX_2.6",
    /// exported since Linux 3.15
    pub time?:          __vdso_time             @ "LINUX_2.6",
}
