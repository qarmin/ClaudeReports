use std::sync::atomic::*;
use std::hint::black_box;

fn main() {
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_add(black_box(0i16), Ordering::SeqCst); println!("fetch_add/i16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_add(black_box(1i16), Ordering::SeqCst); println!("fetch_add/i16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_add(black_box(-1i16), Ordering::SeqCst); println!("fetch_add/i16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_add(black_box(-32768i16), Ordering::SeqCst); println!("fetch_add/i16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_add(black_box(-32767i16), Ordering::SeqCst); println!("fetch_add/i16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_add(black_box(32767i16), Ordering::SeqCst); println!("fetch_add/i16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_add(black_box(32766i16), Ordering::SeqCst); println!("fetch_add/i16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_add(black_box(0i16), Ordering::SeqCst); println!("fetch_add/i16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_add(black_box(0i16), Ordering::SeqCst); println!("fetch_add/i16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_add(black_box(0i16), Ordering::SeqCst); println!("fetch_add/i16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_add(black_box(0i16), Ordering::SeqCst); println!("fetch_add/i16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_add(black_box(0i16), Ordering::SeqCst); println!("fetch_add/i16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_add(black_box(0i16), Ordering::SeqCst); println!("fetch_add/i16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_add(black_box(32766i16), Ordering::SeqCst); println!("fetch_add/i16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_add(black_box(32766i16), Ordering::SeqCst); println!("fetch_add/i16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_add(black_box(32766i16), Ordering::SeqCst); println!("fetch_add/i16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_add(black_box(32766i16), Ordering::SeqCst); println!("fetch_add/i16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_add(black_box(32766i16), Ordering::SeqCst); println!("fetch_add/i16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_add(black_box(32766i16), Ordering::SeqCst); println!("fetch_add/i16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_sub(black_box(0i16), Ordering::SeqCst); println!("fetch_sub/i16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_sub(black_box(1i16), Ordering::SeqCst); println!("fetch_sub/i16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_sub(black_box(-1i16), Ordering::SeqCst); println!("fetch_sub/i16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_sub(black_box(-32768i16), Ordering::SeqCst); println!("fetch_sub/i16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_sub(black_box(-32767i16), Ordering::SeqCst); println!("fetch_sub/i16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_sub(black_box(32767i16), Ordering::SeqCst); println!("fetch_sub/i16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_sub(black_box(32766i16), Ordering::SeqCst); println!("fetch_sub/i16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_sub(black_box(0i16), Ordering::SeqCst); println!("fetch_sub/i16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_sub(black_box(0i16), Ordering::SeqCst); println!("fetch_sub/i16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_sub(black_box(0i16), Ordering::SeqCst); println!("fetch_sub/i16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_sub(black_box(0i16), Ordering::SeqCst); println!("fetch_sub/i16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_sub(black_box(0i16), Ordering::SeqCst); println!("fetch_sub/i16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_sub(black_box(0i16), Ordering::SeqCst); println!("fetch_sub/i16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_sub(black_box(32766i16), Ordering::SeqCst); println!("fetch_sub/i16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_sub(black_box(32766i16), Ordering::SeqCst); println!("fetch_sub/i16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_sub(black_box(32766i16), Ordering::SeqCst); println!("fetch_sub/i16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_sub(black_box(32766i16), Ordering::SeqCst); println!("fetch_sub/i16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_sub(black_box(32766i16), Ordering::SeqCst); println!("fetch_sub/i16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_sub(black_box(32766i16), Ordering::SeqCst); println!("fetch_sub/i16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_and(black_box(0i16), Ordering::SeqCst); println!("fetch_and/i16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_and(black_box(1i16), Ordering::SeqCst); println!("fetch_and/i16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_and(black_box(-1i16), Ordering::SeqCst); println!("fetch_and/i16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_and(black_box(-32768i16), Ordering::SeqCst); println!("fetch_and/i16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_and(black_box(-32767i16), Ordering::SeqCst); println!("fetch_and/i16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_and(black_box(32767i16), Ordering::SeqCst); println!("fetch_and/i16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_and(black_box(32766i16), Ordering::SeqCst); println!("fetch_and/i16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_and(black_box(0i16), Ordering::SeqCst); println!("fetch_and/i16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_and(black_box(0i16), Ordering::SeqCst); println!("fetch_and/i16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_and(black_box(0i16), Ordering::SeqCst); println!("fetch_and/i16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_and(black_box(0i16), Ordering::SeqCst); println!("fetch_and/i16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_and(black_box(0i16), Ordering::SeqCst); println!("fetch_and/i16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_and(black_box(0i16), Ordering::SeqCst); println!("fetch_and/i16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_and(black_box(32766i16), Ordering::SeqCst); println!("fetch_and/i16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_and(black_box(32766i16), Ordering::SeqCst); println!("fetch_and/i16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_and(black_box(32766i16), Ordering::SeqCst); println!("fetch_and/i16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_and(black_box(32766i16), Ordering::SeqCst); println!("fetch_and/i16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_and(black_box(32766i16), Ordering::SeqCst); println!("fetch_and/i16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_and(black_box(32766i16), Ordering::SeqCst); println!("fetch_and/i16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_or(black_box(0i16), Ordering::SeqCst); println!("fetch_or/i16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_or(black_box(1i16), Ordering::SeqCst); println!("fetch_or/i16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_or(black_box(-1i16), Ordering::SeqCst); println!("fetch_or/i16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_or(black_box(-32768i16), Ordering::SeqCst); println!("fetch_or/i16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_or(black_box(-32767i16), Ordering::SeqCst); println!("fetch_or/i16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_or(black_box(32767i16), Ordering::SeqCst); println!("fetch_or/i16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_or(black_box(32766i16), Ordering::SeqCst); println!("fetch_or/i16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_or(black_box(0i16), Ordering::SeqCst); println!("fetch_or/i16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_or(black_box(0i16), Ordering::SeqCst); println!("fetch_or/i16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_or(black_box(0i16), Ordering::SeqCst); println!("fetch_or/i16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_or(black_box(0i16), Ordering::SeqCst); println!("fetch_or/i16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_or(black_box(0i16), Ordering::SeqCst); println!("fetch_or/i16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_or(black_box(0i16), Ordering::SeqCst); println!("fetch_or/i16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_or(black_box(32766i16), Ordering::SeqCst); println!("fetch_or/i16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_or(black_box(32766i16), Ordering::SeqCst); println!("fetch_or/i16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_or(black_box(32766i16), Ordering::SeqCst); println!("fetch_or/i16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_or(black_box(32766i16), Ordering::SeqCst); println!("fetch_or/i16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_or(black_box(32766i16), Ordering::SeqCst); println!("fetch_or/i16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_or(black_box(32766i16), Ordering::SeqCst); println!("fetch_or/i16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_xor(black_box(0i16), Ordering::SeqCst); println!("fetch_xor/i16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_xor(black_box(1i16), Ordering::SeqCst); println!("fetch_xor/i16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_xor(black_box(-1i16), Ordering::SeqCst); println!("fetch_xor/i16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_xor(black_box(-32768i16), Ordering::SeqCst); println!("fetch_xor/i16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_xor(black_box(-32767i16), Ordering::SeqCst); println!("fetch_xor/i16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_xor(black_box(32767i16), Ordering::SeqCst); println!("fetch_xor/i16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_xor(black_box(32766i16), Ordering::SeqCst); println!("fetch_xor/i16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_xor(black_box(0i16), Ordering::SeqCst); println!("fetch_xor/i16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_xor(black_box(0i16), Ordering::SeqCst); println!("fetch_xor/i16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_xor(black_box(0i16), Ordering::SeqCst); println!("fetch_xor/i16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_xor(black_box(0i16), Ordering::SeqCst); println!("fetch_xor/i16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_xor(black_box(0i16), Ordering::SeqCst); println!("fetch_xor/i16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_xor(black_box(0i16), Ordering::SeqCst); println!("fetch_xor/i16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_xor(black_box(32766i16), Ordering::SeqCst); println!("fetch_xor/i16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_xor(black_box(32766i16), Ordering::SeqCst); println!("fetch_xor/i16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_xor(black_box(32766i16), Ordering::SeqCst); println!("fetch_xor/i16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_xor(black_box(32766i16), Ordering::SeqCst); println!("fetch_xor/i16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_xor(black_box(32766i16), Ordering::SeqCst); println!("fetch_xor/i16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_xor(black_box(32766i16), Ordering::SeqCst); println!("fetch_xor/i16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_nand(black_box(0i16), Ordering::SeqCst); println!("fetch_nand/i16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_nand(black_box(1i16), Ordering::SeqCst); println!("fetch_nand/i16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_nand(black_box(-1i16), Ordering::SeqCst); println!("fetch_nand/i16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_nand(black_box(-32768i16), Ordering::SeqCst); println!("fetch_nand/i16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_nand(black_box(-32767i16), Ordering::SeqCst); println!("fetch_nand/i16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_nand(black_box(32767i16), Ordering::SeqCst); println!("fetch_nand/i16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_nand(black_box(32766i16), Ordering::SeqCst); println!("fetch_nand/i16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_nand(black_box(0i16), Ordering::SeqCst); println!("fetch_nand/i16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_nand(black_box(0i16), Ordering::SeqCst); println!("fetch_nand/i16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_nand(black_box(0i16), Ordering::SeqCst); println!("fetch_nand/i16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_nand(black_box(0i16), Ordering::SeqCst); println!("fetch_nand/i16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_nand(black_box(0i16), Ordering::SeqCst); println!("fetch_nand/i16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_nand(black_box(0i16), Ordering::SeqCst); println!("fetch_nand/i16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_nand(black_box(32766i16), Ordering::SeqCst); println!("fetch_nand/i16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_nand(black_box(32766i16), Ordering::SeqCst); println!("fetch_nand/i16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_nand(black_box(32766i16), Ordering::SeqCst); println!("fetch_nand/i16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_nand(black_box(32766i16), Ordering::SeqCst); println!("fetch_nand/i16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_nand(black_box(32766i16), Ordering::SeqCst); println!("fetch_nand/i16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_nand(black_box(32766i16), Ordering::SeqCst); println!("fetch_nand/i16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_max(black_box(0i16), Ordering::SeqCst); println!("fetch_max/i16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_max(black_box(1i16), Ordering::SeqCst); println!("fetch_max/i16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_max(black_box(-1i16), Ordering::SeqCst); println!("fetch_max/i16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_max(black_box(-32768i16), Ordering::SeqCst); println!("fetch_max/i16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_max(black_box(-32767i16), Ordering::SeqCst); println!("fetch_max/i16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_max(black_box(32767i16), Ordering::SeqCst); println!("fetch_max/i16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_max(black_box(32766i16), Ordering::SeqCst); println!("fetch_max/i16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_max(black_box(0i16), Ordering::SeqCst); println!("fetch_max/i16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_max(black_box(0i16), Ordering::SeqCst); println!("fetch_max/i16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_max(black_box(0i16), Ordering::SeqCst); println!("fetch_max/i16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_max(black_box(0i16), Ordering::SeqCst); println!("fetch_max/i16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_max(black_box(0i16), Ordering::SeqCst); println!("fetch_max/i16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_max(black_box(0i16), Ordering::SeqCst); println!("fetch_max/i16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_max(black_box(32766i16), Ordering::SeqCst); println!("fetch_max/i16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_max(black_box(32766i16), Ordering::SeqCst); println!("fetch_max/i16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_max(black_box(32766i16), Ordering::SeqCst); println!("fetch_max/i16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_max(black_box(32766i16), Ordering::SeqCst); println!("fetch_max/i16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_max(black_box(32766i16), Ordering::SeqCst); println!("fetch_max/i16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_max(black_box(32766i16), Ordering::SeqCst); println!("fetch_max/i16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_min(black_box(0i16), Ordering::SeqCst); println!("fetch_min/i16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_min(black_box(1i16), Ordering::SeqCst); println!("fetch_min/i16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_min(black_box(-1i16), Ordering::SeqCst); println!("fetch_min/i16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_min(black_box(-32768i16), Ordering::SeqCst); println!("fetch_min/i16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_min(black_box(-32767i16), Ordering::SeqCst); println!("fetch_min/i16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_min(black_box(32767i16), Ordering::SeqCst); println!("fetch_min/i16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_min(black_box(32766i16), Ordering::SeqCst); println!("fetch_min/i16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_min(black_box(0i16), Ordering::SeqCst); println!("fetch_min/i16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_min(black_box(0i16), Ordering::SeqCst); println!("fetch_min/i16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_min(black_box(0i16), Ordering::SeqCst); println!("fetch_min/i16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_min(black_box(0i16), Ordering::SeqCst); println!("fetch_min/i16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_min(black_box(0i16), Ordering::SeqCst); println!("fetch_min/i16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.fetch_min(black_box(0i16), Ordering::SeqCst); println!("fetch_min/i16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.fetch_min(black_box(32766i16), Ordering::SeqCst); println!("fetch_min/i16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.fetch_min(black_box(32766i16), Ordering::SeqCst); println!("fetch_min/i16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.fetch_min(black_box(32766i16), Ordering::SeqCst); println!("fetch_min/i16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.fetch_min(black_box(32766i16), Ordering::SeqCst); println!("fetch_min/i16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.fetch_min(black_box(32766i16), Ordering::SeqCst); println!("fetch_min/i16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.fetch_min(black_box(32766i16), Ordering::SeqCst); println!("fetch_min/i16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.swap(black_box(0i16), Ordering::SeqCst); println!("swap/i16/0: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let r = a.compare_exchange(black_box(0i16), black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let wrong = black_box(0i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/0: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.swap(black_box(1i16), Ordering::SeqCst); println!("swap/i16/1: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let r = a.compare_exchange(black_box(1i16), black_box(1i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let wrong = black_box(1i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(1i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(1i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/1: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.swap(black_box(-1i16), Ordering::SeqCst); println!("swap/i16/2: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let r = a.compare_exchange(black_box(-1i16), black_box(-1i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let wrong = black_box(-1i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(-1i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(-1i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/2: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.swap(black_box(-32768i16), Ordering::SeqCst); println!("swap/i16/3: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let r = a.compare_exchange(black_box(-32768i16), black_box(-32768i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let wrong = black_box(-32768i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(-32768i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(-32768i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/3: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.swap(black_box(-32767i16), Ordering::SeqCst); println!("swap/i16/4: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let r = a.compare_exchange(black_box(-32767i16), black_box(-32767i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let wrong = black_box(-32767i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(-32767i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(-32767i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/4: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.swap(black_box(32767i16), Ordering::SeqCst); println!("swap/i16/5: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let r = a.compare_exchange(black_box(32767i16), black_box(32767i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let wrong = black_box(32767i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32767i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32767i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/5: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.swap(black_box(32766i16), Ordering::SeqCst); println!("swap/i16/6: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let r = a.compare_exchange(black_box(32766i16), black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let wrong = black_box(32766i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32766i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/6: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.swap(black_box(0i16), Ordering::SeqCst); println!("swap/i16/7: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let r = a.compare_exchange(black_box(1i16), black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let wrong = black_box(1i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/7: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.swap(black_box(0i16), Ordering::SeqCst); println!("swap/i16/8: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let r = a.compare_exchange(black_box(-1i16), black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let wrong = black_box(-1i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/8: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.swap(black_box(0i16), Ordering::SeqCst); println!("swap/i16/9: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let r = a.compare_exchange(black_box(-32768i16), black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let wrong = black_box(-32768i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/9: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.swap(black_box(0i16), Ordering::SeqCst); println!("swap/i16/10: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let r = a.compare_exchange(black_box(-32767i16), black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let wrong = black_box(-32767i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/10: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.swap(black_box(0i16), Ordering::SeqCst); println!("swap/i16/11: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let r = a.compare_exchange(black_box(32767i16), black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let wrong = black_box(32767i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/11: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let old = a.swap(black_box(0i16), Ordering::SeqCst); println!("swap/i16/12: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let r = a.compare_exchange(black_box(32766i16), black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let wrong = black_box(32766i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(0i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32766i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(0i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/12: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let old = a.swap(black_box(32766i16), Ordering::SeqCst); println!("swap/i16/13: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let r = a.compare_exchange(black_box(0i16), black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let wrong = black_box(0i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(0i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32766i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/13: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let old = a.swap(black_box(32766i16), Ordering::SeqCst); println!("swap/i16/14: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let r = a.compare_exchange(black_box(1i16), black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let wrong = black_box(1i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(1i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32766i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/14: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let old = a.swap(black_box(32766i16), Ordering::SeqCst); println!("swap/i16/15: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let r = a.compare_exchange(black_box(-1i16), black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let wrong = black_box(-1i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-1i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32766i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/15: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let old = a.swap(black_box(32766i16), Ordering::SeqCst); println!("swap/i16/16: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let r = a.compare_exchange(black_box(-32768i16), black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let wrong = black_box(-32768i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32768i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32766i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/16: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let old = a.swap(black_box(32766i16), Ordering::SeqCst); println!("swap/i16/17: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let r = a.compare_exchange(black_box(-32767i16), black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let wrong = black_box(-32767i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(-32767i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32766i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/17: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let old = a.swap(black_box(32766i16), Ordering::SeqCst); println!("swap/i16/18: old={} new={}", old, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let r = a.compare_exchange(black_box(32767i16), black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_match/i16/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let wrong = black_box(32767i16).wrapping_add(1); let r = a.compare_exchange(wrong, black_box(32766i16), Ordering::SeqCst, Ordering::SeqCst); println!("cmpxchg_nomatch/i16/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
    { let a = AtomicI16::new(black_box(32767i16)); let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box(32766i16) { Some(v.wrapping_sub(1)) } else { None }); println!("fetch_update/i16/18: {:?} new={}", r, a.load(Ordering::SeqCst)); }
}
