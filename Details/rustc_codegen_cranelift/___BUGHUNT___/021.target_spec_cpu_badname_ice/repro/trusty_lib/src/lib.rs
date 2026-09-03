#![no_std]

#[inline(never)]
pub fn checksum(n: u64) -> u64 {
    let mut acc: u64 = 0;
    let mut i: u64 = 0;
    while i < n {
        acc = acc.wrapping_mul(31).wrapping_add(i ^ 0xa5a5);
        i += 1;
    }
    acc
}
