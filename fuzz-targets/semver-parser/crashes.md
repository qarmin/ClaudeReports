# semver-parser - crashes

Source: https://github.com/steveklabnik/semver-parser @ b43ab0311bd3c938dee40311e0ad11cb20a32939

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let identifier = Identifier::Numeric(u64::MAX);
let string = identifier.to_string();
let _ = &lt;semver_parser::RangeSet as ::core::str::FromStr&gt;::from_str(&amp;string);</pre></td><td valign="top"><pre>thread 'main' panicked at semver-parser/src/range.rs:72:27:
attempt to add with overflow
backtrace:
  &lt;semver_parser::range::Partial&gt;::inc_major (semver-parser/src/range.rs:72:27)
  semver_parser::range::simple::from_pair_iterator (semver-parser/src/range.rs:176:47)
  semver_parser::range::from_pair_iterator (semver-parser/src/range.rs:128:39)
  semver_parser::range_set::from_pair_iterator (semver-parser/src/range_set.rs:70:27)
  &lt;semver_parser::range_set::RangeSet&gt;::parse (semver-parser/src/range_set.rs:35:9)
  &lt;semver_parser::range_set::RangeSet as core::str::traits::FromStr&gt;::from_str (semver-parser/src/range_set.rs:44:9)
  repro_semver_parser_src_range_rs_72_27_094a9f::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let string = include_str!("string_0.txt").to_string();
let mut parser = Parser::new(&amp;string).unwrap();
let _ = parser.version();</pre></td><td valign="top"><pre>thread 'main' panicked at semver-parser/src/parser.rs:159:13:
Cannot have more than 255 identifiers
backtrace:
  &lt;semver_parser::parser::Parser&gt;::bounded_identifier (semver-parser/src/parser.rs:159:13)
  &lt;semver_parser::parser::Parser&gt;::bounded_identifier (semver-parser/src/parser.rs:177:31)
... (previous line repeated 10 more times)</pre></td></tr>
<tr><td valign="top"><pre>let version = Version { major: 0u64, minor: u64::MAX, patch: 0u64, pre: vec![],
    build: vec![] };
let string = version.to_string();
let _ = &lt;semver_parser::RangeSet as ::core::str::FromStr&gt;::from_str(&amp;string);</pre></td><td valign="top"><pre>thread 'main' panicked at semver-parser/src/range.rs:77:27:
attempt to add with overflow
backtrace:
  &lt;semver_parser::range::Partial&gt;::inc_minor (semver-parser/src/range.rs:77:27)
  semver_parser::range::simple (semver-parser/src/range.rs:375:34)
  semver_parser::range::simple::from_pair_iterator (semver-parser/src/range.rs:192:37)
  semver_parser::range::from_pair_iterator (semver-parser/src/range.rs:128:39)
  semver_parser::range_set::from_pair_iterator (semver-parser/src/range_set.rs:70:27)
  &lt;semver_parser::range_set::RangeSet&gt;::parse (semver-parser/src/range_set.rs:35:9)
  &lt;semver_parser::range_set::RangeSet as core::str::traits::FromStr&gt;::from_str (semver-parser/src/range_set.rs:44:9)
  repro_semver_parser_src_range_rs_77_27_81b087::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let version = Version { major: 0u64, minor: 0u64, patch: u64::MAX, pre: vec![],
    build: vec![] };
let string = version.to_string();
let _ = &lt;semver_parser::RangeSet as ::core::str::FromStr&gt;::from_str(&amp;string);</pre></td><td valign="top"><pre>thread 'main' panicked at semver-parser/src/range.rs:82:27:
attempt to add with overflow
backtrace:
  &lt;semver_parser::range::Partial&gt;::inc_patch (semver-parser/src/range.rs:82:27)
  semver_parser::range::simple (semver-parser/src/range.rs:369:50)
  semver_parser::range::simple::from_pair_iterator (semver-parser/src/range.rs:192:37)
  semver_parser::range::from_pair_iterator (semver-parser/src/range.rs:128:39)
  semver_parser::range_set::from_pair_iterator (semver-parser/src/range_set.rs:70:27)
  &lt;semver_parser::range_set::RangeSet&gt;::parse (semver-parser/src/range_set.rs:35:9)
  &lt;semver_parser::range_set::RangeSet as core::str::traits::FromStr&gt;::from_str (semver-parser/src/range_set.rs:44:9)
  repro_semver_parser_src_range_rs_82_27_164e20::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let identifier = Identifier::Numeric(4294967295u64);
let string = identifier.to_string();
let identifier2 = identifier.clone().concat(&amp;string);
let string2 = identifier2.to_string();
let _ = RangeSet::parse(&amp;string2, Compat::Npm);</pre></td><td valign="top"><pre>thread 'main' panicked at semver-parser/src/range.rs:481:76:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: PosOverflow }
backtrace:
  semver_parser::range::simple::parse_partial (semver-parser/src/range.rs:481:76)
  semver_parser::range::simple::from_pair_iterator (semver-parser/src/range.rs:164:39)
  semver_parser::range::from_pair_iterator (semver-parser/src/range.rs:128:39)
  semver_parser::range_set::from_pair_iterator (semver-parser/src/range_set.rs:70:27)
  &lt;semver_parser::range_set::RangeSet&gt;::parse (semver-parser/src/range_set.rs:35:9)
  repro_semver_parser_src_range_rs_481_76_0dd8a8::main (src/main.rs:10:13)</pre></td></tr>
</table>
