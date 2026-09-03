use std::hint::black_box;

fn main() {
    { let v: u16 = black_box(0u16 as u16); let r = v as i64; println!("direct/u16->i64/0: {}", r); }
    { let v: u16 = black_box(0u16 as u16); let reinterpreted: i16 = unsafe { std::mem::transmute(v) }; let back: u16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i64; println!("transmute-roundtrip/u16->i64/0: {}", r); }
    { let v: u16 = black_box(1u16 as u16); let r = v as i64; println!("direct/u16->i64/1: {}", r); }
    { let v: u16 = black_box(1u16 as u16); let reinterpreted: i16 = unsafe { std::mem::transmute(v) }; let back: u16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i64; println!("transmute-roundtrip/u16->i64/1: {}", r); }
    { let v: u16 = black_box(65535u16 as u16); let r = v as i64; println!("direct/u16->i64/2: {}", r); }
    { let v: u16 = black_box(65535u16 as u16); let reinterpreted: i16 = unsafe { std::mem::transmute(v) }; let back: u16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i64; println!("transmute-roundtrip/u16->i64/2: {}", r); }
    { let v: u16 = black_box(65534u16 as u16); let r = v as i64; println!("direct/u16->i64/3: {}", r); }
    { let v: u16 = black_box(65534u16 as u16); let reinterpreted: i16 = unsafe { std::mem::transmute(v) }; let back: u16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i64; println!("transmute-roundtrip/u16->i64/3: {}", r); }
    { let v: u16 = black_box(32768u16 as u16); let r = v as i64; println!("direct/u16->i64/4: {}", r); }
    { let v: u16 = black_box(32768u16 as u16); let reinterpreted: i16 = unsafe { std::mem::transmute(v) }; let back: u16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i64; println!("transmute-roundtrip/u16->i64/4: {}", r); }
    { let v: u16 = black_box(32767u16 as u16); let r = v as i64; println!("direct/u16->i64/5: {}", r); }
    { let v: u16 = black_box(32767u16 as u16); let reinterpreted: i16 = unsafe { std::mem::transmute(v) }; let back: u16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i64; println!("transmute-roundtrip/u16->i64/5: {}", r); }
    { let v: u16 = black_box(32769u16 as u16); let r = v as i64; println!("direct/u16->i64/6: {}", r); }
    { let v: u16 = black_box(32769u16 as u16); let reinterpreted: i16 = unsafe { std::mem::transmute(v) }; let back: u16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i64; println!("transmute-roundtrip/u16->i64/6: {}", r); }
}
