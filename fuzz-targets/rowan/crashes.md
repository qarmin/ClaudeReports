# rowan - crashes

Source: https://github.com/rust-analyzer/rowan @ b6f1cb6494a99a2264186d110b1b170989774814

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let syntax_kind = SyntaxKind(0u16);
let mut green_node_builder = GreenNodeBuilder::new();
let checkpoint = green_node_builder.checkpoint();
green_node_builder.token(syntax_kind, "");
green_node_builder.start_node(syntax_kind);
green_node_builder.start_node_at(checkpoint, SyntaxKind(0u16));</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/green/builder.rs:111:13:
checkpoint no longer valid, was an unmatched start_node_at called?
backtrace:
  &lt;rowan::green::builder::GreenNodeBuilder&gt;::start_node_at (rowan/src/green/builder.rs:111:13)
  repro_rowan_src_green_builder_rs_111_13_7e91a8::main (src/main.rs:10:24)</pre></td></tr>
<tr><td valign="top"><pre>let syntax_kind = SyntaxKind(0u16);
let syntax_node = SyntaxNode::new_root(
    GreenNode::new::&lt;Vec&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt;&gt;&gt;(SyntaxKind(1u16), vec![]));
let syntax_node2 = syntax_node.clone_subtree();
let _ = syntax_node2.replace_with(
    GreenNode::new::&lt;Vec&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt;&gt;&gt;(syntax_kind, vec![]));</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/cursor.rs:373:9:
assertion `left == right` failed
  left: SyntaxKind(1)
 right: SyntaxKind(0)
backtrace:
  &lt;rowan::cursor::SyntaxNode&gt;::replace_with (rowan/src/cursor.rs:373:9)
  repro_rowan_src_cursor_rs_373_9_938b3e::main (src/main.rs:9:26)</pre></td></tr>
<tr><td valign="top"><pre>let mut green_node_builder = GreenNodeBuilder::new();
green_node_builder.finish_node();</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/green/builder.rs:65:54:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;rowan::green::builder::GreenNodeBuilder&gt;::finish_node (rowan/src/green/builder.rs:65:54)
  repro_rowan_src_green_builder_rs_65_54_f394a0::main (src/main.rs:5:24)</pre></td></tr>
