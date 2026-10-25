use tiff::decoder::Limits;
use tiff::decoder::TiffHeader;
use tiff::tags::ByteOrder;
use tiff::tags::IfdPointer;
use tiff::tags::TiffVariant;

fn main() {
    let tiff_header = TiffHeader { byte_order: ByteOrder::BigEndian, variant: TiffVariant::BigTiff, first_ifd: IfdPointer(0u64) };
    let decoder = tiff_header.open::<::std::io::Cursor<Vec<u8>>>(std::io::Cursor::new(vec![]));
    let decoder2 = decoder.with_limits(Limits::default());
    let mut decoder3 = decoder2.with_limits(Limits::unlimited());
    let _ = decoder3.read_string(100000000000000usize);
}
