#!/bin/bash

set -eux

for target in \
	aarch64-unknown-linux-gnu \
	arm-unknown-linux-gnueabi \
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
	x86_64-unknown-linux-gnux32 \
	x86_64-unknown-linux-gnu; do
	cargo doc --target "$target"
done

for target in \
	loongarch64-unknown-linux-gnu \
	riscv32gc-unknown-linux-gnu \
	armeb-unknown-linux-gnueabi; do
	cargo +nightly -Z build-std=core doc --target "$target"
done
