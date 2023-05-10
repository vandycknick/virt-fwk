#! /usr/bin/env bash

BUILDER_OBJ=/builder/obj
ROOTFS_MNT=/builder/rootfs

[[ -e "$BUILDER_OBJ/$DISTRO.img" ]] && rm "$BUILDER_OBJ/$DISTRO.img"

dd if=/dev/zero of="$BUILDER_OBJ/$DISTRO.img" bs=1M count=800

mkfs.ext4 "$BUILDER_OBJ/$DISTRO.img"

mkdir -p "$ROOTFS_MNT"
mount "$BUILDER_OBJ/$DISTRO.img" "$ROOTFS_MNT"

for d in bin etc home lib lib64 opt root sbin usr var; do tar c "/$d" | tar x -C "$ROOTFS_MNT"; done
for dir in run proc sys; do mkdir "$ROOTFS_MNT/${dir}"; done

# Override the nameserver to Cloudflare for simplicity
cat <<EOF > "$ROOTFS_MNT/etc/resolv.conf"
nameserver 1.1.1.1
EOF

umount "$ROOTFS_MNT"

cp "$BUILDER_OBJ/$DISTRO.img" "/app/assets/$DISTRO.img"
