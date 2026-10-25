use vello_common::record::Node;

fn main() {
    let node = Node { draws: 0u32..1u32, layer: None };
    let _ = node.draws_in::<u32>(&[]);
}
