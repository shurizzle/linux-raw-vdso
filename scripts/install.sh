#!/bin/bash

set -eux

cd

set_env() {
	local triple="$1" \
		prefix="$2" \
		runner="$3" \
		lcase_triple \
		ucase_triple

	lcase_triple="$(printf %s "$triple" | tr '-' '_')"
	ucase_triple="$(printf %s "$triple" | tr 'a-z-' 'A-Z_')"

	(
		echo "CC_${lcase_triple}=\"${prefix}gcc\""
		echo "CXX_${lcase_triple}=\"${prefix}g++\""
		echo "AR_${lcase_triple}=\"${prefix}ar\""
		echo "CARGO_TARGET_${ucase_triple}_LINKER=\"${prefix}gcc\""
		echo "CARGO_TARGET_${ucase_triple}_RUNNER=\"$runner\""
	) >>"$ENVFILE"
}

remove_wget=false
if ! which wget 2>/dev/null >/dev/null; then
	remove_wget=true
fi
apt-get update -y
apt-get upgrade -y
apt-get install -y build-essential wget qemu-user gcc-x86-64-linux-gnu \
	gcc-i686-linux-gnu gcc-arm-linux-gnueabi gcc-aarch64-linux-gnu \
	gcc-riscv64-linux-gnu gcc-powerpc64-linux-gnu gcc-powerpc64le-linux-gnu \
	gcc-powerpc-linux-gnu gcc-mips-linux-gnu gcc-mipsel-linux-gnu \
	gcc-mips64el-linux-gnuabi64 gcc-mips64-linux-gnuabi64 gcc-s390x-linux-gnu

if ! which rustup 2>/dev/null >/dev/null; then
	RUSTUP_HOME="/opt/rust"
	export RUSTUP_HOME
	CARGO_HOME="/opt/rust"
	export CARGO_HOME
	cd
	wget https://sh.rustup.rs -O rustup-init
	chmod +x rustup-init
	./rustup-init -y --no-modify-path
	PATH="$PATH:/opt/rust/bin"
	(
		echo 'RUSTUP_HOME="/opt/rust"'
		echo 'PATH="$PATH:/opt/rust/bin"'
	) >>"$ENVFILE"

	rm -f rustup-init
fi
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

# install loongarch64 toolchain
wget https://github.com/loongson/build-tools/releases/download/2022.09.06/loongarch64-clfs-6.3-cross-tools-gcc-glibc.tar.xz
tar xf loongarch64-clfs-6.3-cross-tools-gcc-glibc.tar.xz
mkdir -p /opt
mv cross-tools /opt/loongarch64-unknown-linux-gnu
rm -f loongarch64-clfs-6.3-cross-tools-gcc-glibc.tar.xz
for tool in addr2line ar as c++ c++filt cpp elfedit g++ gcc gcc-ar gcc-nm \
	gcc-ranlib gcov gcov-dump gcov-tool gprof ld ld.bfd lto-dump nm objcopy \
	objdump ranlib readelf size strings strip; do
	ln -s "/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-$tool" "/bin/loongarch64-linux-gnu-$tool"
done

# install loongarch64 qemu
wget https://github.com/loongson/build-tools/releases/download/2022.09.06/qemu-loongarch64
chmod +x qemu-loongarch64
mv qemu-loongarch64 /bin/qemu-loongarch64

# setup loongarch64
triple="loongarch64-unknown-linux-gnu"
rustup target add loongarch64-unknown-linux-gnu --toolchain nightly

set_env "$triple" \
	/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu- \
	"env LD_LIBRARY_PATH=/lib64 qemu-loongarch64 -L /opt/loongarch64-unknown-linux-gnu/target/usr"

# setup i686
triple="i686-unknown-linux-gnu"
rustup target add "$triple"
set_env "$triple" \
	"i686-linux-gnu-" \
	"qemu-i386 -L /usr/i686-linux-gnu"

# setup arm
triple="arm-unknown-linux-gnueabi"
rustup target add "$triple"
set_env "$triple" \
	"arm-linux-gnueabi-" \
	"qemu-arm -L /usr/arm-linux-gnueabi"

# setup aarch64
triple="aarch64-unknown-linux-gnu"
rustup target add "$triple"
set_env "$triple" \
	"aarch64-linux-gnu-" \
	"qemu-aarch64 -L /usr/aarch64-linux-gnu"

# setup riscv64
triple="riscv64gc-unknown-linux-gnu"
rustup target add "$triple"
set_env "$triple" \
	"riscv64-linux-gnu-" \
	"qemu-riscv64 -L /usr/riscv64-linux-gnu"

# setup powerpc
triple="powerpc-unknown-linux-gnu"
rustup target add "$triple"
set_env "$triple" \
	"powerpc-linux-gnu-" \
	"qemu-ppc -L /usr/powerpc-linux-gnu"

# setup powerpc64
triple="powerpc64-unknown-linux-gnu"
rustup target add "$triple"
set_env "$triple" \
	"powerpc64-linux-gnu-" \
	"qemu-ppc64 -L /usr/powerpc64-linux-gnu"

# setup powerpc64le
triple="powerpc64le-unknown-linux-gnu"
rustup target add "$triple"
set_env "$triple" \
	"powerpc64le-linux-gnu-" \
	"qemu-ppc64le -L /usr/powerpc64le-linux-gnu"

# setup mips
triple="mips-unknown-linux-gnu"
rustup target add "$triple"
set_env "$triple" \
	"mips-linux-gnu-" \
	"qemu-mips -L /usr/mips-linux-gnu"

# setup mipsel
triple="mipsel-unknown-linux-gnu"
rustup target add "$triple"
set_env "$triple" \
	"mipsel-linux-gnu-" \
	"qemu-mipsel -L /usr/mipsel-linux-gnu"

# setup mips64
triple="mips64-unknown-linux-gnuabi64"
rustup target add "$triple"
set_env "$triple" \
	"mips64-linux-gnuabi64-" \
	"qemu-mips64 -L /usr/mips64-linux-gnuabi64"

# setup mips64el
triple="mips64el-unknown-linux-gnuabi64"
rustup target add "$triple"
set_env "$triple" \
	"mips64el-linux-gnuabi64-" \
	"qemu-mips64el -L /usr/mips64el-linux-gnuabi64"

# setup s390x
triple="s390x-unknown-linux-gnu"
rustup target add "$triple"
set_env "$triple" \
	"s390x-linux-gnu-" \
	"qemu-s390x -L /usr/s390x-linux-gnu"

if $remove_wget; then
	apt-get -y purge wget
fi
apt-get -y autoremove
