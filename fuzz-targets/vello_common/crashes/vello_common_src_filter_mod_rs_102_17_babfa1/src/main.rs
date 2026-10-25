use kurbo::Affine;
use vello_common::filter::PreparedFilter;
use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterPrimitive;

fn main() {
    let _ = PreparedFilter::new(&Filter::from_primitive(FilterPrimitive::Image { image_id: 0u32, transform: None }), &Affine::scale(0.0f64));
}
