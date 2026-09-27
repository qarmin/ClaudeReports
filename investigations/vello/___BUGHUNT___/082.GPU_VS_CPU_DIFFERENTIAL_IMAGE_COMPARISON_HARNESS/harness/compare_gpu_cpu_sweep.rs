// Copyright 2026 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Candidate 082 scratch harness: run `compare_gpu_cpu` (the project's own GPU-vs-CPU
//! differential rendering test, patched with candidate 065's already-recorded self-comparison
//! fix so the comparison is real) across every scene in `scenes::test_scenes::test_scenes()`,
//! not just the 10 scenes `compare_gpu_cpu.rs` already covers. Reports the FLIP mean for each
//! scene so a real, previously-undetectable-because-of-065 GPU-vs-CPU divergence, if one exists
//! in any of the untested scenes, would show up here.
//!
//! Not part of the permanent test suite - scratch investigation for this candidate only.

use scenes::test_scenes;
use vello::AaConfig;
use vello_tests::{TestParams, compare_gpu_cpu_sync, encode_test_scene};

#[test]
#[ignore = "candidate 082 scratch sweep, not part of the normal suite"]
fn sweep_all_scenes() {
    sweep_with_aa(AaConfig::Area);
}

#[test]
#[ignore = "candidate 082 scratch sweep, not part of the normal suite"]
fn sweep_all_scenes_msaa16() {
    sweep_with_aa(AaConfig::Msaa16);
}

fn sweep_with_aa(aa: AaConfig) {
    let set = test_scenes::test_scenes();
    let mut failures = Vec::new();
    for scene in set.scenes {
        let name = scene.config.name.clone();
        let mut params = TestParams::new(format!("sweep_{name}"), 800, 800);
        params.anti_aliasing = aa;
        let encoded = encode_test_scene(scene, &mut params);
        match compare_gpu_cpu_sync(encoded, params) {
            Ok(comparison) => {
                let mean = comparison.statistics.as_ref().map(|s| s.mean());
                println!("scene={name:<40} mean={mean:?}");
                if let Some(mean) = mean {
                    if mean > 0.01 {
                        failures.push(format!("{name}: mean={mean}"));
                    }
                }
            }
            Err(e) => {
                println!("scene={name:<40} ERROR={e:?}");
                failures.push(format!("{name}: error={e:?}"));
            }
        }
    }
    assert!(failures.is_empty(), "failures: {failures:#?}");
}
