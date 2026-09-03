use std::hint::black_box;

fn main() {
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_add(b); println!("checked_add/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_add(b); println!("overflowing_add/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_add(b); println!("wrapping_add/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_add(b); println!("saturating_add/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_sub(b); println!("checked_sub/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_sub(b); println!("overflowing_sub/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_sub(b); println!("wrapping_sub/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_sub(b); println!("saturating_sub/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.checked_mul(b); println!("checked_mul/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.overflowing_mul(b); println!("overflowing_mul/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.wrapping_mul(b); println!("wrapping_mul/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/0: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/1: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/2: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/3: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/4: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/5: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/6: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/7: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/8: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/9: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/10: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/11: {:?}", r); }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/12: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/13: {:?}", r); }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/14: {:?}", r); }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/15: {:?}", r); }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/16: {:?}", r); }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/17: {:?}", r); }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); let r = a.saturating_mul(b); println!("saturating_mul/u32/18: {:?}", r); }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/0: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/1: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/2: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/3: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/4: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/5: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/6: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/7: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/8: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/9: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/10: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/11: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/12: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/13: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/14: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/15: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/16: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/17: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_div(b); println!("checked_div/u32/18: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/0: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/1: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/2: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/3: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/4: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/5: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/6: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/7: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/8: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/9: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/10: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/11: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/12: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/13: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/14: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/15: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/16: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/17: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/u32/18: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/0: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/1: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/2: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/3: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/4: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/5: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/6: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/7: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/8: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/9: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/10: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/11: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/12: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/13: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/14: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/15: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/16: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/17: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/u32/18: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/0: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/1: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/2: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/3: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/4: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/5: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/6: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/7: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/8: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/9: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/10: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/11: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/12: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/13: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/14: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/15: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/16: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/17: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/u32/18: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/0: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/1: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/2: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/3: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/4: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/5: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/6: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/7: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/8: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/9: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/10: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/11: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/12: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/13: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/14: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/15: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/16: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/17: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/u32/18: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/0: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(1u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/1: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(4294967295u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/2: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(4294967294u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/3: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483648u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/4: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483647u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/5: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/6: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/7: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/8: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/9: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/10: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/11: {:?}", r); } }
    { let a: u32 = black_box(2147483649u32 as u32); let b: u32 = black_box(0u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/12: {:?}", r); } }
    { let a: u32 = black_box(0u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/13: {:?}", r); } }
    { let a: u32 = black_box(1u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/14: {:?}", r); } }
    { let a: u32 = black_box(4294967295u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/15: {:?}", r); } }
    { let a: u32 = black_box(4294967294u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/16: {:?}", r); } }
    { let a: u32 = black_box(2147483648u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/17: {:?}", r); } }
    { let a: u32 = black_box(2147483647u32 as u32); let b: u32 = black_box(2147483649u32 as u32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/u32/18: {:?}", r); } }
}
