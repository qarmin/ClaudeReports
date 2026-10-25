use drm_fourcc::DrmFourcc;

fn main() {
    let drm_fourcc = DrmFourcc::R16f;
    let _ = drm_fourcc.to_string();
}
