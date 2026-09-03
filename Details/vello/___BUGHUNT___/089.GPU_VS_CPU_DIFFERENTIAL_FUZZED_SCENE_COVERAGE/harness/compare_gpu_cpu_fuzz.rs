// Copyright 2026 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Candidate 089 differential harness: build a corpus of randomly generated scenes (rather than
//! only the hand-authored example scenes candidate 082 already swept) and compare the real
//! `vello` GPU renderer against the real `vello_cpu` renderer for each one via
//! `vello_tests::compare_gpu_cpu_sync` (patched per candidate 065's already-recorded fix, so the
//! comparison is real rather than a hardcoded 0.0 self-comparison).
//!
//! Not part of the permanent test suite - scratch investigation for this candidate only.
//! Deliberately uses a small self-contained PRNG (no `rand` dependency) to keep this a one-off
//! harness, not a real fuzzing campaign.

use vello::kurbo::{Affine, BezPath, Cap, Circle, Join, Point, Rect, RoundedRect, Shape, Stroke};
use vello::peniko::{
    Blob, BlendMode, Brush, Color, ColorStop, Compose, Extend, Fill, Gradient, ImageAlphaType,
    ImageBrush, ImageData, ImageFormat, Mix,
};
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

    fn bool(&mut self) -> bool {
        self.next_u64() & 1 == 0
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

fn random_shape(rng: &mut Rng, w: f64, h: f64) -> BezPath {
    match rng.index(4) {
        0 => Rect::new(
            rng.range(-20.0, w),
            rng.range(-20.0, h),
            rng.range(0.0, w + 20.0),
            rng.range(0.0, h + 20.0),
        )
        .to_path(0.1),
        1 => {
            let radius = rng.range(0.0, w.min(h) / 2.0);
            RoundedRect::new(
                rng.range(-20.0, w),
                rng.range(-20.0, h),
                rng.range(0.0, w + 20.0),
                rng.range(0.0, h + 20.0),
                radius,
            )
            .to_path(0.1)
        }
        2 => Circle::new(
            (rng.range(0.0, w), rng.range(0.0, h)),
            rng.range(0.0, w.min(h)),
        )
        .to_path(0.1),
        _ => {
            // Random closed polygon/curve mix, including possibly-degenerate points
            // (repeated points, self-intersections, near-zero-area shapes).
            let mut path = BezPath::new();
            let n = 3 + rng.index(5);
            let start = Point::new(rng.range(-10.0, w), rng.range(-10.0, h));
            path.move_to(start);
            for _ in 0..n {
                let p = Point::new(rng.range(-10.0, w + 10.0), rng.range(-10.0, h + 10.0));
                if rng.bool() {
                    path.line_to(p);
                } else {
                    let c1 = Point::new(rng.range(-10.0, w + 10.0), rng.range(-10.0, h + 10.0));
                    let c2 = Point::new(rng.range(-10.0, w + 10.0), rng.range(-10.0, h + 10.0));
                    path.curve_to(c1, c2, p);
                }
            }
            path.close_path();
            path
        }
    }
}

fn random_stops(rng: &mut Rng) -> Vec<ColorStop> {
    let n = 2 + rng.index(3);
    (0..n)
        .map(|i| ColorStop {
            offset: (i as f32) / (n as f32 - 1.0).max(1.0),
            color: rng.color().into(),
        })
        .collect()
}

fn random_extend(rng: &mut Rng) -> Extend {
    match rng.index(3) {
        0 => Extend::Pad,
        1 => Extend::Repeat,
        _ => Extend::Reflect,
    }
}

fn random_brush(rng: &mut Rng, w: f64, h: f64) -> Brush {
    match rng.index(4) {
        0 => Brush::Solid(rng.color()),
        1 => {
            let start = (rng.range(0.0, w), rng.range(0.0, h));
            // Occasionally degenerate: start == end.
            let end = if rng.index(6) == 0 {
                start
            } else {
                (rng.range(0.0, w), rng.range(0.0, h))
            };
            let grad = Gradient::new_linear(start, end)
                .with_stops(random_stops(rng).as_slice())
                .with_extend(random_extend(rng));
            Brush::Gradient(grad)
        }
        2 => {
            let center = (rng.range(0.0, w), rng.range(0.0, h));
            // Occasionally degenerate: zero radius (candidate 086's class).
            let radius = if rng.index(6) == 0 {
                0.0
            } else {
                rng.range(0.0, w.min(h)) as f32
            };
            let grad = Gradient::new_radial(center, radius)
                .with_stops(random_stops(rng).as_slice())
                .with_extend(random_extend(rng));
            Brush::Gradient(grad)
        }
        _ => {
            let center = (rng.range(0.0, w), rng.range(0.0, h));
            let start_angle = rng.range(0.0, std::f64::consts::TAU) as f32;
            // Occasionally degenerate: start_angle == end_angle (candidate 081's class).
            let end_angle = if rng.index(6) == 0 {
                start_angle
            } else {
                rng.range(0.0, std::f64::consts::TAU) as f32
            };
            let grad = Gradient::new_sweep(center, start_angle, end_angle)
                .with_stops(random_stops(rng).as_slice())
                .with_extend(random_extend(rng));
            Brush::Gradient(grad)
        }
    }
}

fn random_transform(rng: &mut Rng, w: f64, h: f64) -> Affine {
    let translate = Affine::translate((rng.range(-w / 4.0, w / 4.0), rng.range(-h / 4.0, h / 4.0)));
    let rotate = Affine::rotate(rng.range(0.0, std::f64::consts::TAU));
    let scale = if rng.index(8) == 0 {
        // Occasionally near-degenerate scale.
        Affine::scale(rng.range(0.0, 0.01))
    } else {
        Affine::scale_non_uniform(rng.range(0.1, 2.0), rng.range(0.1, 2.0))
    };
    translate * rotate * scale
}

fn random_stroke(rng: &mut Rng) -> Stroke {
    let join = match rng.index(3) {
        0 => Join::Bevel,
        1 => Join::Miter,
        _ => Join::Round,
    };
    let cap = match rng.index(3) {
        0 => Cap::Butt,
        1 => Cap::Square,
        _ => Cap::Round,
    };
    Stroke {
        width: rng.range(0.0, 12.0),
        join,
        miter_limit: rng.range(1.0, 10.0),
        start_cap: cap,
        end_cap: cap,
        dash_pattern: Default::default(),
        dash_offset: 0.0,
    }
}

fn random_scene(seed: u64, w: f64, h: f64) -> Scene {
    let mut rng = Rng::new(seed);
    let mut scene = Scene::new();

    let n_shapes = 2 + rng.index(6);
    let mut depth = 0_u32;
    for _ in 0..n_shapes {
        // Occasionally push a clip/blend layer, popped before the scene ends.
        if depth < 3 && rng.index(4) == 0 {
            let clip_shape = random_shape(&mut rng, w, h);
            let alpha = rng.range(0.2, 1.0) as f32;
            let mix = match rng.index(4) {
                0 => Mix::Normal,
                1 => Mix::Multiply,
                2 => Mix::Screen,
                _ => Mix::Darken,
            };
            let compose = match rng.index(3) {
                0 => Compose::SrcOver,
                1 => Compose::Clear,
                _ => Compose::Xor,
            };
            scene.push_layer(
                Fill::NonZero,
                BlendMode::new(mix, compose),
                alpha,
                random_transform(&mut rng, w, h),
                &clip_shape,
            );
            depth += 1;
            continue;
        }
        if depth > 0 && rng.index(4) == 0 {
            scene.pop_layer();
            depth -= 1;
            continue;
        }

        let shape = random_shape(&mut rng, w, h);
        let transform = random_transform(&mut rng, w, h);
        let brush = random_brush(&mut rng, w, h);
        let fill_rule = if rng.bool() {
            Fill::NonZero
        } else {
            Fill::EvenOdd
        };

        if rng.bool() {
            scene.fill(fill_rule, transform, &brush, None, &shape);
        } else {
            let stroke = random_stroke(&mut rng);
            scene.stroke(&stroke, transform, &brush, None, &shape);
        }
    }
    while depth > 0 {
        scene.pop_layer();
        depth -= 1;
    }

    // Occasionally include a tiny random solid-color "image" brush fill, to exercise the image
    // path too (not just gradients/solids).
    if rng.index(3) == 0 {
        let iw = 2 + rng.index(4) as u32;
        let ih = 2 + rng.index(4) as u32;
        let mut data = vec![0_u8; (iw * ih * 4) as usize];
        for px in data.chunks_exact_mut(4) {
            px[0] = rng.index(256) as u8;
            px[1] = rng.index(256) as u8;
            px[2] = rng.index(256) as u8;
            px[3] = rng.index(256) as u8;
        }
        let image = ImageData {
            data: Blob::new(std::sync::Arc::new(data)),
            format: ImageFormat::Rgba8,
            width: iw,
            height: ih,
            alpha_type: ImageAlphaType::AlphaPremultiplied,
        };
        let brush = ImageBrush::new(image).with_extend(random_extend(&mut rng));
        let shape = random_shape(&mut rng, w, h);
        scene.fill(Fill::NonZero, random_transform(&mut rng, w, h), &brush, None, &shape);
    }

    scene
}

/// Runs `count` randomly generated scenes through the real GPU-vs-CPU differential comparison,
/// starting at `seed_base` (so a failing seed can be re-run in isolation).
fn run_fuzz_sweep(seed_base: u64, count: u64, aa: AaConfig) {
    let mut failures = Vec::new();
    let mut worst: Option<(u64, f32)> = None;
    for i in 0..count {
        let seed = seed_base + i;
        let scene = random_scene(seed, 400.0, 400.0);
        let mut params = TestParams::new(format!("fuzz_{seed}"), 400, 400);
        params.anti_aliasing = aa;
        match compare_gpu_cpu_sync(scene, params) {
            Ok(comparison) => {
                let mean = comparison.statistics.as_ref().map(|s| s.mean());
                if let Some(mean) = mean {
                    if worst.is_none_or(|(_, m)| mean > m) {
                        worst = Some((seed, mean));
                    }
                    if mean > 0.01 {
                        println!("seed={seed} mean={mean} FAIL");
                        failures.push(format!("seed={seed}: mean={mean}"));
                    }
                }
            }
            Err(e) => {
                println!("seed={seed} ERROR={e:?}");
                failures.push(format!("seed={seed}: error={e:?}"));
            }
        }
    }
    println!(
        "ran {count} fuzzed scenes starting at seed {seed_base}, {} failures, worst={worst:?}",
        failures.len()
    );
    assert!(failures.is_empty(), "failures: {failures:#?}");
}

#[test]
#[ignore = "candidate 089 scratch fuzz sweep, not part of the normal suite"]
fn fuzz_sweep_area() {
    run_fuzz_sweep(1, 200, AaConfig::Area);
}

#[test]
#[ignore = "candidate 089 scratch fuzz sweep, not part of the normal suite"]
fn fuzz_sweep_msaa16() {
    run_fuzz_sweep(1, 80, AaConfig::Msaa16);
}
