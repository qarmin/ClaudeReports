use rowan::GreenNodeBuilder;
use rowan::NodeCache;

fn main() {
    let mut node_cache = NodeCache::default();
    let green_node_builder = GreenNodeBuilder::with_cache(&mut node_cache);
    green_node_builder.finish();
}
