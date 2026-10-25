use memchr::arch::all::twoway::FinderRev;

fn main() {
    let finder_rev = FinderRev::new(&vec![86u8, 86u8, 93u8]);
    let _ = finder_rev.rfind(&vec![93u8], &vec![0u8]);
}
