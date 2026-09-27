// Candidate 096 repro.
//
// Reproduces a real end-to-end crash (SIGABRT, no diagnostic message) reachable through this
// project's own real GPU-vs-CPU differential comparison harness,
// `vello_tests::compare_gpu_cpu_sync` (the exact real public entry point patched by
// `patch.diff`, targeting `vello_tests::get_scene_image`), when it is used to sweep a
// realistically-sized batch of scenes containing dashed strokes - one of the three scene
// categories (dashed strokes, glyph runs, blurred rounded rects) this candidate's own harness
// (see `harness/compare_gpu_cpu_fuzz_text_dash_blur.rs` in this candidate's folder) added
// coverage for, that candidates 082/089's own differential-fuzzing grammar never exercised.
//
// The crash is NOT specific to dashed-stroke *content* - it reproduces identically with plain
// dashed strokes alone (this repro), with blurred rounded rects alone, and with glyph runs mixed
// in (see RESULT.md). The common factor is purely the number of independent
// `vello::util::RenderContext` (a real `wgpu::Instance`/`Device`/`Queue`) + `vello::Renderer`
// instances created and dropped within one process - exactly the pattern
// `vello_tests::get_scene_image` used for *every single render* before this candidate's patch
// (its own source already flagged this as wasteful via a `// TODO: Reuse the same
// RenderContext?` comment, and `vello::util::RenderContext::new`'s own doc comment separately
// says creating a `wgpu::Instance` "should only be done rarely").
//
// Under the SwiftShader software Vulkan ICD used in this environment (no real GPU hardware was
// available - see RESULT.md), each such `RenderContext` leaks a `memfd:swiftshader_jit` file
// descriptor that is never released on drop (visible via `/proc/<pid>/fd` on Linux). Once the
// process's open-fd count reaches exactly 1024 (`FD_SETSIZE`, a classic hardcoded
// `select()`/`fd_set` limit), the process aborts with signal 6 (SIGABRT) with no panic message
// and no other diagnostic output at all.
//
// Usage: cargo run --release (a real wgpu::Backends::VULKAN-capable device is required - set
// VK_ICD_FILENAMES to a usable Vulkan ICD if no real GPU is available, e.g. SwiftShader's own
// ICD json).

use vello::kurbo::{Affine, BezPath, Cap, Join, Rect, Shape, Stroke};
use vello::peniko::{Brush, Color, ColorStop, Extend, Gradient};
use vello::{AaConfig, Scene};
use vello_tests::{TestParams, compare_gpu_cpu_sync};

/// Minimal xorshift64* PRNG - deterministic, seedable, no external dependency.
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.f64() * (hi - lo)
    }
    fn index(&mut self, n: usize) -> usize {
        (self.next_u64() as usize) % n
    }
    fn color(&mut self) -> Color {
        Color::from_rgba8(
            self.index(256) as u8,
            self.index(256) as u8,
            self.index(256) as u8,
            self.index(256) as u8,
        )
    }
}

/// Random dashed-stroke scene - one of the three scene categories this candidate's harness added
/// coverage for (candidates 082/089's own grammars never used `Stroke::dash_pattern`).
fn random_dash_scene(seed: u64, w: f64, h: f64) -> Scene {
    let mut rng = Rng::new(seed);
    let mut scene = Scene::new();
    let n_shapes = 2 + rng.index(6);
    for _ in 0..n_shapes {
        let shape: BezPath = Rect::new(
            rng.range(-20.0, w),
            rng.range(-20.0, h),
            rng.range(0.0, w + 20.0),
            rng.range(0.0, h + 20.0),
        )
        .to_path(0.1);
        let n_dashes = 1 + rng.index(4);
        let dashes: Vec<f64> = (0..n_dashes)
            .map(|_| {
                if rng.index(8) == 0 {
                    0.0
                } else {
                    rng.range(0.1, 15.0)
                }
            })
            .collect();
        let offset = rng.range(0.0, 10.0);
        let stroke = Stroke::new(rng.range(0.0, 12.0))
            .with_caps(Cap::Butt)
            .with_join(Join::Miter)
            .with_dashes(offset, dashes);
        let stops = vec![
            ColorStop {
                offset: 0.0,
                color: rng.color().into(),
            },
            ColorStop {
                offset: 1.0,
                color: rng.color().into(),
            },
        ];
        let brush = Brush::Gradient(
            Gradient::new_linear((0.0, 0.0), (w, h))
                .with_stops(stops.as_slice())
                .with_extend(Extend::Pad),
        );
        scene.stroke(&stroke, Affine::IDENTITY, &brush, None, &shape);
    }
    scene
}

fn fd_count() -> usize {
    std::fs::read_dir("/proc/self/fd").unwrap().count()
}

fn main() {
    // Matches candidates 082/089's own canvas size (400x400). Each `compare_gpu_cpu_sync` call
    // renders the scene once via the real GPU path and once via the real CPU-shader-shim path
    // (`vello_tests::get_scene_image`, called twice per scene) - the exact real entry point this
    // candidate's `patch.diff` fixes.
    for i in 0..600_u64 {
        let seed = 1 + i;
        println!("about_to_run seed={seed} open_fds={}", fd_count());
        let scene = random_dash_scene(seed, 400.0, 400.0);
        let mut params = TestParams::new(format!("repro096_{seed}"), 400, 400);
        params.anti_aliasing = AaConfig::Area;
        compare_gpu_cpu_sync(scene, params).expect("compare_gpu_cpu_sync failed");
    }
    println!("completed 600 scenes without crashing");
}
