use rowan::GreenNode;
use rowan::GreenToken;
use rowan::SyntaxKind;
use rowan::cursor::SyntaxNode;

fn main() {
    let syntax_kind = SyntaxKind(0u16);
    let syntax_kind2 = SyntaxKind(0u16);
    let green_node = GreenNode::new::<Vec<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>>>(syntax_kind, vec![<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenToken>>::from(GreenToken::with_trivia(syntax_kind2, "\u{feff}", vec![], vec![]))]);
    let green_token = GreenToken::from(GreenToken::with_trivia(syntax_kind2, "1", vec![], vec![]));
    let syntax_node = SyntaxNode::new_root(green_node.clone());
    let syntax_text = syntax_node.text_without_outer_trivia();
    let _ = syntax_text.char_at(<rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken> as ::core::convert::From<rowan::GreenToken>>::from(green_token.clone()).text_len());
}
