KERNEL_VERSION:=${KERNEL_VERSION}
ARCHITECTURE:=x86

kernel:
	mkdir -p ./obj/linux-${KERNEL_VERSION} && \
		cp ./configs/linux-${KERNEL_VERSION}-${ARCHITECTURE}.config ./obj/linux-${KERNEL_VERSION}/.config && \
		cd linux-${KERNEL_VERSION} && \
		make O=../obj/linux-${KERNEL_VERSION} bzImage -j$(nproc)
	cp ./obj/linux-${KERNEL_VERSION}/arch/x86/boot/bzImage ./obj/kernel-${KERNEL_VERSION}

.PHONY: initramfs
initramfs:
	mkdir -p ./initramfs
	mkdir -pv ./initramfs/{bin,sbin,etc,proc,sys,usr/{bin,sbin}}
	cp ./busybox ./initramfs/bin/busybox
	chmod +x ./initramfs/bin/busybox
	cp ./init.sh ./initramfs/init
	cd ./initramfs; find . -print0 | cpio --null --create --verbose --format=newc | gzip -9 > ../obj/initramfs
	cp ./obj/initramfs /app/assets
