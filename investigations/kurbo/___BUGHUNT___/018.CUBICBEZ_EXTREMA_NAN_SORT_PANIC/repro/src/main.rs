use kurbo::{CubicBez, ParamCurveExtrema, Point};

fn grid_test() -> (u64, u64) {
    let vals = [0.0_f64, 1.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1e300, -1e300];
    let mut panics = 0u64;
    let mut total = 0u64;
    for &a in &vals {
      for &b in &vals {
        for &c in &vals {
          for &d in &vals {
            for &e in &vals {
              for &f in &vals {
                for &g in &vals {
                  for &h in &vals {
                    total += 1;
                    let cb = CubicBez::new(
                        Point::new(a, b),
                        Point::new(c, d),
                        Point::new(e, f),
                        Point::new(g, h),
                    );
                    let r = std::panic::catch_unwind(|| cb.extrema());
                    if r.is_err() {
                        panics += 1;
                        if panics <= 5 {
                            println!("PANIC on {:?} {:?} {:?} {:?}", (a,b), (c,d), (e,f), (g,h));
                        }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    (total, panics)
}

fn random_bits_test(n: u64) -> u64 {
    let mut state: u64 = 0x853c49e6748fea9bu64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut panics = 0u64;
    for i in 0..n {
        let bits: [u64;4] = [next(), next(), next(), next()];
        let pts: Vec<Point> = bits.iter().map(|&b| {
            let xb = (b & 0xFFFFFFFF) | ((b & 0xFFFFFFFF) << 32);
            let yb = b;
            Point::new(f64::from_bits(xb), f64::from_bits(yb))
        }).collect();
        let cb = CubicBez::new(pts[0], pts[1], pts[2], pts[3]);
        let r = std::panic::catch_unwind(|| cb.extrema());
        if r.is_err() {
            panics += 1;
            if panics <= 5 {
                println!("PANIC iter {} on {:?}", i, pts);
            }
        }
    }
    panics
}

fn manual_boundary_cases() {
    let nan = f64::NAN;
    let inf = f64::INFINITY;
    let cases: Vec<(&str, Point, Point, Point, Point)> = vec![
        ("all NaN", Point::new(nan,nan), Point::new(nan,nan), Point::new(nan,nan), Point::new(nan,nan)),
        ("p1 NaN only", Point::new(0.0,0.0), Point::new(nan,nan), Point::new(1.0,1.0), Point::new(2.0,0.0)),
        ("p2 NaN only", Point::new(0.0,0.0), Point::new(1.0,1.0), Point::new(nan,nan), Point::new(2.0,0.0)),
        ("p0 NaN only", Point::new(nan,nan), Point::new(1.0,1.0), Point::new(0.5,0.5), Point::new(2.0,0.0)),
        ("p3 NaN only", Point::new(0.0,0.0), Point::new(1.0,1.0), Point::new(0.5,0.5), Point::new(nan,nan)),
        ("mixed inf/nan", Point::new(0.0,0.0), Point::new(inf,nan), Point::new(0.5,0.5), Point::new(2.0,0.0)),
        ("all inf", Point::new(inf,inf), Point::new(inf,inf), Point::new(inf,inf), Point::new(inf,inf)),
        ("x NaN via subtraction inf-inf", Point::new(inf,0.0), Point::new(inf,1.0), Point::new(0.0,0.5), Point::new(2.0,0.0)),
    ];
    for (name, p0, p1, p2, p3) in cases {
        let c = CubicBez::new(p0, p1, p2, p3);
        let result = std::panic::catch_unwind(|| c.extrema());
        match result {
            Ok(v) => println!("{name}: OK -> {:?}", v),
            Err(_) => println!("{name}: PANIC"),
        }
    }
}

fn main() {
    println!("=== Approach 1: manual boundary-value construction ===");
    manual_boundary_cases();

    println!("\n=== Approach 2: exhaustive structured grid (8 coords x 8 special values = 8^8) ===");
    let (total, panics) = grid_test();
    println!("grid total={} panics={}", total, panics);

    println!("\n=== Approach 3: random raw-bit-pattern fuzz (covers arbitrary NaN payloads/denormals) ===");
    let n = 20_000_000u64;
    let panics = random_bits_test(n);
    println!("random n={} panics={}", n, panics);
}
