use memchr::arch::all::packedpair::Pair;
use memchr::arch::x86_64::sse2::packedpair::Finder;

fn main() {
    let _ = Finder::with_pair(&vec![], Pair::new(&vec![110u8, 79u8]).unwrap());
}
