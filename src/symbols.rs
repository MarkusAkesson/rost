#![allow(non_snake_case)]
use crate::println;

extern "C" {
    static _sheap: u8;
    static _eheap: u8;
    static _heap_size: u8;
    static _sbss: u8;
    static _ebss: u8;
    static _srodata: u8;
    static _sdata: u8;
    static _edata: u8;
    static _sstack: u8;
    static _estack: u8;
    static _stext: u8;
}

pub fn KERNEL_STACK_START() -> usize {
    unsafe {
        return &_sstack as *const u8 as usize;
    }
}

pub fn KERNEL_STACK_END() -> usize {
    unsafe {
        return &_estack as *const u8 as usize;
    }
}

pub fn HEAP_START() -> usize {
    unsafe {
        return &_sheap as *const u8 as usize;
    }
}

pub fn HEAP_END() -> usize {
    unsafe {
        return &_eheap as *const u8 as usize;
    }
}

pub fn HEAP_SIZE() -> usize {
    unsafe {
        return &_heap_size as *const u8 as usize;
    }
}

pub fn TEXT_START() -> usize {
    unsafe {
        return &_stext as *const u8 as usize;
    }
}

pub fn RODATA_START() -> usize {
    unsafe {
        return &_sdata as *const u8 as usize;
    }
}

pub fn RODATA_END() -> usize {
    unsafe {
        return &_edata as *const u8 as usize;
    }
}
pub fn DATA_START() -> usize {
    unsafe {
        return &_sdata as *const u8 as usize;
    }
}

pub fn DATA_END() -> usize {
    unsafe {
        return &_edata as *const u8 as usize;
    }
}

pub fn BSS_START() -> usize {
    unsafe {
        return &_sbss as *const u8 as usize;
    }
}

pub fn BSS_END() -> usize {
    unsafe {
        return &_ebss as *const u8 as usize;
    }
}

pub fn dump_symbols() {
    println!("Symbols:");
    println!("\tHeap start:         0x{:X}", HEAP_START());
    println!("\tHeap end:           0x{:X}", HEAP_END());
    println!("\tHeap size:          0x{:X}", HEAP_SIZE());
    println!("\tKernel stack start: 0x{:X}", KERNEL_STACK_START());
    println!("\tKernel stack end:   0x{:X}", KERNEL_STACK_END());
    println!("\tText start:         0x{:X}", TEXT_START());
    println!("\tRO Data start:      0x{:X}", RODATA_START());
    println!("\tRO Data end:        0x{:X}", RODATA_END());
    println!("\tData start:         0x{:X}", DATA_START());
    println!("\tData end:           0x{:X}", DATA_END());
    println!("\tBss start:          0x{:X}", BSS_START());
    println!("\tBss end:            0x{:X}", BSS_END());
}
