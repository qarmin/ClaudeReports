use std::hint::black_box;

fn main() {
    { let v: i32 = black_box(0i32 as i32); let r = v as usize; println!("direct/i32->usize/0: {}", r); }
    { let v: i32 = black_box(0i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as usize; println!("transmute-roundtrip/i32->usize/0: {}", r); }
    { let v: i32 = black_box(1i32 as i32); let r = v as usize; println!("direct/i32->usize/1: {}", r); }
    { let v: i32 = black_box(1i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as usize; println!("transmute-roundtrip/i32->usize/1: {}", r); }
    { let v: i32 = black_box(-1i32 as i32); let r = v as usize; println!("direct/i32->usize/2: {}", r); }
    { let v: i32 = black_box(-1i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as usize; println!("transmute-roundtrip/i32->usize/2: {}", r); }
    { let v: i32 = black_box(-2147483648i32 as i32); let r = v as usize; println!("direct/i32->usize/3: {}", r); }
    { let v: i32 = black_box(-2147483648i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as usize; println!("transmute-roundtrip/i32->usize/3: {}", r); }
    { let v: i32 = black_box(-2147483647i32 as i32); let r = v as usize; println!("direct/i32->usize/4: {}", r); }
    { let v: i32 = black_box(-2147483647i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as usize; println!("transmute-roundtrip/i32->usize/4: {}", r); }
    { let v: i32 = black_box(2147483647i32 as i32); let r = v as usize; println!("direct/i32->usize/5: {}", r); }
    { let v: i32 = black_box(2147483647i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as usize; println!("transmute-roundtrip/i32->usize/5: {}", r); }
    { let v: i32 = black_box(2147483646i32 as i32); let r = v as usize; println!("direct/i32->usize/6: {}", r); }
    { let v: i32 = black_box(2147483646i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as usize; println!("transmute-roundtrip/i32->usize/6: {}", r); }
}
