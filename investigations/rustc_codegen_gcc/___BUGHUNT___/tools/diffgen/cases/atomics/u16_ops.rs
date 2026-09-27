use std::sync::atomic::*;
use std::hint::black_box;

fn main() {
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_add(black_box(0u16), Ordering::SeqCst); println!("fetch_add/u16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_add(black_box(1u16), Ordering::SeqCst); println!("fetch_add/u16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_add(black_box(65535u16), Ordering::SeqCst); println!("fetch_add/u16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_add(black_box(65534u16), Ordering::SeqCst); println!("fetch_add/u16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_add(black_box(32768u16), Ordering::SeqCst); println!("fetch_add/u16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_add(black_box(32767u16), Ordering::SeqCst); println!("fetch_add/u16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_add(black_box(32769u16), Ordering::SeqCst); println!("fetch_add/u16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_add(black_box(0u16), Ordering::SeqCst); println!("fetch_add/u16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_add(black_box(0u16), Ordering::SeqCst); println!("fetch_add/u16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_add(black_box(0u16), Ordering::SeqCst); println!("fetch_add/u16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_add(black_box(0u16), Ordering::SeqCst); println!("fetch_add/u16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_add(black_box(0u16), Ordering::SeqCst); println!("fetch_add/u16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_add(black_box(0u16), Ordering::SeqCst); println!("fetch_add/u16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_add(black_box(32769u16), Ordering::SeqCst); println!("fetch_add/u16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_add(black_box(32769u16), Ordering::SeqCst); println!("fetch_add/u16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_add(black_box(32769u16), Ordering::SeqCst); println!("fetch_add/u16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_add(black_box(32769u16), Ordering::SeqCst); println!("fetch_add/u16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_add(black_box(32769u16), Ordering::SeqCst); println!("fetch_add/u16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_add(black_box(32769u16), Ordering::SeqCst); println!("fetch_add/u16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_sub(black_box(0u16), Ordering::SeqCst); println!("fetch_sub/u16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_sub(black_box(1u16), Ordering::SeqCst); println!("fetch_sub/u16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_sub(black_box(65535u16), Ordering::SeqCst); println!("fetch_sub/u16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_sub(black_box(65534u16), Ordering::SeqCst); println!("fetch_sub/u16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_sub(black_box(32768u16), Ordering::SeqCst); println!("fetch_sub/u16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_sub(black_box(32767u16), Ordering::SeqCst); println!("fetch_sub/u16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_sub(black_box(32769u16), Ordering::SeqCst); println!("fetch_sub/u16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_sub(black_box(0u16), Ordering::SeqCst); println!("fetch_sub/u16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_sub(black_box(0u16), Ordering::SeqCst); println!("fetch_sub/u16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_sub(black_box(0u16), Ordering::SeqCst); println!("fetch_sub/u16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_sub(black_box(0u16), Ordering::SeqCst); println!("fetch_sub/u16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_sub(black_box(0u16), Ordering::SeqCst); println!("fetch_sub/u16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_sub(black_box(0u16), Ordering::SeqCst); println!("fetch_sub/u16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_sub(black_box(32769u16), Ordering::SeqCst); println!("fetch_sub/u16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_sub(black_box(32769u16), Ordering::SeqCst); println!("fetch_sub/u16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_sub(black_box(32769u16), Ordering::SeqCst); println!("fetch_sub/u16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_sub(black_box(32769u16), Ordering::SeqCst); println!("fetch_sub/u16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_sub(black_box(32769u16), Ordering::SeqCst); println!("fetch_sub/u16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_sub(black_box(32769u16), Ordering::SeqCst); println!("fetch_sub/u16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_and(black_box(0u16), Ordering::SeqCst); println!("fetch_and/u16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_and(black_box(1u16), Ordering::SeqCst); println!("fetch_and/u16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_and(black_box(65535u16), Ordering::SeqCst); println!("fetch_and/u16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_and(black_box(65534u16), Ordering::SeqCst); println!("fetch_and/u16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_and(black_box(32768u16), Ordering::SeqCst); println!("fetch_and/u16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_and(black_box(32767u16), Ordering::SeqCst); println!("fetch_and/u16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_and(black_box(32769u16), Ordering::SeqCst); println!("fetch_and/u16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_and(black_box(0u16), Ordering::SeqCst); println!("fetch_and/u16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_and(black_box(0u16), Ordering::SeqCst); println!("fetch_and/u16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_and(black_box(0u16), Ordering::SeqCst); println!("fetch_and/u16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_and(black_box(0u16), Ordering::SeqCst); println!("fetch_and/u16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_and(black_box(0u16), Ordering::SeqCst); println!("fetch_and/u16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_and(black_box(0u16), Ordering::SeqCst); println!("fetch_and/u16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_and(black_box(32769u16), Ordering::SeqCst); println!("fetch_and/u16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_and(black_box(32769u16), Ordering::SeqCst); println!("fetch_and/u16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_and(black_box(32769u16), Ordering::SeqCst); println!("fetch_and/u16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_and(black_box(32769u16), Ordering::SeqCst); println!("fetch_and/u16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_and(black_box(32769u16), Ordering::SeqCst); println!("fetch_and/u16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_and(black_box(32769u16), Ordering::SeqCst); println!("fetch_and/u16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_or(black_box(0u16), Ordering::SeqCst); println!("fetch_or/u16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_or(black_box(1u16), Ordering::SeqCst); println!("fetch_or/u16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_or(black_box(65535u16), Ordering::SeqCst); println!("fetch_or/u16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_or(black_box(65534u16), Ordering::SeqCst); println!("fetch_or/u16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_or(black_box(32768u16), Ordering::SeqCst); println!("fetch_or/u16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_or(black_box(32767u16), Ordering::SeqCst); println!("fetch_or/u16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_or(black_box(32769u16), Ordering::SeqCst); println!("fetch_or/u16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_or(black_box(0u16), Ordering::SeqCst); println!("fetch_or/u16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_or(black_box(0u16), Ordering::SeqCst); println!("fetch_or/u16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_or(black_box(0u16), Ordering::SeqCst); println!("fetch_or/u16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_or(black_box(0u16), Ordering::SeqCst); println!("fetch_or/u16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_or(black_box(0u16), Ordering::SeqCst); println!("fetch_or/u16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_or(black_box(0u16), Ordering::SeqCst); println!("fetch_or/u16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_or(black_box(32769u16), Ordering::SeqCst); println!("fetch_or/u16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_or(black_box(32769u16), Ordering::SeqCst); println!("fetch_or/u16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_or(black_box(32769u16), Ordering::SeqCst); println!("fetch_or/u16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_or(black_box(32769u16), Ordering::SeqCst); println!("fetch_or/u16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_or(black_box(32769u16), Ordering::SeqCst); println!("fetch_or/u16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_or(black_box(32769u16), Ordering::SeqCst); println!("fetch_or/u16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_xor(black_box(0u16), Ordering::SeqCst); println!("fetch_xor/u16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_xor(black_box(1u16), Ordering::SeqCst); println!("fetch_xor/u16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_xor(black_box(65535u16), Ordering::SeqCst); println!("fetch_xor/u16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_xor(black_box(65534u16), Ordering::SeqCst); println!("fetch_xor/u16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_xor(black_box(32768u16), Ordering::SeqCst); println!("fetch_xor/u16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_xor(black_box(32767u16), Ordering::SeqCst); println!("fetch_xor/u16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_xor(black_box(32769u16), Ordering::SeqCst); println!("fetch_xor/u16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_xor(black_box(0u16), Ordering::SeqCst); println!("fetch_xor/u16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_xor(black_box(0u16), Ordering::SeqCst); println!("fetch_xor/u16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_xor(black_box(0u16), Ordering::SeqCst); println!("fetch_xor/u16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_xor(black_box(0u16), Ordering::SeqCst); println!("fetch_xor/u16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_xor(black_box(0u16), Ordering::SeqCst); println!("fetch_xor/u16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_xor(black_box(0u16), Ordering::SeqCst); println!("fetch_xor/u16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_xor(black_box(32769u16), Ordering::SeqCst); println!("fetch_xor/u16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_xor(black_box(32769u16), Ordering::SeqCst); println!("fetch_xor/u16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_xor(black_box(32769u16), Ordering::SeqCst); println!("fetch_xor/u16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_xor(black_box(32769u16), Ordering::SeqCst); println!("fetch_xor/u16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_xor(black_box(32769u16), Ordering::SeqCst); println!("fetch_xor/u16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_xor(black_box(32769u16), Ordering::SeqCst); println!("fetch_xor/u16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_nand(black_box(0u16), Ordering::SeqCst); println!("fetch_nand/u16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_nand(black_box(1u16), Ordering::SeqCst); println!("fetch_nand/u16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_nand(black_box(65535u16), Ordering::SeqCst); println!("fetch_nand/u16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_nand(black_box(65534u16), Ordering::SeqCst); println!("fetch_nand/u16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_nand(black_box(32768u16), Ordering::SeqCst); println!("fetch_nand/u16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_nand(black_box(32767u16), Ordering::SeqCst); println!("fetch_nand/u16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_nand(black_box(32769u16), Ordering::SeqCst); println!("fetch_nand/u16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_nand(black_box(0u16), Ordering::SeqCst); println!("fetch_nand/u16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_nand(black_box(0u16), Ordering::SeqCst); println!("fetch_nand/u16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_nand(black_box(0u16), Ordering::SeqCst); println!("fetch_nand/u16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_nand(black_box(0u16), Ordering::SeqCst); println!("fetch_nand/u16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_nand(black_box(0u16), Ordering::SeqCst); println!("fetch_nand/u16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_nand(black_box(0u16), Ordering::SeqCst); println!("fetch_nand/u16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_nand(black_box(32769u16), Ordering::SeqCst); println!("fetch_nand/u16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_nand(black_box(32769u16), Ordering::SeqCst); println!("fetch_nand/u16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_nand(black_box(32769u16), Ordering::SeqCst); println!("fetch_nand/u16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_nand(black_box(32769u16), Ordering::SeqCst); println!("fetch_nand/u16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_nand(black_box(32769u16), Ordering::SeqCst); println!("fetch_nand/u16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_nand(black_box(32769u16), Ordering::SeqCst); println!("fetch_nand/u16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_max(black_box(0u16), Ordering::SeqCst); println!("fetch_max/u16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_max(black_box(1u16), Ordering::SeqCst); println!("fetch_max/u16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_max(black_box(65535u16), Ordering::SeqCst); println!("fetch_max/u16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_max(black_box(65534u16), Ordering::SeqCst); println!("fetch_max/u16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_max(black_box(32768u16), Ordering::SeqCst); println!("fetch_max/u16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_max(black_box(32767u16), Ordering::SeqCst); println!("fetch_max/u16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_max(black_box(32769u16), Ordering::SeqCst); println!("fetch_max/u16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_max(black_box(0u16), Ordering::SeqCst); println!("fetch_max/u16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_max(black_box(0u16), Ordering::SeqCst); println!("fetch_max/u16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_max(black_box(0u16), Ordering::SeqCst); println!("fetch_max/u16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_max(black_box(0u16), Ordering::SeqCst); println!("fetch_max/u16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_max(black_box(0u16), Ordering::SeqCst); println!("fetch_max/u16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_max(black_box(0u16), Ordering::SeqCst); println!("fetch_max/u16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_max(black_box(32769u16), Ordering::SeqCst); println!("fetch_max/u16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_max(black_box(32769u16), Ordering::SeqCst); println!("fetch_max/u16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_max(black_box(32769u16), Ordering::SeqCst); println!("fetch_max/u16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_max(black_box(32769u16), Ordering::SeqCst); println!("fetch_max/u16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_max(black_box(32769u16), Ordering::SeqCst); println!("fetch_max/u16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_max(black_box(32769u16), Ordering::SeqCst); println!("fetch_max/u16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_min(black_box(0u16), Ordering::SeqCst); println!("fetch_min/u16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_min(black_box(1u16), Ordering::SeqCst); println!("fetch_min/u16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_min(black_box(65535u16), Ordering::SeqCst); println!("fetch_min/u16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_min(black_box(65534u16), Ordering::SeqCst); println!("fetch_min/u16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_min(black_box(32768u16), Ordering::SeqCst); println!("fetch_min/u16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_min(black_box(32767u16), Ordering::SeqCst); println!("fetch_min/u16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_min(black_box(32769u16), Ordering::SeqCst); println!("fetch_min/u16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_min(black_box(0u16), Ordering::SeqCst); println!("fetch_min/u16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_min(black_box(0u16), Ordering::SeqCst); println!("fetch_min/u16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_min(black_box(0u16), Ordering::SeqCst); println!("fetch_min/u16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_min(black_box(0u16), Ordering::SeqCst); println!("fetch_min/u16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_min(black_box(0u16), Ordering::SeqCst); println!("fetch_min/u16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.fetch_min(black_box(0u16), Ordering::SeqCst); println!("fetch_min/u16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.fetch_min(black_box(32769u16), Ordering::SeqCst); println!("fetch_min/u16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.fetch_min(black_box(32769u16), Ordering::SeqCst); println!("fetch_min/u16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.fetch_min(black_box(32769u16), Ordering::SeqCst); println!("fetch_min/u16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.fetch_min(black_box(32769u16), Ordering::SeqCst); println!("fetch_min/u16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.fetch_min(black_box(32769u16), Ordering::SeqCst); println!("fetch_min/u16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.fetch_min(black_box(32769u16), Ordering::SeqCst); println!("fetch_min/u16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.swap(black_box(0u16), Ordering::SeqCst); println!("swap/u16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let r = a.compare_exchange(black_box(0u16), black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let wrong = black_box(0u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.swap(black_box(1u16), Ordering::SeqCst); println!("swap/u16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let r = a.compare_exchange(black_box(1u16), black_box(1u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let wrong = black_box(1u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(1u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(1u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.swap(black_box(65535u16), Ordering::SeqCst); println!("swap/u16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let r = a.compare_exchange(black_box(65535u16), black_box(65535u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let wrong = black_box(65535u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(65535u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(65535u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.swap(black_box(65534u16), Ordering::SeqCst); println!("swap/u16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let r = a.compare_exchange(black_box(65534u16), black_box(65534u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let wrong = black_box(65534u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(65534u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(65534u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.swap(black_box(32768u16), Ordering::SeqCst); println!("swap/u16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let r = a.compare_exchange(black_box(32768u16), black_box(32768u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let wrong = black_box(32768u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32768u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32768u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.swap(black_box(32767u16), Ordering::SeqCst); println!("swap/u16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let r = a.compare_exchange(black_box(32767u16), black_box(32767u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let wrong = black_box(32767u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32767u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32767u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.swap(black_box(32769u16), Ordering::SeqCst); println!("swap/u16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let r = a.compare_exchange(black_box(32769u16), black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let wrong = black_box(32769u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32769u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.swap(black_box(0u16), Ordering::SeqCst); println!("swap/u16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let r = a.compare_exchange(black_box(1u16), black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let wrong = black_box(1u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.swap(black_box(0u16), Ordering::SeqCst); println!("swap/u16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let r = a.compare_exchange(black_box(65535u16), black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let wrong = black_box(65535u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.swap(black_box(0u16), Ordering::SeqCst); println!("swap/u16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let r = a.compare_exchange(black_box(65534u16), black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let wrong = black_box(65534u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.swap(black_box(0u16), Ordering::SeqCst); println!("swap/u16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let r = a.compare_exchange(black_box(32768u16), black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let wrong = black_box(32768u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.swap(black_box(0u16), Ordering::SeqCst); println!("swap/u16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let r = a.compare_exchange(black_box(32767u16), black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let wrong = black_box(32767u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let old = a.swap(black_box(0u16), Ordering::SeqCst); println!("swap/u16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let r = a.compare_exchange(black_box(32769u16), black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let wrong = black_box(32769u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32769u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let old = a.swap(black_box(32769u16), Ordering::SeqCst); println!("swap/u16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let r = a.compare_exchange(black_box(0u16), black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let wrong = black_box(0u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(0u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32769u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let old = a.swap(black_box(32769u16), Ordering::SeqCst); println!("swap/u16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let r = a.compare_exchange(black_box(1u16), black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let wrong = black_box(1u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(1u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32769u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let old = a.swap(black_box(32769u16), Ordering::SeqCst); println!("swap/u16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let r = a.compare_exchange(black_box(65535u16), black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let wrong = black_box(65535u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65535u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32769u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let old = a.swap(black_box(32769u16), Ordering::SeqCst); println!("swap/u16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let r = a.compare_exchange(black_box(65534u16), black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let wrong = black_box(65534u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(65534u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32769u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let old = a.swap(black_box(32769u16), Ordering::SeqCst); println!("swap/u16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let r = a.compare_exchange(black_box(32768u16), black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let wrong = black_box(32768u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32768u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32769u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let old = a.swap(black_box(32769u16), Ordering::SeqCst); println!("swap/u16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let r = a.compare_exchange(black_box(32767u16), black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/u16/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let wrong = black_box(32767u16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32769u16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/u16/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicU16::new(black_box(32767u16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32769u16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/u16/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
}
