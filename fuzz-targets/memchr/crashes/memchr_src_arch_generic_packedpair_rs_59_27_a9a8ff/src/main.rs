use memchr::arch::all::packedpair::Pair;
use memchr::arch::x86_64::avx2::packedpair::Finder;

fn main() {
    let _ = Finder::with_pair(&vec![97u8], Pair::new(&vec![89u8, 32u8]).unwrap());
}
