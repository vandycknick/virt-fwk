#!/bin/busybox sh

/bin/busybox --install -s /bin
/bin/busybox --install -s /sbin

mount -t devtmpfs  devtmpfs  /dev
mount -t proc      proc      /proc
mount -t sysfs     sysfs     /sys
mount -t tmpfs     tmpfs     /tmp

# Create device nodes
# mknod /dev/null c 1 3
# mknod /dev/tty c 5 0
mdev -s

#Function for parsing command line options with "=" in them
# get_opt("init=/sbin/init") will return "/sbin/init"
get_opt() {
    echo "$@" | cut -d "=" -f 2
}

init="/sbin/init"
root="/dev/sda"

#Process command line options
for i in $(cat /proc/cmdline); do
    case $i in
        root\=*)
            root=$(get_opt $i)
            ;;
        init\=*)
            init=$(get_opt $i)
            ;;
    esac
done

# Mount the root device
mkdir -p /mnt/root
mount "${root}" /mnt/root

# Unmount all other mounts so that the ram used by
# the initramfs can be cleared after switch_root
umount /proc
umount /sys

# Switch to the new root and execute init
exec switch_root /mnt/root "${init}"

# This will only be run if the exec above failed
echo "Failed to switch_root, dropping to a shell"
exec sh
