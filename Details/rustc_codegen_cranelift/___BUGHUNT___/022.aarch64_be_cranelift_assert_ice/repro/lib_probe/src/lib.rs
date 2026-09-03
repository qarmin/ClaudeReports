#![no_std]

pub fn checksum(data: &[u32]) -> u32 {
    let mut acc: u32 = 0x1234_5678;
    for (i, &v) in data.iter().enumerate() {
        acc = acc.rotate_left((i as u32) & 31) ^ v.wrapping_mul(0x9E37_79B9);
    }
    acc
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
