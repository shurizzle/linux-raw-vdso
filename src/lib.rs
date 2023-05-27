#![no_std]

#[cfg(all(
    target_os = "linux",
    any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "powerpc",
        target_arch = "powerpc64",
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "s390x",
    )
))]
mod imp;
#[cfg(all(
    target_os = "linux",
    any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "powerpc",
        target_arch = "powerpc64",
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "s390x",
    )
))]
pub use imp::*;
