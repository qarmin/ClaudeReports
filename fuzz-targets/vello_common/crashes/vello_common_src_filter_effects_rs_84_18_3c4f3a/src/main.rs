use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterFunction;

fn main() {
    let _ = Filter::from_function(FilterFunction::Contrast { amount: 0.0f32 });
}
