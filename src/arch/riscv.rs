use crate::page::PAGE_SIZE;

use riscv;

use core::arch::asm;
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
    unsafe {
        return core::ptr::read_volatile(0x0200_bff8 as *const usize)
    }
}

/// Check if interrupt is enabled
pub fn intr_get() -> bool {
    riscv::register::sstatus::read().sie()
}

pub fn thread_pointer() -> usize {
    let mut hart_id: usize;
    unsafe { asm!("mv {}, tp", out(reg) hart_id) }
    hart_id
}
