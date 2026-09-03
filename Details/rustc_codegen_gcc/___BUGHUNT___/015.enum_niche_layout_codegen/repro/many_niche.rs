#![allow(dead_code)]
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Small { NegOne = -1, Zero = 0, One = 1 }

enum Many {
    Data(Small),
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,
    V32,
    V33,
    V34,
    V35,
    V36,
    V37,
    V38,
    V39,
    V40,
    V41,
    V42,
    V43,
    V44,
    V45,
    V46,
    V47,
    V48,
    V49,
    V50,
    V51,
    V52,
    V53,
    V54,
    V55,
    V56,
    V57,
    V58,
    V59,
    V60,
    V61,
    V62,
    V63,
    V64,
    V65,
    V66,
    V67,
    V68,
    V69,
    V70,
    V71,
    V72,
    V73,
    V74,
    V75,
    V76,
    V77,
    V78,
    V79,
    V80,
    V81,
    V82,
    V83,
    V84,
    V85,
    V86,
    V87,
    V88,
    V89,
    V90,
    V91,
    V92,
    V93,
    V94,
    V95,
    V96,
    V97,
    V98,
    V99,
    V100,
    V101,
    V102,
    V103,
    V104,
    V105,
    V106,
    V107,
    V108,
    V109,
    V110,
    V111,
    V112,
    V113,
    V114,
    V115,
    V116,
    V117,
    V118,
    V119,
    V120,
    V121,
    V122,
    V123,
    V124,
    V125,
    V126,
    V127,
    V128,
    V129,
    V130,
    V131,
    V132,
    V133,
    V134,
    V135,
    V136,
    V137,
    V138,
    V139,
}

fn extract(m: &Many) -> Option<Small> {
    match m {
        Many::Data(x) => Some(*x),
        _ => None,
    }
}

fn main() {
    use std::hint::black_box;
    println!("size={}", std::mem::size_of::<Many>());
    for v in [Small::NegOne, Small::Zero, Small::One] {
        let m = black_box(Many::Data(v));
        assert_eq!(extract(&m), Some(v), "Data({:?}) round-trip failed", v);
    }
    match black_box(Many::V0) {
        Many::V0 => {}
        _ => panic!("V0 misdecoded"),
    }
    match black_box(Many::V1) {
        Many::V1 => {}
        _ => panic!("V1 misdecoded"),
    }
    match black_box(Many::V70) {
        Many::V70 => {}
        _ => panic!("V70 misdecoded"),
    }
    match black_box(Many::V138) {
        Many::V138 => {}
        _ => panic!("V138 misdecoded"),
    }
    match black_box(Many::V139) {
        Many::V139 => {}
        _ => panic!("V139 misdecoded"),
    }
    println!("many niche ok");
}
