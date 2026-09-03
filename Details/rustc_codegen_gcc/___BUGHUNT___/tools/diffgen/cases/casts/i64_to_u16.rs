use std::hint::black_box;

fn main() {
    { let v: i64 = black_box(0i64 as i64); let r = v as u16; println!("direct/i64->u16/0: {}", r); }
    { let v: i64 = black_box(0i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/i64->u16/0: {}", r); }
    { let v: i64 = black_box(1i64 as i64); let r = v as u16; println!("direct/i64->u16/1: {}", r); }
    { let v: i64 = black_box(1i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/i64->u16/1: {}", r); }
    { let v: i64 = black_box(-1i64 as i64); let r = v as u16; println!("direct/i64->u16/2: {}", r); }
    { let v: i64 = black_box(-1i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/i64->u16/2: {}", r); }
    { let v: i64 = black_box(-9223372036854775808i64 as i64); let r = v as u16; println!("direct/i64->u16/3: {}", r); }
    { let v: i64 = black_box(-9223372036854775808i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/i64->u16/3: {}", r); }
    { let v: i64 = black_box(-9223372036854775807i64 as i64); let r = v as u16; println!("direct/i64->u16/4: {}", r); }
    { let v: i64 = black_box(-9223372036854775807i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/i64->u16/4: {}", r); }
    { let v: i64 = black_box(9223372036854775807i64 as i64); let r = v as u16; println!("direct/i64->u16/5: {}", r); }
    { let v: i64 = black_box(9223372036854775807i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/i64->u16/5: {}", r); }
    { let v: i64 = black_box(9223372036854775806i64 as i64); let r = v as u16; println!("direct/i64->u16/6: {}", r); }
    { let v: i64 = black_box(9223372036854775806i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/i64->u16/6: {}", r); }
}
