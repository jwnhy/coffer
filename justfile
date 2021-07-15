TARGET := "riscv64imac-unknown-none-elf"
BINARY := "coffer"
PAYLOAD := "target/"+TARGET+"/debug/"+BINARY
RELEASE := "target/"+TARGET+"/release/"+BINARY
KERNEL := "../base_module/target/riscv64imac-unknown-none-elf/debug/base_module"


build:
  cargo build
  rust-objcopy {{PAYLOAD}} -O binary  coffer.striped

release:
  cargo build --release
  for sec in 'abbrev' 'addr' 'aranges' 'info' 'line' 'line_str' 'ranges' 'rnglists' 'str' 'str_offsets'; do \
    rust-objcopy {{RELEASE}} --remove-section .rvbt_$sec; \
  done
  rust-objcopy -g {{RELEASE}}
  rust-objcopy {{RELEASE}} -O binary coffer.striped
  cp coffer.striped ~/tools/tina/device/config/chips/d1/bin/opensbi_sun20iw1p1.bin

xfel: release
  xfel version
  xfel ddr ddr3
  xfel write 0x40000000 coffer.striped
  xfel write 0x42000000 ~/tools/tina/out/d1-nezha/image/u-boot.fex
  xfel exec 0x40000000

copy-debug:
	for sec in 'abbrev' 'addr' 'aranges' 'info' 'line' 'line_str' 'ranges' 'rnglists' 'str' 'str_offsets'; do \
		rust-objcopy {{PAYLOAD}} --dump-section .debug_$sec=tmp_$sec; \
		riscv64-unknown-elf-objcopy {{PAYLOAD}} --update-section .rvbt_$sec=tmp_$sec; \
	done
	rm tmp*; 

qemu-m MACHINE: build copy-debug
	qemu-system-riscv64 -machine {{MACHINE}} -bios {{PAYLOAD}} -kernel {{KERNEL}} -nographic

qemu: build copy-debug
	qemu-system-riscv64 -machine virt -bios {{PAYLOAD}} -kernel {{KERNEL}}  -nographic

gdb: build copy-debug
	qemu-system-riscv64 -S -s -machine virt -bios {{PAYLOAD}} -kernel {{KERNEL}} -nographic

gdb-m MACHINE: build copy-debug
	qemu-system-riscv64 -S -s -machine {{MACHINE}} -bios {{PAYLOAD}} -kernel {{KERNEL}} -nographic
