use std::hint::black_box;

fn main() {
    { let v: i32 = black_box(0i32 as i32); let r = v as i16; println!("direct/i32->i16/0: {}", r); }
    { let v: i32 = black_box(0i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i32->i16/0: {}", r); }
    { let v: i32 = black_box(1i32 as i32); let r = v as i16; println!("direct/i32->i16/1: {}", r); }
    { let v: i32 = black_box(1i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i32->i16/1: {}", r); }
    { let v: i32 = black_box(-1i32 as i32); let r = v as i16; println!("direct/i32->i16/2: {}", r); }
    { let v: i32 = black_box(-1i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i32->i16/2: {}", r); }
    { let v: i32 = black_box(-2147483648i32 as i32); let r = v as i16; println!("direct/i32->i16/3: {}", r); }
    { let v: i32 = black_box(-2147483648i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i32->i16/3: {}", r); }
    { let v: i32 = black_box(-2147483647i32 as i32); let r = v as i16; println!("direct/i32->i16/4: {}", r); }
    { let v: i32 = black_box(-2147483647i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i32->i16/4: {}", r); }
    { let v: i32 = black_box(2147483647i32 as i32); let r = v as i16; println!("direct/i32->i16/5: {}", r); }
    { let v: i32 = black_box(2147483647i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i32->i16/5: {}", r); }
    { let v: i32 = black_box(2147483646i32 as i32); let r = v as i16; println!("direct/i32->i16/6: {}", r); }
    { let v: i32 = black_box(2147483646i32 as i32); let reinterpreted: u32 = unsafe { std::mem::transmute(v) }; let back: i32 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/i32->i16/6: {}", r); }
}
