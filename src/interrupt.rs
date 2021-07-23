use crate::plic::{self, InterruptId};
use crate::uart::uart_interrupt;

use log::error;

/// Handle an interrupt from PLIC
///
/// Cause if the scause code.

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
    error!("timer interrupt not implemented");
}

#[no_mangle]
pub fn handle_interrupt(code: u32) {
    match code {
        9 => plic_interrupt(),
        1 => timer_interrupt(),
        _ => error!("Unknown Interrupt Code: {}", code),
    }
}
