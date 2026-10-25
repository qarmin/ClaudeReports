use vello_common::filter::PreparedFilter;
use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterGraph;
use vello_common::transforms::RootTransforms;
use vello_common::transforms::Transforms;

fn main() {
    let root_transforms = RootTransforms::new();
    let affine = root_transforms.effective_path_transform(&Transforms::default());
    let _ = PreparedFilter::new(&Filter { graph: std::sync::Arc::new(FilterGraph::new()) }, &affine);
}
