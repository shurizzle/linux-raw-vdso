__asm__(".symver sigreturn,__kernel_sigreturn@@LINUX_2.5");
void sigreturn(void) {}
__asm__(".symver rt_sigreturn,__kernel_rt_sigreturn@@LINUX_2.5");
void rt_sigreturn(void) {}
__asm__(".symver vsyscall,__kernel_vsyscall@@LINUX_2.5");
void vsyscall(void) {}
__asm__(".symver clock_gettime,__vdso_clock_gettime@@LINUX_2.6");
void clock_gettime(void) {}
__asm__(".symver gettimeofday,__vdso_gettimeofday@@LINUX_2.6");
void gettimeofday(void) {}
__asm__(".symver time,__vdso_time@@LINUX_2.6");
void time(void) {}
