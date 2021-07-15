use crate::memory::pmp::PmpFlags;
use crate::memory::pmp::{pmpaddr_read, pmpcfg_read};
use crate::{print, println};
use riscv::register::misa::MXL;
use riscv::register::mstatus::{MPP, SPP};
use riscv::register::mtvec::TrapMode;

macro_rules! dbg_bool{
    () => {};
    ($val:expr $(,)?) => {
        match $val {
            //if $val is true return a green string
            true => {
                "\x1b[1m\x1b[32mtrue\x1b[0m"
            }
            //if $val is false return a red string
            false => {
                "\x1b[1m\x1b[31mfalse\x1b[0m"
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg_bool!($val)),+,)
    };
}

macro_rules! flag_println {
    () => {};
    ($fmt:expr, $flag:expr) => {
        println!($fmt, dbg_bool!($flag));
    };
}

pub fn print_misa() {
    if let Some(misa) = riscv::register::misa::read() {
        let xlen = match misa.mxl() {
            MXL::XLEN32 => 32,
            MXL::XLEN64 => 64,
            MXL::XLEN128 => 128,
        };
        println!("ISA: riscv{}, ", xlen);
        print!("Supported Extensions:");
        for ext in 'A'..='Z' {
            if misa.has_extension(ext) {
                print!("{}", ext);
            }
        }
        println!("");
    } else {
        println!("[ERROR] read misa failed")
    }
}

pub fn print_mstatus() {
    let mstatus = riscv::register::mstatus::read();
    println!("mstatus: {:?}", mstatus);

    println!("[xIE] Interrupt Status:");
    flag_println!("\tMachine Mode Interrupt: {}", mstatus.mie());
    flag_println!("\tSupervisor Mode Interrupt: {}", mstatus.sie());
    flag_println!("\tUser Mode Interrupt: {}", mstatus.uie());

    println!("[xPIE] Previous Interrupt Status:");
    flag_println!("\tPrevious Machine Interrupt: {}", mstatus.mpie());
    flag_println!("\tPrevious Supervisor Interrupt: {}", mstatus.spie());
    flag_println!("\tPrevious User Interrupt: {}", mstatus.upie());

    println!("[xPP] Previous Privilege Level:");
    let mpp = match mstatus.mpp() {
        MPP::User => "User",
        MPP::Supervisor => "Supervisor",
        MPP::Machine => "Machine",
    };
    let spp = match mstatus.spp() {
        SPP::User => "User",
        SPP::Supervisor => "Supervisor",
    };
    println!("\tPrevious Machine Privilege: {}", mpp);
    println!("\tPrevious Supervisor Privilege: {}", spp);
    println!("\tPrevious User Privilege: {}", "User");
    // TODO: Add MXLEN, what is it btw?

    println!("Memory Privileges:");
    println!(
        "\t[MPRV] Memory R/W: {}",
        if mstatus.mprv() { mpp } else { "Machine" }
    );

    flag_println!("\t[MXR] eXecutable is Readable: {}", mstatus.mxr());
    flag_println!("\t[SUM] Supervisor User Access: {}", mstatus.sum());

    println!("Virtualization Support:");
    flag_println!("\t[TVM] Trap satp R/W: {}", mstatus.tvm());
    flag_println!("\t[TSR] Trap sret: {}", mstatus.tsr());
    flag_println!("\t[TW] Trap WFI: {}", mstatus.tw());
    // TODO: Add FS/XS
}

pub fn print_mtvec() {
    let mtvec = riscv::register::mtvec::read();
    println!("Machine Trap Handler:");
    if let Some(mode) = mtvec.trap_mode() {
        println!(
            "\t[MODE] Trap Mode: {}",
            if mode == TrapMode::Direct {
                "Direct"
            } else {
                "Vectorized"
            }
        );
    } else {
        println!("\t[ERROR] Unknown Trap Mode");
    }
    println!("\t[BASE] Trap Handler Addr: 0x{:x}", mtvec.address());
}

