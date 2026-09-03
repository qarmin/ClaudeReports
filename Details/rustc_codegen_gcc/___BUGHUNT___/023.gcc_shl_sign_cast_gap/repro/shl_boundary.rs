use std::hint::black_box;

fn main() {
    // a unsigned, b signed, same width (32-bit): u32 << i32
    let a: u32 = black_box(0xC000_0001);
    let b: i32 = black_box(3);
    let got = a << b;
    let expected: u32 = 0xC000_0001u32.wrapping_shl(3);
    assert_eq!(got, expected, "u32 << i32 mismatch: got {:#x} expected {:#x}", got, expected);

    // a signed, b unsigned, same width (32-bit): i32 << u32, shifting into sign bit
    let a2: i32 = black_box(0x60000000u32 as i32); // positive, but shift will set sign bit
    let b2: u32 = black_box(1);
    let got2 = a2 << b2;
    let expected2: i32 = (0x60000000u32 as i32).wrapping_shl(1);
    assert_eq!(got2, expected2, "i32 << u32 mismatch: got {:#x} expected {:#x}", got2, expected2);

    // a signed negative, b unsigned, same width (64-bit)
    let a3: i64 = black_box(-1i64);
    let b3: u64 = black_box(5);
    let got3 = a3 << b3;
    let expected3: i64 = (-1i64).wrapping_shl(5);
    assert_eq!(got3, expected3, "i64 << u64 mismatch: got {} expected {}", got3, expected3);

    // a unsigned max, b signed, same width (64-bit)
    let a4: u64 = black_box(u64::MAX);
    let b4: i64 = black_box(63);
    let got4 = a4 << b4;
    let expected4: u64 = u64::MAX.wrapping_shl(63);
    assert_eq!(got4, expected4, "u64 << i64 mismatch: got {:#x} expected {:#x}", got4, expected4);

    // 8-bit mixed sign, shift near width
    let a5: u8 = black_box(0xFF);
    let b5: i8 = black_box(7);
    let got5 = a5 << b5;
    let expected5: u8 = 0xFFu8.wrapping_shl(7);
    assert_eq!(got5, expected5, "u8 << i8 mismatch: got {:#x} expected {:#x}", got5, expected5);

    let a6: i16 = black_box(-32768i16); // MIN
    let b6: u16 = black_box(1);
    let got6 = a6 << b6;
    let expected6: i16 = (-32768i16).wrapping_shl(1);
    assert_eq!(got6, expected6, "i16 << u16 mismatch: got {} expected {}", got6, expected6);

    println!("all shl checks passed");
}
