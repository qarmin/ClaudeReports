#![allow(dead_code)]
use std::hint::black_box;

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Small {
    NegOne = -1,
    Zero = 0,
    One = 1,
}

enum NicheLow {
    A(Small),
    Other1,
    Other2,
}

enum NicheHigh {
    Other1,
    Other2,
    A(Small),
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum DirectSigned {
    NegTwo = -2,
    NegOne = -1,
    Zero = 0,
}

fn get_small(n: NicheLow) -> Option<Small> {
    match n {
        NicheLow::A(s) => Some(s),
        _ => None,
    }
}

fn get_small_high(n: NicheHigh) -> Option<Small> {
    match n {
        NicheHigh::A(s) => Some(s),
        _ => None,
    }
}

fn main() {
    // Direct tag encoding with signed repr, sign-extension through `as i64`.
    let d = black_box(DirectSigned::NegOne);
    let as_i64 = d as i64;
    assert_eq!(as_i64, -1i64, "DirectSigned::NegOne as i64 should be -1, got {}", as_i64);

    let d2 = black_box(DirectSigned::NegTwo);
    let as_i64_2 = d2 as i64;
    assert_eq!(as_i64_2, -2i64, "DirectSigned::NegTwo as i64 should be -2, got {}", as_i64_2);

    // Niche encoding, niche variants "below" the used range (Other1/Other2 low)
    for s in [Small::NegOne, Small::Zero, Small::One] {
        let wrapped = black_box(NicheLow::A(s));
        let extracted = get_small(wrapped);
        assert_eq!(extracted, Some(s), "NicheLow round-trip failed for {:?}", s);
    }
    match black_box(NicheLow::Other1) {
        NicheLow::Other1 => {}
        _ => panic!("NicheLow::Other1 misdecoded"),
    }
    match black_box(NicheLow::Other2) {
        NicheLow::Other2 => {}
        _ => panic!("NicheLow::Other2 misdecoded"),
    }

    // Niche encoding, niche variants "above" the used range
    for s in [Small::NegOne, Small::Zero, Small::One] {
        let wrapped = black_box(NicheHigh::A(s));
        let extracted = get_small_high(wrapped);
        assert_eq!(extracted, Some(s), "NicheHigh round-trip failed for {:?}", s);
    }
    match black_box(NicheHigh::Other1) {
        NicheHigh::Other1 => {}
        _ => panic!("NicheHigh::Other1 misdecoded"),
    }
    match black_box(NicheHigh::Other2) {
        NicheHigh::Other2 => {}
        _ => panic!("NicheHigh::Other2 misdecoded"),
    }

    // Common real-world niche cases.
    let opt_bool: Option<bool> = black_box(Some(true));
    assert_eq!(opt_bool, Some(true));
    let opt_ref: Option<&i32> = black_box(None);
    assert_eq!(opt_ref, None);
    let opt_nz: Option<std::num::NonZeroI8> = black_box(std::num::NonZeroI8::new(-1));
    assert_eq!(opt_nz.map(|v| v.get()), Some(-1));
    let opt_nz_none: Option<std::num::NonZeroI8> = black_box(None);
    assert_eq!(opt_nz_none, None);

    println!("all good");
}
