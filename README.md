# `Coffer` RISC-V Trusted Execution Environment

![Issue](https://img.shields.io/github/issues/jwnhy/coffer)
![Rust](https://img.shields.io/badge/language-rust-yellowgreen)
![riscv64](https://img.shields.io/badge/platform-riscv64-lightgrey)

## Table of contents

1. [What is Coffer?](#whatiscoffer)
2. [Quickstart with QEMU.](#quickqemu)
3. [Quickstart with Nezha D1.](#quicknezha)
4. [Current Status.](#status)
5. [Contact.](#contact)
6. [License and Copyright.](#license)

## What is Coffer? <a name="whatiscoffer"></a>

Coffer is designed to be an flexible, software-based trusted execution environment.
It requires a minimal hardware primitive (PMP is all you need!)
to provide a powerful software interface for TEE.

[![asciicast](https://asciinema.org/a/427543.svg)](https://asciinema.org/a/427543)

## Quickstart with QEMU <a name="quickqemu"></a>

To run Linux with Coffer, you will need to prepare a Linux image and a rootfs.

One may refer to [this tutorial (EN)](https://risc-v-getting-started-guide.readthedocs.io/en/latest/linux-qemu.html)
or [this tutorial (CN)](https://zhuanlan.zhihu.com/p/258394849)
to learn how to build your own image and rootfs.

Once your Linux/rootfs is ready,
you can run just one line to get coffer booting Linux in qemu.

```bash
just qemu <path-to-your-kernel> <path-to-your-rootfs>
```

## Quickstart with [Nezha D1](https://d1.docs.allwinnertech.com) <a name="quicknezha"></a>

To run Linux with Coffer on Nezha D1 SoC,
you will first need to get the Nezha SDK ready.

One may refer to the [official guide](https://d1.docs.allwinnertech.com/study/study_2getsdk/)
to learn how to get Allwinner SDK setup.

To replace OpenSBI with Coffer, you will need to run the following command.

```bash
# Make other stuff using Allwinner SDK
just release sunxi # build coffer for target platform Nezha D1
cp coffer <path-to-allwinner-sdk>/device/config/chips/d1/bin/opensbi_sun20iw1p1.bin # replace OpenSBI with Coffer
```

Once copied into the SDK, you can continue the tutorial provided by AllWinner
and run Linux on D1 with Coffer enabled.

## Current Status <a name="status"></a>

Coffer has serveral goals to archive in terms of both security and functionality.

- [x] Linux-capable Bootload
- [x] SBI Standard Implementation
- [x] Runtime Memory Protection
- [x] I/O Space Protection
- [ ] Firmware Specific Binary Interface
- [ ] Port to SiFive Unleashed Board
- [ ] Enclave Memory Migration

## Contact <a name="contact"></a>

You can email <luhy2017@mail.sustech.edu.cn> if you have any questions about Coffer.

## License and Copyright <a name="license"></a>

See [LICENSE](https://github.com/jwnhy/coffer/blob/main/LICENSE)
for distribution and use of source code, binaries, and documentation.
