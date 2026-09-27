use std::hint::black_box;

fn main() {
    { let v: isize = black_box(0isize as isize); let r = v as i16; println!("direct/isize->i16/0: {}", r); }
    { let v: isize = black_box(0isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/isize->i16/0: {}", r); }
    { let v: isize = black_box(1isize as isize); let r = v as i16; println!("direct/isize->i16/1: {}", r); }
    { let v: isize = black_box(1isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/isize->i16/1: {}", r); }
    { let v: isize = black_box(-1isize as isize); let r = v as i16; println!("direct/isize->i16/2: {}", r); }
    { let v: isize = black_box(-1isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/isize->i16/2: {}", r); }
    { let v: isize = black_box(-9223372036854775808isize as isize); let r = v as i16; println!("direct/isize->i16/3: {}", r); }
    { let v: isize = black_box(-9223372036854775808isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/isize->i16/3: {}", r); }
    { let v: isize = black_box(-9223372036854775807isize as isize); let r = v as i16; println!("direct/isize->i16/4: {}", r); }
    { let v: isize = black_box(-9223372036854775807isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/isize->i16/4: {}", r); }
    { let v: isize = black_box(9223372036854775807isize as isize); let r = v as i16; println!("direct/isize->i16/5: {}", r); }
    { let v: isize = black_box(9223372036854775807isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/isize->i16/5: {}", r); }
    { let v: isize = black_box(9223372036854775806isize as isize); let r = v as i16; println!("direct/isize->i16/6: {}", r); }
    { let v: isize = black_box(9223372036854775806isize as isize); let reinterpreted: u64 = unsafe { std::mem::transmute(v) }; let back: isize = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i16; println!("transmute-roundtrip/isize->i16/6: {}", r); }
}
