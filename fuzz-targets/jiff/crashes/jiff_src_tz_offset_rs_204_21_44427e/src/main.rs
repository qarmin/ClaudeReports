use jiff::fmt::strtime::BrokenDownTime;
use jiff::tz::Offset;

fn main() {
    let _ = Offset::constant(100i8).to_datetime(BrokenDownTime::parse(vec![127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8, 127u8], include_bytes!("image_0.tiff").to_vec()).unwrap().timestamp().unwrap());
}
