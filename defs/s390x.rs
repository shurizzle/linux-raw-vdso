/// vDSO for `s390x`
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    pub clock_getres:  __kernel_clock_getres    @ "LINUX_2.6.29",
    pub clock_gettime: __kernel_clock_gettime   @ "LINUX_2.6.29",
    pub gettimeofday:  __kernel_gettimeofday    @ "LINUX_2.6.29",
}
