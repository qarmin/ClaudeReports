use std::sync::atomic::*;
use std::hint::black_box;

fn main() {
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_add(black_box(0u8), Ordering::SeqCst); println!("fetch_add/u8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_add(black_box(1u8), Ordering::SeqCst); println!("fetch_add/u8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_add(black_box(255u8), Ordering::SeqCst); println!("fetch_add/u8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_add(black_box(254u8), Ordering::SeqCst); println!("fetch_add/u8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_add(black_box(128u8), Ordering::SeqCst); println!("fetch_add/u8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_add(black_box(127u8), Ordering::SeqCst); println!("fetch_add/u8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_add(black_box(129u8), Ordering::SeqCst); println!("fetch_add/u8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_add(black_box(0u8), Ordering::SeqCst); println!("fetch_add/u8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_add(black_box(0u8), Ordering::SeqCst); println!("fetch_add/u8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_add(black_box(0u8), Ordering::SeqCst); println!("fetch_add/u8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_add(black_box(0u8), Ordering::SeqCst); println!("fetch_add/u8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_add(black_box(0u8), Ordering::SeqCst); println!("fetch_add/u8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_add(black_box(0u8), Ordering::SeqCst); println!("fetch_add/u8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_add(black_box(129u8), Ordering::SeqCst); println!("fetch_add/u8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_add(black_box(129u8), Ordering::SeqCst); println!("fetch_add/u8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_add(black_box(129u8), Ordering::SeqCst); println!("fetch_add/u8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_add(black_box(129u8), Ordering::SeqCst); println!("fetch_add/u8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_add(black_box(129u8), Ordering::SeqCst); println!("fetch_add/u8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_add(black_box(129u8), Ordering::SeqCst); println!("fetch_add/u8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_sub(black_box(0u8), Ordering::SeqCst); println!("fetch_sub/u8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_sub(black_box(1u8), Ordering::SeqCst); println!("fetch_sub/u8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_sub(black_box(255u8), Ordering::SeqCst); println!("fetch_sub/u8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_sub(black_box(254u8), Ordering::SeqCst); println!("fetch_sub/u8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_sub(black_box(128u8), Ordering::SeqCst); println!("fetch_sub/u8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_sub(black_box(127u8), Ordering::SeqCst); println!("fetch_sub/u8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_sub(black_box(129u8), Ordering::SeqCst); println!("fetch_sub/u8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_sub(black_box(0u8), Ordering::SeqCst); println!("fetch_sub/u8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_sub(black_box(0u8), Ordering::SeqCst); println!("fetch_sub/u8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_sub(black_box(0u8), Ordering::SeqCst); println!("fetch_sub/u8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_sub(black_box(0u8), Ordering::SeqCst); println!("fetch_sub/u8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_sub(black_box(0u8), Ordering::SeqCst); println!("fetch_sub/u8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_sub(black_box(0u8), Ordering::SeqCst); println!("fetch_sub/u8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_sub(black_box(129u8), Ordering::SeqCst); println!("fetch_sub/u8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_sub(black_box(129u8), Ordering::SeqCst); println!("fetch_sub/u8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_sub(black_box(129u8), Ordering::SeqCst); println!("fetch_sub/u8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_sub(black_box(129u8), Ordering::SeqCst); println!("fetch_sub/u8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_sub(black_box(129u8), Ordering::SeqCst); println!("fetch_sub/u8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_sub(black_box(129u8), Ordering::SeqCst); println!("fetch_sub/u8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_and(black_box(0u8), Ordering::SeqCst); println!("fetch_and/u8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_and(black_box(1u8), Ordering::SeqCst); println!("fetch_and/u8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_and(black_box(255u8), Ordering::SeqCst); println!("fetch_and/u8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_and(black_box(254u8), Ordering::SeqCst); println!("fetch_and/u8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_and(black_box(128u8), Ordering::SeqCst); println!("fetch_and/u8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_and(black_box(127u8), Ordering::SeqCst); println!("fetch_and/u8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_and(black_box(129u8), Ordering::SeqCst); println!("fetch_and/u8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_and(black_box(0u8), Ordering::SeqCst); println!("fetch_and/u8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_and(black_box(0u8), Ordering::SeqCst); println!("fetch_and/u8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_and(black_box(0u8), Ordering::SeqCst); println!("fetch_and/u8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_and(black_box(0u8), Ordering::SeqCst); println!("fetch_and/u8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_and(black_box(0u8), Ordering::SeqCst); println!("fetch_and/u8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_and(black_box(0u8), Ordering::SeqCst); println!("fetch_and/u8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_and(black_box(129u8), Ordering::SeqCst); println!("fetch_and/u8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_and(black_box(129u8), Ordering::SeqCst); println!("fetch_and/u8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_and(black_box(129u8), Ordering::SeqCst); println!("fetch_and/u8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_and(black_box(129u8), Ordering::SeqCst); println!("fetch_and/u8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_and(black_box(129u8), Ordering::SeqCst); println!("fetch_and/u8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_and(black_box(129u8), Ordering::SeqCst); println!("fetch_and/u8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_or(black_box(0u8), Ordering::SeqCst); println!("fetch_or/u8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_or(black_box(1u8), Ordering::SeqCst); println!("fetch_or/u8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_or(black_box(255u8), Ordering::SeqCst); println!("fetch_or/u8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_or(black_box(254u8), Ordering::SeqCst); println!("fetch_or/u8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_or(black_box(128u8), Ordering::SeqCst); println!("fetch_or/u8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_or(black_box(127u8), Ordering::SeqCst); println!("fetch_or/u8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_or(black_box(129u8), Ordering::SeqCst); println!("fetch_or/u8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_or(black_box(0u8), Ordering::SeqCst); println!("fetch_or/u8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_or(black_box(0u8), Ordering::SeqCst); println!("fetch_or/u8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_or(black_box(0u8), Ordering::SeqCst); println!("fetch_or/u8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_or(black_box(0u8), Ordering::SeqCst); println!("fetch_or/u8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_or(black_box(0u8), Ordering::SeqCst); println!("fetch_or/u8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_or(black_box(0u8), Ordering::SeqCst); println!("fetch_or/u8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_or(black_box(129u8), Ordering::SeqCst); println!("fetch_or/u8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_or(black_box(129u8), Ordering::SeqCst); println!("fetch_or/u8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_or(black_box(129u8), Ordering::SeqCst); println!("fetch_or/u8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_or(black_box(129u8), Ordering::SeqCst); println!("fetch_or/u8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_or(black_box(129u8), Ordering::SeqCst); println!("fetch_or/u8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_or(black_box(129u8), Ordering::SeqCst); println!("fetch_or/u8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_xor(black_box(0u8), Ordering::SeqCst); println!("fetch_xor/u8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_xor(black_box(1u8), Ordering::SeqCst); println!("fetch_xor/u8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_xor(black_box(255u8), Ordering::SeqCst); println!("fetch_xor/u8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_xor(black_box(254u8), Ordering::SeqCst); println!("fetch_xor/u8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_xor(black_box(128u8), Ordering::SeqCst); println!("fetch_xor/u8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_xor(black_box(127u8), Ordering::SeqCst); println!("fetch_xor/u8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_xor(black_box(129u8), Ordering::SeqCst); println!("fetch_xor/u8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_xor(black_box(0u8), Ordering::SeqCst); println!("fetch_xor/u8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_xor(black_box(0u8), Ordering::SeqCst); println!("fetch_xor/u8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_xor(black_box(0u8), Ordering::SeqCst); println!("fetch_xor/u8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_xor(black_box(0u8), Ordering::SeqCst); println!("fetch_xor/u8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_xor(black_box(0u8), Ordering::SeqCst); println!("fetch_xor/u8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_xor(black_box(0u8), Ordering::SeqCst); println!("fetch_xor/u8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_xor(black_box(129u8), Ordering::SeqCst); println!("fetch_xor/u8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_xor(black_box(129u8), Ordering::SeqCst); println!("fetch_xor/u8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_xor(black_box(129u8), Ordering::SeqCst); println!("fetch_xor/u8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_xor(black_box(129u8), Ordering::SeqCst); println!("fetch_xor/u8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_xor(black_box(129u8), Ordering::SeqCst); println!("fetch_xor/u8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_xor(black_box(129u8), Ordering::SeqCst); println!("fetch_xor/u8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_nand(black_box(0u8), Ordering::SeqCst); println!("fetch_nand/u8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_nand(black_box(1u8), Ordering::SeqCst); println!("fetch_nand/u8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_nand(black_box(255u8), Ordering::SeqCst); println!("fetch_nand/u8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_nand(black_box(254u8), Ordering::SeqCst); println!("fetch_nand/u8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_nand(black_box(128u8), Ordering::SeqCst); println!("fetch_nand/u8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_nand(black_box(127u8), Ordering::SeqCst); println!("fetch_nand/u8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_nand(black_box(129u8), Ordering::SeqCst); println!("fetch_nand/u8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_nand(black_box(0u8), Ordering::SeqCst); println!("fetch_nand/u8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_nand(black_box(0u8), Ordering::SeqCst); println!("fetch_nand/u8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_nand(black_box(0u8), Ordering::SeqCst); println!("fetch_nand/u8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_nand(black_box(0u8), Ordering::SeqCst); println!("fetch_nand/u8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_nand(black_box(0u8), Ordering::SeqCst); println!("fetch_nand/u8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_nand(black_box(0u8), Ordering::SeqCst); println!("fetch_nand/u8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_nand(black_box(129u8), Ordering::SeqCst); println!("fetch_nand/u8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_nand(black_box(129u8), Ordering::SeqCst); println!("fetch_nand/u8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_nand(black_box(129u8), Ordering::SeqCst); println!("fetch_nand/u8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_nand(black_box(129u8), Ordering::SeqCst); println!("fetch_nand/u8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_nand(black_box(129u8), Ordering::SeqCst); println!("fetch_nand/u8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_nand(black_box(129u8), Ordering::SeqCst); println!("fetch_nand/u8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_max(black_box(0u8), Ordering::SeqCst); println!("fetch_max/u8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_max(black_box(1u8), Ordering::SeqCst); println!("fetch_max/u8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_max(black_box(255u8), Ordering::SeqCst); println!("fetch_max/u8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_max(black_box(254u8), Ordering::SeqCst); println!("fetch_max/u8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_max(black_box(128u8), Ordering::SeqCst); println!("fetch_max/u8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_max(black_box(127u8), Ordering::SeqCst); println!("fetch_max/u8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_max(black_box(129u8), Ordering::SeqCst); println!("fetch_max/u8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_max(black_box(0u8), Ordering::SeqCst); println!("fetch_max/u8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_max(black_box(0u8), Ordering::SeqCst); println!("fetch_max/u8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_max(black_box(0u8), Ordering::SeqCst); println!("fetch_max/u8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_max(black_box(0u8), Ordering::SeqCst); println!("fetch_max/u8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_max(black_box(0u8), Ordering::SeqCst); println!("fetch_max/u8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_max(black_box(0u8), Ordering::SeqCst); println!("fetch_max/u8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_max(black_box(129u8), Ordering::SeqCst); println!("fetch_max/u8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_max(black_box(129u8), Ordering::SeqCst); println!("fetch_max/u8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_max(black_box(129u8), Ordering::SeqCst); println!("fetch_max/u8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_max(black_box(129u8), Ordering::SeqCst); println!("fetch_max/u8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_max(black_box(129u8), Ordering::SeqCst); println!("fetch_max/u8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_max(black_box(129u8), Ordering::SeqCst); println!("fetch_max/u8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_min(black_box(0u8), Ordering::SeqCst); println!("fetch_min/u8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_min(black_box(1u8), Ordering::SeqCst); println!("fetch_min/u8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_min(black_box(255u8), Ordering::SeqCst); println!("fetch_min/u8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_min(black_box(254u8), Ordering::SeqCst); println!("fetch_min/u8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_min(black_box(128u8), Ordering::SeqCst); println!("fetch_min/u8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_min(black_box(127u8), Ordering::SeqCst); println!("fetch_min/u8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_min(black_box(129u8), Ordering::SeqCst); println!("fetch_min/u8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_min(black_box(0u8), Ordering::SeqCst); println!("fetch_min/u8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_min(black_box(0u8), Ordering::SeqCst); println!("fetch_min/u8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_min(black_box(0u8), Ordering::SeqCst); println!("fetch_min/u8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_min(black_box(0u8), Ordering::SeqCst); println!("fetch_min/u8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_min(black_box(0u8), Ordering::SeqCst); println!("fetch_min/u8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.fetch_min(black_box(0u8), Ordering::SeqCst); println!("fetch_min/u8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.fetch_min(black_box(129u8), Ordering::SeqCst); println!("fetch_min/u8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.fetch_min(black_box(129u8), Ordering::SeqCst); println!("fetch_min/u8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.fetch_min(black_box(129u8), Ordering::SeqCst); println!("fetch_min/u8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.fetch_min(black_box(129u8), Ordering::SeqCst); println!("fetch_min/u8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.fetch_min(black_box(129u8), Ordering::SeqCst); println!("fetch_min/u8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.fetch_min(black_box(129u8), Ordering::SeqCst); println!("fetch_min/u8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.swap(black_box(0u8), Ordering::SeqCst); println!("swap/u8/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let r = a.compare_exchange(black_box(0u8), black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let wrong = black_box(0u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.swap(black_box(1u8), Ordering::SeqCst); println!("swap/u8/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let r = a.compare_exchange(black_box(1u8), black_box(1u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let wrong = black_box(1u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(1u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(1u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.swap(black_box(255u8), Ordering::SeqCst); println!("swap/u8/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let r = a.compare_exchange(black_box(255u8), black_box(255u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let wrong = black_box(255u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(255u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(255u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.swap(black_box(254u8), Ordering::SeqCst); println!("swap/u8/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let r = a.compare_exchange(black_box(254u8), black_box(254u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let wrong = black_box(254u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(254u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(254u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.swap(black_box(128u8), Ordering::SeqCst); println!("swap/u8/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let r = a.compare_exchange(black_box(128u8), black_box(128u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let wrong = black_box(128u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(128u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(128u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.swap(black_box(127u8), Ordering::SeqCst); println!("swap/u8/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let r = a.compare_exchange(black_box(127u8), black_box(127u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let wrong = black_box(127u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(127u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(127u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.swap(black_box(129u8), Ordering::SeqCst); println!("swap/u8/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let r = a.compare_exchange(black_box(129u8), black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let wrong = black_box(129u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(129u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.swap(black_box(0u8), Ordering::SeqCst); println!("swap/u8/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let r = a.compare_exchange(black_box(1u8), black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let wrong = black_box(1u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.swap(black_box(0u8), Ordering::SeqCst); println!("swap/u8/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let r = a.compare_exchange(black_box(255u8), black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let wrong = black_box(255u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.swap(black_box(0u8), Ordering::SeqCst); println!("swap/u8/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let r = a.compare_exchange(black_box(254u8), black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let wrong = black_box(254u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.swap(black_box(0u8), Ordering::SeqCst); println!("swap/u8/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let r = a.compare_exchange(black_box(128u8), black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let wrong = black_box(128u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.swap(black_box(0u8), Ordering::SeqCst); println!("swap/u8/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let r = a.compare_exchange(black_box(127u8), black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let wrong = black_box(127u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let old = a.swap(black_box(0u8), Ordering::SeqCst); println!("swap/u8/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let r = a.compare_exchange(black_box(129u8), black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let wrong = black_box(129u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(129u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let old = a.swap(black_box(129u8), Ordering::SeqCst); println!("swap/u8/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let r = a.compare_exchange(black_box(0u8), black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let wrong = black_box(0u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(0u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(129u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let old = a.swap(black_box(129u8), Ordering::SeqCst); println!("swap/u8/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let r = a.compare_exchange(black_box(1u8), black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let wrong = black_box(1u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(1u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(129u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let old = a.swap(black_box(129u8), Ordering::SeqCst); println!("swap/u8/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let r = a.compare_exchange(black_box(255u8), black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let wrong = black_box(255u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(255u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(129u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let old = a.swap(black_box(129u8), Ordering::SeqCst); println!("swap/u8/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let r = a.compare_exchange(black_box(254u8), black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let wrong = black_box(254u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(254u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(129u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let old = a.swap(black_box(129u8), Ordering::SeqCst); println!("swap/u8/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let r = a.compare_exchange(black_box(128u8), black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let wrong = black_box(128u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(128u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(129u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let old = a.swap(black_box(129u8), Ordering::SeqCst); println!("swap/u8/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let r = a.compare_exchange(black_box(127u8), black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u8/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let wrong = black_box(127u8).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(129u8), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u8/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU8::new(black_box(127u8)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(129u8) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u8/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
}
