__asm__(".symver gettimeofday,__vdso_gettimeofday@@LINUX_2.6");
void gettimeofday(void) {}
__asm__(".symver clock_gettime,__vdso_clock_gettime@@LINUX_2.6");
void clock_gettime(void) {}
__asm__(".symver clock_getres,__vdso_clock_getres@@LINUX_2.6");
void clock_getres(void) {}
__asm__(".symver clock_gettime64,__vdso_clock_gettime64@@LINUX_2.6");
void clock_gettime64(void) {}
