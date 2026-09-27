use std::hint::black_box;

fn main() {
    { let v: u32 = black_box(0u32 as u32); let r = v as isize; println!("direct/u32->isize/0: {}", r); }
    { let v: u32 = black_box(0u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/u32->isize/0: {}", r); }
    { let v: u32 = black_box(1u32 as u32); let r = v as isize; println!("direct/u32->isize/1: {}", r); }
    { let v: u32 = black_box(1u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/u32->isize/1: {}", r); }
    { let v: u32 = black_box(4294967295u32 as u32); let r = v as isize; println!("direct/u32->isize/2: {}", r); }
    { let v: u32 = black_box(4294967295u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/u32->isize/2: {}", r); }
    { let v: u32 = black_box(4294967294u32 as u32); let r = v as isize; println!("direct/u32->isize/3: {}", r); }
    { let v: u32 = black_box(4294967294u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/u32->isize/3: {}", r); }
    { let v: u32 = black_box(2147483648u32 as u32); let r = v as isize; println!("direct/u32->isize/4: {}", r); }
    { let v: u32 = black_box(2147483648u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/u32->isize/4: {}", r); }
    { let v: u32 = black_box(2147483647u32 as u32); let r = v as isize; println!("direct/u32->isize/5: {}", r); }
    { let v: u32 = black_box(2147483647u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/u32->isize/5: {}", r); }
    { let v: u32 = black_box(2147483649u32 as u32); let r = v as isize; println!("direct/u32->isize/6: {}", r); }
    { let v: u32 = black_box(2147483649u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as isize; println!("transmute-roundtrip/u32->isize/6: {}", r); }
}
