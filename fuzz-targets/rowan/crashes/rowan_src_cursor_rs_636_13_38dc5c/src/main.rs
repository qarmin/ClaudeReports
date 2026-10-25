use rowan::GreenNode;
use rowan::GreenToken;
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

fn main() {
    let green_token = GreenToken::new(SyntaxKind(0u16), "a");
    let syntax_kind = SyntaxKind(0u16);
    let syntax_node = SyntaxNode::new_root(GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(SyntaxKind(0u16), vec![<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenNode>>::from(GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(syntax_kind, vec![])), <rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenToken>>::from(green_token.clone())]));
    let syntax_node2 = syntax_node.last_child().unwrap();
    let _ = syntax_node2.covering_element(syntax_node.text_range());
}
