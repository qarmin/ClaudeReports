use std::hint::black_box;

fn main() {
    { let v: i8 = black_box(0i8 as i8); let r = v as i16; println!("direct/i8->i16/0: {}", r); }
    { let v: i8 = black_box(0i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i8->i16/0: {}", r); }
    { let v: i8 = black_box(1i8 as i8); let r = v as i16; println!("direct/i8->i16/1: {}", r); }
    { let v: i8 = black_box(1i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i8->i16/1: {}", r); }
    { let v: i8 = black_box(-1i8 as i8); let r = v as i16; println!("direct/i8->i16/2: {}", r); }
    { let v: i8 = black_box(-1i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i8->i16/2: {}", r); }
    { let v: i8 = black_box(-128i8 as i8); let r = v as i16; println!("direct/i8->i16/3: {}", r); }
    { let v: i8 = black_box(-128i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i8->i16/3: {}", r); }
    { let v: i8 = black_box(-127i8 as i8); let r = v as i16; println!("direct/i8->i16/4: {}", r); }
    { let v: i8 = black_box(-127i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i8->i16/4: {}", r); }
    { let v: i8 = black_box(127i8 as i8); let r = v as i16; println!("direct/i8->i16/5: {}", r); }
    { let v: i8 = black_box(127i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i8->i16/5: {}", r); }
    { let v: i8 = black_box(126i8 as i8); let r = v as i16; println!("direct/i8->i16/6: {}", r); }
    { let v: i8 = black_box(126i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i8->i16/6: {}", r); }
}
