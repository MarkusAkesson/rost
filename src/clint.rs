use riscv::register::*;

pub fn timer_init() {
    let hart = mhartid::read();

    // Enable machine mode timer interrupts
    mstatus::set_mie();
    mie::set_mtimer();
}
