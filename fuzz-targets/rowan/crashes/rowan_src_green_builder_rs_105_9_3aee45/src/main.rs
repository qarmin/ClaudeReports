use rowan::GreenNodeBuilder;
use rowan::GreenToken;
use rowan::SyntaxKind;

fn main() {
    let syntax_kind = SyntaxKind(0u16);
    let green_token = GreenToken::new(syntax_kind, "");
    let string = green_token.to_string();
    let mut green_node_builder = GreenNodeBuilder::new();
    green_node_builder.token(syntax_kind, &string);
    let checkpoint = green_node_builder.checkpoint();
    let mut green_node_builder2 = GreenNodeBuilder::new();
    green_node_builder2.start_node_at(checkpoint, syntax_kind);
}
