# fluent - crashes

Source: https://github.com/projectfluent/fluent-rs @ 0f87afbbc3fe7d20c61a3fc75cb5d0e123aefdda

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let fluent_args = FluentArgs::with_capacity(1000000000usize);
let _ = fluent_args.iter();</pre></td><td valign="top"><pre>out of memory: memory allocation of 144000000000 bytes failed
backtrace:
  fluent_bundle::args::FluentArgs::with_capacity (src/args.rs:66)
  repro_out_of_memory_in_fluent_bundle_fluentargs_iter_7a309a::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = FluentArgs::with_capacity(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 144000000000 bytes failed
backtrace:
  fluent_bundle::args::FluentArgs::with_capacity (src/args.rs:66)
  repro_out_of_memory_in_fluent_bundle_fluentargs_with_capacity_ed3733::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = FluentArgs::with_capacity(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from fluent-bundle/src/args.rs:66:14)
capacity overflow
backtrace:
  &lt;fluent_bundle::args::FluentArgs&gt;::with_capacity (fluent-bundle/src/args.rs:66:14)
  repro_fluent_bundle_src_args_rs_66_14_982143::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut fluent_number_options = FluentNumberOptions::default();
let fluent_args = FluentArgs::with_capacity(1000000000usize);
fluent_number_options.merge(&amp;fluent_args);</pre></td><td valign="top"><pre>out of memory: memory allocation of 144000000000 bytes failed
backtrace:
  fluent_bundle::args::FluentArgs::with_capacity (src/args.rs:66)
  repro_out_of_memory_in_fluent_bundle_types_fluentnumberoptions_mer_3087d5::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = fluent_syntax::unicode::unescape_unicode_to_string("\\\u{dfe2a}");</pre></td><td valign="top"><pre>thread 'main' panicked at fluent-syntax/src/unicode.rs:119:27:
start byte index 2 is not a char boundary; it is inside '\u{dfe2a}' (bytes 1..5 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  fluent_syntax::unicode::unescape::&lt;alloc::string::String&gt; (fluent-syntax/src/unicode.rs:119:27)
  fluent_syntax::unicode::unescape_unicode_to_string (fluent-syntax/src/unicode.rs:138:17)
  repro_fluent_syntax_src_unicode_rs_119_27_ece7bd::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let fluent_args = FluentArgs::with_capacity(1000000000usize);
let _ = NUMBER(&amp;vec, &amp;fluent_args);</pre></td><td valign="top"><pre>out of memory: memory allocation of 144000000000 bytes failed
backtrace:
  fluent_bundle::args::FluentArgs::with_capacity (src/args.rs:66)
  repro_out_of_memory_in_fluent_bundle_builtins_number_9b8487::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = fluent_syntax::unicode::unescape_unicode_to_string("\\\u{62599}\\");</pre></td><td valign="top"><pre>thread 'main' panicked at fluent-syntax/src/unicode.rs:93:31:
start byte index 2 is not a char boundary; it is inside '\u{62599}' (bytes 1..5 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  fluent_syntax::unicode::unescape::&lt;alloc::string::String&gt; (fluent-syntax/src/unicode.rs:93:31)
  fluent_syntax::unicode::unescape_unicode_to_string (fluent-syntax/src/unicode.rs:138:17)
  repro_fluent_syntax_src_unicode_rs_93_31_237043::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let fluent_args = FluentArgs::with_capacity(1000000000usize);
let fluent_value = NUMBER(&amp;vec, &amp;fluent_args);
let _ = fluent_value.into_owned();</pre></td><td valign="top"><pre>out of memory: memory allocation of 144000000000 bytes failed
backtrace:
  fluent_bundle::args::FluentArgs::with_capacity (src/args.rs:66)
  repro_out_of_memory_in_fluent_bundle_fluentvalue_into_owned_1337dd::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
</table>
