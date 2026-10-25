use kurbo::Cap;
use kurbo::CuspType;
use peniko::ImageFormat;
use vello_common::encode::SweepKind;
use vello_common::filter::PreparedFilter;
use vello_common::filter::gaussian_blur::GaussianBlur;
use vello_common::filter_effects::EdgeMode;
use vello_common::multi_atlas::Atlas;
use vello_common::multi_atlas::AtlasId;
use vello_common::multi_atlas::AtlasUsageStats;

fn main() {
    let _ = AtlasUsageStats { allocated_area: 1u32, total_area: 1503970760u32, allocated_count: 1u32 };
    let _ = Cap::Square;
    let _ = ImageFormat::Bgra8;
    let _ = SweepKind { start_angle: -0.25f32, inv_angle_delta: f32::NAN };
    let atlas = Atlas::new(AtlasId(1u32), 8u16, 13959u16);
    let _ = atlas.stats();
    let _ = CuspType::Loop;
    let _ = PreparedFilter::GaussianBlur(GaussianBlur::new(f32::INFINITY, EdgeMode::None));
}
