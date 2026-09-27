use std::hint::black_box;

fn main() {
    { let v: i32 = black_box(0i32 as i32); let r = v as isize; println!("direct/i32->isize/0: {}", r); }
    { let v: i32 = black_box(0i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i32->isize/0: {}", r); }
    { let v: i32 = black_box(1i32 as i32); let r = v as isize; println!("direct/i32->isize/1: {}", r); }
    { let v: i32 = black_box(1i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i32->isize/1: {}", r); }
    { let v: i32 = black_box(-1i32 as i32); let r = v as isize; println!("direct/i32->isize/2: {}", r); }
    { let v: i32 = black_box(-1i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i32->isize/2: {}", r); }
    { let v: i32 = black_box(-2147483648i32 as i32); let r = v as isize; println!("direct/i32->isize/3: {}", r); }
    { let v: i32 = black_box(-2147483648i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i32->isize/3: {}", r); }
    { let v: i32 = black_box(-2147483647i32 as i32); let r = v as isize; println!("direct/i32->isize/4: {}", r); }
    { let v: i32 = black_box(-2147483647i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i32->isize/4: {}", r); }
    { let v: i32 = black_box(2147483647i32 as i32); let r = v as isize; println!("direct/i32->isize/5: {}", r); }
    { let v: i32 = black_box(2147483647i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i32->isize/5: {}", r); }
    { let v: i32 = black_box(2147483646i32 as i32); let r = v as isize; println!("direct/i32->isize/6: {}", r); }
    { let v: i32 = black_box(2147483646i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/i32->isize/6: {}", r); }
}
