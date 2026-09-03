use std::hint::black_box;

fn main() {
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_add(b); println!("checked_add/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_add(b); println!("overflowing_add/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_add(b); println!("wrapping_add/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_add(b); println!("saturating_add/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_sub(b); println!("checked_sub/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_sub(b); println!("overflowing_sub/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_sub(b); println!("wrapping_sub/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_sub(b); println!("saturating_sub/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.checked_mul(b); println!("checked_mul/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.overflowing_mul(b); println!("overflowing_mul/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.wrapping_mul(b); println!("wrapping_mul/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/0: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/1: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/2: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/3: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/4: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/5: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/6: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/7: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/8: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/9: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/10: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/11: {:?}", r); }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/12: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/13: {:?}", r); }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/14: {:?}", r); }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/15: {:?}", r); }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/16: {:?}", r); }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/17: {:?}", r); }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); let r = a.saturating_mul(b); println!("saturating_mul/i32/18: {:?}", r); }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/0: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/1: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/2: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/3: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/4: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/5: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/6: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/7: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/8: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/9: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/10: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/11: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/12: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/13: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/14: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/15: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/16: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/17: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_div(b); println!("checked_div/i32/18: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/0: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/1: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/2: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/3: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/4: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/5: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/6: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/7: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/8: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/9: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/10: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/11: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/12: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/13: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/14: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/15: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/16: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/17: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i32/18: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/0: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/1: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/2: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/3: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/4: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/5: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/6: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/7: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/8: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/9: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/10: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/11: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/12: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/13: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/14: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/15: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/16: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/17: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i32/18: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/0: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/1: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/2: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/3: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/4: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/5: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/6: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/7: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/8: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/9: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/10: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/11: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/12: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/13: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/14: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/15: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/16: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/17: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i32/18: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/0: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/1: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/2: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/3: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/4: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/5: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/6: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/7: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/8: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/9: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/10: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/11: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/12: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/13: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/14: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/15: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/16: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/17: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i32/18: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/0: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(1i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/1: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(-1i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/2: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(-2147483648i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/3: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(-2147483647i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/4: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483647i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/5: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/6: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/7: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/8: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/9: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/10: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/11: {:?}", r); } }
    { let a: i32 = black_box(2147483646i32 as i32); let b: i32 = black_box(0i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/12: {:?}", r); } }
    { let a: i32 = black_box(0i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/13: {:?}", r); } }
    { let a: i32 = black_box(1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/14: {:?}", r); } }
    { let a: i32 = black_box(-1i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/15: {:?}", r); } }
    { let a: i32 = black_box(-2147483648i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/16: {:?}", r); } }
    { let a: i32 = black_box(-2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/17: {:?}", r); } }
    { let a: i32 = black_box(2147483647i32 as i32); let b: i32 = black_box(2147483646i32 as i32); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i32/18: {:?}", r); } }
}
