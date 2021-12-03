#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use core::sync::atomic::{AtomicBool, Ordering};

use rost::klog;
use rost::mem;
use rost::plic;
use rost::rand::xorshift::XorShift;
use rost::trap;
use rost::uart;

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
    csrr a1, mhartid
    mv tp, a1
    mret
"#
);

/// Initiates the kernel
///
/// Go to supervised mode when initialization is done
#[no_mangle]
fn kinit() {
    unsafe {
        mem::init();
        plic::init();
        mem::enable_mmu();
        trap::hartinit();
    }
    plic::hartinit();

    info!("Setup done, jumping to supervisor mode");

    unsafe {
        mstatus::set_mpp(mstatus::MPP::Supervisor);
        mepc::write(kmain as usize);
        goto_supervised();
    }
}

/// Kernel main
/// Never returns.
#[no_mangle]
fn kmain() -> ! {
    // Release the other HARTs
    //BOOT.store(true, Ordering::Relaxed);
    info!("Enabling interrutps");
    unsafe {
        // enable interrupts
        riscv::register::sstatus::set_sie();
        sstatus::set_spp(sstatus::SPP::Supervisor);
        // enable software interrupt
        riscv::register::sie::set_ssoft();
        riscv::register::sie::set_sext();
    }

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
    kmain();
}

/// Entrypoint for the rust code.
///
/// Setup kernel logger and initate uart.
/// Never returns
#[entry]
fn kentry() -> ! {
    if mhartid::read() == 0 {
        klog::init(LevelFilter::Trace).expect("Failed to setup logger");
        uart::Uart::new(uart::UART_BASE_ADDR).init();

        info!("Booting Rost ...");
        info!("Current hart: {}", mhartid::read());

        kinit();
    } else {
        while !BOOT.load(Ordering::Relaxed) {
                rost::arch::riscv::wait();
        }
        hartinit();
    }
    loop {
        rost::arch::riscv::wait();
    }
}

