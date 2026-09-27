use std::hint::black_box;

fn main() {
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_add(b); println!("checked_add/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.checked_add(b); println!("checked_add/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.checked_add(b); println!("checked_add/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.checked_add(b); println!("checked_add/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.checked_add(b); println!("checked_add/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.checked_add(b); println!("checked_add/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_add(b); println!("checked_add/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_add(b); println!("checked_add/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_add(b); println!("checked_add/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_add(b); println!("checked_add/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_add(b); println!("checked_add/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_add(b); println!("checked_add/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_add(b); println!("checked_add/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_add(b); println!("checked_add/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_add(b); println!("checked_add/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_add(b); println!("checked_add/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_add(b); println!("checked_add/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_add(b); println!("checked_add/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_add(b); println!("checked_add/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_add(b); println!("overflowing_add/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_add(b); println!("wrapping_add/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_add(b); println!("saturating_add/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_sub(b); println!("checked_sub/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_sub(b); println!("overflowing_sub/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_sub(b); println!("wrapping_sub/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_sub(b); println!("saturating_sub/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.checked_mul(b); println!("checked_mul/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.overflowing_mul(b); println!("overflowing_mul/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.wrapping_mul(b); println!("wrapping_mul/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/0: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/1: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/2: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/3: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/4: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/5: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/6: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/7: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/8: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/9: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/10: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/11: {:?}", r); }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/12: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/13: {:?}", r); }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/14: {:?}", r); }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/15: {:?}", r); }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/16: {:?}", r); }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/17: {:?}", r); }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); let r = a.saturating_mul(b); println!("saturating_mul/isize/18: {:?}", r); }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/0: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/1: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/2: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/3: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/4: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/5: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/6: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/7: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/8: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/9: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/10: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/11: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/12: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/13: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/14: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/15: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/16: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/17: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_div(b); println!("checked_div/isize/18: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/0: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/1: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/2: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/3: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/4: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/5: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/6: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/7: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/8: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/9: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/10: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/11: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/12: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/13: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/14: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/15: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/16: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/17: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/isize/18: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/0: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/1: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/2: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/3: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/4: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/5: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/6: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/7: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/8: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/9: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/10: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/11: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/12: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/13: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/14: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/15: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/16: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/17: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/isize/18: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/0: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/1: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/2: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/3: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/4: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/5: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/6: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/7: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/8: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/9: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/10: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/11: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/12: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/13: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/14: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/15: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/16: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/17: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/isize/18: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/0: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/1: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/2: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/3: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/4: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/5: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/6: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/7: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/8: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/9: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/10: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/11: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/12: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/13: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/14: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/15: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/16: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/17: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/isize/18: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/0: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(1isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/1: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(-1isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/2: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(-9223372036854775808isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/3: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(-9223372036854775807isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/4: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775807isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/5: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/6: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/7: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/8: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/9: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/10: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/11: {:?}", r); } }
    { let a: isize = black_box(9223372036854775806isize as isize); let b: isize = black_box(0isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/12: {:?}", r); } }
    { let a: isize = black_box(0isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/13: {:?}", r); } }
    { let a: isize = black_box(1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/14: {:?}", r); } }
    { let a: isize = black_box(-1isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/15: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775808isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/16: {:?}", r); } }
    { let a: isize = black_box(-9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/17: {:?}", r); } }
    { let a: isize = black_box(9223372036854775807isize as isize); let b: isize = black_box(9223372036854775806isize as isize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/isize/18: {:?}", r); } }
}
