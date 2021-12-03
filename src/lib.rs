#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]

pub mod arch;
pub mod interrupt;
pub mod klog;
pub mod mem;
pub mod page;
pub mod plic;
pub mod rand;
pub mod symbols;
pub mod trap;
pub mod uart;

/// Panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("hart {} aborting: ", arch::riscv::thread_pointer());
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }
    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}
