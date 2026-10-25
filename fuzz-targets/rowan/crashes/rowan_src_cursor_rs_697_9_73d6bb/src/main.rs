use rowan::GreenNode;
use rowan::GreenToken;
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

fn main() {
    let syntax_kind = SyntaxKind(0u16);
    let syntax_kind2 = SyntaxKind(1u16);
    let green_node = GreenNode::from(GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(syntax_kind, vec![<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenToken>>::from(GreenToken::with_trivia(syntax_kind2, "", vec![], vec![]))]));
    let syntax_node = SyntaxNode::new_root(green_node.clone());
    let syntax_token = syntax_node.first_token().unwrap();
    let _ = syntax_token.replace_with(GreenToken::new(syntax_kind, ""));
}
