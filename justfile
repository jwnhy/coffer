TARGET := "riscv64imac-unknown-none-elf"
BINARY := "coffer"
DEBUG := "target/"+TARGET+"/debug/"+BINARY
RELEASE := "target/"+TARGET+"/release/"+BINARY
KERNEL := "../base_module/target/riscv64imac-unknown-none-elf/debug/base_module"


debug BOARD:
  cargo rustc --features "{{ BOARD }}" -- {{ ("-Clink-args=-Tlink-"+BOARD+"-64.ld") }}

release BOARD:
  cargo rustc --release --features "{{ BOARD }}" -- {{ ("-Clink-args=-Tlink-"+BOARD+"-64.ld") }}
  rust-objcopy {{RELEASE}} -O binary coffer.striped
  cp coffer.striped ~/tools/tina/device/config/chips/d1/bin/opensbi_sun20iw1p1.bin

xfel: (release "sunxi")
  xfel version
  xfel ddr ddr3
  xfel write 0x40000000 coffer.striped
  xfel write 0x42000000 ~/tools/tina/out/d1-nezha/image/u-boot.fex
  xfel exec 0x40000000

qemu: (debug "virt")
  qemu-system-riscv64 -M virt -m 256M -nographic -bios {{DEBUG}} -kernel ../linux-riscv/linux/arch/riscv/boot/Image -drive file=../linux-riscv/rootfs.img,format=raw,id=hd0  -device virtio-blk-device,drive=hd0 -append "root=/dev/vda rw console=ttyS0"

gdb: (debug "virt")
  qemu-system-riscv64 -S -s -M virt -m 256M -nographic -bios {{DEBUG}} -kernel ../linux-riscv/linux/arch/riscv/boot/Image -drive file=../linux-riscv/rootfs.img,format=raw,id=hd0  -device virtio-blk-device,drive=hd0 -append "root=/dev/vda rw console=ttyS0"
