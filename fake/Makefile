all: aarch64.so arm.so loongarch64.so mips.so mipsel.so mips64.so mips64el.so powerpc.so powerpc64.so powerpc64le.so x86_64.so riscv64.so s390x.so x86.so clean

aarch64.so:
	aarch64-linux-gnu-gcc -c -fPIC -o aarch64.o aarch64.c
	aarch64-linux-gnu-gcc -shared -o aarch64.so aarch64.o -Wl,--version-script,aarch64.map

arm.so:
	arm-linux-gnueabi-gcc -c -fPIC -o arm.o arm.c
	arm-linux-gnueabi-gcc -shared -o arm.so arm.o -Wl,--version-script,arm.map

loongarch64.so:
	loongarch64-linux-gnu-gcc -c -fPIC -o loongarch64.o loongarch64.c
	loongarch64-linux-gnu-gcc -shared -o loongarch64.so loongarch64.o -Wl,--version-script,loongarch64.map

mips.so:
	mips-linux-gnu-gcc -c -fPIC -o mips.o mips.c
	mips-linux-gnu-gcc -shared -o mips.so mips.o -Wl,--version-script,mips.map

mipsel.so:
	mipsel-linux-gnu-gcc -c -fPIC -o mipsel.o mips.c
	mipsel-linux-gnu-gcc -shared -o mipsel.so mipsel.o -Wl,--version-script,mips.map

mips64.so:
	mips64-linux-gnuabi64-gcc -c -fPIC -o mips64.o mips.c
	mips64-linux-gnuabi64-gcc -shared -o mips64.so mips64.o -Wl,--version-script,mips.map

mips64el.so:
	mips64el-linux-gnuabi64-gcc -c -fPIC -o mips64el.o mips.c
	mips64el-linux-gnuabi64-gcc -shared -o mips64el.so mips64el.o -Wl,--version-script,mips.map

powerpc.so:
	powerpc-linux-gnu-gcc -c -fPIC -o powerpc.o powerpc.c
	powerpc-linux-gnu-gcc -shared -o powerpc.so powerpc.o -Wl,--version-script,powerpc.map

powerpc64.so:
	powerpc64-linux-gnu-gcc -c -fPIC -o powerpc64.o powerpc64.c
	powerpc64-linux-gnu-gcc -shared -o powerpc64.so powerpc64.o -Wl,--version-script,powerpc64.map

powerpc64le.so:
	powerpc64le-linux-gnu-gcc -c -fPIC -o powerpc64le.o powerpc64.c
	powerpc64le-linux-gnu-gcc -shared -o powerpc64le.so powerpc64le.o -Wl,--version-script,powerpc64.map

x86_64.so:
	# x86-64-linux-gnu-gcc -c -fPIC -o x86_64.o x86_64.c
	# x86-64-linux-gnu-gcc -shared -o x86_64.so x86_64.o -Wl,--version-script,x86_64.map
	gcc -c -fPIC -o x86_64.o x86_64.c
	gcc -shared -o x86_64.so x86_64.o -Wl,--version-script,x86_64.map

# riscv32.so:
# 	riscv32-linux-gnu-gcc -c -fPIC -o riscv32.o riscv.c
# 	riscv32-linux-gnu-gcc -shared -o riscv32.so riscv32.o -Wl,--version-script,riscv.map

riscv64.so:
	riscv64-linux-gnu-gcc -c -fPIC -o riscv64.o riscv.c
	riscv64-linux-gnu-gcc -shared -o riscv64.so riscv64.o -Wl,--version-script,riscv.map

x86.so:
	i686-linux-gnu-gcc -c -fPIC -o x86.o x86.c
	i686-linux-gnu-gcc -shared -o x86.so x86.o -Wl,--version-script,x86.map

# x32.so:
# 	x86-64-linux-gnux32-gcc -c -fPIC -o x32.o x32.c
# 	x86-64-linux-gnux32-gcc -shared -o x32.so x32.o -Wl,--version-script,x32.map

s390x.so:
	s390x-linux-gnu-gcc -c -fPIC -o s390x.o s390x.c
	s390x-linux-gnu-gcc -shared -o s390x.so s390x.o -Wl,--version-script,s390x.map

clean:
	rm -rf *.o
