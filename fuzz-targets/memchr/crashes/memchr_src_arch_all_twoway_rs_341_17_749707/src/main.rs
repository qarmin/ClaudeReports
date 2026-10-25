use memchr::arch::all::twoway::FinderRev;

fn main() {
    let finder_rev = FinderRev::new(&vec![0u8, 0u8]);
    let _ = finder_rev.rfind(&vec![0u8], &vec![109u8]);
}
