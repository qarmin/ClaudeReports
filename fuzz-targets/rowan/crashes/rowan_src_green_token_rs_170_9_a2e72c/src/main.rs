use rowan::GreenToken;
use rowan::SyntaxKind;

fn main() {
    let green_token = GreenToken::new(SyntaxKind(0u16), "");
    let syntax_kind = SyntaxKind(0u16);
    let syntax_kind2 = SyntaxKind(0u16);
    let _ = GreenToken::with_trivia(syntax_kind2, "", vec![GreenToken::with_trivia(syntax_kind, "", vec![], vec![green_token.clone()])], vec![]);
}
