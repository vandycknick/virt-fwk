FROM almalinux:9

ARG KERNEL_CHANNEL=v6.x
ARG KERNEL_VERSION=6.1.26
ARG BUSYBOX_VERSION=1.36.0

ENV KERNEL_CHANNEL=$KERNEL_CHANNEL
ENV KERNEL_VERSION=$KERNEL_VERSION
ENV BUSYBOX_VERSION=$BUSYBOX_VERSION

RUN dnf install -y epel-release && \
    dnf install -y 'dnf-command(config-manager)' && \
    dnf install -y dnf-plugins-core && \
    dnf config-manager -y --enable epel && \
    dnf config-manager -y --set-enabled crb && \
    dnf update -y && \
    dnf group install -y 'Development Tools' && \
    dnf install -y ncurses-devel \
        bison \
        flex \
        elfutils-libelf-devel \
        openssl-devel \
        bc \
        dwarves

WORKDIR /builder

RUN curl -L https://cdn.kernel.org/pub/linux/kernel/${KERNEL_CHANNEL}/linux-${KERNEL_VERSION}.tar.xz | tar xJf - && \
    curl -L https://busybox.net/downloads/binaries/1.28.1-defconfig-multiarch/busybox-x86_64 -o busybox

COPY ./tools .

ENTRYPOINT ["make"]
