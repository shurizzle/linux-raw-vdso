# linux-vdso-raw

![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/shurizzle/linux-raw-vdso/unit-test.yml?branch=master&style=for-the-badge)
[![Crates.io](https://img.shields.io/crates/v/linux-raw-vdso?style=for-the-badge)](https://crates.io/crates/linux-raw-vdso)
[![docs.rs](https://img.shields.io/docsrs/linux-raw-vdso?style=for-the-badge)](https://docs.rs/linux-raw-vdso)
![Crates.io](https://img.shields.io/crates/l/linux-raw-vdso?style=for-the-badge)

Cross-arch implementation of vDSO parsing.
For more informations about vDSO and why it's important in Linux you can see
[here](https://man7.org/linux/man-pages/man7/vdso.7.html).

### #![no_std]

This library is entirely `no_std`.

### Platforms

|    arch     | implemented | tested |
|-------------|-------------|--------|
| aarch64     |      ✅     |   ✅   |
| aarch64_be  |      ✅     |   ❌   |
| arm         |      ✅     |   ✅   |
| armeb       |      ✅     |   ❌   |
| loongarch64 |      ✅     |   ✅   |
| mips        |      ✅     |   ✅   |
| mipsel      |      ✅     |   ✅   |
| mips64      |      ✅     |   ✅   |
| mips64el    |      ✅     |   ✅   |
| mips64n32   |      ✅     |   ❌   |
| mips64n32el |      ✅     |   ❌   |
| powerpc     |      ✅     |   ✅   |
| powerpc64   |      ✅     |   ✅   |
| powerpc64le |      ✅     |   ✅   |
| riscv32     |      ✅     |   ❌   |
| riscv64     |      ✅     |   ✅   |
| s390x       |      ✅     |   ✅   |
| x86_64      |      ✅     |   ✅   |
| x86         |      ✅     |   ✅   |
| x32         |      ✅     |   ❌   |

# Code generation

All the code in `src/arch` is generated by the inner crate in the `vdso-gen`
directory so don't touch auto generated files please.
If you need to add or change something look in `defs` directory instead.

### MSRV
1.38.0
