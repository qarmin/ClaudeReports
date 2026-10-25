use zlib_rs::crc32::Crc32Fold;

fn main() {
    let mut crc32_fold = Crc32Fold::new();
    crc32_fold.fold_copy(&mut vec![], &vec![32u8]);
}
