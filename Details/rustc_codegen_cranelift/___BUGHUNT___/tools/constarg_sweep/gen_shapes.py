import os
HDR = '''use std::arch::x86_64::*;
#[allow(dead_code)]
fn mk(x: i8) -> __m128i { unsafe { _mm_set1_epi8(x) } }
'''
shapes = {}

shapes['s01_literal'] = '''
#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i) -> i32 { _mm_cmpestri::<0>(a, 3, b, 3) }
fn main() { unsafe { println!("{}", go(mk(1), mk(1))); } }
'''

shapes['s02_constgeneric_wrapper'] = '''
#[target_feature(enable = "sse4.2")]
unsafe fn wrap<const N: i32>(a: __m128i, b: __m128i) -> i32 { _mm_cmpestri::<N>(a, 3, b, 3) }
fn main() { unsafe { println!("{}", wrap::<0>(mk(1), mk(1))); } }
'''

shapes['s03_inline_wrapper'] = '''
#[inline]
#[target_feature(enable = "sse4.2")]
unsafe fn wrap<const N: i32>(a: __m128i, b: __m128i) -> i32 { _mm_cmpestri::<N>(a, 3, b, 3) }
fn main() { unsafe { println!("{}", wrap::<0>(mk(1), mk(1))); } }
'''

shapes['s04_branch_same_imm'] = '''
#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i, c: bool) -> i32 {
    if c { _mm_cmpestri::<0>(a, 3, b, 3) } else { _mm_cmpestri::<0>(a, 4, b, 4) }
}
fn main() { unsafe { println!("{} {}", go(mk(1), mk(1), true), go(mk(1), mk(1), false)); } }
'''

shapes['s05_branch_diff_imm'] = '''
#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i, c: bool) -> i32 {
    if c { _mm_cmpestri::<0>(a, 3, b, 3) } else { _mm_cmpestri::<12>(a, 3, b, 3) }
}
fn main() { unsafe { println!("{} {}", go(mk(1), mk(1), true), go(mk(1), mk(1), false)); } }
'''

shapes['s06_loop'] = '''
#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i, n: i32) -> i32 {
    let mut s = 0;
    for i in 0..n { s += _mm_cmpestri::<0>(a, i, b, i); }
    s
}
fn main() { unsafe { println!("{}", go(mk(1), mk(1), 4)); } }
'''

shapes['s07_closure'] = '''
#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i) -> i32 {
    let f = |la: i32| _mm_cmpestri::<0>(a, la, b, la);
    f(3) + f(4)
}
fn main() { unsafe { println!("{}", go(mk(1), mk(1))); } }
'''

shapes['s08_match3'] = '''
#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i, k: u32) -> i32 {
    match k {
        0 => _mm_cmpestri::<0>(a, 3, b, 3),
        1 => _mm_cmpestri::<0>(a, 4, b, 4),
        _ => _mm_cmpestri::<0>(a, 5, b, 5),
    }
}
fn main() { unsafe { println!("{} {} {}", go(mk(1),mk(1),0), go(mk(1),mk(1),1), go(mk(1),mk(1),2)); } }
'''

shapes['s09_cvtps_ph'] = '''
#[target_feature(enable = "f16c")]
unsafe fn go(a: __m128, c: bool) -> i32 {
    let r = if c { _mm_cvtps_ph::<0>(a) } else { _mm_cvtps_ph::<0>(_mm_add_ps(a, a)) };
    _mm_extract_epi16::<0>(r)
}
fn main() { unsafe { println!("{} {}", go(_mm_set1_ps(1.5), true), go(_mm_set1_ps(1.5), false)); } }
'''

shapes['s10_sha1rnds4'] = '''
#[target_feature(enable = "sha,sse2")]
unsafe fn go(a: __m128i, b: __m128i, c: bool) -> i32 {
    let r = if c { _mm_sha1rnds4_epu32::<0>(a, b) } else { _mm_sha1rnds4_epu32::<0>(b, a) };
    _mm_extract_epi32::<0>(r)
}
fn main() { unsafe { println!("{} {}", go(mk(1), mk(2), true), go(mk(1), mk(2), false)); } }
'''

shapes['s11_clmul'] = '''
#[target_feature(enable = "pclmulqdq,sse2")]
unsafe fn go(a: __m128i, b: __m128i, c: bool) -> i32 {
    let r = if c { _mm_clmulepi64_si128::<0>(a, b) } else { _mm_clmulepi64_si128::<0>(b, a) };
    _mm_extract_epi32::<0>(r)
}
fn main() { unsafe { println!("{} {}", go(mk(1), mk(2), true), go(mk(1), mk(2), false)); } }
'''

shapes['s12_aeskeygen'] = '''
#[target_feature(enable = "aes,sse2")]
unsafe fn go(a: __m128i, c: bool) -> i32 {
    let r = if c { _mm_aeskeygenassist_si128::<0>(a) } else { _mm_aeskeygenassist_si128::<0>(_mm_add_epi8(a,a)) };
    _mm_extract_epi32::<0>(r)
}
fn main() { unsafe { println!("{} {}", go(mk(1), true), go(mk(1), false)); } }
'''

shapes['s13_cmpestrm_branch'] = '''
#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i, c: bool) -> i32 {
    let r = if c { _mm_cmpestrm::<0>(a, 3, b, 3) } else { _mm_cmpestrm::<0>(a, 4, b, 4) };
    _mm_extract_epi32::<0>(r)
}
fn main() { unsafe { println!("{} {}", go(mk(1), mk(1), true), go(mk(1), mk(1), false)); } }
'''

shapes['s14_generic_fn_two_inst'] = '''
#[inline]
#[target_feature(enable = "sse4.2")]
unsafe fn wrap<const N: i32>(a: __m128i, la: i32, b: __m128i) -> i32 { _mm_cmpestri::<N>(a, la, b, la) }
#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i, c: bool) -> i32 {
    if c { wrap::<0>(a, 3, b) } else { wrap::<0>(a, 4, b) }
}
fn main() { unsafe { println!("{} {}", go(mk(1),mk(1),true), go(mk(1),mk(1),false)); } }
'''

shapes['s15_extract_insert_branch'] = '''
#[target_feature(enable = "sse4.1")]
unsafe fn go(a: __m128i, c: bool) -> i32 {
    let r = if c { _mm_insert_epi8::<3>(a, 7) } else { _mm_insert_epi8::<3>(_mm_add_epi8(a,a), 9) };
    _mm_extract_epi8::<3>(r)
}
fn main() { unsafe { println!("{} {}", go(mk(1), true), go(mk(1), false)); } }
'''

shapes['s16_recursive_imm'] = '''
#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i, n: u32) -> i32 {
    if n == 0 { return 0; }
    _mm_cmpestri::<0>(a, n as i32, b, 3) + go(a, b, n - 1)
}
fn main() { unsafe { println!("{}", go(mk(1), mk(1), 3)); } }
'''
import pathlib
HERE = pathlib.Path(__file__).resolve().parent
os.makedirs(HERE / 'shapes', exist_ok=True)
for name, body in shapes.items():
    with open(HERE / f'shapes/{name}.rs', 'w') as f:
        f.write(HDR + body)
print(len(shapes))
