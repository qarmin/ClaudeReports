use rowan::GreenNodeBuilder;
use rowan::NodeCache;
use rowan::SyntaxKind;

fn main() {
    let syntax_kind = SyntaxKind(0u16);
    let mut node_cache = NodeCache::default();
    let mut green_node_builder = GreenNodeBuilder::with_cache(&mut node_cache);
    green_node_builder.token(syntax_kind, "");
    green_node_builder.finish();
}
