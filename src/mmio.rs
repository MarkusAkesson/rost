/// # Safety
///
/// As the mmio funcitons operate on raw pointers
/// the functions will be marked unsafe
pub unsafe fn mmio_write(address: usize, offset: usize, value: u8) {
    let reg = address as *mut u8;

    reg.add(offset).write_volatile(value);
}

/// # Safety
///
/// As the mmio funcitons operate on raw pointers
/// the functions will be marked unsafe
pub unsafe fn mmio_read(address: usize, offset: usize) -> u8 {
    let reg = address as *mut u8;

    reg.add(offset).read_volatile()
}
