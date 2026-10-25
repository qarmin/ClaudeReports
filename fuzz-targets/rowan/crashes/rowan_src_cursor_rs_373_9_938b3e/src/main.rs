use rowan::GreenNode;
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

fn main() {
    let syntax_kind = SyntaxKind(0u16);
    let syntax_node = SyntaxNode::new_root(GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(SyntaxKind(1u16), vec![]));
    let syntax_node2 = syntax_node.clone_subtree();
    let _ = syntax_node2.replace_with(GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(syntax_kind, vec![]));
}
