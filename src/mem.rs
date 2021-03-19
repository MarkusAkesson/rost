use crate::arch;
use crate::page::{self, Attribute, PageTable, KERNEL_PAGE_TABLE};
use crate::plic::PLIC_BASE;
use crate::symbols::*;
use crate::uart;

use log::info;

pub unsafe fn init() {
    info!("Initiating memory");
    page::init();

    let pgtable: &mut PageTable = &mut *(&KERNEL_PAGE_TABLE as *const _ as *mut _); // to bypass mut ref

    let regions = &[
        (DATA_START(), DATA_END(), Attribute::ReadExecute as usize),
        (
            RODATA_START(),
            RODATA_END(),
            Attribute::ReadExecute as usize,
        ),
        (
            TEXT_START(),
            RODATA_START(),
            Attribute::ReadExecute as usize,
        ),
        (BSS_START(), BSS_END(), Attribute::ReadWrite as usize),
        (
            KERNEL_STACK_END(),
            KERNEL_STACK_START(),
            Attribute::ReadWrite as usize,
        ),
        (HEAP_START(), HEAP_END(), Attribute::ReadWrite as usize),
        (
            uart::UART_BASE_ADDR,
            uart::UART_BASE_ADDR,
            Attribute::ReadWrite as usize,
        ),
    ];

    info!("Mapping the kernel");
    pgtable.id_map_ranges(regions);
    pgtable.id_map_range(
        PLIC_BASE,
        PLIC_BASE + 0x40_0000,
        Attribute::ReadWrite as usize,
    );
    info!("Memory Initiated");
}

pub fn enable_mmu() {
    info!("Enabling mmu");
    let root_ppn = &KERNEL_PAGE_TABLE as *const PageTable as usize;
    let satp_val = arch::riscv::build_satp(8, 0, root_ppn);
    unsafe {
        asm!("csrw satp, {}", in(reg) satp_val);
        riscv::asm::sfence_vma(0, 0);
    }
}
