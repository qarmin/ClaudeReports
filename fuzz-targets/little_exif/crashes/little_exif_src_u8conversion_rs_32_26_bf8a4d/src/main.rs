use little_exif::endian::Endian;
use little_exif::exif_tag::ExifTag;
use little_exif::exif_tag_format::ExifTagFormat;
use little_exif::ifd::ExifTagGroup;

fn main() {
    let _ = ExifTag::from_u16_with_data(0u16, &ExifTagFormat::FLOAT, &vec![0u8], &Endian::Little, &ExifTagGroup::GENERIC);
}
