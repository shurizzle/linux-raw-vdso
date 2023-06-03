__asm__(".symver rt_sigreturn,__vdso_rt_sigreturn@@LINUX_4.15");
void rt_sigreturn(void) {}
__asm__(".symver gettimeofday,__vdso_gettimeofday@@LINUX_4.15");
void gettimeofday(void) {}
__asm__(".symver clock_gettime,__vdso_clock_gettime@@LINUX_4.15");
void clock_gettime(void) {}
__asm__(".symver clock_getres,__vdso_clock_getres@@LINUX_4.15");
void clock_getres(void) {}
__asm__(".symver getcpu,__vdso_getcpu@@LINUX_4.15");
void getcpu(void) {}
__asm__(".symver flush_icache,__vdso_flush_icache@@LINUX_4.15");
void flush_icache(void) {}
