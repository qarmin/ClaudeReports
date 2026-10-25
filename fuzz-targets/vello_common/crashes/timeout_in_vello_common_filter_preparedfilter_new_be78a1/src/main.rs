use kurbo::Affine;
use kurbo::Point;
use vello_common::filter::PreparedFilter;
use vello_common::filter_effects::EdgeMode;
use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterPrimitive;

fn main() {
    let _ = PreparedFilter::new(&Filter::from_primitive(FilterPrimitive::GaussianBlur { std_deviation: 1.0356042e24f32, edge_mode: EdgeMode::None }), &Affine::rotate_about(0.0f64, Point { x: 0.0f64, y: 1.0f64 }));
}
