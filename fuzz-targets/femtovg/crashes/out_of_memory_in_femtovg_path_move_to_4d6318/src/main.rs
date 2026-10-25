use femtovg::Paint;
use femtovg::Path;
use femtovg::Solidity;

fn main() {
    let f32_v = Paint::FONT_WEIGHT_EXTRA_BOLD;
    let mut path = Path::default();
    path.arc(0.0f32, f32_v, 10000000000.0f32, 0.0f32, 0.5f32, Solidity::default());
    let path2 = path.dashed(&[f32_v], f32_v);
    let mut path3 = path2.dashed(&[], 0.0f32);
    path3.move_to(f32_v, 0.0f32);
}
