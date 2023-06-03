/// vDSO for `arm`
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    /// exported since Linux 4.1
    pub gettimeofday?:  __vdso_gettimeofday    @ "LINUX_2.6",
    /// exported since Linux 4.1
    pub clock_gettime?: __vdso_clock_gettime   @ "LINUX_2.6",
}
