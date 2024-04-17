use crate::clint;
use crate::plic::{self, InterruptId};
use crate::uart::uart_interrupt;

use riscv::register;

use log::{debug, error};

/// Handle an interrupt from PLIC
///
fn plic_interrupt() {
    let plic = plic::plic();
    if let Some(id) = plic.next() {
        match id {
            InterruptId::Uart0 => uart_interrupt(),
            InterruptId::Unknown => error!("Unkown PLIC interrupt ({:?}", id),
        }
        plic.complete(id as u32);
    }
}

fn timer_interrupt() {
    debug!("Tick");
    unsafe {
        debug!("{:?}", register::sip::Sip::stimer(&register::sip::read()));
        debug!("{:?}", register::sip::Sip::ssoft(&register::sip::read()));
        register::sip::clear_ssoft();
        debug!("{:?}", register::sip::read());
        debug!("{:?}", register::sip::Sip::stimer(&register::sip::read()));
        debug!("{:?}", register::sip::Sip::ssoft(&register::sip::read()));
        clint::debug();
    };
}

#[no_mangle]
pub fn handle_interrupt(code: u32) {
    match code {
        9 => plic_interrupt(),
        1 => timer_interrupt(),
        _ => error!("Unknown Interrupt Code: {}", code),
    }
}
