use rowan::GreenNodeBuilder;
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

fn main() {
    let syntax_kind = SyntaxKind(0u16);
    let mut green_node_builder = GreenNodeBuilder::new();
    green_node_builder.start_node(syntax_kind);
    green_node_builder.token(syntax_kind, "a");
    green_node_builder.start_node(syntax_kind);
    green_node_builder.finish_node();
    green_node_builder.finish_node();
    let green_node = green_node_builder.finish();
    let syntax_node = SyntaxNode::new_root(green_node.clone());
    let syntax_node2 = syntax_node.first_child().unwrap();
    let syntax_node3 = syntax_node2.parent().unwrap();
    let _ = syntax_node2.child_or_token_at_range(syntax_node3.text_range_without_outer_trivia());
}
