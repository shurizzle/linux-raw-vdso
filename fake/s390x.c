__asm__(".symver clock_getres,__kernel_clock_getres@@LINUX_2.6.29");
void clock_getres(void) {}
__asm__(".symver clock_gettime,__kernel_clock_gettime@@LINUX_2.6.29");
void clock_gettime(void) {}
__asm__(".symver gettimeofday,__kernel_gettimeofday@@LINUX_2.6.29");
void gettimeofday(void) {}