<tr><td valign="top"><pre>let green_token = GreenToken::new(SyntaxKind(0u16), "");
let syntax_kind = SyntaxKind(0u16);
let syntax_kind2 = SyntaxKind(0u16);
let _ = GreenToken::with_trivia(syntax_kind2, "",
    vec![GreenToken::with_trivia(syntax_kind, "", vec![],
    vec![green_token.clone()])], vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/green/token.rs:170:9:
trivia tokens cannot themselves carry trivia
backtrace:
  &lt;rowan::green::token::GreenToken&gt;::with_trivia (rowan/src/green/token.rs:170:9)
  repro_rowan_src_green_token_rs_170_9_a2e72c::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let syntax_kind = SyntaxKind(0u16);
let green_token = GreenToken::new(syntax_kind, "");
let string = green_token.to_string();
let mut green_node_builder = GreenNodeBuilder::new();
green_node_builder.token(syntax_kind, &amp;string);
let checkpoint = green_node_builder.checkpoint();
let mut green_node_builder2 = GreenNodeBuilder::new();
green_node_builder2.start_node_at(checkpoint, syntax_kind);</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/green/builder.rs:105:9:
checkpoint no longer valid, was finish_node called early?
backtrace:
  &lt;rowan::green::builder::GreenNodeBuilder&gt;::start_node_at (rowan/src/green/builder.rs:105:9)
  repro_rowan_src_green_builder_rs_105_9_3aee45::main (src/main.rs:13:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut node_cache = NodeCache::default();
let green_node_builder = GreenNodeBuilder::with_cache(&amp;mut node_cache);
green_node_builder.finish();</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/green/builder.rs:125:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;rowan::green::builder::GreenNodeBuilder&gt;::finish (rowan/src/green/builder.rs:125:9)
  repro_rowan_src_green_builder_rs_125_9_282bd3::main (src/main.rs:7:24)</pre></td></tr>
<tr><td valign="top"><pre>let syntax_kind = SyntaxKind(0u16);
let mut node_cache = NodeCache::default();
let mut green_node_builder = GreenNodeBuilder::with_cache(&amp;mut node_cache);
green_node_builder.token(syntax_kind, "");
green_node_builder.finish();</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/green/builder.rs:128:38:
explicit panic
backtrace:
  &lt;rowan::green::builder::GreenNodeBuilder&gt;::finish (rowan/src/green/builder.rs:128:38)
  repro_rowan_src_green_builder_rs_128_38_2e4249::main (src/main.rs:10:24)</pre></td></tr>
<tr><td valign="top"><pre>let mut green_node_builder = GreenNodeBuilder::new();
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
let _ = syntax_node3.token_at_offset(
    syntax_node.text_without_outer_trivia().len());</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/cursor.rs:601:9:
Bad offset: range 0..0 offset 1
backtrace:
  &lt;rowan::cursor::SyntaxNode&gt;::token_at_offset (rowan/src/cursor.rs:601:9)
  repro_rowan_src_cursor_rs_601_9_cd648e::main (src/main.rs:18:26)</pre></td></tr>
<tr><td valign="top"><pre>let syntax_kind = SyntaxKind(0u16);
let syntax_kind2 = SyntaxKind(1u16);
let green_node = GreenNode::from(
    GreenNode::new::&lt;Vec&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt;&gt;&gt;(syntax_kind, vec![&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt; as ::core::convert::From&lt;rowan::GreenToken&gt;&gt;::from(
    GreenToken::with_trivia(syntax_kind2, "", vec![], vec![]))]));
let syntax_node = SyntaxNode::new_root(green_node.clone());
let syntax_token = syntax_node.first_token().unwrap();
let _ = syntax_token.replace_with(GreenToken::new(syntax_kind, ""));</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/cursor.rs:697:9:
assertion `left == right` failed
  left: SyntaxKind(1)
 right: SyntaxKind(0)
backtrace:
  &lt;rowan::cursor::SyntaxToken&gt;::replace_with (rowan/src/cursor.rs:697:9)
  repro_rowan_src_cursor_rs_697_9_73d6bb::main (src/main.rs:12:26)</pre></td></tr>
<tr><td valign="top"><pre>let green_token = GreenToken::new(SyntaxKind(0u16), "a");
let syntax_kind = SyntaxKind(0u16);
let syntax_node = SyntaxNode::new_root(
    GreenNode::new::&lt;Vec&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt;&gt;&gt;(SyntaxKind(0u16),
    vec![&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt; as ::core::convert::From&lt;rowan::GreenNode&gt;&gt;::from(
    GreenNode::new::&lt;Vec&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt;&gt;&gt;(syntax_kind, vec![])),
    &lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt; as ::core::convert::From&lt;rowan::GreenToken&gt;&gt;::from(
    green_token.clone())]));
let syntax_node2 = syntax_node.last_child().unwrap();
let _ = syntax_node2.covering_element(syntax_node.text_range());</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/cursor.rs:636:13:
Bad range: node range 0..0, range 0..1
backtrace:
  &lt;rowan::cursor::SyntaxNode&gt;::covering_element (rowan/src/cursor.rs:636:13)
  repro_rowan_src_cursor_rs_636_13_38dc5c::main (src/main.rs:11:26)</pre></td></tr>
<tr><td valign="top"><pre>let syntax_kind = SyntaxKind(0u16);
let syntax_kind2 = SyntaxKind(0u16);
let green_node = GreenNode::new::&lt;Vec&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt;&gt;&gt;(syntax_kind, vec![&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt; as ::core::convert::From&lt;rowan::GreenToken&gt;&gt;::from(
    GreenToken::with_trivia(syntax_kind2, "\u{feff}", vec![], vec![]))]);
let green_token = GreenToken::from(GreenToken::with_trivia(syntax_kind2, "1",
    vec![], vec![]));
let syntax_node = SyntaxNode::new_root(green_node.clone());
let syntax_text = syntax_node.text_without_outer_trivia();
let _ = syntax_text.char_at(&lt;rowan::NodeOrToken&lt;rowan::GreenNode,
    rowan::GreenToken&gt; as ::core::convert::From&lt;rowan::GreenToken&gt;&gt;::from(
    green_token.clone()).text_len());</pre></td><td valign="top"><pre>thread 'main' panicked at rowan/src/syntax_text.rs:52:33:
start byte index 1 is not a char boundary; it is inside '\u{feff}' (bytes 0..3 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;rowan::syntax_text::SyntaxText&gt;::char_at::{closure#0} (rowan/src/syntax_text.rs:52:33)
  &lt;rowan::syntax_text::SyntaxText&gt;::try_for_each_chunk::&lt;&lt;rowan::syntax_text::SyntaxText&gt;::char_at::{closure#0}, char&gt;::{closure#0} (rowan/src/syntax_text.rs:95:51)
  &lt;rowan::syntax_text::SyntaxText&gt;::try_fold_chunks::&lt;(), &lt;rowan::syntax_text::SyntaxText&gt;::try_for_each_chunk&lt;&lt;rowan::syntax_text::SyntaxText&gt;::char_at::{closure#0}, char&gt;::{closure#0}, char&gt;::{closure#0} (rowan/src/syntax_text.rs:88:56)
  &lt;&amp;mut core::iter::adapters::filter_map::filter_map_try_fold&lt;rowan::cursor::SyntaxToken, (rowan::cursor::SyntaxToken, text_size::range::TextRange), (), core::result::Result&lt;(), char&gt;, &lt;rowan::syntax_text::SyntaxText&gt;::tokens_with_ranges::{closure#2}, &lt;rowan::syntax_text::SyntaxText&gt;::try_fold_chunk... (+16 chars)
  &lt;&amp;mut &amp;mut core::iter::adapters::filter_map::filter_map_try_fold&lt;rowan::cursor::SyntaxToken, (rowan::cursor::SyntaxToken, text_size::range::TextRange), (), core::result::Result&lt;(), char&gt;, &lt;rowan::syntax_text::SyntaxText&gt;::tokens_with_ranges::{closure#2}, &lt;rowan::syntax_text::SyntaxText&gt;::try_fold_... (+16 chars)
  &lt;rowan::cursor::PreorderWithTokens as core::iter::traits::iterator::Iterator&gt;::try_fold::&lt;(), core::iter::adapters::filter_map::filter_map_try_fold&lt;rowan::utility_types::WalkEvent&lt;rowan::utility_types::NodeOrToken&lt;rowan::cursor::SyntaxNode, rowan::cursor::SyntaxToken&gt;&gt;, rowan::utility_types::NodeO... (+17 chars)
  &lt;rowan::syntax_text::SyntaxText&gt;::try_fold_chunks::&lt;(), &lt;rowan::syntax_text::SyntaxText&gt;::try_for_each_chunk&lt;&lt;rowan::syntax_text::SyntaxText&gt;::char_at::{closure#0}, char&gt;::{closure#0}, char&gt; (rowan/src/syntax_text.rs:88:14)
  &lt;rowan::syntax_text::SyntaxText&gt;::try_for_each_chunk::&lt;&lt;rowan::syntax_text::SyntaxText&gt;::char_at::{closure#0}, char&gt; (rowan/src/syntax_text.rs:95:14)
  &lt;rowan::syntax_text::SyntaxText&gt;::char_at (rowan/src/syntax_text.rs:48:24)
  repro_rowan_src_syntax_text_rs_52_33_0c2633::main (src/main.rs:13:25)</pre></td></tr>
<tr><td valign="top"><pre>let syntax_kind = SyntaxKind(0u16);
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
let _ = syntax_node2.child_or_token_at_range(
    syntax_node3.text_range_without_outer_trivia());</pre></td><td valign="top"><pre>thread 'main' panicked at text-size/src/range.rs:431:14:
TextRange -offset overflowed
backtrace:
  &lt;text_size::range::TextRange as core::ops::arith::Sub&lt;text_size::size::TextSize&gt;&gt;::sub (text-size/src/range.rs:431:14)
  &lt;rowan::cursor::SyntaxNode&gt;::child_or_token_at_range (rowan/src/cursor.rs:653:25)
  repro_text_size_src_range_rs_431_14_1658a4::main (src/main.rs:17:26)</pre></td></tr>
</table>
