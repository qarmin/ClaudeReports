//! Standalone `BezPath::from_svg` bench with a *controllable number of decimal digits*.
//!
//! The `___OPTIMIZATIONS___/harness/kurbo_bench` `svg-parse` scene builds its input with
//! `BezPath::to_svg`, which formats every coordinate with `{}` (shortest round-trip), producing
//! ~17-significant-digit numbers. Real SVG path data written by Inkscape/Illustrator/web
//! tooling carries 1-6 digits. Those two inputs exercise completely different parts of
//! `core`'s `dec2flt`, so a `get_number` change has to be measured on both.
//!
//! Output format is deliberately identical to `kurbo_bench`'s so `tools/paired.sh` works
//! unchanged:
//!
//! ```text
//! scene svg-parse-2 iters 50 warmup 5
//! RESULT median 0.1234 ms  mean 0.1240  p95 0.1300  spread 0.1230..0.1300
//! CHECK 0x...
//! ```

use std::hint::black_box;
use std::time::Instant;

use kurbo::{BezPath, Point};

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn f(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.f() * (hi - lo)
    }
}

/// Same generator as `kurbo_bench::gen_path`, so the "full precision" case is the same input the
/// canonical harness's `svg-parse` scene uses.
fn gen_path(subpaths: usize, per: usize, seed: u64) -> BezPath {
    let mut rng = Rng::new(seed);
    let mut path = BezPath::new();
    for _ in 0..subpaths {
        let cx = rng.range(100.0, 900.0);
        let cy = rng.range(100.0, 900.0);
        let r = rng.range(20.0, 120.0);
        let mut prev = Point::new(cx + r, cy);
        path.move_to(prev);
        for i in 0..per {
            let a0 = (i as f64) / (per as f64) * std::f64::consts::TAU;
            let a1 = ((i + 1) as f64) / (per as f64) * std::f64::consts::TAU;
            let rr = r * rng.range(0.6, 1.5);
            let p3 = Point::new(cx + rr * a1.cos(), cy + rr * a1.sin());
            let k = r * 0.55 * rng.range(0.5, 2.0);
            let p1 = Point::new(prev.x - k * a0.sin(), prev.y + k * a0.cos());
            let p2 = Point::new(p3.x + k * a1.sin(), p3.y - k * a1.cos());
            path.curve_to(p1, p2, p3);
            prev = p3;
        }
        path.close_path();
    }
    path
}

/// `to_svg`, but with every coordinate rounded to `dec` decimal places - i.e. what a real SVG
/// exporter emits. `dec == usize::MAX` means "use kurbo's own `to_svg`" (full precision).
fn to_svg_prec(path: &BezPath, dec: usize) -> String {
    use kurbo::PathEl;
    if dec == usize::MAX {
        return path.to_svg();
    }
    let n = |v: f64| format!("{v:.dec$}");
    let mut s = String::new();
    for (i, el) in path.elements().iter().enumerate() {
        if i > 0 {
            s.push(' ');
        }
        match *el {
            PathEl::MoveTo(p) => s += &format!("M{},{}", n(p.x), n(p.y)),
            PathEl::LineTo(p) => s += &format!("L{},{}", n(p.x), n(p.y)),
            PathEl::QuadTo(p1, p2) => {
                s += &format!("Q{},{} {},{}", n(p1.x), n(p1.y), n(p2.x), n(p2.y));
            }
            PathEl::CurveTo(p1, p2, p3) => {
                s += &format!(
                    "C{},{} {},{} {},{}",
                    n(p1.x),
                    n(p1.y),
                    n(p2.x),
                    n(p2.y),
                    n(p3.x),
                    n(p3.y)
                );
            }
            PathEl::ClosePath => s.push('Z'),
        }
    }
    s
}

struct Hash(u64);

impl Hash {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
    fn f(&mut self, v: f64) {
        self.0 ^= v.to_bits();
        self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
    }
    fn path(&mut self, p: &BezPath) {
        use kurbo::PathEl;
        for el in p.elements() {
            match *el {
                PathEl::MoveTo(p) => {
                    self.f(1.0);
                    self.f(p.x);
                    self.f(p.y);
                }
                PathEl::LineTo(p) => {
                    self.f(2.0);
                    self.f(p.x);
                    self.f(p.y);
                }
                PathEl::QuadTo(a, b) => {
                    self.f(3.0);
                    self.f(a.x);
                    self.f(a.y);
                    self.f(b.x);
                    self.f(b.y);
                }
                PathEl::CurveTo(a, b, c) => {
                    self.f(4.0);
                    self.f(a.x);
                    self.f(a.y);
                    self.f(b.x);
                    self.f(b.y);
                    self.f(c.x);
                    self.f(c.y);
                }
                PathEl::ClosePath => self.f(5.0),
            }
        }
    }
}

