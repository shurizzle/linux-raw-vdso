#!/bin/bash

set -eux

make -C fake all clean

for target in \
	aarch64-unknown-linux-gnu \
	arm-unknown-linux-gnueabi \
	loongarch64-unknown-linux-gnu \
	mips-unknown-linux-gnu \
	mipsel-unknown-linux-gnu \
	mips64-unknown-linux-gnuabi64 \
	mips64el-unknown-linux-gnuabi64 \
	powerpc-unknown-linux-gnu \
	powerpc64-unknown-linux-gnu \
	powerpc64le-unknown-linux-gnu \
	riscv64gc-unknown-linux-gnu \
	s390x-unknown-linux-gnu \
	i686-unknown-linux-gnu \
	x86_64-unknown-linux-gnu; do
	cargo +nightly -Z build-std test --target "$target" --release
done
