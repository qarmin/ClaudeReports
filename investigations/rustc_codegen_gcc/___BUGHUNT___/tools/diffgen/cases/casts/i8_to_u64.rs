use std::hint::black_box;

fn main() {
    { let v: i8 = black_box(0i8 as i8); let r = v as u64; println!("direct/i8->u64/0: {}", r); }
    { let v: i8 = black_box(0i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/i8->u64/0: {}", r); }
    { let v: i8 = black_box(1i8 as i8); let r = v as u64; println!("direct/i8->u64/1: {}", r); }
    { let v: i8 = black_box(1i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/i8->u64/1: {}", r); }
    { let v: i8 = black_box(-1i8 as i8); let r = v as u64; println!("direct/i8->u64/2: {}", r); }
    { let v: i8 = black_box(-1i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/i8->u64/2: {}", r); }
    { let v: i8 = black_box(-128i8 as i8); let r = v as u64; println!("direct/i8->u64/3: {}", r); }
    { let v: i8 = black_box(-128i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/i8->u64/3: {}", r); }
    { let v: i8 = black_box(-127i8 as i8); let r = v as u64; println!("direct/i8->u64/4: {}", r); }
    { let v: i8 = black_box(-127i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/i8->u64/4: {}", r); }
    { let v: i8 = black_box(127i8 as i8); let r = v as u64; println!("direct/i8->u64/5: {}", r); }
    { let v: i8 = black_box(127i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/i8->u64/5: {}", r); }
    { let v: i8 = black_box(126i8 as i8); let r = v as u64; println!("direct/i8->u64/6: {}", r); }
    { let v: i8 = black_box(126i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/i8->u64/6: {}", r); }
}
