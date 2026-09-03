use std::hint::black_box;

fn main() {
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_add(b); println!("checked_add/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_add(b); println!("overflowing_add/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_add(b); println!("wrapping_add/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_add(b); println!("saturating_add/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_sub(b); println!("checked_sub/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_sub(b); println!("overflowing_sub/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_sub(b); println!("wrapping_sub/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_sub(b); println!("saturating_sub/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.checked_mul(b); println!("checked_mul/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.overflowing_mul(b); println!("overflowing_mul/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.wrapping_mul(b); println!("wrapping_mul/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/0: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/1: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/2: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/3: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/4: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/5: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/6: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/7: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/8: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/9: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/10: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/11: {:?}", r); }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/12: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/13: {:?}", r); }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/14: {:?}", r); }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/15: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/16: {:?}", r); }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/17: {:?}", r); }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); let r = a.saturating_mul(b); println!("saturating_mul/i64/18: {:?}", r); }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/0: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/1: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/2: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/3: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/4: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/5: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/6: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/7: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/8: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/9: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/10: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/11: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/12: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/13: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/14: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/15: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/16: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/17: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_div(b); println!("checked_div/i64/18: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/0: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/1: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/2: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/3: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/4: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/5: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/6: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/7: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/8: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/9: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/10: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/11: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/12: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/13: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/14: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/15: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/16: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/17: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i64/18: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/0: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/1: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/2: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/3: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/4: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/5: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/6: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/7: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/8: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/9: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/10: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/11: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/12: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/13: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/14: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/15: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/16: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/17: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i64/18: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/0: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/1: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/2: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/3: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/4: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/5: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/6: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/7: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/8: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/9: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/10: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/11: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/12: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/13: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/14: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/15: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/16: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/17: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i64/18: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/0: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/1: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/2: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/3: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/4: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/5: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/6: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/7: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/8: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/9: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/10: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/11: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/12: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/13: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/14: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/15: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/16: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/17: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i64/18: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/0: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(1i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/1: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(-1i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/2: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(-9223372036854775808i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/3: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(-9223372036854775807i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/4: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775807i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/5: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/6: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/7: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/8: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/9: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/10: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/11: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775806i64 as i64); let b: i64 = black_box(0i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/12: {:?}", r); } }
    { let a: i64 = black_box(0i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/13: {:?}", r); } }
    { let a: i64 = black_box(1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/14: {:?}", r); } }
    { let a: i64 = black_box(-1i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/15: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775808i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/16: {:?}", r); } }
    { let a: i64 = black_box(-9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/17: {:?}", r); } }
    { let a: i64 = black_box(9223372036854775807i64 as i64); let b: i64 = black_box(9223372036854775806i64 as i64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i64/18: {:?}", r); } }
}
