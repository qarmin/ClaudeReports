# regex - crashes

Source: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::next_eoi_state(
    &amp;&lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::new("").unwrap(),
    StateID::from_ne_bytes([0u8, 1u8, 0u8, 0u8]).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/sparse.rs:1514:39:
range start index 256 out of range for slice of length 65
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;regex_automata::dfa::sparse::Transitions&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::state (regex-automata/src/dfa/sparse.rs:1514:39)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::next_eoi_state (regex-automata/src/dfa/sparse.rs:1169:17)
  repro_regex_automata_src_dfa_sparse_rs_1514_39_99e146::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let builder = PikeVM::builder();
let pike_v_m = builder.build_many::&lt;String&gt;(&amp;["".to_string(),
    "".to_string()]).unwrap();
let mut cache = Cache::new(&amp;PikeVM::new("").unwrap());
let mut captures = Captures::all(GroupInfo::empty());
let vec = vec![];
pike_v_m.captures::&lt;regex_automata::Input&gt;(&amp;mut cache,
    Input::new::&lt;Vec&lt;u8&gt;&gt;(&amp;vec), &amp;mut captures);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/util/sparse_set.rs:197:32:
index out of bounds: the len is 5 but the index is 8
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[regex_automata::util::primitives::StateID]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[regex_automata::util::primitives::StateID] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;regex_automata::util::sparse_set::SparseSet&gt;::contains (regex-automata/src/util/sparse_set.rs:197:32)
  &lt;regex_automata::util::sparse_set::SparseSet&gt;::insert (regex-automata/src/util/sparse_set.rs:173:17)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::epsilon_closure_explore (regex-automata/src/nfa/thompson/pikevm.rs:1683:26)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::epsilon_closure (regex-automata/src/nfa/thompson/pikevm.rs:1631:26)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search_imp (regex-automata/src/nfa/thompson/pikevm.rs:1364:22)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search_slots_imp (regex-automata/src/nfa/thompson/pikevm.rs:1146:29)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search_slots (regex-automata/src/nfa/thompson/pikevm.rs:1127:24)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search (regex-automata/src/nfa/thompson/pikevm.rs:1027:24)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::captures::&lt;regex_automata::util::search::Input&gt; (regex-automata/src/nfa/thompson/pikevm.rs:865:14)
  repro_regex_automata_src_util_sparse_set_rs_197_32_96a160::main (src/main.rs:13:14)</pre></td></tr>
<tr><td valign="top"><pre>let d_f_a = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::
    always_match().unwrap();
let _ = d_f_a.write_to_native_endian(&amp;mut include_bytes!(
    "data_0.bin").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/util/wire.rs:877:12:
range end index 4 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;regex_automata::util::wire::LE as regex_automata::util::wire::Endian&gt;::write_u32 (regex-automata/src/util/wire.rs:877:12)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::write_to::&lt;regex_automata::util::wire::LE&gt; (regex-automata/src/dfa/sparse.rs:796:13)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::write_to_native_endian (regex-automata/src/dfa/sparse.rs:781:14)
  repro_regex_automata_src_util_wire_rs_877_12_3e8a02::main (src/main.rs:3:19)</pre></td></tr>
<tr><td valign="top"><pre>let span = Span { start: 0usize, end: 0usize };
let span2 = span.offset(1usize);
let _ = span2.offset(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/util/search.rs:848:23:
attempt to add with overflow
backtrace:
  &lt;regex_automata::util::search::Span&gt;::offset (regex-automata/src/util/search.rs:848:23)
  repro_regex_automata_src_util_search_rs_848_23_be4efc::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::match_len(
    &amp;&lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt;&gt;::new_many::&lt;String&gt;(
    &amp;[]).unwrap(), StateID::from(0u8));</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/dense.rs:3016:9:
assertion failed: self.is_match_state(id)
backtrace:
  &lt;regex_automata::dfa::dense::DFA&lt;alloc::vec::Vec&lt;u32&gt;&gt;&gt;::match_pattern_len (regex-automata/src/dfa/dense.rs:3016:9)
  &lt;regex_automata::dfa::dense::DFA&lt;alloc::vec::Vec&lt;u32&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::match_len (regex-automata/src/dfa/dense.rs:3231:14)
  repro_regex_automata_src_dfa_dense_rs_3016_9_0b533d::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let config = Config::new();
let usize_v = config.get_visited_capacity();
let _ = &lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::match_pattern(&amp;Builder::new().build_many::&lt;String&gt;(
    &amp;["".to_string(), "".to_string()]).unwrap(),
    regex_automata::nfa::thompson::Builder::new().add_empty().unwrap(),
    usize_v);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/dense.rs:3067:9:
assertion failed: self.is_match_state(id)
backtrace:
  &lt;regex_automata::dfa::dense::DFA&lt;alloc::vec::Vec&lt;u32&gt;&gt;&gt;::match_state_index (regex-automata/src/dfa/dense.rs:3067:9)
  &lt;regex_automata::dfa::dense::DFA&lt;alloc::vec::Vec&lt;u32&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::match_pattern (regex-automata/src/dfa/dense.rs:3244:32)
  repro_regex_automata_src_dfa_dense_rs_3067_9_c616d7::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::match_pattern(
    &amp;&lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::never_match().unwrap(),
    StateID::from(1u8), 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/sparse.rs:2343:58:
range start index 4 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;regex_automata::dfa::sparse::State&gt;::pattern_id (regex-automata/src/dfa/sparse.rs:2343:58)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::match_pattern (regex-automata/src/dfa/sparse.rs:1192:27)
  repro_regex_automata_src_dfa_sparse_rs_2343_58_e6d9f7::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let prefilter = Prefilter::from_hir_prefix(MatchKind::All,
    &amp;Parser::new().parse("sv").unwrap()).unwrap();
let _ = prefilter.find(&amp;vec![], Span { start: 0usize, end: 1usize });</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/util/prefilter/memmem.rs:47:39:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;[u8] as core::ops::index::Index&lt;regex_automata::util::search::Span&gt;&gt;::index (regex-automata/src/util/search.rs:863:14)
  &lt;regex_automata::util::prefilter::memmem::Memmem as regex_automata::util::prefilter::PrefilterI&gt;::find (regex-automata/src/util/prefilter/memmem.rs:47:39)
  &lt;regex_automata::util::prefilter::Prefilter&gt;::find (regex-automata/src/util/prefilter/mod.rs:356:22)
  repro_regex_automata_src_util_prefilter_memmem_rs_47_39_6c1e48::main (src/main.rs:8:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::next_eoi_state(
    &amp;&lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt;&gt;::never_match().unwrap(),
    &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::start_state(
    &amp;&lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::new_many::&lt;String&gt;(
    &amp;[]).unwrap(), &amp;Config::new()).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/dense.rs:3221:9:
index out of bounds: the len is 6 but the index is 19
backtrace:
  &lt;regex_automata::dfa::dense::DFA&lt;alloc::vec::Vec&lt;u32&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::next_eoi_state (regex-automata/src/dfa/dense.rs:3221:9)
  repro_regex_automata_src_dfa_dense_rs_3221_9_f5b9a5::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::next_state(&amp;Builder::new().build_many::&lt;String&gt;(&amp;[]).unwrap(),
    &lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::universal_start_state(&amp;Builder::new().build_many::&lt;String&gt;(
    &amp;["".to_string()]).unwrap(), Anchored::No).unwrap(), 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/dense.rs:3198:9:
index out of bounds: the len is 6 but the index is 8
backtrace:
  &lt;regex_automata::dfa::dense::DFA&lt;alloc::vec::Vec&lt;u32&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::next_state (regex-automata/src/dfa/dense.rs:3198:9)
  repro_regex_automata_src_dfa_dense_rs_3198_9_311486::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::next_state(&amp;&lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::new(
    "").unwrap(), StateID::try_from(1u16).unwrap(), 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/sparse.rs:2301:21:
attempt to subtract with overflow
backtrace:
  &lt;regex_automata::dfa::sparse::State&gt;::next (regex-automata/src/dfa/sparse.rs:2301:21)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::next_state (regex-automata/src/dfa/sparse.rs:1155:32)
  repro_regex_automata_src_dfa_sparse_rs_2301_21_85a10c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut cache = Cache::new(&amp;Regex::new("").unwrap());
cache.reset(&amp;Regex::builder().build_from_hir(&amp;regex_syntax::parse(
    "(1)").unwrap()).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/meta/wrappers.rs:492:29:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;regex_automata::meta::wrappers::OnePassCache&gt;::reset (regex-automata/src/meta/wrappers.rs:492:29)
  &lt;regex_automata::meta::strategy::Core as regex_automata::meta::strategy::Strategy&gt;::reset_cache (regex-automata/src/meta/strategy.rs:695:23)
  &lt;regex_automata::meta::regex::Cache&gt;::reset (regex-automata/src/meta/regex.rs:2422:22)
  repro_regex_automata_src_meta_wrappers_rs_492_29_d5ef18::main (src/main.rs:6:11)</pre></td></tr>
<tr><td valign="top"><pre>let regex = Regex::new_many::&lt;String&gt;(&amp;[]).unwrap();
let captures = regex.create_captures();
let string = captures.interpolate_string("", "a");
let builder = Builder::new();
let d_f_a = builder.build(&amp;string).unwrap();
let mut cache = Cache::new(&amp;Builder::new().build_many::&lt;String&gt;(
    &amp;["".to_string()]).unwrap());
let vec = vec![];
let find_matches = regex.find_iter::&lt;regex_automata::Input&gt;(
    Input::new::&lt;Vec&lt;u8&gt;&gt;(&amp;vec));
let input = find_matches.input();
let _ = d_f_a.start_state_reverse(&amp;mut cache, input);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/hybrid/dfa.rs:2631:9:
assertion failed: self.as_ref().is_valid(id)
backtrace:
  &lt;regex_automata (regex-automata/src/hybrid/dfa.rs:2631:9)
  &lt;regex_automata (regex-automata/src/hybrid/dfa.rs:2196:14)
  &lt;regex_automata (regex-automata/src/hybrid/dfa.rs:1574:32)
  &lt;regex_automata (regex-automata/src/hybrid/dfa.rs:1636:14)
  repro_regex_automata_src_hybrid_dfa_rs_2631_9_41bea2::main (src/main.rs:16:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::match_pattern(
    &amp;&lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::new_many::&lt;String&gt;(
    &amp;[]).unwrap(), StateID::new_unchecked(0usize), 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/util/wire.rs:588:14:
range end index 4 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  regex_automata::util::wire::read_pattern_id_unchecked (regex-automata/src/util/wire.rs:588:14)
  &lt;regex_automata::dfa::sparse::State&gt;::pattern_id (regex-automata/src/dfa/sparse.rs:2343:9)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::match_pattern (regex-automata/src/dfa/sparse.rs:1192:27)
  repro_regex_automata_src_util_wire_rs_588_14_0bc7f9::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let d_f_a = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::new_many::&lt;String&gt;(
    &amp;[]).unwrap();
let _ = d_f_a.write_to_big_endian(&amp;mut include_bytes!("data_0.bin").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/util/wire.rs:891:12:
range end index 4 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;regex_automata::util::wire::BE as regex_automata::util::wire::Endian&gt;::write_u32 (regex-automata/src/util/wire.rs:891:12)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::write_to::&lt;regex_automata::util::wire::BE&gt; (regex-automata/src/dfa/sparse.rs:796:13)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::write_to_big_endian (regex-automata/src/dfa/sparse.rs:727:14)
  repro_regex_automata_src_util_wire_rs_891_12_d4a688::main (src/main.rs:3:19)</pre></td></tr>
<tr><td valign="top"><pre>let builder = Builder::default();
let regex = builder.build_from_dfas(DFA::never_match().unwrap(),
    DFA::new("").unwrap());
let _ = regex.pattern_len();</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/hybrid/regex.rs:551:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;regex_automata (regex-automata/src/hybrid/regex.rs:551:9)
  repro_regex_automata_src_hybrid_regex_rs_551_9_64a724::main (src/main.rs:7:19)</pre></td></tr>
<tr><td valign="top"><pre>let pike_v_m = PikeVM::never_match().unwrap();
let mut cache = pike_v_m.create_cache();
let builder = Builder::new();
let regex = builder.build_from_hir(&amp;Hir::dot(Dot::AnyCharExcept('a'))).unwrap();
let vec = vec![];
let captures_matches = regex.captures_iter::&lt;regex_automata::Input&gt;(
    Input::new::&lt;Vec&lt;u8&gt;&gt;(&amp;vec));
let input = captures_matches.input();
let builder2 = regex_automata::dfa::onepass::Builder::new();
let d_f_a = builder2.build("").unwrap();
let mut captures = d_f_a.create_captures();
pike_v_m.search(&amp;mut cache, input, &amp;mut captures);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/nfa/thompson/pikevm.rs:2157:17:
attempt to subtract with overflow
backtrace:
  &lt;regex_automata::nfa::thompson::pikevm::SlotTable&gt;::all_absent (regex-automata/src/nfa/thompson/pikevm.rs:2157:17)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search_imp (regex-automata/src/nfa/thompson/pikevm.rs:1363:45)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search_slots_imp (regex-automata/src/nfa/thompson/pikevm.rs:1146:29)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search_slots (regex-automata/src/nfa/thompson/pikevm.rs:1099:27)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search (regex-automata/src/nfa/thompson/pikevm.rs:1027:24)
  repro_regex_automata_src_nfa_thompson_pikevm_rs_2157_17_85f53b::main (src/main.rs:18:14)</pre></td></tr>
<tr><td valign="top"><pre>let mut parser = Parser::new();
let _ = parser.parse_with_comments("a");
let _ = parser.parse("");</pre></td><td valign="top"><pre>thread 'main' panicked at regex-syntax/src/ast/parse.rs:983:9:
assertion `left == right` failed: parser can only be used once
  left: 1
 right: 0
backtrace:
  &lt;regex_syntax::ast::parse::ParserI&lt;&amp;mut regex_syntax::ast::parse::Parser&gt;&gt;::parse_with_comments (regex-syntax/src/ast/parse.rs:983:9)
  &lt;regex_syntax::ast::parse::ParserI&lt;&amp;mut regex_syntax::ast::parse::Parser&gt;&gt;::parse (regex-syntax/src/ast/parse.rs:977:14)
  &lt;regex_syntax::ast::parse::Parser&gt;::parse (regex-syntax/src/ast/parse.rs:363:37)
  repro_regex_syntax_src_ast_parse_rs_983_9_99eb80::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let regex = Regex::try_from("".to_string()).unwrap();
let string = "\u{c0aa7}".to_string();
let match_v = regex.find_at(&amp;string, 1usize).unwrap();
let _ = match_v.as_str();</pre></td><td valign="top"><pre>thread 'main' panicked at regex-lite/src/string.rs:1571:23:
start byte index 1 is not a char boundary; it is inside '\u{c0aa7}' (bytes 0..4 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;regex_lite::string::Match&gt;::as_str (regex-lite/src/string.rs:1571:23)
  repro_regex_lite_src_string_rs_1571_23_f61dec::main (src/main.rs:7:21)</pre></td></tr>
<tr><td valign="top"><pre>let builder = Regex::builder();
let regex = builder.build_many_from_hir::&lt;regex_syntax::hir::Hir&gt;(&amp;[]).unwrap();
let mut cache = regex.create_cache();
cache.reset(&amp;Regex::new_many::&lt;String&gt;(
    &amp;["𲺶\0\0\0\0\0\0\0\0\0\0\0\0\0".to_string(), "".to_string(), "".to_string(),
    "".to_string()]).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/meta/wrappers.rs:784:29:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;regex_automata::meta::wrappers::HybridCache&gt;::reset (regex-automata/src/meta/wrappers.rs:784:29)
  &lt;regex_automata::meta::strategy::Core as regex_automata::meta::strategy::Strategy&gt;::reset_cache (regex-automata/src/meta/strategy.rs:696:22)
  &lt;regex_automata::meta::regex::Cache&gt;::reset (regex-automata/src/meta/regex.rs:2422:22)
  repro_regex_automata_src_meta_wrappers_rs_784_29_62bacb::main (src/main.rs:7:11)</pre></td></tr>
<tr><td valign="top"><pre>let pike_v_m = PikeVM::always_match().unwrap();
let mut cache = pike_v_m.create_cache();
let vec = vec![];
let input = Input::new::&lt;Vec&lt;u8&gt;&gt;(&amp;vec);
let _ = pike_v_m.search_slots(&amp;mut cache, &amp;input, &amp;mut vec![None, None, None,
    None, None]);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/nfa/thompson/pikevm.rs:2149:24:
range end index 9 out of range for slice of length 8
backtrace:
  &lt;[core::option::Option&lt;regex_automata::util::primitives::NonMaxUsize&gt;] as core::ops::index::IndexMut&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;regex_automata::nfa::thompson::pikevm::SlotTable&gt;::for_state (regex-automata/src/nfa/thompson/pikevm.rs:2149:24)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::epsilon_closure_explore (regex-automata/src/nfa/thompson/pikevm.rs:1692:37)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::epsilon_closure (regex-automata/src/nfa/thompson/pikevm.rs:1631:26)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search_imp (regex-automata/src/nfa/thompson/pikevm.rs:1364:22)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search_slots_imp (regex-automata/src/nfa/thompson/pikevm.rs:1146:29)
  &lt;regex_automata::nfa::thompson::pikevm::PikeVM&gt;::search_slots (regex-automata/src/nfa/thompson/pikevm.rs:1099:27)
  repro_regex_automata_src_nfa_thompson_pikevm_rs_2149_24_d8a348::main (src/main.rs:9:22)</pre></td></tr>
<tr><td valign="top"><pre>let builder = Regex::builder();
let regex = builder.build("q/+�\u{9f633} S\u{8753d}\u{69001}").unwrap();
let mut cache = Cache::new(&amp;builder.build("").unwrap());
let vec = vec![0u8, 0u8 /* x17 */, 0u8];
let input = Input::new::&lt;Vec&lt;u8&gt;&gt;(&amp;vec);
let _ = regex.search_slots_with(&amp;mut cache, &amp;input, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/meta/wrappers.rs:633:42:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;regex_automata::meta::wrappers::HybridEngine&gt;::try_search (regex-automata/src/meta/wrappers.rs:633:42)
  &lt;regex_automata::meta::strategy::Core as regex_automata::meta::strategy::Strategy&gt;::search (regex-automata/src/meta/strategy.rs:727:21)
  &lt;regex_automata::meta::strategy::Core as regex_automata::meta::strategy::Strategy&gt;::search_slots (regex-automata/src/meta/strategy.rs:816:26)
  &lt;regex_automata::meta::regex::Regex&gt;::search_slots_with (regex-automata/src/meta/regex.rs:1456:24)
  repro_regex_automata_src_meta_wrappers_rs_633_42_afa280::main (src/main.rs:11:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::match_pattern(
    &amp;&lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::never_match().unwrap(),
    StateID::from(0u8), 10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/sparse.rs:2342:21:
attempt to multiply with overflow
backtrace:
  &lt;regex_automata::dfa::sparse::State&gt;::pattern_id (regex-automata/src/dfa/sparse.rs:2342:21)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::match_pattern (regex-automata/src/dfa/sparse.rs:1192:27)
  repro_regex_automata_src_dfa_sparse_rs_2342_21_44c474::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let prefilter = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst,
    &amp;regex_syntax::parse("a").unwrap()).unwrap();
let _ = prefilter.find(&amp;vec![], Span::from(1usize..0usize));</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/util/prefilter/memchr.rs:43:45:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;[u8] as core::ops::index::Index&lt;regex_automata::util::search::Span&gt;&gt;::index (regex-automata/src/util/search.rs:863:14)
  &lt;regex_automata::util::prefilter::memchr::Memchr as regex_automata::util::prefilter::PrefilterI&gt;::find (regex-automata/src/util/prefilter/memchr.rs:43:45)
  &lt;regex_automata::util::prefilter::Prefilter&gt;::find (regex-automata/src/util/prefilter/mod.rs:356:22)
  repro_regex_automata_src_util_prefilter_memchr_rs_43_45_1428ac::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::next_state(&amp;&lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::new(
    "[[test]]\nname ").unwrap(),
    &lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::universal_start_state(&amp;Builder::default().build_from_nfa(
    &amp;NFA::compiler().build("&lt;svg \
    xmtp:3.o182\"Bp-col\"/&gt;iearG").unwrap()).unwrap(), Anchored::No).unwrap(),
    0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/sparse.rs:1530:27:
range end index 152 out of range for slice of length 138
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;regex_automata::dfa::sparse::Transitions&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::state (regex-automata/src/dfa/sparse.rs:1530:27)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::next_state (regex-automata/src/dfa/sparse.rs:1155:17)
  repro_regex_automata_src_dfa_sparse_rs_1530_27_1f2ce5::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let compiler = NFA::compiler();
let _ = compiler.build_from_hir(&amp;Hir::repetition(Repetition { min: 0u32,
    max: Some(100000000u32), greedy: false,
    sub: Box::new(regex_automata::util::syntax::parse("a").unwrap()) }));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  regex_automata::nfa::thompson::builder::Builder::add (src/nfa/thompson/builder.rs:1118)
  regex_automata::nfa::thompson::builder::Builder::add_union_reverse (src/nfa/thompson/builder.rs:743)
  regex_automata::nfa::thompson::compiler::Compiler::add_union_reverse (src/nfa/thompson/compiler.rs:1682)
  regex_automata::nfa::thompson::compiler::Compiler::c_bounded (src/nfa/thompson/compiler.rs:1231)
  regex_automata::nfa::thompson::compiler::Compiler::c_repetition (src/nfa/thompson/compiler.rs:1174)
  regex_automata::nfa::thompson::compiler::Compiler::c (src/nfa/thompson/compiler.rs:1028)
  regex_automata::nfa::thompson::compiler::Compiler::c_cap (src/nfa/thompson/compiler.rs:1157)
  regex_automata::nfa::thompson::compiler::{impl#4}::compile::{closure#1}&lt;&amp;regex_syntax::hir::Hir&gt; (src/nfa/thompson/compiler.rs:1002)
  regex_automata::nfa::thompson::compiler::Compiler::c_alt_iter&lt;core::iter::adapters::map::Map&lt;core::slice::iter::Iter&lt;&amp;regex_syntax::hir::Hir&gt;, regex_automata::nfa::thompson::compiler::{impl#4}::compile::{closure_env#1}&lt;&amp;regex_syntax::hir::Hir&gt;&gt;&gt; (src/nfa/thompson/compiler.rs:1110)
  regex_automata::nfa::thompson::compiler::Compiler::compile&lt;&amp;regex_syntax::hir::Hir&gt; (src/nfa/thompson/compiler.rs:1000)
  regex_automata::nfa::thompson::compiler::Compiler::build_many_from_hir&lt;&amp;regex_syntax::hir::Hir&gt; (src/nfa/thompson/compiler.rs:903)
  regex_automata::nfa::thompson::compiler::Compiler::build_from_hir (src/nfa/thompson/compiler.rs:859)
  repro_out_of_memory_in_regex_automata_nfa_thompson_compiler_build__163560::main (src/main.rs:7)
  (21 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let regex = Regex::new("; hm4K]l&amp;\u{3fec0}*f\u{934b4} ").unwrap();
let mut cache = Cache::new(&amp;Regex::builder().build("").unwrap());
let vec = vec![108u8, 66u8, 0u8, 124u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 0u8,
    56u8, 47u8, 0u8, 0u8];
let input = Input::new::&lt;Vec&lt;u8&gt;&gt;(&amp;vec);
let _ = regex.search_half_with(&amp;mut cache, &amp;input);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/meta/wrappers.rs:653:49:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;regex_automata::meta::wrappers::HybridEngine&gt;::try_search_half_fwd (regex-automata/src/meta/wrappers.rs:653:49)
  &lt;regex_automata::meta::strategy::Core as regex_automata::meta::strategy::Strategy&gt;::search_half (regex-automata/src/meta/strategy.rs:759:21)
  &lt;regex_automata::meta::regex::Regex&gt;::search_half_with (regex-automata/src/meta/regex.rs:1301:24)
  repro_regex_automata_src_meta_wrappers_rs_653_49_1511e9::main (src/main.rs:10:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Repetition { span: Span { start: Position { offset: 255usize,
    line: 1usize, column: 2147483648usize }, end: Position::new(5usize,
    4294967295usize,
    13648082024310080854usize) }.with_end(Position { offset: 0usize,
    line: 2129180065523257685usize, column: 8usize }),
    op: RepetitionOp { span: Span::splat(Position { offset: 2usize,
    line: 2usize, column: 8usize }), kind: RepetitionKind::OneOrMore },
    greedy: true,
    ast: Box::new(Ast::class_perl(ClassPerl { span: Span::new(
    Position { offset: 8393290005580950419usize, line: 4294967295usize,
    column: 2usize }, Position::new(11usize, 8796093022208usize, 4usize)),
    kind: ClassPerlKind::Word, negated: false })) };
let _ = HexLiteralKind::UnicodeLong;
let utf8_sequence = Utf8Sequence::One(Utf8Range { start: 227u8, end: 8u8 });
let _ = utf8_sequence.matches(&amp;vec![8u8, 166u8, 161u8, 112u8, 92u8, 44u8, 50u8,
    29u8, 60u8, 168u8, 172u8, 45u8, 20u8]);
let bounded_backtracker = BoundedBacktracker::new(" ".repeat(429)).unwrap();
let _ = bounded_backtracker.get_nfa();
let _ = Anchored::No;
let regex = Builder::default().build_sparse("# This file represent tests that \
    may be expensive to run on some regex engines.\n# For example, \
    tests" /* ... 2831 more chars */).unwrap();
let _ = regex.forward();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  hashbrown::map::make_hash&lt;[u8], std::hash::random::RandomState&gt; (hashbrown/src/map.rs:241)
  hashbrown::map::HashMap&lt;regex_automata::util::determinize::state::State, regex_automata::util::primitives::StateID, std::hash::random::RandomState, alloc::alloc::Global&gt;::get&lt;regex_automata::util::determinize::state::State, regex_automata::util::primitives::StateID, std::hash::random::RandomState,... (+56 chars)
  regex_automata::dfa::determinize::Runner::maybe_add_state (src/dfa/determinize.rs:509)
  regex_automata::dfa::determinize::Runner::cached_state (src/dfa/determinize.rs:317)
  regex_automata::dfa::determinize::Runner::run (src/dfa/determinize.rs:244)
  regex_automata::dfa::determinize::Config::run (src/dfa/determinize.rs:76)
  regex_automata::dfa::dense::Builder::build_from_nfa (src/dfa/dense.rs:1260)
  regex_automata::dfa::dense::Builder::build_many&lt;&amp;str&gt; (src/dfa/dense.rs:1179)
  regex_automata::dfa::regex::Builder::build_many&lt;&amp;str&gt; (src/dfa/regex.rs:745)
  regex_automata::dfa::regex::Builder::build_many_sparse&lt;&amp;str&gt; (src/dfa/regex.rs:755)
  regex_automata::dfa::regex::Builder::build_sparse (src/dfa/regex.rs:724)
  repro_timeout_in_regex_automata_dfa_regex_regex_regex_automata_dfa_32f52b::main (src/main.rs:24)
  (12 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let builder = Builder::new();
let _ = builder.build_many::&lt;String&gt;(
    &amp;["=9B^ve\u{6874c}Y4\u{cbc25}'?\u{41f4f}`;'😀\u{df101}\u{a992b}dZ\u{fbdbf\
    }".to_string(), "\u{8e4b2}]?6%\u{2ff58}JL\u{f670b}*rE87zH-\u{109d6f}- \
    \u{353a1}\u{1a14f}\u{e96c3}".to_string(),
    "a\u{301}\u{301}\u{301}".to_string(),
    include_str!("string_0.txt").to_string()]);
let _ = SmallIndex::try_from(1000u32);
let compiler = NFA::compiler();
let n_f_a = compiler.build_many::&lt;String&gt;(&amp;["Zażółć gęślą \
    jaźń".to_string()]).unwrap();
let byte_classes = n_f_a.byte_classes();
let _ = byte_classes.iter();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  regex_automata::util::primitives::SmallIndex::new_unchecked (src/util/primitives.rs:188)
  regex_automata::util::primitives::StateID::new_unchecked (src/util/primitives.rs:456)
  regex_automata::util::sparse_set::SparseSet::insert (src/util/sparse_set.rs:187)
  regex_automata::util::determinize::next::{closure#0} (src/util/determinize/mod.rs:122)
  regex_automata::util::determinize::state::Repr::iter_nfa_state_ids&lt;regex_automata::util::determinize::next::{closure_env#0}&gt; (src/util/determinize/state.rs:534)
  regex_automata::util::determinize::state::State::iter_nfa_state_ids&lt;regex_automata::util::determinize::next::{closure_env#0}&gt; (src/util/determinize/state.rs:172)
  regex_automata::util::determinize::next (src/util/determinize/mod.rs:121)
  regex_automata::dfa::determinize::Runner::cached_state (src/dfa/determinize.rs:308)
  regex_automata::dfa::determinize::Runner::run (src/dfa/determinize.rs:244)
  regex_automata::dfa::determinize::Config::run (src/dfa/determinize.rs:76)
  regex_automata::dfa::dense::Builder::build_from_nfa (src/dfa/dense.rs:1260)
  regex_automata::dfa::dense::Builder::build_many&lt;alloc::string::String&gt; (regex-automata/src/dfa/dense.rs:1179)
  repro_timeout_in_regex_automata_dfa_dense_builder_build_many_fdf8da::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::match_pattern(
    &amp;&lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::new_many::&lt;String&gt;(
    &amp;[include_str!("string_0.txt").to_string(), " ".to_string(),
    "\u{200b}\u{200d}".to_string()]).unwrap(),
    StateID::try_from(2049u32).unwrap(), 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/sparse.rs:1521:35:
mid &gt; len
backtrace:
  &lt;[u8]&gt;::split_at (core/src/slice/mod.rs:1964:21)
  &lt;regex_automata::dfa::sparse::Transitions&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::state (regex-automata/src/dfa/sparse.rs:1521:35)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::match_pattern (regex-automata/src/dfa/sparse.rs:1192:17)
  repro_regex_automata_src_dfa_sparse_rs_1521_35_6ad61f::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::match_pattern(
    &amp;&lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt;&gt;::new_many::&lt;String&gt;(
    &amp;["H+".to_string(), "Zażółć gęślą j".to_string()]).unwrap(),
    StateID::new_unchecked(255usize), 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/dense.rs:4651:9:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;regex_automata::dfa::dense::MatchStates&lt;alloc::vec::Vec&lt;u32&gt;&gt;&gt;::pattern_id (regex-automata/src/dfa/dense.rs:4651:9)
  &lt;regex_automata::dfa::dense::DFA&lt;alloc::vec::Vec&lt;u32&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::match_pattern (regex-automata/src/dfa/dense.rs:3245:17)
  repro_regex_automata_src_dfa_dense_rs_4651_9_9c813a::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let group_info = GroupInfo::default();
let usize_v = group_info.explicit_slot_len();
let regex = Regex::new_many::&lt;String&gt;(
    &amp;["\u{ff0c6}\u{5c4ee}\u{200b}|R.".to_string()]).unwrap();
let builder = Builder::new();
let regex2 = builder.build_many::&lt;String&gt;(&amp;[]).unwrap();
let mut cache = regex2.create_cache();
let mut pattern_set = PatternSet::new(usize_v);
let vec = vec![73u8, 70u8];
let input = Input::new::&lt;Vec&lt;u8&gt;&gt;(&amp;vec);
regex.which_overlapping_matches_with(&amp;mut cache, &amp;input, &amp;mut pattern_set);</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/meta/wrappers.rs:739:49:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;regex_automata::meta::wrappers::HybridEngine&gt;::try_which_overlapping_matches (regex-automata/src/meta/wrappers.rs:739:49)
  &lt;regex_automata::meta::strategy::Core as regex_automata::meta::strategy::Strategy&gt;::which_overlapping_matches (regex-automata/src/meta/strategy.rs:889:32)
  &lt;regex_automata::meta::regex::Regex&gt;::which_overlapping_matches_with (regex-automata/src/meta/regex.rs:1505:24)
  repro_regex_automata_src_meta_wrappers_rs_739_49_b29aa9::main (src/main.rs:17:11)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let input = Input::new::&lt;Vec&lt;u8&gt;&gt;(&amp;vec);
let _ = &lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::match_len(
    &amp;&lt;regex_automata::dfa::sparse::DFA&lt;Vec&lt;u8&gt;&gt;&gt;::always_match().unwrap(),
    &lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt; as regex_automata::dfa::
    Automaton&gt;::start_state_forward(
    &amp;&lt;regex_automata::dfa::dense::DFA&lt;Vec&lt;u32&gt;&gt;&gt;::new("مر").unwrap(),
    &amp;input).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at regex-automata/src/dfa/sparse.rs:1529:37:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;regex_automata::dfa::sparse::Transitions&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::state (regex-automata/src/dfa/sparse.rs:1529:37)
  &lt;regex_automata::dfa::sparse::DFA&lt;alloc::vec::Vec&lt;u8&gt;&gt; as regex_automata::dfa::automaton::Automaton&gt;::match_len (regex-automata/src/dfa/sparse.rs:1179:17)
  repro_regex_automata_src_dfa_sparse_rs_1529_37_1d7599::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let regex = Builder::new().build_many_sparse::&lt;String&gt;(&amp;[include_str!(
    "string_0.txt").to_string(), "\u{200b}\u{200d}".to_string(),
    include_str!("string_1.txt").to_string(),
    include_str!("string_2.txt").to_string(), "&lt;svg \
    xmlns=\"http://www.w3.org/2000/svg\" width=\"NaN\" height=\"166\" \
    viewBox=\"0 0 16 37\"&gt;&lt;defs&gt;&lt;linearGr" /* ... 150 more chars */.to_string(),
    include_str!("string_3.txt").to_string()]).unwrap();
let _ = regex.reverse();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  regex_automata::util::primitives::{impl#96}::index_mut&lt;regex_automata::util::primitives::StateID&gt; (src/util/primitives.rs:582)
  regex_automata::util::sparse_set::SparseSet::insert (src/util/sparse_set.rs:188)
  regex_automata::util::determinize::next::{closure#0} (src/util/determinize/mod.rs:122)
  regex_automata::util::determinize::state::Repr::iter_nfa_state_ids&lt;regex_automata::util::determinize::next::{closure_env#0}&gt; (src/util/determinize/state.rs:534)
  regex_automata::util::determinize::state::State::iter_nfa_state_ids&lt;regex_automata::util::determinize::next::{closure_env#0}&gt; (src/util/determinize/state.rs:172)
  regex_automata::util::determinize::next (src/util/determinize/mod.rs:121)
  regex_automata::dfa::determinize::Runner::cached_state (src/dfa/determinize.rs:308)
  regex_automata::dfa::determinize::Runner::run (src/dfa/determinize.rs:244)
  regex_automata::dfa::determinize::Config::run (src/dfa/determinize.rs:76)
  regex_automata::dfa::dense::Builder::build_from_nfa (src/dfa/dense.rs:1260)
  regex_automata::dfa::dense::Builder::build_many&lt;alloc::string::String&gt; (regex-automata/src/dfa/dense.rs:1179)
  regex_automata::dfa::regex::Builder::build_many&lt;alloc::string::String&gt; (regex-automata/src/dfa/regex.rs:733)
  regex_automata::dfa::regex::Builder::build_many_sparse&lt;alloc::string::String&gt; (regex-automata/src/dfa/regex.rs:755)
  repro_timeout_in_regex_automata_dfa_regex_regex_regex_automata_dfa_8f16b6::main (src/main.rs:4)
  (3 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let hir = Hir::repetition(Repetition { min: 0u32, max: Some(1000000000u32),
    greedy: false, sub: Box::new(regex_automata::util::syntax::parse_with("a",
    &amp;Config::new()).unwrap()) });
let builder = Builder::new();
let _ = builder.build_from_nfa(Compiler::new().build_from_hir(&amp;hir).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  regex_automata::nfa::thompson::builder::Builder::add (src/nfa/thompson/builder.rs:1118)
  regex_automata::nfa::thompson::builder::Builder::add_union_reverse (src/nfa/thompson/builder.rs:743)
  regex_automata::nfa::thompson::compiler::Compiler::add_union_reverse (src/nfa/thompson/compiler.rs:1682)
  regex_automata::nfa::thompson::compiler::Compiler::c_bounded (src/nfa/thompson/compiler.rs:1231)
  regex_automata::nfa::thompson::compiler::Compiler::c_repetition (src/nfa/thompson/compiler.rs:1174)
  regex_automata::nfa::thompson::compiler::Compiler::c (src/nfa/thompson/compiler.rs:1028)
  regex_automata::nfa::thompson::compiler::Compiler::c_cap (src/nfa/thompson/compiler.rs:1157)
  regex_automata::nfa::thompson::compiler::{impl#4}::compile::{closure#1}&lt;&amp;regex_syntax::hir::Hir&gt; (src/nfa/thompson/compiler.rs:1002)
  regex_automata::nfa::thompson::compiler::Compiler::c_alt_iter&lt;core::iter::adapters::map::Map&lt;core::slice::iter::Iter&lt;&amp;regex_syntax::hir::Hir&gt;, regex_automata::nfa::thompson::compiler::{impl#4}::compile::{closure_env#1}&lt;&amp;regex_syntax::hir::Hir&gt;&gt;&gt; (src/nfa/thompson/compiler.rs:1110)
  regex_automata::nfa::thompson::compiler::Compiler::compile&lt;&amp;regex_syntax::hir::Hir&gt; (src/nfa/thompson/compiler.rs:1000)
  regex_automata::nfa::thompson::compiler::Compiler::build_many_from_hir&lt;&amp;regex_syntax::hir::Hir&gt; (src/nfa/thompson/compiler.rs:903)
  regex_automata::nfa::thompson::compiler::Compiler::build_from_hir (src/nfa/thompson/compiler.rs:859)
  repro_out_of_memory_in_regex_automata_nfa_thompson_backtrack_build_8326ca::main (src/main.rs:10)
  (21 std/libc frames omitted)</pre></td></tr>
</table>
