# syn - crashes

Source: https://github.com/dtolnay/syn @ 9ff9287583767e53dcfb70e3b1b350996908bd8b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let meta = Meta::Path(Path { leading_colon: None,
    segments: &lt;syn::punctuated::Punctuated&lt;syn::PathSegment,
    syn::token::PathSep&gt;&gt;::new() });
let _ = meta.require_list();</pre></td><td valign="top"><pre>thread 'main' panicked at syn/src/attr.rs:540:39:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;syn::attr::Meta&gt;::require_list (syn/src/attr.rs:540:39)
  repro_syn_src_attr_rs_540_39_963494::main (src/main.rs:6:18)</pre></td></tr>
<tr><td valign="top"><pre>let meta = Meta::from(Path { leading_colon: None,
    segments: &lt;syn::punctuated::Punctuated&lt;syn::PathSegment,
    syn::token::PathSep&gt;&gt;::new() });
let _ = meta.require_name_value();</pre></td><td valign="top"><pre>thread 'main' panicked at syn/src/attr.rs:558:39:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;syn::attr::Meta&gt;::require_name_value (syn/src/attr.rs:558:39)
  repro_syn_src_attr_rs_558_39_11cde2::main (src/main.rs:6:18)</pre></td></tr>
<tr><td valign="top"><pre>let path = Path { leading_colon: None,
    segments: &lt;syn::punctuated::Punctuated&lt;syn::PathSegment,
    syn::token::PathSep&gt;&gt;::new() };
let _ = path.require_ident();</pre></td><td valign="top"><pre>thread 'main' panicked at syn/src/path.rs:100:39:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;syn::path::Path&gt;::require_ident::{closure#0} (syn/src/path.rs:100:39)
  &lt;syn::path::Path&gt;::require_ident (syn/src/path.rs:98:26)
  repro_syn_src_path_rs_100_39_27af08::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let attribute = Attribute { pound_token: Pound::default(),
    style: AttrStyle::Inner(Not::default()), bracket_token: Bracket::default(),
    meta: Meta::from(Path { leading_colon: None,
    segments: &lt;syn::punctuated::Punctuated&lt;syn::PathSegment,
    syn::token::PathSep&gt; as ::core::default::Default&gt;::default() }) };
let _ = attribute.parse_args::&lt;syn::token::As&gt;();</pre></td><td valign="top"><pre>thread 'main' panicked at syn/src/attr.rs:252:39:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;syn::attr::Attribute&gt;::parse_args_with::&lt;&lt;syn::token::As as syn::parse::Parse&gt;::parse&gt; (syn/src/attr.rs:252:39)
  &lt;syn::attr::Attribute&gt;::parse_args::&lt;syn::token::As&gt; (syn/src/attr.rs:227:14)
  repro_syn_src_attr_rs_252_39_83c099::main (src/main.rs:11:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Member::from(4294967296usize);</pre></td><td valign="top"><pre>thread 'main' panicked at syn/src/expr.rs:1140:9:
assertion failed: index &lt; u32::MAX as usize
backtrace:
  &lt;syn::expr::Index as core::convert::From&lt;usize&gt;&gt;::from (syn/src/expr.rs:1140:9)
  &lt;syn::expr::Member as core::convert::From&lt;usize&gt;&gt;::from (syn/src/expr.rs:1077:25)
  repro_syn_src_expr_rs_1140_9_58da46::main (src/main.rs:4:13)</pre></td></tr>
</table>
