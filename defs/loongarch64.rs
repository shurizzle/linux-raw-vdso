/// vDSO for `loongarch64`
#[derive(Debug, Copy, Clone)]
pub struct Vdso {
    pub getcpu:         __vdso_getcpu           @ "LINUX_5.10",
    pub clock_getres:   __vdso_clock_getres     @ "LINUX_5.10",
    pub clock_gettime:  __vdso_clock_gettime    @ "LINUX_5.10",
    pub gettimeofday:   __vdso_gettimeofday     @ "LINUX_5.10",
    pub rt_sigreturn:   __vdso_rt_sigreturn     @ "LINUX_5.10",
}
