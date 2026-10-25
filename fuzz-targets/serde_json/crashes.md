# serde_json - crashes

Source: https://github.com/serde-rs/json @ 8c535a260ce561eaab4111aafc29038eb9f77ff3

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut map = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt; as ::core::default::Default&gt;::default();
let _ = map.shift_insert(1usize, "".to_string(),
    Value::Object(&lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(0usize)));</pre></td><td valign="top"><pre>thread 'main' panicked at json/src/map.rs:151:18:
index out of bounds: the len is 0 but the index is 1. Expected index &lt;= len
backtrace:
  indexmap::util::assert_index_le (indexmap/src/util.rs:19:5)
  &lt;indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std (indexmap/src/map.rs:687:17)
  &lt;serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;&gt;::shift_insert (json/src/map.rs:151:18)
  repro_json_src_map_rs_151_18_129200::main (src/main.rs:5:17)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_map_string_serde_json_value_with_784997::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize).shift_insert(0usize,
    "".to_string(), Value::from(0u32)).unwrap();
let _ = value.as_array();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_value_as_array_282855::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = Value::from(&lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize));
let _ = value.get::&lt;String&gt;("".to_string());</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_value_get_string_b65a76::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(10000000000usize).shift_insert(0usize,
    "".to_string(), Value::from(0u8)).unwrap();
let _ = value.as_object();</pre></td><td valign="top"><pre>out of memory: memory allocation of 154618822672 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_value_as_object_07cab0::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Value::from(&lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_value_from_serde_json_map_string_c7b98c::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize).insert("".to_string(),
    &lt;serde_json::Value as ::core::str::FromStr&gt;::from_str(
    "").unwrap()).unwrap();
let _ = value.get::&lt;usize&gt;(1usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_value_get_usize_80a541::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let map = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize);
let _ = map.iter();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_map_string_serde_json_value_iter_70d911::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Value::Object(&lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(100000000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 1266637395197968 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_value_object_bcd659::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let map = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize);
let _ = map.keys();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_map_string_serde_json_value_keys_60aa5c::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let map = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize);
let string = "".to_string();
let _ = map.get::&lt;String&gt;(&amp;string);</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_map_string_serde_json_value_get__479686::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let map = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize);
let _ = map.values();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_map_string_serde_json_value_valu_b4ddfc::main (src/main.rs:2)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut map = &lt;serde_json::Map&lt;String,
    serde_json::Value&gt; as ::core::default::Default&gt;::default();
let _ = map.shift_insert(0usize, "".to_string(),
    Value::from(&lt;serde_json::Map&lt;String,
    serde_json::Value&gt;&gt;::with_capacity(1000000000usize)));</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity&lt;alloc::string::String, serde_json::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;alloc::string::String, serde_json::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  serde_json::map::Map&lt;alloc::string::String, serde_json::value::Value&gt;::with_capacity (json/src/map.rs:68)
  repro_out_of_memory_in_serde_json_map_string_serde_json_value_shif_fdcaf8::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
</table>
