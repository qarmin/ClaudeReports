use std::hint::black_box;

fn main() {
    { let v: usize = black_box(0usize as usize); let r = v as u32; println!("direct/usize->u32/0: {}", r); }
    { let v: usize = black_box(0usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/usize->u32/0: {}", r); }
    { let v: usize = black_box(1usize as usize); let r = v as u32; println!("direct/usize->u32/1: {}", r); }
    { let v: usize = black_box(1usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/usize->u32/1: {}", r); }
    { let v: usize = black_box(18446744073709551615usize as usize); let r = v as u32; println!("direct/usize->u32/2: {}", r); }
    { let v: usize = black_box(18446744073709551615usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/usize->u32/2: {}", r); }
    { let v: usize = black_box(18446744073709551614usize as usize); let r = v as u32; println!("direct/usize->u32/3: {}", r); }
    { let v: usize = black_box(18446744073709551614usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/usize->u32/3: {}", r); }
    { let v: usize = black_box(9223372036854775808usize as usize); let r = v as u32; println!("direct/usize->u32/4: {}", r); }
    { let v: usize = black_box(9223372036854775808usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/usize->u32/4: {}", r); }
    { let v: usize = black_box(9223372036854775807usize as usize); let r = v as u32; println!("direct/usize->u32/5: {}", r); }
    { let v: usize = black_box(9223372036854775807usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/usize->u32/5: {}", r); }
    { let v: usize = black_box(9223372036854775809usize as usize); let r = v as u32; println!("direct/usize->u32/6: {}", r); }
    { let v: usize = black_box(9223372036854775809usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/usize->u32/6: {}", r); }
}
