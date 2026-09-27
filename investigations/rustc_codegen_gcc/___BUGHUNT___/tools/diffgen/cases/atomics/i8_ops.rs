use std::sync::atomic::*;
use std::hint::black_box;

fn main() {
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_add(black_box(0i8), Ordering::SeqCst); println!("fetch_add/i8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_add(black_box(1i8), Ordering::SeqCst); println!("fetch_add/i8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_add(black_box(-1i8), Ordering::SeqCst); println!("fetch_add/i8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_add(black_box(-128i8), Ordering::SeqCst); println!("fetch_add/i8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_add(black_box(-127i8), Ordering::SeqCst); println!("fetch_add/i8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_add(black_box(127i8), Ordering::SeqCst); println!("fetch_add/i8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_add(black_box(126i8), Ordering::SeqCst); println!("fetch_add/i8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_add(black_box(0i8), Ordering::SeqCst); println!("fetch_add/i8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_add(black_box(0i8), Ordering::SeqCst); println!("fetch_add/i8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_add(black_box(0i8), Ordering::SeqCst); println!("fetch_add/i8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_add(black_box(0i8), Ordering::SeqCst); println!("fetch_add/i8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_add(black_box(0i8), Ordering::SeqCst); println!("fetch_add/i8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_add(black_box(0i8), Ordering::SeqCst); println!("fetch_add/i8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_add(black_box(126i8), Ordering::SeqCst); println!("fetch_add/i8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_add(black_box(126i8), Ordering::SeqCst); println!("fetch_add/i8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_add(black_box(126i8), Ordering::SeqCst); println!("fetch_add/i8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_add(black_box(126i8), Ordering::SeqCst); println!("fetch_add/i8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_add(black_box(126i8), Ordering::SeqCst); println!("fetch_add/i8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_add(black_box(126i8), Ordering::SeqCst); println!("fetch_add/i8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_sub(black_box(0i8), Ordering::SeqCst); println!("fetch_sub/i8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_sub(black_box(1i8), Ordering::SeqCst); println!("fetch_sub/i8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_sub(black_box(-1i8), Ordering::SeqCst); println!("fetch_sub/i8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_sub(black_box(-128i8), Ordering::SeqCst); println!("fetch_sub/i8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_sub(black_box(-127i8), Ordering::SeqCst); println!("fetch_sub/i8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_sub(black_box(127i8), Ordering::SeqCst); println!("fetch_sub/i8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_sub(black_box(126i8), Ordering::SeqCst); println!("fetch_sub/i8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_sub(black_box(0i8), Ordering::SeqCst); println!("fetch_sub/i8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_sub(black_box(0i8), Ordering::SeqCst); println!("fetch_sub/i8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_sub(black_box(0i8), Ordering::SeqCst); println!("fetch_sub/i8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_sub(black_box(0i8), Ordering::SeqCst); println!("fetch_sub/i8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_sub(black_box(0i8), Ordering::SeqCst); println!("fetch_sub/i8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_sub(black_box(0i8), Ordering::SeqCst); println!("fetch_sub/i8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_sub(black_box(126i8), Ordering::SeqCst); println!("fetch_sub/i8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_sub(black_box(126i8), Ordering::SeqCst); println!("fetch_sub/i8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_sub(black_box(126i8), Ordering::SeqCst); println!("fetch_sub/i8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_sub(black_box(126i8), Ordering::SeqCst); println!("fetch_sub/i8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_sub(black_box(126i8), Ordering::SeqCst); println!("fetch_sub/i8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_sub(black_box(126i8), Ordering::SeqCst); println!("fetch_sub/i8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_and(black_box(0i8), Ordering::SeqCst); println!("fetch_and/i8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_and(black_box(1i8), Ordering::SeqCst); println!("fetch_and/i8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_and(black_box(-1i8), Ordering::SeqCst); println!("fetch_and/i8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_and(black_box(-128i8), Ordering::SeqCst); println!("fetch_and/i8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_and(black_box(-127i8), Ordering::SeqCst); println!("fetch_and/i8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_and(black_box(127i8), Ordering::SeqCst); println!("fetch_and/i8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_and(black_box(126i8), Ordering::SeqCst); println!("fetch_and/i8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_and(black_box(0i8), Ordering::SeqCst); println!("fetch_and/i8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_and(black_box(0i8), Ordering::SeqCst); println!("fetch_and/i8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_and(black_box(0i8), Ordering::SeqCst); println!("fetch_and/i8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_and(black_box(0i8), Ordering::SeqCst); println!("fetch_and/i8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_and(black_box(0i8), Ordering::SeqCst); println!("fetch_and/i8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_and(black_box(0i8), Ordering::SeqCst); println!("fetch_and/i8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_and(black_box(126i8), Ordering::SeqCst); println!("fetch_and/i8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_and(black_box(126i8), Ordering::SeqCst); println!("fetch_and/i8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_and(black_box(126i8), Ordering::SeqCst); println!("fetch_and/i8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_and(black_box(126i8), Ordering::SeqCst); println!("fetch_and/i8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_and(black_box(126i8), Ordering::SeqCst); println!("fetch_and/i8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_and(black_box(126i8), Ordering::SeqCst); println!("fetch_and/i8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_or(black_box(0i8), Ordering::SeqCst); println!("fetch_or/i8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_or(black_box(1i8), Ordering::SeqCst); println!("fetch_or/i8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_or(black_box(-1i8), Ordering::SeqCst); println!("fetch_or/i8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_or(black_box(-128i8), Ordering::SeqCst); println!("fetch_or/i8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_or(black_box(-127i8), Ordering::SeqCst); println!("fetch_or/i8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_or(black_box(127i8), Ordering::SeqCst); println!("fetch_or/i8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_or(black_box(126i8), Ordering::SeqCst); println!("fetch_or/i8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_or(black_box(0i8), Ordering::SeqCst); println!("fetch_or/i8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_or(black_box(0i8), Ordering::SeqCst); println!("fetch_or/i8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_or(black_box(0i8), Ordering::SeqCst); println!("fetch_or/i8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_or(black_box(0i8), Ordering::SeqCst); println!("fetch_or/i8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_or(black_box(0i8), Ordering::SeqCst); println!("fetch_or/i8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_or(black_box(0i8), Ordering::SeqCst); println!("fetch_or/i8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_or(black_box(126i8), Ordering::SeqCst); println!("fetch_or/i8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_or(black_box(126i8), Ordering::SeqCst); println!("fetch_or/i8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_or(black_box(126i8), Ordering::SeqCst); println!("fetch_or/i8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_or(black_box(126i8), Ordering::SeqCst); println!("fetch_or/i8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_or(black_box(126i8), Ordering::SeqCst); println!("fetch_or/i8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_or(black_box(126i8), Ordering::SeqCst); println!("fetch_or/i8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_xor(black_box(0i8), Ordering::SeqCst); println!("fetch_xor/i8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_xor(black_box(1i8), Ordering::SeqCst); println!("fetch_xor/i8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_xor(black_box(-1i8), Ordering::SeqCst); println!("fetch_xor/i8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_xor(black_box(-128i8), Ordering::SeqCst); println!("fetch_xor/i8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_xor(black_box(-127i8), Ordering::SeqCst); println!("fetch_xor/i8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_xor(black_box(127i8), Ordering::SeqCst); println!("fetch_xor/i8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_xor(black_box(126i8), Ordering::SeqCst); println!("fetch_xor/i8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_xor(black_box(0i8), Ordering::SeqCst); println!("fetch_xor/i8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_xor(black_box(0i8), Ordering::SeqCst); println!("fetch_xor/i8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_xor(black_box(0i8), Ordering::SeqCst); println!("fetch_xor/i8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_xor(black_box(0i8), Ordering::SeqCst); println!("fetch_xor/i8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_xor(black_box(0i8), Ordering::SeqCst); println!("fetch_xor/i8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_xor(black_box(0i8), Ordering::SeqCst); println!("fetch_xor/i8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_xor(black_box(126i8), Ordering::SeqCst); println!("fetch_xor/i8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_xor(black_box(126i8), Ordering::SeqCst); println!("fetch_xor/i8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_xor(black_box(126i8), Ordering::SeqCst); println!("fetch_xor/i8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_xor(black_box(126i8), Ordering::SeqCst); println!("fetch_xor/i8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_xor(black_box(126i8), Ordering::SeqCst); println!("fetch_xor/i8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_xor(black_box(126i8), Ordering::SeqCst); println!("fetch_xor/i8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_nand(black_box(0i8), Ordering::SeqCst); println!("fetch_nand/i8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_nand(black_box(1i8), Ordering::SeqCst); println!("fetch_nand/i8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_nand(black_box(-1i8), Ordering::SeqCst); println!("fetch_nand/i8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_nand(black_box(-128i8), Ordering::SeqCst); println!("fetch_nand/i8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_nand(black_box(-127i8), Ordering::SeqCst); println!("fetch_nand/i8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_nand(black_box(127i8), Ordering::SeqCst); println!("fetch_nand/i8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_nand(black_box(126i8), Ordering::SeqCst); println!("fetch_nand/i8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_nand(black_box(0i8), Ordering::SeqCst); println!("fetch_nand/i8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_nand(black_box(0i8), Ordering::SeqCst); println!("fetch_nand/i8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_nand(black_box(0i8), Ordering::SeqCst); println!("fetch_nand/i8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_nand(black_box(0i8), Ordering::SeqCst); println!("fetch_nand/i8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_nand(black_box(0i8), Ordering::SeqCst); println!("fetch_nand/i8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_nand(black_box(0i8), Ordering::SeqCst); println!("fetch_nand/i8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_nand(black_box(126i8), Ordering::SeqCst); println!("fetch_nand/i8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_nand(black_box(126i8), Ordering::SeqCst); println!("fetch_nand/i8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_nand(black_box(126i8), Ordering::SeqCst); println!("fetch_nand/i8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_nand(black_box(126i8), Ordering::SeqCst); println!("fetch_nand/i8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_nand(black_box(126i8), Ordering::SeqCst); println!("fetch_nand/i8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_nand(black_box(126i8), Ordering::SeqCst); println!("fetch_nand/i8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_max(black_box(0i8), Ordering::SeqCst); println!("fetch_max/i8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_max(black_box(1i8), Ordering::SeqCst); println!("fetch_max/i8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_max(black_box(-1i8), Ordering::SeqCst); println!("fetch_max/i8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_max(black_box(-128i8), Ordering::SeqCst); println!("fetch_max/i8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_max(black_box(-127i8), Ordering::SeqCst); println!("fetch_max/i8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_max(black_box(127i8), Ordering::SeqCst); println!("fetch_max/i8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_max(black_box(126i8), Ordering::SeqCst); println!("fetch_max/i8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_max(black_box(0i8), Ordering::SeqCst); println!("fetch_max/i8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_max(black_box(0i8), Ordering::SeqCst); println!("fetch_max/i8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_max(black_box(0i8), Ordering::SeqCst); println!("fetch_max/i8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_max(black_box(0i8), Ordering::SeqCst); println!("fetch_max/i8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_max(black_box(0i8), Ordering::SeqCst); println!("fetch_max/i8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_max(black_box(0i8), Ordering::SeqCst); println!("fetch_max/i8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_max(black_box(126i8), Ordering::SeqCst); println!("fetch_max/i8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_max(black_box(126i8), Ordering::SeqCst); println!("fetch_max/i8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_max(black_box(126i8), Ordering::SeqCst); println!("fetch_max/i8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_max(black_box(126i8), Ordering::SeqCst); println!("fetch_max/i8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_max(black_box(126i8), Ordering::SeqCst); println!("fetch_max/i8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_max(black_box(126i8), Ordering::SeqCst); println!("fetch_max/i8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_min(black_box(0i8), Ordering::SeqCst); println!("fetch_min/i8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_min(black_box(1i8), Ordering::SeqCst); println!("fetch_min/i8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_min(black_box(-1i8), Ordering::SeqCst); println!("fetch_min/i8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_min(black_box(-128i8), Ordering::SeqCst); println!("fetch_min/i8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_min(black_box(-127i8), Ordering::SeqCst); println!("fetch_min/i8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_min(black_box(127i8), Ordering::SeqCst); println!("fetch_min/i8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_min(black_box(126i8), Ordering::SeqCst); println!("fetch_min/i8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_min(black_box(0i8), Ordering::SeqCst); println!("fetch_min/i8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_min(black_box(0i8), Ordering::SeqCst); println!("fetch_min/i8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_min(black_box(0i8), Ordering::SeqCst); println!("fetch_min/i8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_min(black_box(0i8), Ordering::SeqCst); println!("fetch_min/i8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_min(black_box(0i8), Ordering::SeqCst); println!("fetch_min/i8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.fetch_min(black_box(0i8), Ordering::SeqCst); println!("fetch_min/i8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.fetch_min(black_box(126i8), Ordering::SeqCst); println!("fetch_min/i8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.fetch_min(black_box(126i8), Ordering::SeqCst); println!("fetch_min/i8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.fetch_min(black_box(126i8), Ordering::SeqCst); println!("fetch_min/i8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.fetch_min(black_box(126i8), Ordering::SeqCst); println!("fetch_min/i8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.fetch_min(black_box(126i8), Ordering::SeqCst); println!("fetch_min/i8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.fetch_min(black_box(126i8), Ordering::SeqCst); println!("fetch_min/i8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.swap(black_box(0i8), Ordering::SeqCst); println!("swap/i8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let r = a.compare_exchange(black_box(0i8), black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let wrong = black_box(0i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.swap(black_box(1i8), Ordering::SeqCst); println!("swap/i8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let r = a.compare_exchange(black_box(1i8), black_box(1i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let wrong = black_box(1i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(1i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(1i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.swap(black_box(-1i8), Ordering::SeqCst); println!("swap/i8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let r = a.compare_exchange(black_box(-1i8), black_box(-1i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let wrong = black_box(-1i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(-1i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(-1i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.swap(black_box(-128i8), Ordering::SeqCst); println!("swap/i8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let r = a.compare_exchange(black_box(-128i8), black_box(-128i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let wrong = black_box(-128i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(-128i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(-128i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.swap(black_box(-127i8), Ordering::SeqCst); println!("swap/i8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let r = a.compare_exchange(black_box(-127i8), black_box(-127i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let wrong = black_box(-127i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(-127i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(-127i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.swap(black_box(127i8), Ordering::SeqCst); println!("swap/i8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let r = a.compare_exchange(black_box(127i8), black_box(127i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let wrong = black_box(127i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(127i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(127i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.swap(black_box(126i8), Ordering::SeqCst); println!("swap/i8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let r = a.compare_exchange(black_box(126i8), black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let wrong = black_box(126i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(126i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.swap(black_box(0i8), Ordering::SeqCst); println!("swap/i8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let r = a.compare_exchange(black_box(1i8), black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let wrong = black_box(1i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.swap(black_box(0i8), Ordering::SeqCst); println!("swap/i8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let r = a.compare_exchange(black_box(-1i8), black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let wrong = black_box(-1i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.swap(black_box(0i8), Ordering::SeqCst); println!("swap/i8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let r = a.compare_exchange(black_box(-128i8), black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let wrong = black_box(-128i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.swap(black_box(0i8), Ordering::SeqCst); println!("swap/i8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let r = a.compare_exchange(black_box(-127i8), black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let wrong = black_box(-127i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.swap(black_box(0i8), Ordering::SeqCst); println!("swap/i8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let r = a.compare_exchange(black_box(127i8), black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let wrong = black_box(127i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let old = a.swap(black_box(0i8), Ordering::SeqCst); println!("swap/i8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let r = a.compare_exchange(black_box(126i8), black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let wrong = black_box(126i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(126i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let old = a.swap(black_box(126i8), Ordering::SeqCst); println!("swap/i8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let r = a.compare_exchange(black_box(0i8), black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let wrong = black_box(0i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(0i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(126i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let old = a.swap(black_box(126i8), Ordering::SeqCst); println!("swap/i8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let r = a.compare_exchange(black_box(1i8), black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let wrong = black_box(1i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(1i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(126i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let old = a.swap(black_box(126i8), Ordering::SeqCst); println!("swap/i8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let r = a.compare_exchange(black_box(-1i8), black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let wrong = black_box(-1i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-1i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(126i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let old = a.swap(black_box(126i8), Ordering::SeqCst); println!("swap/i8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let r = a.compare_exchange(black_box(-128i8), black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let wrong = black_box(-128i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-128i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(126i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let old = a.swap(black_box(126i8), Ordering::SeqCst); println!("swap/i8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let r = a.compare_exchange(black_box(-127i8), black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let wrong = black_box(-127i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(-127i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(126i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let old = a.swap(black_box(126i8), Ordering::SeqCst); println!("swap/i8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let r = a.compare_exchange(black_box(127i8), black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i8/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let wrong = black_box(127i8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(126i8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i8/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI8::new(black_box(127i8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(126i8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i8/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
}
