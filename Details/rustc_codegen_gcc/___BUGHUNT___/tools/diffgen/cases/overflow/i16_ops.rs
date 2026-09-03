use std::hint::black_box;

fn main() {
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_add(b); println!("checked_add/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_add(b); println!("overflowing_add/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_add(b); println!("wrapping_add/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_add(b); println!("saturating_add/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_sub(b); println!("checked_sub/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_sub(b); println!("overflowing_sub/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_sub(b); println!("wrapping_sub/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_sub(b); println!("saturating_sub/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.checked_mul(b); println!("checked_mul/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.overflowing_mul(b); println!("overflowing_mul/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.wrapping_mul(b); println!("wrapping_mul/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/0: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/1: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/2: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/3: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/4: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/5: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/6: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/7: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/8: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/9: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/10: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/11: {:?}", r); }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/12: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/13: {:?}", r); }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/14: {:?}", r); }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/15: {:?}", r); }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/16: {:?}", r); }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/17: {:?}", r); }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); let r = a.saturating_mul(b); println!("saturating_mul/i16/18: {:?}", r); }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/0: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/1: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/2: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/3: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/4: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/5: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/6: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/7: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/8: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/9: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/10: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/11: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/12: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/13: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/14: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/15: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/16: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/17: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_div(b); println!("checked_div/i16/18: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/0: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/1: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/2: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/3: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/4: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/5: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/6: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/7: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/8: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/9: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/10: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/11: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/12: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/13: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/14: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/15: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/16: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/17: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_div(b); println!("overflowing_div/i16/18: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/0: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/1: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/2: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/3: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/4: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/5: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/6: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/7: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/8: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/9: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/10: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/11: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/12: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/13: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/14: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/15: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/16: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/17: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_div(b); println!("wrapping_div/i16/18: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/0: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/1: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/2: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/3: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/4: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/5: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/6: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/7: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/8: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/9: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/10: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/11: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/12: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/13: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/14: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/15: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/16: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/17: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.checked_rem(b); println!("checked_rem/i16/18: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/0: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/1: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/2: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/3: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/4: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/5: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/6: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/7: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/8: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/9: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/10: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/11: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/12: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/13: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/14: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/15: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/16: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/17: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.overflowing_rem(b); println!("overflowing_rem/i16/18: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/0: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(1i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/1: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(-1i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/2: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(-32768i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/3: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(-32767i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/4: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32767i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/5: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/6: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/7: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/8: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/9: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/10: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/11: {:?}", r); } }
    { let a: i16 = black_box(32766i16 as i16); let b: i16 = black_box(0i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/12: {:?}", r); } }
    { let a: i16 = black_box(0i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/13: {:?}", r); } }
    { let a: i16 = black_box(1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/14: {:?}", r); } }
    { let a: i16 = black_box(-1i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/15: {:?}", r); } }
    { let a: i16 = black_box(-32768i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/16: {:?}", r); } }
    { let a: i16 = black_box(-32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/17: {:?}", r); } }
    { let a: i16 = black_box(32767i16 as i16); let b: i16 = black_box(32766i16 as i16); if b != 0 { let r = a.wrapping_rem(b); println!("wrapping_rem/i16/18: {:?}", r); } }
}
