use vello_common::transforms::RootTransforms;

fn main() {
    let mut root_transforms = RootTransforms::new();
    root_transforms.pop_root();
    let _ = root_transforms.root_transform();
}
