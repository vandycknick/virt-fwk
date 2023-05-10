DISTRO:=ubuntu-22.04
KERNEL_VERSION:=6.1.26

.PHONY: build-kernel
build-kernel:
	@docker volume create kernel-build-cache
	@docker build -f tools/Dockerfile -t kernel-builder .
	@docker run -t --mount source=kernel-build-cache,target=/builder/obj -v $(shell pwd):/app kernel-builder kernel

.PHONY: build-initramfs
build-initramfs:
	@docker volume create kernel-build-cache
	@docker build -f tools/Dockerfile -t kernel-builder .
	@docker run -t --mount source=kernel-build-cache,target=/builder/obj -v $(shell pwd):/app kernel-builder initramfs

.PHONY: build-rootfs
build-rootfs:
	@docker volume create kernel-build-cache
	@docker build -f tools/rootfs/Dockerfile.${DISTRO} -t rootfs-${DISTRO} .
	@docker run -it --mount source=kernel-build-cache,target=/builder/obj -v $(shell pwd):/app  --privileged --cap-add=CAP_MKNOD rootfs-${DISTRO}

.PHONY: build-shell
build-shell:
	@docker run -it --mount source=kernel-build-cache,target=/builder/obj -v $(shell pwd):/app --entrypoint bash  kernel-builder

.PHONY: debug
debug:
	cargo build --bin simple-vm
	codesign -f --entitlement ./virt-fwk.entitlements -s - target/debug/simple-vm

.PHONY: simple-vm
simple-vm: debug
	@./target/debug/simple-vm --kernel $(shell pwd)/assets/kernel-${KERNEL_VERSION} --initrd $(shell pwd)/assets/initramfs --disk $(shell pwd)/assets/${DISTRO}.img
