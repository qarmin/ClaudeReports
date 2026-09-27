use std::hint::black_box;

fn main() {
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_add(b); println!("checked_add/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_add(b); println!("overflowing_add/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_add(b); println!("wrapping_add/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_add(b); println!("saturating_add/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_sub(b); println!("checked_sub/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_sub(b); println!("overflowing_sub/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_sub(b); println!("wrapping_sub/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_sub(b); println!("saturating_sub/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.checked_mul(b); println!("checked_mul/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.overflowing_mul(b); println!("overflowing_mul/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.wrapping_mul(b); println!("wrapping_mul/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/0: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/1: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/2: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/3: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/4: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/5: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/6: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/7: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/8: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/9: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/10: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/11: {:?}", r); }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/12: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/13: {:?}", r); }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/14: {:?}", r); }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/15: {:?}", r); }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/16: {:?}", r); }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/17: {:?}", r); }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); let r = a.saturating_mul(b); println!("saturating_mul/u8/18: {:?}", r); }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/0: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/1: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/2: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/3: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/4: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/5: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/6: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/7: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/8: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/9: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/10: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/11: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/12: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/13: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/14: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/15: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/16: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/17: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_div(b); println!("checked_div/u8/18: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/0: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/1: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/2: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/3: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/4: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/5: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/6: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/7: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/8: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/9: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/10: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/11: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/12: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/13: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/14: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/15: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/16: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/17: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u8/18: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/0: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/1: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/2: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/3: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/4: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/5: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/6: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/7: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/8: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/9: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/10: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/11: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/12: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/13: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/14: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/15: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/16: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/17: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u8/18: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/0: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/1: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/2: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/3: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/4: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/5: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/6: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/7: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/8: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/9: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/10: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/11: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/12: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/13: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/14: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/15: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/16: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/17: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u8/18: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/0: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/1: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/2: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/3: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/4: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/5: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/6: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/7: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/8: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/9: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/10: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/11: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/12: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/13: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/14: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/15: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/16: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/17: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u8/18: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/0: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(1u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/1: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(255u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/2: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(254u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/3: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(128u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/4: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(127u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/5: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/6: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/7: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/8: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/9: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/10: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/11: {:?}", r); } }
    { let a: u8 = black_box(129u8 as u8); let b: u8 = black_box(0u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/12: {:?}", r); } }
    { let a: u8 = black_box(0u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/13: {:?}", r); } }
    { let a: u8 = black_box(1u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/14: {:?}", r); } }
    { let a: u8 = black_box(255u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/15: {:?}", r); } }
    { let a: u8 = black_box(254u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/16: {:?}", r); } }
    { let a: u8 = black_box(128u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/17: {:?}", r); } }
    { let a: u8 = black_box(127u8 as u8); let b: u8 = black_box(129u8 as u8); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u8/18: {:?}", r); } }
}
