use tiff::decoder::TiffHeader;
use tiff::tags::IfdPointer;
use tiff::tags::TiffVariant;
use tiff::tags::Type;
use tiff::tags::ValueBuffer;

fn main() {
    let tiff_header = TiffHeader { byte_order: ValueBuffer::empty(Type::BYTE).byte_order(), variant: TiffVariant::BigTiff, first_ifd: IfdPointer(0u64) };
    let mut decoder = tiff_header.open::<::std::io::Cursor<Vec<u8>>>(std::io::Cursor::new(vec![]));
    let _ = decoder.read_string(10000000000000000000usize);
}