/// Cross-checks `from_svg`'s number lexer against `core`'s own `str::parse::<f64>()` bit for bit,
/// over a wide spread of token shapes (digit counts, decimal placement, signs, exponents). Any
/// fast path added to `SvgLexer::get_number` has to agree with `core` on every one of these.
fn verify() -> ! {
    let mut rng = Rng::new(12345);
    let mut checked = 0_u64;
    let mut bad = 0_u64;

    let mut cases: Vec<String> = vec![
        "0".into(),
        "-0".into(),
        "+0".into(),
        "0.0".into(),
        "-0.0".into(),
        "1".into(),
        "1.".into(),
        ".5".into(),
        "-.5".into(),
        "+.5".into(),
        "0.1".into(),
        "1e5".into(),
        "1E5".into(),
        "1e-5".into(),
        "1e+5".into(),
        "1.5e22".into(),
        "1.5e23".into(),
        "1.5e-22".into(),
        "1.5e-23".into(),
        "9007199254740992".into(),
        "9007199254740993".into(),
        "9007199254740992e-1".into(),
        "1234567890123456789".into(),
        "12345678901234567890".into(),
        "0000000000000000000005".into(),
        "1e308".into(),
        "1e-308".into(),
        "1e999".into(),
        "1e-999".into(),
        "5e-324".into(),
        "0.000000000000000000000000001".into(),
        "123456789012345678901234567890.12345".into(),
    ];

    for _ in 0..400_000 {
        let ndig = 1 + (rng.next_u64() % 22) as usize;
        let point = (rng.next_u64() % (ndig as u64 + 2)) as usize;
        let mut s = String::new();
        match rng.next_u64() % 4 {
            0 => s.push('-'),
            1 => s.push('+'),
            _ => {}
        }
        for i in 0..ndig {
            if i == point {
                s.push('.');
            }
            s.push((b'0' + (rng.next_u64() % 10) as u8) as char);
        }
        if point >= ndig {
            s.push('.');
        }
        if rng.next_u64() % 3 == 0 {
            s.push(if rng.next_u64() % 2 == 0 { 'e' } else { 'E' });
            match rng.next_u64() % 3 {
                0 => s.push('-'),
                1 => s.push('+'),
                _ => {}
            }
            let ed = 1 + (rng.next_u64() % 4);
            for _ in 0..ed {
                s.push((b'0' + (rng.next_u64() % 10) as u8) as char);
            }
        }
        cases.push(s);
    }

    for c in &cases {
        let want: f64 = match c.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        // `H<n>` sets x from a single number, so the parsed f64 lands verbatim in the path.
        let src = format!("M0,0 H{c}");
        let got = match BezPath::from_svg(&src) {
            Ok(p) => match p.elements()[1] {
                kurbo::PathEl::LineTo(pt) => pt.x,
                other => {
                    eprintln!("MISMATCH {c:?}: unexpected element {other:?}");
                    bad += 1;
                    continue;
                }
            },
            Err(e) => {
                eprintln!("MISMATCH {c:?}: from_svg failed ({e}) but str::parse gave {want}");
                bad += 1;
                continue;
            }
        };
        checked += 1;
        if got.to_bits() != want.to_bits() {
            if bad < 20 {
                eprintln!("MISMATCH {c:?}: from_svg {got:e} ({:#x}) != str::parse {want:e} ({:#x})", got.to_bits(), want.to_bits());
            }
            bad += 1;
        }
    }

    println!("VERIFY checked {checked} mismatches {bad}");
    std::process::exit(if bad == 0 { 0 } else { 1 });
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let scene = args.get(1).map(String::as_str).unwrap_or("svg-parse-2");
    let iters: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50);

    if scene == "verify" {
        verify();
    }

    let dec = match scene {
        "svg-parse-full" => usize::MAX,
        "svg-parse-1" => 1,
        "svg-parse-2" => 2,
        "svg-parse-3" => 3,
        "svg-parse-6" => 6,
        _ => {
            eprintln!(
                "scenes: svg-parse-full svg-parse-1 svg-parse-2 svg-parse-3 svg-parse-6 [iters]"
            );
            std::process::exit(2);
        }
    };

    let path = gen_path(200, 8, 9);
    let s = to_svg_prec(&path, dec);

    let warmup = std::cmp::max(2, iters / 10);
    println!("scene {scene} iters {iters} warmup {warmup} bytes {}", s.len());

    let mut first_check: Option<u64> = None;
    let mut times = Vec::with_capacity(iters);
    for i in 0..(warmup + iters) {
        let t0 = Instant::now();
        let p = BezPath::from_svg(black_box(&s)).unwrap();
        let mut h = Hash::new();
        h.path(&p);
        let check = black_box(h.0);
        let dt = t0.elapsed().as_secs_f64() * 1e3;
        match first_check {
            None => first_check = Some(check),
            Some(c) if c != check => {
                eprintln!("NONDETERMINISTIC: check {check:#x} != {c:#x}");
                std::process::exit(3);
            }
            _ => {}
        }
        if i >= warmup {
            times.push(dt);
        }
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = times[times.len() / 2];
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let p95 = times[(times.len() * 95 / 100).min(times.len() - 1)];
    println!(
        "RESULT median {median:.4} ms  mean {mean:.4}  p95 {p95:.4}  spread {:.4}..{:.4}",
        times[0],
        times[times.len() - 1]
    );
    println!("CHECK {:#018x}", first_check.unwrap());
}
