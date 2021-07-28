TARGET := "riscv64imac-unknown-none-elf"
BINARY := "coffer"
DEBUG := "target/"+TARGET+"/debug/"+BINARY
RELEASE := "target/"+TARGET+"/release/"+BINARY

DEFAULT_KERNEL := "../linux-riscv/linux/arch/riscv/boot/Image"
DEFAULT_ROOTFS := "../linux-riscv/rootfs.img"

debug BOARD:
  cargo rustc --features "{{ BOARD }}" -- {{ ("-Clink-args=-Tlink-"+BOARD+"-64.ld") }}
  rust-objcopy {{DEBUG}} -O binary coffer

release BOARD:
  cargo rustc --release --features "{{ BOARD }}" -- {{ ("-Clink-args=-Tlink-"+BOARD+"-64.ld") }}
  rust-objcopy {{RELEASE}} -O binary coffer

# xfel tool, just for sunxi
xfel: (release "sunxi")
  xfel version
  xfel ddr ddr3
  xfel write 0x40000000 coffer.striped
  xfel write 0x42000000 ~/tools/tina/out/d1-nezha/image/u-boot.fex
  xfel exec 0x40000000

# run coffer with Linux
qemu KERNEL=DEFAULT_KERNEL ROOTFS=DEFAULT_ROOTFS: (debug "virt")
  qemu-system-riscv64 -M virt -m 256M -nographic -bios {{DEBUG}} -kernel {{KERNEL}} -drive file={{ROOTFS}},format=raw,id=hd0  -device virtio-blk-device,drive=hd0 -append "root=/dev/vda rw console=ttyS0"

# run coffer with Linux and gdb
gdb KERNEL=DEFAULT_KERNEL ROOTFS=DEFAULT_ROOTFS: (debug "virt")
  qemu-system-riscv64 -S -s -M virt -m 256M -nographic -bios {{DEBUG}} -kernel {{KERNEL}} -drive file={{ROOTFS}},format=raw,id=hd0  -device virtio-blk-device,drive=hd0 -append "root=/dev/vda rw console=ttyS0"
