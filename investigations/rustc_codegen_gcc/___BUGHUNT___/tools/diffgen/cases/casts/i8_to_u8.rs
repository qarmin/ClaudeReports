use std::hint::black_box;

fn main() {
    { let v: i8 = black_box(0i8 as i8); let r = v as u8; println!("direct/i8->u8/0: {}", r); }
    { let v: i8 = black_box(0i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u8; println!("transmute-roundtrip/i8->u8/0: {}", r); }
    { let v: i8 = black_box(1i8 as i8); let r = v as u8; println!("direct/i8->u8/1: {}", r); }
    { let v: i8 = black_box(1i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u8; println!("transmute-roundtrip/i8->u8/1: {}", r); }
    { let v: i8 = black_box(-1i8 as i8); let r = v as u8; println!("direct/i8->u8/2: {}", r); }
    { let v: i8 = black_box(-1i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u8; println!("transmute-roundtrip/i8->u8/2: {}", r); }
    { let v: i8 = black_box(-128i8 as i8); let r = v as u8; println!("direct/i8->u8/3: {}", r); }
    { let v: i8 = black_box(-128i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u8; println!("transmute-roundtrip/i8->u8/3: {}", r); }
    { let v: i8 = black_box(-127i8 as i8); let r = v as u8; println!("direct/i8->u8/4: {}", r); }
    { let v: i8 = black_box(-127i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u8; println!("transmute-roundtrip/i8->u8/4: {}", r); }
    { let v: i8 = black_box(127i8 as i8); let r = v as u8; println!("direct/i8->u8/5: {}", r); }
    { let v: i8 = black_box(127i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u8; println!("transmute-roundtrip/i8->u8/5: {}", r); }
    { let v: i8 = black_box(126i8 as i8); let r = v as u8; println!("direct/i8->u8/6: {}", r); }
    { let v: i8 = black_box(126i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u8; println!("transmute-roundtrip/i8->u8/6: {}", r); }
}
