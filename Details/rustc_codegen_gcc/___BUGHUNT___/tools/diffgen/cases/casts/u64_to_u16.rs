use std::hint::black_box;

fn main() {
    { let v: u64 = black_box(0u64 as u64); let r = v as u16; println!("direct/u64->u16/0: {}", r); }
    { let v: u64 = black_box(0u64 as u64); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: u64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/u64->u16/0: {}", r); }
    { let v: u64 = black_box(1u64 as u64); let r = v as u16; println!("direct/u64->u16/1: {}", r); }
    { let v: u64 = black_box(1u64 as u64); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: u64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/u64->u16/1: {}", r); }
    { let v: u64 = black_box(18446744073709551615u64 as u64); let r = v as u16; println!("direct/u64->u16/2: {}", r); }
    { let v: u64 = black_box(18446744073709551615u64 as u64); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: u64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/u64->u16/2: {}", r); }
    { let v: u64 = black_box(18446744073709551614u64 as u64); let r = v as u16; println!("direct/u64->u16/3: {}", r); }
    { let v: u64 = black_box(18446744073709551614u64 as u64); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: u64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/u64->u16/3: {}", r); }
    { let v: u64 = black_box(9223372036854775808u64 as u64); let r = v as u16; println!("direct/u64->u16/4: {}", r); }
    { let v: u64 = black_box(9223372036854775808u64 as u64); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: u64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/u64->u16/4: {}", r); }
    { let v: u64 = black_box(9223372036854775807u64 as u64); let r = v as u16; println!("direct/u64->u16/5: {}", r); }
    { let v: u64 = black_box(9223372036854775807u64 as u64); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: u64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/u64->u16/5: {}", r); }
    { let v: u64 = black_box(9223372036854775809u64 as u64); let r = v as u16; println!("direct/u64->u16/6: {}", r); }
    { let v: u64 = black_box(9223372036854775809u64 as u64); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: u64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/u64->u16/6: {}", r); }
}
