__asm__(".symver rt_sigreturn,__kernel_rt_sigreturn@@LINUX_2.6.39");
void rt_sigreturn(void) {}
__asm__(".symver gettimeofday,__kernel_gettimeofday@@LINUX_2.6.39");
void gettimeofday(void) {}
__asm__(".symver clock_gettime,__kernel_clock_gettime@@LINUX_2.6.39");
void clock_gettime(void) {}
__asm__(".symver clock_getres,__kernel_clock_getres@@LINUX_2.6.39");
void clock_getres(void) {}
