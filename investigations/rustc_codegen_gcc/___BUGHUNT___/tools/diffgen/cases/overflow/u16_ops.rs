use std::hint::black_box;

fn main() {
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_add(b); println!("checked_add/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_add(b); println!("overflowing_add/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_add(b); println!("wrapping_add/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_add(b); println!("saturating_add/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_sub(b); println!("checked_sub/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_sub(b); println!("overflowing_sub/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_sub(b); println!("wrapping_sub/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_sub(b); println!("saturating_sub/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.checked_mul(b); println!("checked_mul/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.overflowing_mul(b); println!("overflowing_mul/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.wrapping_mul(b); println!("wrapping_mul/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/0: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/1: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/2: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/3: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/4: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/5: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/6: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/7: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/8: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/9: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/10: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/11: {:?}", r); }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/12: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/13: {:?}", r); }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/14: {:?}", r); }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/15: {:?}", r); }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/16: {:?}", r); }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/17: {:?}", r); }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); let r = a.saturating_mul(b); println!("saturating_mul/u16/18: {:?}", r); }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/0: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/1: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/2: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/3: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/4: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/5: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/6: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/7: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/8: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/9: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/10: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/11: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/12: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/13: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/14: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/15: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/16: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/17: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_div(b); println!("checked_div/u16/18: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/0: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/1: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/2: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/3: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/4: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/5: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/6: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/7: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/8: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/9: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/10: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/11: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/12: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/13: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/14: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/15: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/16: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/17: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u16/18: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/0: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/1: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/2: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/3: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/4: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/5: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/6: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/7: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/8: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/9: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/10: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/11: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/12: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/13: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/14: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/15: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/16: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/17: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u16/18: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/0: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/1: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/2: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/3: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/4: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/5: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/6: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/7: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/8: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/9: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/10: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/11: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/12: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/13: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/14: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/15: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/16: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/17: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u16/18: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/0: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/1: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/2: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/3: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/4: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/5: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/6: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/7: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/8: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/9: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/10: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/11: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/12: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/13: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/14: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/15: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/16: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/17: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u16/18: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/0: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(1u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/1: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(65535u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/2: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(65534u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/3: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32768u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/4: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32767u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/5: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/6: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/7: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/8: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/9: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/10: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/11: {:?}", r); } }
    { let a: u16 = black_box(32769u16 as u16); let b: u16 = black_box(0u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/12: {:?}", r); } }
    { let a: u16 = black_box(0u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/13: {:?}", r); } }
    { let a: u16 = black_box(1u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/14: {:?}", r); } }
    { let a: u16 = black_box(65535u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/15: {:?}", r); } }
    { let a: u16 = black_box(65534u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/16: {:?}", r); } }
    { let a: u16 = black_box(32768u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/17: {:?}", r); } }
    { let a: u16 = black_box(32767u16 as u16); let b: u16 = black_box(32769u16 as u16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u16/18: {:?}", r); } }
}
