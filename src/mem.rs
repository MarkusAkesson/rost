use crate::arch;
use crate::page::{self, Attribute, PageTable, KERNEL_PAGE_TABLE};
use crate::plic::PLIC_BASE;
use crate::symbols::*;
use crate::uart;

use log::info;

use core::arch::asm;

pub struct Region {
    start: usize,
    end: usize,
    flags: Attribute,
    name: &'static str,
}

impl Region {
    pub fn new(start: usize, end: usize, flags: Attribute, name: &'static str) -> Self {
        Self {
            start, end, flags, name
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn start_addr(&self) -> usize {
        self.start
    }

    pub fn end_addr(&self) -> usize {
        self.end
    }

    pub fn flags(&self) -> Attribute {
        self.flags
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

pub unsafe fn init() {
    info!("Initiating memory");
    page::init();

    let pgtable: &mut PageTable = &mut *(&KERNEL_PAGE_TABLE as *const _ as *mut _); // to bypass mut ref

    let regions = &[
        Region::new(DATA_START(), DATA_END(), Attribute::ReadExecute, "DATA"),
        //Region::new(
        //    RODATA_START(),
        //    RODATA_END(),
        //    Attribute::ReadExecute,
        //    "RODATA",
        //),
        Region::new(
            TEXT_START(),
            RODATA_START(),
            Attribute::ReadExecute,
            "Text",
        ),
        Region::new(BSS_START(), BSS_END(), Attribute::ReadWrite, "BSS"),
        Region::new(
            KERNEL_STACK_END(),
            KERNEL_STACK_START(),
            Attribute::ReadWrite,
            "KERNEL_STACK",
        ),
        //FIXME: (HEAP_START(), HEAP_END(), Attribute::ReadWrite as usize, "Heap"),
        Region::new(
            uart::UART_BASE_ADDR,
            uart::UART_BASE_ADDR,
            Attribute::ReadWrite,
            "Uart",
        ),
        Region::new(
            PLIC_BASE,
            PLIC_BASE + 0x400_000,
            Attribute::ReadWrite,
            "PLIC_BASE",
        ),
    ];

    info!("Mapping the kernel");
    pgtable.id_map_ranges(regions);
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
