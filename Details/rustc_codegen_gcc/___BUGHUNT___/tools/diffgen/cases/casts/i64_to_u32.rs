use std::hint::black_box;

fn main() {
    { let v: i64 = black_box(0i64 as i64); let r = v as u32; println!("direct/i64->u32/0: {}", r); }
    { let v: i64 = black_box(0i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i64->u32/0: {}", r); }
    { let v: i64 = black_box(1i64 as i64); let r = v as u32; println!("direct/i64->u32/1: {}", r); }
    { let v: i64 = black_box(1i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i64->u32/1: {}", r); }
    { let v: i64 = black_box(-1i64 as i64); let r = v as u32; println!("direct/i64->u32/2: {}", r); }
    { let v: i64 = black_box(-1i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i64->u32/2: {}", r); }
    { let v: i64 = black_box(-9223372036854775808i64 as i64); let r = v as u32; println!("direct/i64->u32/3: {}", r); }
    { let v: i64 = black_box(-9223372036854775808i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i64->u32/3: {}", r); }
    { let v: i64 = black_box(-9223372036854775807i64 as i64); let r = v as u32; println!("direct/i64->u32/4: {}", r); }
    { let v: i64 = black_box(-9223372036854775807i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i64->u32/4: {}", r); }
    { let v: i64 = black_box(9223372036854775807i64 as i64); let r = v as u32; println!("direct/i64->u32/5: {}", r); }
    { let v: i64 = black_box(9223372036854775807i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i64->u32/5: {}", r); }
    { let v: i64 = black_box(9223372036854775806i64 as i64); let r = v as u32; println!("direct/i64->u32/6: {}", r); }
    { let v: i64 = black_box(9223372036854775806i64 as i64); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: i64 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i64->u32/6: {}", r); }
}
