use std::hint::black_box;

fn main() {
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_add(b); println!("checked_add/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_add(b); println!("overflowing_add/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_add(b); println!("wrapping_add/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_add(b); println!("saturating_add/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_sub(b); println!("checked_sub/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_sub(b); println!("overflowing_sub/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_sub(b); println!("wrapping_sub/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_sub(b); println!("saturating_sub/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.checked_mul(b); println!("checked_mul/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.overflowing_mul(b); println!("overflowing_mul/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.wrapping_mul(b); println!("wrapping_mul/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/0: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/1: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/2: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/3: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/4: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/5: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/6: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/7: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/8: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/9: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/10: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/11: {:?}", r); }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/12: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/13: {:?}", r); }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/14: {:?}", r); }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/15: {:?}", r); }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/16: {:?}", r); }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/17: {:?}", r); }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); let r = a.saturating_mul(b); println!("saturating_mul/u64/18: {:?}", r); }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/0: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/1: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/2: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/3: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/4: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/5: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/6: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/7: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/8: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/9: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/10: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/11: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/12: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/13: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/14: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/15: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/16: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/17: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_div(b); println!("checked_div/u64/18: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/0: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/1: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/2: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/3: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/4: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/5: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/6: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/7: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/8: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/9: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/10: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/11: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/12: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/13: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/14: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/15: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/16: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/17: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u64/18: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/0: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/1: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/2: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/3: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/4: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/5: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/6: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/7: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/8: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/9: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/10: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/11: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/12: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/13: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/14: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/15: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/16: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/17: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u64/18: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/0: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/1: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/2: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/3: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/4: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/5: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/6: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/7: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/8: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/9: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/10: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/11: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/12: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/13: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/14: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/15: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/16: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/17: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u64/18: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/0: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/1: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/2: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/3: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/4: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/5: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/6: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/7: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/8: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/9: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/10: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/11: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/12: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/13: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/14: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/15: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/16: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/17: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u64/18: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/0: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(1u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/1: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(18446744073709551615u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/2: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(18446744073709551614u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/3: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775808u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/4: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775807u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/5: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/6: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/7: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/8: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/9: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/10: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/11: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775809u64 as u64); let b: u64 = black_box(0u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/12: {:?}", r); } }
    { let a: u64 = black_box(0u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/13: {:?}", r); } }
    { let a: u64 = black_box(1u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/14: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551615u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/15: {:?}", r); } }
    { let a: u64 = black_box(18446744073709551614u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/16: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775808u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/17: {:?}", r); } }
    { let a: u64 = black_box(9223372036854775807u64 as u64); let b: u64 = black_box(9223372036854775809u64 as u64); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u64/18: {:?}", r); } }
}
