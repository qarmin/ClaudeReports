// Reproduction for: `vello_tests::compare_gpu_cpu` never actually compares the GPU-rendered
// image against the CPU-rendered image - it builds *both* of the `RgbaImage`s it feeds into the
// `nv_flip` perceptual-diff computation from `cpu_rendered.data` (see
// `vello_tests/src/compare.rs`, the two `image::RgbaImage::from_raw(cpu_rendered.width,
// cpu_rendered.height, cpu_rendered.data.as_ref().to_vec())` calls). The real `gpu_rendered.data`
// is stored on the returned `GpuCpuComparison` struct (as a public field) but is never read by
// the FLIP computation itself.
//
// This calls the real, unmodified, public `vello_tests::compare_gpu_cpu_sync` entry point -
// exactly the function the project's own `compare_gpu_cpu.rs` test suite calls - driven by a
// real `wgpu` device (this process picks whatever backend/adapter is available; see this
// candidate's justfile for how a real device is obtained in an environment with no GPU
// hardware). No hand-authored stand-in for wgpu or for the renderer is used anywhere.

use anyhow::Result;
use scenes::test_scenes;
use vello_tests::{TestParams, compare_gpu_cpu_sync, encode_test_scene};

fn main() -> Result<()> {
    let test_scene = test_scenes::splash_with_tiger();
    let mut params = TestParams::new("repro_compare_splash", 600, 600);
    let scene = encode_test_scene(test_scene, &mut params);

    let comparison = compare_gpu_cpu_sync(scene, params)?;

    let reported_mean = comparison
        .statistics
        .as_ref()
        .expect("first run for this scene, so a `statistics` value must exist")
        .mean();

    // The real GPU-rendered and CPU-rendered images, exactly as produced by the renderer -
    // these are the correct data the FLIP comparison *should* have used.
    let cpu_bytes = comparison.cpu_rendered.data.as_ref();
    let gpu_bytes = comparison.gpu_rendered.data.as_ref();
    let differing_bytes = cpu_bytes
        .iter()
        .zip(gpu_bytes.iter())
        .filter(|(a, b)| a != b)
        .count();

    println!(
        "compare_gpu_cpu_sync reported statistics.mean() = {reported_mean}\n\
         real cpu_rendered vs real gpu_rendered: {differing_bytes} of {} raw bytes differ",
        cpu_bytes.len()
    );

    // The bug guarantees `reported_mean` is *exactly* 0.0 for every scene, no matter what the
    // real GPU renderer actually produced - because the FLIP computation is fed
    // `cpu_rendered.data` for both of its two inputs. A correct implementation would report a
    // FLIP score computed from the real `gpu_rendered.data` vs the real `cpu_rendered.data`,
    // which need not be exactly 0.0.
    if reported_mean == 0.0_f32 {
        println!(
            "REPRODUCED: compare_gpu_cpu_sync's FLIP score is a hardcoded 0.0 (self-comparison), \
             it never actually consults the real GPU-rendered image."
        );
        std::process::exit(1);
    }

    println!(
        "did not reproduce: statistics.mean() was not exactly 0.0 ({reported_mean}) - the FLIP \
         score seems to have been computed from the real GPU-rendered image after all"
    );
    Ok(())
}
