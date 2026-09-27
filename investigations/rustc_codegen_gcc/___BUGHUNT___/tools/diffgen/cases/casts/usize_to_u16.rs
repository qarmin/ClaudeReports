use std::hint::black_box;

fn main() {
    { let v: usize = black_box(0usize as usize); let r = v as u16; println!("direct/usize->u16/0: {}", r); }
    { let v: usize = black_box(0usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/usize->u16/0: {}", r); }
    { let v: usize = black_box(1usize as usize); let r = v as u16; println!("direct/usize->u16/1: {}", r); }
    { let v: usize = black_box(1usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/usize->u16/1: {}", r); }
    { let v: usize = black_box(18446744073709551615usize as usize); let r = v as u16; println!("direct/usize->u16/2: {}", r); }
    { let v: usize = black_box(18446744073709551615usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/usize->u16/2: {}", r); }
    { let v: usize = black_box(18446744073709551614usize as usize); let r = v as u16; println!("direct/usize->u16/3: {}", r); }
    { let v: usize = black_box(18446744073709551614usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/usize->u16/3: {}", r); }
    { let v: usize = black_box(9223372036854775808usize as usize); let r = v as u16; println!("direct/usize->u16/4: {}", r); }
    { let v: usize = black_box(9223372036854775808usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/usize->u16/4: {}", r); }
    { let v: usize = black_box(9223372036854775807usize as usize); let r = v as u16; println!("direct/usize->u16/5: {}", r); }
    { let v: usize = black_box(9223372036854775807usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/usize->u16/5: {}", r); }
    { let v: usize = black_box(9223372036854775809usize as usize); let r = v as u16; println!("direct/usize->u16/6: {}", r); }
    { let v: usize = black_box(9223372036854775809usize as usize); let reinterpreted: i64 = unsafe { std::mem::transmute(v) }; let back: usize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u16; println!("transmute-roundtrip/usize->u16/6: {}", r); }
}
