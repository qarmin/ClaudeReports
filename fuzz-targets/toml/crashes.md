# toml - crashes

Source: toml 1.1.8+spec-1.1.0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;toml_parser::Span as ::core::ops::Add&lt;usize&gt;&gt;::add(Span::new_unchecked(
    10000000000000000000usize, 4294967295usize), 18446744073709551614usize);</pre></td><td valign="top"><pre>thread 'main' panicked at toml_parser-1.1.5+spec/src/source.rs:252:20:
attempt to add with overflow
backtrace:
  &lt;toml_parser::source::Span as core::ops::arith::Add&lt;usize&gt;&gt;::add (toml_parser-1.1.5+spec/src/source.rs:252:20)
  repro_toml_parser_1_1_5_spec_src_source_rs_252_20_7bec53::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut array_of_tables = ArrayOfTables::new();
let _ = array_of_tables.remove(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at toml_edit-0.25.17+spec/src/array_of_tables.rs:132:14:
removal index (is 0) should be &lt; len (is 0)
backtrace:
  &lt;toml_edit::array_of_tables::ArrayOfTables&gt;::remove (toml_edit-0.25.17+spec/src/array_of_tables.rs:132:14)
  repro_toml_edit_0_25_17_spec_src_array_of_tables_rs_132_14_129f3b::main (src/main.rs:5:29)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;toml::map::Map&lt;String, toml::Value&gt;&gt;::with_capacity(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_map_map_string_toml_value_with_capacit_ce2536::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Array::default().remove(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at toml_edit-0.25.17+spec/src/array.rs:306:35:
removal index (is 0) should be &lt; len (is 0)
backtrace:
  &lt;toml_edit::array::Array&gt;::remove (toml_edit-0.25.17+spec/src/array.rs:306:35)
  repro_toml_edit_0_25_17_spec_src_array_rs_306_35_9dfd9d::main (src/main.rs:4:30)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Value::from(&lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_value_from_toml_map_map_string_toml_va_d8bfb1::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = Value::Table(&lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(1000000000usize));
let _ = value.get::&lt;String&gt;("".to_string());</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_value_get_string_d82d8e::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let map = &lt;toml::map::Map&lt;String, toml::Value&gt;&gt;::with_capacity(1000000000usize);
let _ = map.iter();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_map_map_string_toml_value_iter_9d1b02::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let offset = Offset::Custom { minutes: i16::MIN };
let _ = offset.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at toml_datetime-1.1.2+spec/src/datetime.rs:285:21:
attempt to multiply with overflow
backtrace:
  &lt;toml_datetime::datetime::Offset as core::fmt::Display&gt;::fmt (toml_datetime-1.1.2+spec/src/datetime.rs:285:21)
  &lt;toml_datetime::datetime::Offset as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;toml_datetime::datetime::Offset as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_toml_datetime_1_1_2_spec_src_datetime_rs_285_21_f8a03c::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;toml_parser::Span as ::core::ops::Add&lt;usize&gt;&gt;::add(Span::new_unchecked(
    0usize, usize::MAX), 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at toml_parser-1.1.5+spec/src/source.rs:253:18:
attempt to add with overflow
backtrace:
  &lt;toml_parser::source::Span as core::ops::arith::Add&lt;usize&gt;&gt;::add (toml_parser-1.1.5+spec/src/source.rs:253:18)
  repro_toml_parser_1_1_5_spec_src_source_rs_253_18_b16a21::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let event = Event::new_unchecked(EventKind::InlineTableOpen, None,
    Span::new_unchecked(1usize, 0usize));
let span = event.span();
let _ = span.len();</pre></td><td valign="top"><pre>thread 'main' panicked at toml_parser-1.1.5+spec/src/source.rs:215:9:
attempt to subtract with overflow
backtrace:
  &lt;toml_parser::source::Span&gt;::len (toml_parser-1.1.5+spec/src/source.rs:215:9)
  repro_toml_parser_1_1_5_spec_src_source_rs_215_9_b96292::main (src/main.rs:8:18)</pre></td></tr>
<tr><td valign="top"><pre>let map = &lt;toml::map::Map&lt;String, toml::Value&gt;&gt;::with_capacity(1000000000usize);
let _ = map.values();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_map_map_string_toml_value_values_17a9ba::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Value::Table(&lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_value_table_6c5a3a::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let map = &lt;toml::map::Map&lt;String, toml::Value&gt;&gt;::with_capacity(1000000000usize);
let _ = map.keys();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_map_map_string_toml_value_keys_aa96eb::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = Value::Table(&lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(1000000000usize));
let _ = value.as_table();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_value_as_table_7a74ca::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = Value::Datetime(Datetime::from(Date { year: 0u16, month: 0u8,
    day: 0u8 }));
let _ = value.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at toml-1.1.8+spec/src/value.rs:389:36:
called `Result::unwrap()` on an `Err` value: Custom("invalid date, expected month between 01 and 12")
backtrace:
  &lt;toml::value::Value as core::fmt::Display&gt;::fmt (toml-1.1.8+spec/src/value.rs:389:36)
  &lt;toml::value::Value as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;toml::value::Value as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_toml_1_1_8_spec_src_value_rs_389_36_c86bb0::main (src/main.rs:7:19)</pre></td></tr>
<tr><td valign="top"><pre>let value = Value::from(&lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(1000000000usize));
let _ = value.as_datetime();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_value_as_datetime_460759::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut map = &lt;toml::map::Map&lt;String,
    toml::Value&gt; as ::core::default::Default&gt;::default();
let _ = map.insert("".to_string(), Value::Table(&lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(1000000000usize)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_map_map_string_toml_value_insert_d7bc71::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = Value::from(0i64);
let _ = value.same_type(&amp;Value::from(&lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(1000000000usize)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_value_same_type_765351::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Value::Array(vec![Value::from(&lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(1000000000usize))]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_value_array_270d5a::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let map = &lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(1000000000000usize);
let string = "".to_string();
let _ = map.get::&lt;String&gt;(&amp;string);</pre></td><td valign="top"><pre>out of memory: memory allocation of 19791209299984 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_map_map_string_toml_value_get_string_9b5f7b::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = Value::Table(&lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(100000000000000000usize));
let _ = value.as_array();</pre></td><td valign="top"><pre>out of memory: memory allocation of 1297036692682702864 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt; (indexmap/src/map.rs:171)
  toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;::with_capacity&lt;alloc::string::String, toml::value::Value&gt; (src/map.rs:77)
  repro_out_of_memory_in_toml_value_as_array_3c5ccf::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;toml::map::Map&lt;String,
    toml::Value&gt;&gt;::with_capacity(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at hashbrown/src/raw.rs:36:40:
Hash table capacity overflow
backtrace:
  &lt;hashbrown::raw::Fallibility&gt;::capacity_overflow (hashbrown/src/raw.rs:36:40)
  &lt;hashbrown::raw::RawTableInner&gt;::fallible_with_capacity::&lt;hashbrown::alloc::inner::Global&gt;::{closure#0} (hashbrown/src/raw.rs:1631:48)
  &lt;hashbrown::raw::RawTableInner&gt;::fallible_with_capacity::&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1631:22)
  &lt;hashbrown::raw::RawTableInner&gt;::with_capacity::&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661:13)
  &lt;hashbrown::raw::RawTable&lt;usize&gt;&gt;::with_capacity_in (hashbrown/src/raw.rs:648:20)
  &lt;hashbrown::raw::RawTable&lt;usize&gt;&gt;::with_capacity (hashbrown/src/raw.rs:601:9)
  &lt;hashbrown::table::HashTable&lt;usize&gt;&gt;::with_capacity (hashbrown/src/table.rs:92:18)
  &lt;indexmap::inner::Core&lt;alloc::string::String, toml::value::Value&gt;&gt;::with_capacity (indexmap/src/inner.rs:122:22)
  &lt;indexmap::map::IndexMap&lt;alloc::string::String, toml::value::Value, foldhash::fast::RandomState&gt;&gt;::with_capacity_and_hasher (indexmap/src/map.rs:171:23)
  &lt;toml::map::Map&lt;alloc::string::String, toml::value::Value&gt;&gt;::with_capacity (toml-1.1.8+spec/src/map.rs:77:18)
  repro_hashbrown_src_raw_rs_36_40_79d5a8::main (src/main.rs:2:13)</pre></td></tr>
</table>
