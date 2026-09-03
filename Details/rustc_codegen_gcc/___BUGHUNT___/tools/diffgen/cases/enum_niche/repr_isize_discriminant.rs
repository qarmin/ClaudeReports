use std::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize, NonZeroI32};
use std::hint::black_box;
use std::mem::size_of;

#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum E {
    V0 = 0,
    V1 = 1,
    V2 = -1,
    V3 = -100,
    V4 = 100,
}

fn main() {
    let v = black_box(E::V0);
    println!("disc/isize/0: as_int={} debug={:?}", v as isize, v);
    let bytes: [u8; size_of::<E>()] = unsafe { std::mem::transmute(v) };
    println!("disc_bytes/isize/0: {:?}", bytes);
    let v = black_box(E::V1);
    println!("disc/isize/1: as_int={} debug={:?}", v as isize, v);
    let bytes: [u8; size_of::<E>()] = unsafe { std::mem::transmute(v) };
    println!("disc_bytes/isize/1: {:?}", bytes);
    let v = black_box(E::V2);
    println!("disc/isize/2: as_int={} debug={:?}", v as isize, v);
    let bytes: [u8; size_of::<E>()] = unsafe { std::mem::transmute(v) };
    println!("disc_bytes/isize/2: {:?}", bytes);
    let v = black_box(E::V3);
    println!("disc/isize/3: as_int={} debug={:?}", v as isize, v);
    let bytes: [u8; size_of::<E>()] = unsafe { std::mem::transmute(v) };
    println!("disc_bytes/isize/3: {:?}", bytes);
    let v = black_box(E::V4);
    println!("disc/isize/4: as_int={} debug={:?}", v as isize, v);
    let bytes: [u8; size_of::<E>()] = unsafe { std::mem::transmute(v) };
    println!("disc_bytes/isize/4: {:?}", bytes);
}