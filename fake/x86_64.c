__asm__(".symver clock_gettime,__vdso_clock_gettime@@LINUX_2.6");
void clock_gettime(void) {}
__asm__(".symver getcpu,__vdso_getcpu@@LINUX_2.6");
void getcpu(void) {}
__asm__(".symver gettimeofday,__vdso_gettimeofday@@LINUX_2.6");
void gettimeofday(void) {}
__asm__(".symver time,__vdso_time@@LINUX_2.6");
void time(void) {}
