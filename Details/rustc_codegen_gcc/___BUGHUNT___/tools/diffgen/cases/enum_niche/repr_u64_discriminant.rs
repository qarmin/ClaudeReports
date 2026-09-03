use std::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize, NonZeroI32};
use std::hint::black_box;
use std::mem::size_of;

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum E {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 100,
}

fn main() {
    let v = black_box(E::V0);
    println!("disc/u64/0: as_int={} debug={:?}", v as u64, v);
    let bytes: [u8; size_of::<E>()] = unsafe { std::mem::transmute(v) };
    println!("disc_bytes/u64/0: {:?}", bytes);
    let v = black_box(E::V1);
    println!("disc/u64/1: as_int={} debug={:?}", v as u64, v);
    let bytes: [u8; size_of::<E>()] = unsafe { std::mem::transmute(v) };
    println!("disc_bytes/u64/1: {:?}", bytes);
    let v = black_box(E::V2);
    println!("disc/u64/2: as_int={} debug={:?}", v as u64, v);
    let bytes: [u8; size_of::<E>()] = unsafe { std::mem::transmute(v) };
    println!("disc_bytes/u64/2: {:?}", bytes);
    let v = black_box(E::V3);
    println!("disc/u64/3: as_int={} debug={:?}", v as u64, v);
    let bytes: [u8; size_of::<E>()] = unsafe { std::mem::transmute(v) };
    println!("disc_bytes/u64/3: {:?}", bytes);
}