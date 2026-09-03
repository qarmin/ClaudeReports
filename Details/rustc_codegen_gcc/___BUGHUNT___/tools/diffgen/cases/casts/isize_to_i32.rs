use std::hint::black_box;

fn main() {
    { let v: isize = black_box(0isize as isize); let r = v as i32; println!("direct/isize->i32/0: {}", r); }
    { let v: isize = black_box(0isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/isize->i32/0: {}", r); }
    { let v: isize = black_box(1isize as isize); let r = v as i32; println!("direct/isize->i32/1: {}", r); }
    { let v: isize = black_box(1isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/isize->i32/1: {}", r); }
    { let v: isize = black_box(-1isize as isize); let r = v as i32; println!("direct/isize->i32/2: {}", r); }
    { let v: isize = black_box(-1isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/isize->i32/2: {}", r); }
    { let v: isize = black_box(-9223372036854775808isize as isize); let r = v as i32; println!("direct/isize->i32/3: {}", r); }
    { let v: isize = black_box(-9223372036854775808isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/isize->i32/3: {}", r); }
    { let v: isize = black_box(-9223372036854775807isize as isize); let r = v as i32; println!("direct/isize->i32/4: {}", r); }
    { let v: isize = black_box(-9223372036854775807isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/isize->i32/4: {}", r); }
    { let v: isize = black_box(9223372036854775807isize as isize); let r = v as i32; println!("direct/isize->i32/5: {}", r); }
    { let v: isize = black_box(9223372036854775807isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/isize->i32/5: {}", r); }
    { let v: isize = black_box(9223372036854775806isize as isize); let r = v as i32; println!("direct/isize->i32/6: {}", r); }
    { let v: isize = black_box(9223372036854775806isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/isize->i32/6: {}", r); }
}
