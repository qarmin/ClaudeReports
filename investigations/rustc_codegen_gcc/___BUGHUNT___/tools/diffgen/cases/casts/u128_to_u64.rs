use std::hint::black_box;

fn main() {
    { let v: u128 = black_box(0u128 as u128); let r = v as u64; println!("direct/u128->u64/0: {}", r); }
    { let v: u128 = black_box(0u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/u128->u64/0: {}", r); }
    { let v: u128 = black_box(1u128 as u128); let r = v as u64; println!("direct/u128->u64/1: {}", r); }
    { let v: u128 = black_box(1u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/u128->u64/1: {}", r); }
    { let v: u128 = black_box(340282366920938463463374607431768211455u128 as u128); let r = v as u64; println!("direct/u128->u64/2: {}", r); }
    { let v: u128 = black_box(340282366920938463463374607431768211455u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/u128->u64/2: {}", r); }
    { let v: u128 = black_box(340282366920938463463374607431768211454u128 as u128); let r = v as u64; println!("direct/u128->u64/3: {}", r); }
    { let v: u128 = black_box(340282366920938463463374607431768211454u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/u128->u64/3: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105728u128 as u128); let r = v as u64; println!("direct/u128->u64/4: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105728u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/u128->u64/4: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105727u128 as u128); let r = v as u64; println!("direct/u128->u64/5: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105727u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/u128->u64/5: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105729u128 as u128); let r = v as u64; println!("direct/u128->u64/6: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105729u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as u64; println!("transmute-roundtrip/u128->u64/6: {}", r); }
}
