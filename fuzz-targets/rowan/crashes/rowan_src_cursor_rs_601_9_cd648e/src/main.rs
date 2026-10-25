use rowan::GreenNodeBuilder;
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

fn main() {
    let mut green_node_builder = GreenNodeBuilder::new();
    let checkpoint = green_node_builder.checkpoint();
    let syntax_kind = SyntaxKind(0u16);
    green_node_builder.start_node(syntax_kind);
    green_node_builder.start_node_at(checkpoint, SyntaxKind(0u16));
    green_node_builder.finish_node();
    green_node_builder.token(syntax_kind, "a");
    green_node_builder.finish_node();
    let green_node = green_node_builder.finish();
    let syntax_node = SyntaxNode::from(SyntaxNode::new_root(green_node.clone()));
    let syntax_node2 = syntax_node.tree_top();
    let syntax_node3 = syntax_node2.first_child().unwrap();
    let _ = syntax_node3.token_at_offset(syntax_node.text_without_outer_trivia().len());
}
