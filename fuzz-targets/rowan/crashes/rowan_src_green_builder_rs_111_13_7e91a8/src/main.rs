use rowan::GreenNodeBuilder;
use rowan::SyntaxKind;

fn main() {
    let syntax_kind = SyntaxKind(0u16);
    let mut green_node_builder = GreenNodeBuilder::new();
    let checkpoint = green_node_builder.checkpoint();
    green_node_builder.token(syntax_kind, "");
    green_node_builder.start_node(syntax_kind);
    green_node_builder.start_node_at(checkpoint, SyntaxKind(0u16));
}
