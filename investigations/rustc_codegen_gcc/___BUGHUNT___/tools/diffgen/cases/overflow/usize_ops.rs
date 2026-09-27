use std::hint::black_box;

fn main() {
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_add(b); println!("checked_add/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.checked_add(b); println!("checked_add/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.checked_add(b); println!("checked_add/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.checked_add(b); println!("checked_add/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.checked_add(b); println!("checked_add/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.checked_add(b); println!("checked_add/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_add(b); println!("checked_add/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_add(b); println!("checked_add/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_add(b); println!("checked_add/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_add(b); println!("checked_add/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_add(b); println!("checked_add/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_add(b); println!("checked_add/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_add(b); println!("checked_add/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_add(b); println!("checked_add/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_add(b); println!("checked_add/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_add(b); println!("checked_add/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_add(b); println!("checked_add/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_add(b); println!("checked_add/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_add(b); println!("checked_add/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_add(b); println!("overflowing_add/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_add(b); println!("wrapping_add/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_add(b); println!("saturating_add/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_sub(b); println!("checked_sub/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_sub(b); println!("overflowing_sub/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_sub(b); println!("wrapping_sub/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_sub(b); println!("saturating_sub/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.checked_mul(b); println!("checked_mul/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.overflowing_mul(b); println!("overflowing_mul/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.wrapping_mul(b); println!("wrapping_mul/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/0: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/1: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/2: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/3: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/4: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/5: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/6: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/7: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/8: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/9: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/10: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/11: {:?}", r); }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/12: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/13: {:?}", r); }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/14: {:?}", r); }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/15: {:?}", r); }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/16: {:?}", r); }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/17: {:?}", r); }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); let r = a.saturating_mul(b); println!("saturating_mul/usize/18: {:?}", r); }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/0: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/1: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/2: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/3: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/4: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/5: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/6: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/7: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/8: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/9: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/10: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/11: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/12: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/13: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/14: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/15: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/16: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/17: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_div(b); println!("checked_div/usize/18: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/0: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/1: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/2: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/3: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/4: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/5: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/6: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/7: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/8: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/9: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/10: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/11: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/12: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/13: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/14: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/15: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/16: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/17: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/usize/18: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/0: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/1: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/2: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/3: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/4: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/5: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/6: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/7: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/8: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/9: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/10: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/11: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/12: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/13: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/14: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/15: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/16: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/17: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/usize/18: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/0: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/1: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/2: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/3: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/4: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/5: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/6: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/7: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/8: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/9: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/10: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/11: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/12: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/13: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/14: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/15: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/16: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/17: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/usize/18: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/0: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/1: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/2: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/3: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/4: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/5: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/6: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/7: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/8: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/9: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/10: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/11: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/12: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/13: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/14: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/15: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/16: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/17: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/usize/18: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/0: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(1usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/1: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(18446744073709551615usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/2: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(18446744073709551614usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/3: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775808usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/4: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775807usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/5: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/6: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/7: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/8: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/9: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/10: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/11: {:?}", r); } }
    { let a: usize = black_box(9223372036854775809usize as usize); let b: usize = black_box(0usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/12: {:?}", r); } }
    { let a: usize = black_box(0usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/13: {:?}", r); } }
    { let a: usize = black_box(1usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/14: {:?}", r); } }
    { let a: usize = black_box(18446744073709551615usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/15: {:?}", r); } }
    { let a: usize = black_box(18446744073709551614usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/16: {:?}", r); } }
    { let a: usize = black_box(9223372036854775808usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/17: {:?}", r); } }
    { let a: usize = black_box(9223372036854775807usize as usize); let b: usize = black_box(9223372036854775809usize as usize); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/usize/18: {:?}", r); } }
}
