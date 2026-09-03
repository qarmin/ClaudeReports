use std::hint::black_box;

fn main() {
    { let v: u32 = black_box(0u32 as u32); let r = v as i16; println!("direct/u32->i16/0: {}", r); }
    { let v: u32 = black_box(0u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/u32->i16/0: {}", r); }
    { let v: u32 = black_box(1u32 as u32); let r = v as i16; println!("direct/u32->i16/1: {}", r); }
    { let v: u32 = black_box(1u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/u32->i16/1: {}", r); }
    { let v: u32 = black_box(4294967295u32 as u32); let r = v as i16; println!("direct/u32->i16/2: {}", r); }
    { let v: u32 = black_box(4294967295u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/u32->i16/2: {}", r); }
    { let v: u32 = black_box(4294967294u32 as u32); let r = v as i16; println!("direct/u32->i16/3: {}", r); }
    { let v: u32 = black_box(4294967294u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/u32->i16/3: {}", r); }
    { let v: u32 = black_box(2147483648u32 as u32); let r = v as i16; println!("direct/u32->i16/4: {}", r); }
    { let v: u32 = black_box(2147483648u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/u32->i16/4: {}", r); }
    { let v: u32 = black_box(2147483647u32 as u32); let r = v as i16; println!("direct/u32->i16/5: {}", r); }
    { let v: u32 = black_box(2147483647u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/u32->i16/5: {}", r); }
    { let v: u32 = black_box(2147483649u32 as u32); let r = v as i16; println!("direct/u32->i16/6: {}", r); }
    { let v: u32 = black_box(2147483649u32 as u32); let reinterpreted: i32 = unsafe { std::mem::transmute(v) }; let back: u32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/u32->i16/6: {}", r); }
}
