TARGET := "riscv64gc-unknown-none-elf"
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

# run coffer with Linux
qemu KERNEL=DEFAULT_KERNEL ROOTFS=DEFAULT_ROOTFS: (debug "sifive")
  qemu-system-riscv64 -M sifive_u -m 256M -nographic -bios {{DEBUG}} -kernel {{KERNEL}} -drive file={{ROOTFS}},format=raw

# run coffer with Linux and gdb
gdb KERNEL=DEFAULT_KERNEL ROOTFS=DEFAULT_ROOTFS: (debug "sifive")
  qemu-system-riscv64 -S -s -M sifive_u -m 256M -nographic -bios {{DEBUG}} -kernel {{KERNEL}} -drive file={{ROOTFS}},format=raw
