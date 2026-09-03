use std::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize, NonZeroI32};
use std::hint::black_box;
use std::mem::size_of;

fn main() {
    { let s: Option<bool> = Some(black_box(true)); let n: Option<bool> = None::<bool>;
      println!("niche/bool/some: is_some={} debug={:?}", s.is_some(), s);
      println!("niche/bool/none: is_none={} debug={:?}", n.is_none(), n);
      println!("niche_size/bool: {}", size_of::<Option<bool>>() == size_of::<bool>()); }
    { let s: Option<char> = Some(black_box('x')); let n: Option<char> = None::<char>;
      println!("niche/char/some: is_some={} debug={:?}", s.is_some(), s);
      println!("niche/char/none: is_none={} debug={:?}", n.is_none(), n);
      println!("niche_size/char: {}", size_of::<Option<char>>() == size_of::<char>()); }
    { let s: Option<NonZeroU8> = Some(NonZeroU8::new(black_box(5)).unwrap()); let n: Option<NonZeroU8> = None::<NonZeroU8>;
      println!("niche/NonZeroU8/some: is_some={} debug={:?}", s.is_some(), s);
      println!("niche/NonZeroU8/none: is_none={} debug={:?}", n.is_none(), n);
      println!("niche_size/NonZeroU8: {}", size_of::<Option<NonZeroU8>>() == size_of::<NonZeroU8>()); }
    { let s: Option<NonZeroU32> = Some(NonZeroU32::new(black_box(5)).unwrap()); let n: Option<NonZeroU32> = None::<NonZeroU32>;
      println!("niche/NonZeroU32/some: is_some={} debug={:?}", s.is_some(), s);
      println!("niche/NonZeroU32/none: is_none={} debug={:?}", n.is_none(), n);
      println!("niche_size/NonZeroU32: {}", size_of::<Option<NonZeroU32>>() == size_of::<NonZeroU32>()); }
    { let s: Option<NonZeroU64> = Some(NonZeroU64::new(black_box(5)).unwrap()); let n: Option<NonZeroU64> = None::<NonZeroU64>;
      println!("niche/NonZeroU64/some: is_some={} debug={:?}", s.is_some(), s);
      println!("niche/NonZeroU64/none: is_none={} debug={:?}", n.is_none(), n);
      println!("niche_size/NonZeroU64: {}", size_of::<Option<NonZeroU64>>() == size_of::<NonZeroU64>()); }
    { let s: Option<NonZeroI32> = Some(NonZeroI32::new(black_box(-5)).unwrap()); let n: Option<NonZeroI32> = None::<NonZeroI32>;
      println!("niche/NonZeroI32/some: is_some={} debug={:?}", s.is_some(), s);
      println!("niche/NonZeroI32/none: is_none={} debug={:?}", n.is_none(), n);
      println!("niche_size/NonZeroI32: {}", size_of::<Option<NonZeroI32>>() == size_of::<NonZeroI32>()); }
    { let s: Option<NonZeroUsize> = Some(NonZeroUsize::new(black_box(5)).unwrap()); let n: Option<NonZeroUsize> = None::<NonZeroUsize>;
      println!("niche/NonZeroUsize/some: is_some={} debug={:?}", s.is_some(), s);
      println!("niche/NonZeroUsize/none: is_none={} debug={:?}", n.is_none(), n);
      println!("niche_size/NonZeroUsize: {}", size_of::<Option<NonZeroUsize>>() == size_of::<NonZeroUsize>()); }
    { let a: Option<Option<bool>> = Some(Some(black_box(true)));
      let b: Option<Option<bool>> = Some(None::<bool>);
      let c: Option<Option<bool>> = None;
      println!("nested_option_bool: {:?} {:?} {:?}", a, b, c); }
    { let a: Option<Option<NonZeroU32>> = Some(Some(NonZeroU32::new(black_box(7)).unwrap()));
      let b: Option<Option<NonZeroU32>> = Some(None);
      let c: Option<Option<NonZeroU32>> = None;
      println!("nested_option_nzu32: {:?} {:?} {:?}", a, b, c); }
    { let x = black_box(42i32); let r: Option<&i32> = Some(&x); let n: Option<&i32> = None;
      println!("niche_ref: {:?} {} {}", r, *r.unwrap(), n.is_none()); }
    { fn f() -> i32 { black_box(1) } let p: Option<fn() -> i32> = Some(f); let n: Option<fn() -> i32> = None;
      println!("niche_fnptr: {} {}", p.unwrap()(), n.is_none()); }
}
