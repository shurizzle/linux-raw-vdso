#!/bin/bash

SCRIPTPATH="$(
  cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
  pwd -P
)"

cd "${SCRIPTPATH}/.."

set -eux

output_and_exit_code() {
  set +e
  "$@"
  echo $?
  set -e
}

# cargo_test(toolchain, target, arch)
cargo_test() {
  rm -rf Cargo.lock target
  "${SCRIPTPATH}/docker-run.sh" "$1" "$3" \
    cargo test -vvv \
    --target "$2" \
    --release
}

# cargo_clippy(toolchain, arch, target)
cargo_clippy() {
  "${SCRIPTPATH}/docker-run.sh" "$1" "$2" \
    cargo -vvv clippy \
    --target "$3" \
    -- -D warnings
}

# test_nightly(target, arch)
test_nightly() {
  cargo_clippy nightly "$2" "$1"
  RUSTFLAGS="--cfg force_inline_syscalls" cargo_test nightly "$@"
  RUSTFLAGS="--cfg outline_syscalls" cargo_test nightly "$@"
}

# test_stable(target, arch, toolchain?)
test_stable() {
  cargo_clippy stable "$2" "$1"
  RUSTFLAGS="--cfg force_inline_syscalls" cargo_test stable "$@"
  RUSTFLAGS="--cfg outline_syscalls" cargo_test "${3:-1.38.0}" "$@"
}

# test_unstable(target, arch, toolchain?)
test_unstable() {
  cargo_clippy stable "$2" "$1"
  RUSTFLAGS="--cfg outline_syscalls" cargo_test stable "$@"
  RUSTFLAGS="--cfg outline_syscalls" cargo_test "${3:-1.38.0}" "$@"
}

test_x86_64() {
  local libc

  for libc in gnu musl; do
    test_stable "x86_64-unknown-linux-${libc}" x86_64
  done
  test_stable x86_64-linux-android x86_64
}

test_x86() {
  local arch libc

  for arch in i686 i586; do
    for libc in gnu musl; do
      test_stable "${arch}-unknown-linux-${libc}" x86
    done
  done
  test_stable i686-linux-android x86
}

test_arm() {
  local arch libc

  for arch in arm armv5te armv7; do
    for libc in gnu musl; do
      test_stable "${arch}-unknown-linux-${libc}eabi" arm
    done
  done

  for arch in arm armv7; do
    for libc in gnu musl; do
      test_stable "${arch}-unknown-linux-${libc}eabihf" arm
    done
  done

  test_stable thumbv7neon-unknown-linux-gnueabihf arm
  # test_stable thumbv7neon-unknown-linux-musleabihf arm
  # TODO:
  # skip 1.38.0 tests for arm-linux-androideabi since I got compilation errors
  # related to atomics implementation that I don't want to resolve
  cargo_clippy stable arm arm-linux-androideabi
  RUSTFLAGS="--cfg force_inline_syscalls" cargo_test stable arm-linux-androideabi arm
  test_stable armv7-linux-androideabi arm
  test_stable thumbv7neon-linux-androideabi arm
}

test_aarch64() {
  test_stable aarch64-unknown-linux-gnu aarch64
  test_stable aarch64-unknown-linux-musl aarch64
  test_stable aarch64-linux-android aarch64
}

test_riscv64() {
  test_stable riscv64gc-unknown-linux-gnu riscv64 1.42.0
  # test_stable riscv64gc-unknown-linux-musl riscv64 1.42.0
}

test_loongarch64() {
  test_nightly loongarch64-unknown-linux-gnu loongarch64
}

test_powerpc() {
  test_unstable powerpc-unknown-linux-gnu powerpc
  # test_nightly powerpc-unknown-linux-musl powerpc
}

test_powerpc64() {
  test_unstable powerpc64-unknown-linux-gnu powerpc64
  test_unstable powerpc64le-unknown-linux-gnu powerpc64
  # test_nightly powerpc64-unknown-linux-musl powerpc64
  # test_nightly powerpc64le-unknown-linux-musl powerpc64
}

test_mips() {
  local arch libc
  for arch in mips mipsel; do
    for libc in gnu musl; do
      test_unstable "${arch}-unknown-linux-${libc}" mips
    done
  done
}

test_mips64() {
  local arch libc
  for arch in mips64 mips64el; do
    for libc in gnu musl; do
      test_unstable "${arch}-unknown-linux-${libc}abi64" mips64 1.40.0
    done
  done
}

test_s390x() {
  test_unstable s390x-unknown-linux-gnu s390x
  # test_nightly s390x-unknown-linux-musl s390x
}

# TODO: riscv32

if [ $# -eq 0 ]; then
  set -- arm loongarch64 x86_64 x86 aarch64 riscv64 powerpc powerpc64 mips \
    mips64 s390x
fi

while [ $# -ne 0 ]; do
  "test_$1"
  shift
done
