use kurbo::Affine;
use vello_common::filter::FilterData;
use vello_common::filter_effects::CompositeOperator;
use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterPrimitive;

fn main() {
    let _ = FilterData::new(Filter::from_primitive(FilterPrimitive::Composite { operator: CompositeOperator::In }), Affine::new([0.0f64, 0.0f64, f64::NEG_INFINITY, 0.0f64, 0.0f64, 0.0f64]));
}
