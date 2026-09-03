use std::hint::black_box;

fn main() {
    { let v: i8 = black_box(0i8 as i8); let r = v as isize; println!("direct/i8->isize/0: {}", r); }
    { let v: i8 = black_box(0i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i8->isize/0: {}", r); }
    { let v: i8 = black_box(1i8 as i8); let r = v as isize; println!("direct/i8->isize/1: {}", r); }
    { let v: i8 = black_box(1i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i8->isize/1: {}", r); }
    { let v: i8 = black_box(-1i8 as i8); let r = v as isize; println!("direct/i8->isize/2: {}", r); }
    { let v: i8 = black_box(-1i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i8->isize/2: {}", r); }
    { let v: i8 = black_box(-128i8 as i8); let r = v as isize; println!("direct/i8->isize/3: {}", r); }
    { let v: i8 = black_box(-128i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i8->isize/3: {}", r); }
    { let v: i8 = black_box(-127i8 as i8); let r = v as isize; println!("direct/i8->isize/4: {}", r); }
    { let v: i8 = black_box(-127i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i8->isize/4: {}", r); }
    { let v: i8 = black_box(127i8 as i8); let r = v as isize; println!("direct/i8->isize/5: {}", r); }
    { let v: i8 = black_box(127i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i8->isize/5: {}", r); }
    { let v: i8 = black_box(126i8 as i8); let r = v as isize; println!("direct/i8->isize/6: {}", r); }
    { let v: i8 = black_box(126i8 as i8); let reinterpreted: u8 = unsafe { std::mem::transmute(v) }; let back: i8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i8->isize/6: {}", r); }
}
