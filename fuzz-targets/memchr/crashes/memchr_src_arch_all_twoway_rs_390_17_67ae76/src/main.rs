use memchr::arch::all::twoway::FinderRev;

fn main() {
    let finder_rev = FinderRev::new(&include_bytes!("data_0.bin").to_vec());
    let _ = finder_rev.rfind(&include_bytes!("data_1.bin").to_vec(), &vec![101u8, 115u8, 93u8, 10u8, 97u8, 108u8, 108u8, 111u8, 99u8, 32u8, 61u8, 32u8, 91u8, 93u8, 10u8, 100u8, 101u8, 102u8, 97u8, 117u8, 108u8, 116u8, 32u8, 61u8, 32u8, 91u8, 34u8, 45u8]);
}
