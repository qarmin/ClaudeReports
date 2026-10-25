use memchr::arch::all::packedpair::Finder;
use memchr::arch::all::packedpair::Pair;

fn main() {
    let _ = Finder::with_pair(&vec![], Pair::with_indices(&vec![80u8, 78u8], 0u8, 1u8).unwrap());
}
