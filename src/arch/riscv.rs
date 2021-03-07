use crate::page::PAGE_SIZE;

use riscv;

use core::time::Duration;

/// Build satp value from mode, asid and page table base addr
pub fn build_satp(mode: usize, asid: usize, addr: usize) -> usize {
    assert!(addr % PAGE_SIZE == 0);
    mode << 60 | (asid & 0xffff) << 44 | (addr >> 12) & 0xff_ffff_ffff
}

pub fn wait() {
    unsafe {
        riscv::asm::wfi();
    }
}

pub fn uptime() -> Duration {
    let time = rdtime();
    Duration::from_nanos(time as u64 * 100)
}

fn rdtime() -> usize {
    let x: usize;
    unsafe { asm!("rdtime {}", out(reg) x) };
    x
}
