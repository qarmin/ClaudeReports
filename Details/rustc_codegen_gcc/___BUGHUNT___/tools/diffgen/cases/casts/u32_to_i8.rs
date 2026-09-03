use std::hint::black_box;

fn main() {
    { let v: u32 = black_box(0u32 as u32); let r = v as i8; println!("direct/u32->i8/0: {}", r); }
    { let v: u32 = black_box(0u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u32->i8/0: {}", r); }
    { let v: u32 = black_box(1u32 as u32); let r = v as i8; println!("direct/u32->i8/1: {}", r); }
    { let v: u32 = black_box(1u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u32->i8/1: {}", r); }
    { let v: u32 = black_box(4294967295u32 as u32); let r = v as i8; println!("direct/u32->i8/2: {}", r); }
    { let v: u32 = black_box(4294967295u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u32->i8/2: {}", r); }
    { let v: u32 = black_box(4294967294u32 as u32); let r = v as i8; println!("direct/u32->i8/3: {}", r); }
    { let v: u32 = black_box(4294967294u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u32->i8/3: {}", r); }
    { let v: u32 = black_box(2147483648u32 as u32); let r = v as i8; println!("direct/u32->i8/4: {}", r); }
    { let v: u32 = black_box(2147483648u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u32->i8/4: {}", r); }
    { let v: u32 = black_box(2147483647u32 as u32); let r = v as i8; println!("direct/u32->i8/5: {}", r); }
    { let v: u32 = black_box(2147483647u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u32->i8/5: {}", r); }
    { let v: u32 = black_box(2147483649u32 as u32); let r = v as i8; println!("direct/u32->i8/6: {}", r); }
    { let v: u32 = black_box(2147483649u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u32->i8/6: {}", r); }
}
