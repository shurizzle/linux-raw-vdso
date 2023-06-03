__asm__(".symver getcpu,__vdso_getcpu@@LINUX_5.10");
void getcpu(void) {}
__asm__(".symver clock_getres,__vdso_clock_getres@@LINUX_5.10");
void clock_getres(void) {}
__asm__(".symver clock_gettime,__vdso_clock_gettime@@LINUX_5.10");
void clock_gettime(void) {}
__asm__(".symver gettimeofday,__vdso_gettimeofday@@LINUX_5.10");
void gettimeofday(void) {}
__asm__(".symver rt_sigreturn,__vdso_rt_sigreturn@@LINUX_5.10");
void rt_sigreturn(void) {}
