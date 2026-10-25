# tracing-subscriber - crashes

Source: https://github.com/tokio-rs/tracing @ 9d0e557728ecb91e467fc29dfe738b9f28cf6684

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;tracing_subscriber::EnvFilter as ::core::str::FromStr&gt;::from_str(" \
    t�𠿓[");</pre></td><td valign="top"><pre>thread 'main' panicked at tracing-subscriber/src/filter/env/directive.rs:155:43:
end byte index 8 is not a char boundary; it is inside '𠿓' (bytes 5..9 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;tracing_subscriber::filter::env::directive::Directive&gt;::parse (tracing-subscriber/src/filter/env/directive.rs:155:43)
  &lt;tracing_subscriber::filter::env::builder::Builder&gt;::parse::&lt;&amp;str&gt;::{closure#1} (tracing-subscriber/src/filter/env/builder.rs:176:22)
  &lt;tracing_subscriber::filter::env::builder::Builder&gt;::parse::&lt;&amp;str&gt; (tracing-subscriber/src/filter/env/builder.rs:177:14)
  &lt;tracing_subscriber::filter::env::EnvFilter&gt;::try_new::&lt;&amp;str&gt; (tracing-subscriber/src/filter/env/mod.rs:380:25)
  &lt;tracing_subscriber::filter::env::EnvFilter as core::str::traits::FromStr&gt;::from_str (tracing-subscriber/src/filter/env/mod.rs:770:9)
  repro_tracing_subscriber_src_filter_env_directive_rs_155_43_0911f0::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = EnvFilter::try_new::&lt;String&gt;("\n攼=".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at tracing-subscriber/src/filter/env/directive.rs:151:43:
end byte index 3 is not a char boundary; it is inside '攼' (bytes 1..4 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;tracing_subscriber::filter::env::directive::Directive&gt;::parse (tracing-subscriber/src/filter/env/directive.rs:151:43)
  &lt;tracing_subscriber::filter::env::builder::Builder&gt;::parse::&lt;alloc::string::String&gt;::{closure#1} (tracing-subscriber/src/filter/env/builder.rs:176:22)
  &lt;tracing_subscriber::filter::env::builder::Builder&gt;::parse::&lt;alloc::string::String&gt; (tracing-subscriber/src/filter/env/builder.rs:177:14)
  &lt;tracing_subscriber::filter::env::EnvFilter&gt;::try_new::&lt;alloc::string::String&gt; (tracing-subscriber/src/filter/env/mod.rs:380:25)
  repro_tracing_subscriber_src_filter_env_directive_rs_151_43_e8b5fa::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut formatted_fields = &lt;tracing_subscriber::fmt::
    FormattedFields&lt;tracing_subscriber::fmt::format::Pretty&gt;&gt;::new(
    "".to_string());
let mut writer = formatted_fields.as_writer();
let _ = &lt;tracing_subscriber::fmt::time::ChronoUtc as tracing_subscriber::fmt::
    time::FormatTime&gt;::format_time(&amp;ChronoUtc::new("%".to_string()),
    &amp;mut writer);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/fmt.rs:657:14 (called from tracing-subscriber/src/fmt/time/chrono_crate.rs:100:56)
a formatting trait implementation returned an error when the underlying stream did not: Error
backtrace:
  &lt;tracing_subscriber::fmt::time::chrono_crate::ChronoUtc as tracing_subscriber::fmt::time::FormatTime&gt;::format_time (tracing-subscriber/src/fmt/time/chrono_crate.rs:100:56)
  repro_tracing_subscriber_src_fmt_time_chrono_crate_rs_100_56_5d1726::main (src/main.rs:6:13)</pre></td></tr>
</table>
