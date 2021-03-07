#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use rost::arch;
use rost::klog;
use rost::mem;
use rost::uart;

use log::{info, LevelFilter};

use riscv::asm::*;
use riscv::register::*;
use riscv_rt::entry;

extern "C" {
    fn _goto_supervised();
}

global_asm!(
    r#"
.global _goto_supervised
.align 4
_goto_supervised:
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
/// Go to supervised mode
#[no_mangle]
fn kinit() {
    unsafe {
        mstatus::set_mpp(mstatus::MPP::Supervisor);
        mepc::write(kmain as usize);
        _goto_supervised();

        // setup timer/clock
    }
}

/// Runs the main kernel routines
#[no_mangle]
fn kmain() -> ! {
    unsafe {
        mem::init();
    }

    info!("Setup done");

    rost::symbols::dump_symbols();

    loop {
        unsafe {
            wfi();
        }
    }
}

/// Entrypoint for the rust code.
///
/// The assembly code jumps here.
#[entry]
fn kentry() -> ! {
    klog::init(LevelFilter::Trace).expect("Failed to setup logger");
    uart::Uart::new(uart::UART_BASE_ADDR).init();

    info!("Booting Rost ...");
    info!("Current hart: {}", mhartid::read());

    kinit();

    loop {
        unsafe {
            wfi();
        }
    }
}
