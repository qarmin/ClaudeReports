use memchr::arch::all::packedpair::Finder;
use memchr::arch::all::packedpair::Pair;

fn main() {
    let _ = Finder::with_pair(&vec![0u8], Pair::new(&vec![0u8, 0u8]).unwrap());
}
