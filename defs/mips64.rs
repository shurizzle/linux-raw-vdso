/// vDSO for `mips64`
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    /// exported since Linux 4.4
    pub gettimeofday?:  __kernel_gettimeofday    @ "LINUX_2.6",
    /// exported since Linux 4.4
    pub clock_gettime?: __kernel_clock_gettime   @ "LINUX_2.6",
    /// exported since Linux ?
    pub clock_getres?:  __kernel_clock_getres    @ "LINUX_2.6",
}
