use std::hint::black_box;

fn main() {
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_add(b); println!("checked_add/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_add(b); println!("overflowing_add/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_add(b); println!("wrapping_add/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_add(b); println!("saturating_add/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_sub(b); println!("checked_sub/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_sub(b); println!("overflowing_sub/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_sub(b); println!("wrapping_sub/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_sub(b); println!("saturating_sub/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.checked_mul(b); println!("checked_mul/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.overflowing_mul(b); println!("overflowing_mul/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.wrapping_mul(b); println!("wrapping_mul/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/0: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/1: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/2: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/3: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/4: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/5: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/6: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/7: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/8: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/9: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/10: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/11: {:?}", r); }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/12: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/13: {:?}", r); }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/14: {:?}", r); }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/15: {:?}", r); }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/16: {:?}", r); }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/17: {:?}", r); }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); let r = a.saturating_mul(b); println!("saturating_mul/i8/18: {:?}", r); }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/0: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/1: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/2: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/3: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/4: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/5: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/6: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/7: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/8: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/9: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/10: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/11: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/12: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/13: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/14: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/15: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/16: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/17: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_div(b); println!("checked_div/i8/18: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/0: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/1: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/2: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/3: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/4: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/5: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/6: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/7: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/8: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/9: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/10: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/11: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/12: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/13: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/14: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/15: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/16: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/17: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i8/18: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/0: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/1: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/2: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/3: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/4: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/5: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/6: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/7: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/8: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/9: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/10: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/11: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/12: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/13: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/14: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/15: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/16: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/17: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i8/18: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/0: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/1: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/2: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/3: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/4: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/5: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/6: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/7: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/8: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/9: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/10: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/11: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/12: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/13: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/14: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/15: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/16: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/17: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i8/18: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/0: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/1: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/2: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/3: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/4: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/5: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/6: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/7: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/8: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/9: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/10: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/11: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/12: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/13: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/14: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/15: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/16: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/17: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i8/18: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/0: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(1i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/1: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(-1i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/2: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(-128i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/3: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(-127i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/4: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(127i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/5: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/6: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/7: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/8: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/9: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/10: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/11: {:?}", r); } }
    { let a: i8 = black_box(126i8 as i8); let b: i8 = black_box(0i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/12: {:?}", r); } }
    { let a: i8 = black_box(0i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/13: {:?}", r); } }
    { let a: i8 = black_box(1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/14: {:?}", r); } }
    { let a: i8 = black_box(-1i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/15: {:?}", r); } }
    { let a: i8 = black_box(-128i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/16: {:?}", r); } }
    { let a: i8 = black_box(-127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/17: {:?}", r); } }
    { let a: i8 = black_box(127i8 as i8); let b: i8 = black_box(126i8 as i8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i8/18: {:?}", r); } }
}
