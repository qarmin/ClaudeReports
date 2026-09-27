// Pure-math search (no GPU/pipeline) over candidate LineSoup (p0,p1) pairs for which
// path_tiling's own coarse-rasterization formula (copied verbatim, unmodified, from
// vello_shaders/src/cpu/path_tiling.rs's `path_tiling_main`) would produce a tile whose p0/p1
// (tile-relative) coordinates fall outside [0, TILE_WIDTH]/[0, TILE_HEIGHT] - i.e. would trip
// that file's `assert!`s. This is only an *offline search tool* to cheaply explore the float
// input space (no GPU/full-pipeline overhead per iteration); any hit would still need to be
// verified by actually running the real end-to-end pipeline (Scene -> Renderer with
// use_cpu:true) with a real path whose flattened LineSoup line matches the found (p0,p1) - see
// `pipeline_checks.rs` in this same folder for that end-to-end harness.
//
// Sanity check performed this session (not reproduced here as a separate binary, but easy to
// redo: comment out the two `robust_err`/`a -=` lines below): with the `ROBUST_EPSILON`
// correction disabled, this exact harness finds violations within a few hundred thousand trials
// at every tested magnitude - confirming the harness's detection logic is sound, and that the
// negative result below (fix left enabled, ~18M trials, 0 violations) reflects the fix actually
// working, not a broken checker.

const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const TILE_SCALE: f32 = 1.0 / 16.0;
const ONE_MINUS_ULP: f32 = 0.99999994;
const ROBUST_EPSILON: f32 = 2e-7;

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f32,
    y: f32,
}
impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn mul(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s)
    }
    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y)
    }
    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y)
    }
}

fn span(a: f32, b: f32) -> u32 {
    (f32::max(f32::ceil(f32::max(a, b)) - f32::floor(f32::min(a, b)), 1.0)) as u32
}

/// Returns Some((p0,p1)) tile-relative coords if computed, along with whether they violate the
/// [0,W]/[0,H] invariant path_tiling.rs asserts on, for the single tile that would be visited at
/// `seg_within_line` (checked for every value from 0..count so we don't have to also replicate
/// the outer per-seg-count iteration/tile bookkeeping - this only needs the coarse-raster math).
fn check_violation(p0_in: Vec2, p1_in: Vec2) -> Option<(u32, Vec2, Vec2)> {
    let is_down = p1_in.y >= p0_in.y;
    let (xy0, xy1) = if is_down {
        (p0_in, p1_in)
    } else {
        (p1_in, p0_in)
    };
    let s0 = xy0.mul(TILE_SCALE);
    let s1 = xy1.mul(TILE_SCALE);
    let count_x = span(s0.x, s1.x).wrapping_sub(1);
    let count = count_x.wrapping_add(span(s0.y, s1.y));
    if count == 0 || count > 100_000 {
        return None;
    }

    let dx = (s1.x - s0.x).abs();
    let dy = s1.y - s0.y;
    if dx + dy == 0.0 {
        return None;
    }
    let idxdy = 1.0 / (dx + dy);
    let mut a = dx * idxdy;
    let is_positive_slope = s1.x >= s0.x;
    let sign = if is_positive_slope { 1.0 } else { -1.0 };
    let xt0 = (s0.x * sign).floor();
    let c = s0.x * sign - xt0;
    let y0 = s0.y.floor();
    let ytop = if s0.y == s1.y { s0.y.ceil() } else { y0 + 1.0 };
    let b = ((dy * c + dx * (ytop - s0.y)) * idxdy).min(ONE_MINUS_ULP);
    let robust_err = (a * (count as f32 - 1.0) + b).floor() - count_x as f32;
    if robust_err != 0.0 {
        a -= ROBUST_EPSILON.copysign(robust_err);
    }
    let x0 = xt0 * sign + if is_positive_slope { 0.0 } else { -1.0 };

    for seg_within_line in 0..count {
        let z = (a * seg_within_line as f32 + b).floor();
        let x = x0 as i32 + (sign * z) as i32;
        let y = (y0 + seg_within_line as f32 - z) as i32;

        let tile_xy = Vec2::new(x as f32 * TILE_WIDTH as f32, y as f32 * TILE_HEIGHT as f32);
        let tile_xy1 = tile_xy.add(Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32));

        let mut lxy0 = xy0;
        let mut lxy1 = xy1;

        if seg_within_line > 0 {
            let z_prev = (a * (seg_within_line as f32 - 1.0) + b).floor();
            if z == z_prev {
                let mut xt =
                    lxy0.x + (lxy1.x - lxy0.x) * (tile_xy.y - lxy0.y) / (lxy1.y - lxy0.y);
                xt = xt.clamp(tile_xy.x + 1e-3, tile_xy1.x);
                lxy0 = Vec2::new(xt, tile_xy.y);
            } else {
                let x_clip = if is_positive_slope {
                    tile_xy.x
                } else {
                    tile_xy1.x
                };
                let mut yt =
                    lxy0.y + (lxy1.y - lxy0.y) * (x_clip - lxy0.x) / (lxy1.x - lxy0.x);
                yt = yt.clamp(tile_xy.y + 1e-3, tile_xy1.y);
                lxy0 = Vec2::new(x_clip, yt);
            }
        }
        if seg_within_line < count - 1 {
            let z_next = (a * (seg_within_line as f32 + 1.0) + b).floor();
            if z == z_next {
                let mut xt =
                    lxy0.x + (lxy1.x - lxy0.x) * (tile_xy1.y - lxy0.y) / (lxy1.y - lxy0.y);
                xt = xt.clamp(tile_xy.x + 1e-3, tile_xy1.x);
                lxy1 = Vec2::new(xt, tile_xy1.y);
            } else {
                let x_clip = if is_positive_slope {
                    tile_xy1.x
                } else {
                    tile_xy.x
                };
                let mut yt =
                    lxy0.y + (lxy1.y - lxy0.y) * (x_clip - lxy0.x) / (lxy1.x - lxy0.x);
                yt = yt.clamp(tile_xy.y + 1e-3, tile_xy1.y);
                lxy1 = Vec2::new(x_clip, yt);
            }
        }

        let p0 = lxy0.sub(tile_xy);
        let p1 = lxy1.sub(tile_xy);

        let bad = |v: f32, max: f32| !(v >= 0.0 && v <= max);
        if bad(p0.x, TILE_WIDTH as f32)
            || bad(p0.y, TILE_HEIGHT as f32)
            || bad(p1.x, TILE_WIDTH as f32)
            || bad(p1.y, TILE_HEIGHT as f32)
        {
            return Some((seg_within_line, p0, p1));
        }
    }
    None
}

struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn f32_in(&mut self, lo: f32, hi: f32) -> f32 {
        let u = (self.next_u64() >> 40) as f32 / (1u32 << 24) as f32;
        lo + u * (hi - lo)
    }
}

fn main() {
    let mut rng = Rng(0xC0FFEE_1234_5678);
    let mut found = 0;
    // Try many random line segments at varying magnitudes/lengths, biased toward
    // near-grid-aligned / near-axis-aligned configurations (where the historical bugs lived).
    let magnitudes: [f32; 9] = [1.0, 16.0, 100.0, 1e3, 1e4, 1e5, 1e6, 1e7, 3e7];
    for &mag in &magnitudes {
        for trial in 0..2_000_000u64 {
            let cx = rng.f32_in(-mag, mag);
            let cy = rng.f32_in(-mag, mag);
            // Bias: sometimes snap near a tile grid line (multiple of 16) plus tiny epsilon.
            let (cx, cy) = if trial % 3 == 0 {
                let kx = (cx / 16.0).round() * 16.0;
                let ky = (cy / 16.0).round() * 16.0;
                (kx, ky)
            } else {
                (cx, cy)
            };
            let len = rng.f32_in(1e-5, 200.0);
            let ang_bias = trial % 4;
            let (dx, dy) = match ang_bias {
                0 => (len, rng.f32_in(-1e-3, 1e-3) * len),
                1 => (rng.f32_in(-1e-3, 1e-3) * len, len),
                2 => (len, len),
                _ => (rng.f32_in(-len, len), rng.f32_in(-len, len)),
            };
            let p0 = Vec2::new(cx, cy);
            let p1 = Vec2::new(cx + dx, cy + dy);
            if let Some((seg, tp0, tp1)) = check_violation(p0, p1) {
                found += 1;
                println!(
                    "VIOLATION mag={mag} trial={trial} p0={p0:?} p1={p1:?} seg_within_line={seg} tile_p0={tp0:?} tile_p1={tp1:?}"
                );
                if found >= 20 {
                    return;
                }
            }
        }
        eprintln!("done mag={mag}, found so far={found}");
    }
    println!("total found: {found}");
}
