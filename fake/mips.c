__asm__(".symver gettimeofday,__kernel_gettimeofday@@LINUX_2.6");
void gettimeofday(void) {}
__asm__(".symver clock_gettime,__kernel_clock_gettime@@LINUX_2.6");
void clock_gettime(void) {}