pub fn print_medeleg() {
    let medeleg = riscv::register::medeleg::read();
    println!("Machine Exception Delegate:");
    flag_println!(
        "\t0. Instruction Misaligned: {}",
        medeleg.instruction_misaligned()
    );
    flag_println!(
        "\t1. Instruction Access Fault: {}",
        medeleg.instruction_fault()
    );
    flag_println!(
        "\t2. Illegal Instruction: {}",
        medeleg.illegal_instruction()
    );
    flag_println!("\t3. Breakpoint: {}", medeleg.breakpoint());
    flag_println!("\t4. Load Misaligned: {}", medeleg.load_misaligned());
    flag_println!("\t5. Load Access Fault: {}", medeleg.load_fault());
    flag_println!("\t6. Store Misaligned: {}", medeleg.store_misaligned());
    flag_println!("\t7. Store Access Fault: {}", medeleg.store_misaligned());
    flag_println!("\t8. User ECall: {}", medeleg.user_env_call());
    flag_println!("\t9. Supervisor ECall: {}", medeleg.supervisor_env_call());
    flag_println!("\t11. Machine ECall: {}", medeleg.machine_env_call());
    flag_println!(
        "\t12. Instruction Page Fault: {}",
        medeleg.load_page_fault()
    );
    flag_println!("\t13. Load Page Fault: {}", medeleg.store_page_fault());
    flag_println!("\t15. Store Page Fault: {}", medeleg.store_page_fault());
}

pub fn print_mideleg() {
    let mideleg = riscv::register::mideleg::read();
    println!("Machine Interrupt Delegate:");
    flag_println!("\t0. User Soft IRQ: {}", mideleg.usoft());
    flag_println!("\t1. Supervisor Soft IRQ: {}", mideleg.ssoft());
    flag_println!("\t4. User Timer IRQ: {}", mideleg.utimer());
    flag_println!("\t5. Supervisor Timer IRQ: {}", mideleg.stimer());
    flag_println!("\t8. User External IRQ: {}", mideleg.uext());
    flag_println!("\t9. Supervisor External IRQ: {}", mideleg.sext());
}

pub fn print_mip() {
    let mip = riscv::register::mip::read();
    println!("Machine Interrupt Pending:");
    flag_println!("\t0. User Soft IRQ: {}", mip.usoft());
    flag_println!("\t1. Supervisor Soft IRQ: {}", mip.ssoft());
    flag_println!("\t2. Machine Soft IRQ: {}", mip.msoft());
    flag_println!("\t3. User Timer IRQ: {}", mip.utimer());
    flag_println!("\t4. Supervisor Timer IRQ: {}", mip.stimer());
    flag_println!("\t5. Machine Timer IRQ: {}", mip.mtimer());
    flag_println!("\t6. User External IRQ: {}", mip.uext());
    flag_println!("\t7. Supervisor External IRQ: {}", mip.sext());
    flag_println!("\t8. Machine External IRQ: {}", mip.mext());
}

pub fn print_mie() {
    let mie = riscv::register::mie::read();
    println!("Machine Interrupt Enable:");
    flag_println!("\t0. User Soft IRQ: {}", mie.usoft());
    flag_println!("\t1. Supervisor Soft IRQ: {}", mie.ssoft());
    flag_println!("\t2. Machine Soft IRQ: {}", mie.msoft());
    flag_println!("\t3. User Timer IRQ: {}", mie.utimer());
    flag_println!("\t4. Supervisor Timer IRQ: {}", mie.stimer());
    flag_println!("\t5. Machine Timer IRQ: {}", mie.mtimer());
    flag_println!("\t6. User External IRQ: {}", mie.uext());
    flag_println!("\t7. Supervisor External IRQ: {}", mie.sext());
    flag_println!("\t8. Machine External IRQ: {}", mie.mext());
}

pub fn print_mscratch() {
    let mscratch = riscv::register::mscratch::read();
    println!("Machine Scratch Register: 0x{:x}", mscratch);
}

pub fn print_pmp() {
    for i in 0..16 {
        let cfg = pmpcfg_read(i);
        let addr = pmpaddr_read(i);
        if cfg == 0 {
            println!("PMP[{}] Status: Off", i);
        } else {
            println!("PMP[{}] Status: On", i);
            println!("PMP[{}] Config: {:?}", i, PmpFlags::from_bits(cfg));
            println!("PMP[{}] Addr: 0x{:x}", i, addr);
        }
    }
}

pub fn print_machine() {
    print_misa();
    print_mstatus();
    print_mtvec();
    print_medeleg();
    print_mideleg();
    print_mip();
    print_mie();
    print_mscratch();
    print_pmp();
}
