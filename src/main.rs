#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
use core::arch::{asm, global_asm};

use rost::klog;
use rost::mem;
use rost::plic;
use rost::rand::xorshift::XorShift;
use rost::trap;
use rost::uart;
use rost::arch;

use log::{info, LevelFilter};

use riscv::register::*;
use riscv_rt::entry;

/// Blocks harts > 0 from booting, gets set to true when hart 0
/// is initated and interrupts are turned on
static BOOT: AtomicBool = AtomicBool::new(false);

extern "C" {
    fn goto_supervised();
}

global_asm!(
    r#"
.global goto_supervised
.align 4
goto_supervised:
    csrw satp, zero
    li t0, 0xffff
    csrw medeleg, t0
    li t0, 0xffff
    csrw mideleg, t0
    csrr a1, mhartid
    mv tp, a1
    mret
"#
);

/// Initiates the kernel
///
/// Go to supervised mode when initialization is done
#[entry]
fn kinit() -> ! {
    if mhartid::read() == 0 {
        klog::init(LevelFilter::Trace).expect("Failed to setup logger");
        uart::Uart::new(uart::UART_BASE_ADDR).init();

        info!("Booting Rost ...");
        info!("Current hart: {}", mhartid::read());
    }
    
    info!("Jumping to supervisor mode");

    unsafe {
        mstatus::set_mpp(mstatus::MPP::Supervisor);
        mepc::write(kmain as usize);

        asm!("csrw satp, zero");
        asm!("csrw pmpcfg0, 0xF");
        // delegate all interrupts and exceptions to supervisor mode
        asm!("li t0, 0xffff");
        asm!("csrw medeleg, t0");
        asm!("li t0, 0xffff");
        asm!("csrw mideleg, t0");
        // save cpuid to tp
        asm!("csrr a1, mhartid");
        asm!("mv tp, a1");
        // switch to supervisor mode
        info!("{:X} {:X}", mepc::read(), kmain as usize);
        asm!("mret");
        //goto_supervised();
    }

    loop {
        rost::arch::riscv::wait();
    }
}

/// Kernel main
/// Never returns.
#[no_mangle]
unsafe fn kmain() -> ! {
    info!("Initiating hart:{}", arch::riscv::thread_pointer());
    if  arch::riscv::thread_pointer() == 0 {
        mem::init();
        plic::init();
        mem::enable_mmu();
        trap::hartinit();
        plic::hartinit();
        // Release the other HARTs
        //BOOT.store(true, Ordering::Relaxed);
    } else {
        while !BOOT.load(Ordering::Relaxed) {
            rost::arch::riscv::wait();
        }
        hartinit();
    }

    info!("Enabling interrutps");
    // enable interrupts
    riscv::register::sstatus::set_sie();
    sstatus::set_spp(sstatus::SPP::Supervisor);
    // enable software interrupt
    riscv::register::sie::set_ssoft();
    riscv::register::sie::set_sext();

    #[cfg(test)]
    test_main();

    loop {
        rost ::arch::riscv::wait();
    }
}

/// Initiate a hart
///
/// Called for each hart that is starting up.
/// Calls kmain
fn hartinit() {
    info!("Booting hart {}", mhartid::read());
    mem::enable_mmu();
    plic::hartinit();
    unsafe {
        trap::hartinit();
    }
}

