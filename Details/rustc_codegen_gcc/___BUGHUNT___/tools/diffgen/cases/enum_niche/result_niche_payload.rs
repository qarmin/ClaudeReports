use std::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize, NonZeroI32};
use std::hint::black_box;
use std::mem::size_of;

fn main() {
    { let ok: Result<u32, ()> = Ok(black_box(123u32));
      let err: Result<u32, ()> = Err(());
      let ok_val = match ok { Ok(v) => format!("{:?}", v), Err(_) => "ERR".to_string() };
      let err_val = match err { Ok(v) => format!("{:?}", v), Err(_) => "ERR".to_string() };
      println!("result_niche/u32: ok={} err={}", ok_val, err_val); }
    { let ok: Result<i64, ()> = Ok(black_box(-123i64));
      let err: Result<i64, ()> = Err(());
      let ok_val = match ok { Ok(v) => format!("{:?}", v), Err(_) => "ERR".to_string() };
      let err_val = match err { Ok(v) => format!("{:?}", v), Err(_) => "ERR".to_string() };
      println!("result_niche/i64: ok={} err={}", ok_val, err_val); }
    { let ok: Result<bool, ()> = Ok(black_box(true));
      let err: Result<bool, ()> = Err(());
      let ok_val = match ok { Ok(v) => format!("{:?}", v), Err(_) => "ERR".to_string() };
      let err_val = match err { Ok(v) => format!("{:?}", v), Err(_) => "ERR".to_string() };
      println!("result_niche/bool: ok={} err={}", ok_val, err_val); }
}
