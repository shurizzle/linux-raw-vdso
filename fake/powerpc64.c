__asm__(".symver clock_getres,__kernel_clock_getres@@LINUX_2.6.15");
void clock_getres(void) {}
__asm__(".symver clock_gettime,__kernel_clock_gettime@@LINUX_2.6.15");
void clock_gettime(void) {}
__asm__(".symver clock_gettime64,__kernel_clock_gettime64@@LINUX_5.11");
void clock_gettime64(void) {}
__asm__(".symver datapage_offset,__kernel_datapage_offset@@LINUX_2.6.15");
void datapage_offset(void) {}
__asm__(".symver get_syscall_map,__kernel_get_syscall_map@@LINUX_2.6.15");
void get_syscall_map(void) {}
__asm__(".symver get_tbfreq,__kernel_get_tbfreq@@LINUX_2.6.15");
void get_tbfreq(void) {}
__asm__(".symver getcpu,__kernel_getcpu@@LINUX_2.6.15");
void getcpu(void) {}
__asm__(".symver gettimeofday,__kernel_gettimeofday@@LINUX_2.6.15");
void gettimeofday(void) {}
__asm__(".symver sigtramp_rt32,__kernel_sigtramp_rt32@@LINUX_2.6.15");
void sigtramp_rt32(void) {}
__asm__(".symver sigtramp32,__kernel_sigtramp32@@LINUX_2.6.15");
void sigtramp32(void) {}
__asm__(".symver sync_dicache,__kernel_sync_dicache@@LINUX_2.6.15");
void sync_dicache(void) {}
__asm__(".symver sync_dicache_p5,__kernel_sync_dicache_p5@@LINUX_2.6.15");
void sync_dicache_p5(void) {}
