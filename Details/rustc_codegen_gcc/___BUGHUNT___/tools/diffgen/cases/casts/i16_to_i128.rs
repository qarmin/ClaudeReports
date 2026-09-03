use std::hint::black_box;

fn main() {
    { let v: i16 = black_box(0i16 as i16); let r = v as i128; println!("direct/i16->i128/0: {}", r); }
    { let v: i16 = black_box(0i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i128; println!("transmute-roundtrip/i16->i128/0: {}", r); }
    { let v: i16 = black_box(1i16 as i16); let r = v as i128; println!("direct/i16->i128/1: {}", r); }
    { let v: i16 = black_box(1i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i128; println!("transmute-roundtrip/i16->i128/1: {}", r); }
    { let v: i16 = black_box(-1i16 as i16); let r = v as i128; println!("direct/i16->i128/2: {}", r); }
    { let v: i16 = black_box(-1i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i128; println!("transmute-roundtrip/i16->i128/2: {}", r); }
    { let v: i16 = black_box(-32768i16 as i16); let r = v as i128; println!("direct/i16->i128/3: {}", r); }
    { let v: i16 = black_box(-32768i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i128; println!("transmute-roundtrip/i16->i128/3: {}", r); }
    { let v: i16 = black_box(-32767i16 as i16); let r = v as i128; println!("direct/i16->i128/4: {}", r); }
    { let v: i16 = black_box(-32767i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i128; println!("transmute-roundtrip/i16->i128/4: {}", r); }
    { let v: i16 = black_box(32767i16 as i16); let r = v as i128; println!("direct/i16->i128/5: {}", r); }
    { let v: i16 = black_box(32767i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i128; println!("transmute-roundtrip/i16->i128/5: {}", r); }
    { let v: i16 = black_box(32766i16 as i16); let r = v as i128; println!("direct/i16->i128/6: {}", r); }
    { let v: i16 = black_box(32766i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i128; println!("transmute-roundtrip/i16->i128/6: {}", r); }
}
