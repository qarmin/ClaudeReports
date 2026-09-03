use std::hint::black_box;

fn main() {
    { let v: i16 = black_box(0i16 as i16); let r = v as u32; println!("direct/i16->u32/0: {}", r); }
    { let v: i16 = black_box(0i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i16->u32/0: {}", r); }
    { let v: i16 = black_box(1i16 as i16); let r = v as u32; println!("direct/i16->u32/1: {}", r); }
    { let v: i16 = black_box(1i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i16->u32/1: {}", r); }
    { let v: i16 = black_box(-1i16 as i16); let r = v as u32; println!("direct/i16->u32/2: {}", r); }
    { let v: i16 = black_box(-1i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i16->u32/2: {}", r); }
    { let v: i16 = black_box(-32768i16 as i16); let r = v as u32; println!("direct/i16->u32/3: {}", r); }
    { let v: i16 = black_box(-32768i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i16->u32/3: {}", r); }
    { let v: i16 = black_box(-32767i16 as i16); let r = v as u32; println!("direct/i16->u32/4: {}", r); }
    { let v: i16 = black_box(-32767i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i16->u32/4: {}", r); }
    { let v: i16 = black_box(32767i16 as i16); let r = v as u32; println!("direct/i16->u32/5: {}", r); }
    { let v: i16 = black_box(32767i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i16->u32/5: {}", r); }
    { let v: i16 = black_box(32766i16 as i16); let r = v as u32; println!("direct/i16->u32/6: {}", r); }
    { let v: i16 = black_box(32766i16 as i16); let reinterpreted: u16 = unsafe { std::mem::transmute(v) }; let back: i16 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u32; println!("transmute-roundtrip/i16->u32/6: {}", r); }
}
