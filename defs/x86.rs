/// vDSO for `x86`
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    pub sigreturn:        __kernel_sigreturn      @ "LINUX_2.5",
    pub rt_sigreturn:     __kernel_rt_sigreturn   @ "LINUX_2.5",
    pub vsyscall:         __kernel_vsyscall       @ "LINUX_2.5",
    /// exported since Linux 3.15
    pub clock_gettime?:   __vdso_clock_gettime    @ "LINUX_2.6",
    /// exported since Linux 3.15
    pub gettimeofday?:    __vdso_gettimeofday     @ "LINUX_2.6",
    /// exported since Linux 3.15
    pub time?:            __vdso_time             @ "LINUX_2.6",
    /// exported since Linux ?
    pub clock_getres?:    __vdso_clock_getres     @ "LINUX_2.6",
    /// exported since Linux ?
    pub clock_gettime64?: __vdso_clock_gettime64  @ "LINUX_2.6",
    /// exported since Linux ?
    pub getcpu?:          __vdso_getcpu           @ "LINUX_2.6",
}
