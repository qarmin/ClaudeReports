use drm_fourcc::UnrecognizedFourcc;

fn main() {
    let unrecognized_fourcc = UnrecognizedFourcc(2173415155u32);
    let _ = unrecognized_fourcc.string_form();
}
