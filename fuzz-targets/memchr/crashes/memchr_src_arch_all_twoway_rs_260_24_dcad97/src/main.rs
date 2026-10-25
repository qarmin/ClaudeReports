use memchr::arch::all::twoway::Finder;

fn main() {
    let finder = Finder::new(&vec![164u8, 164u8, 0u8]);
    let _ = finder.find(&vec![0u8], &vec![0u8]);
}
