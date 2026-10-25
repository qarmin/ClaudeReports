# serde_yaml - crashes

Source: https://github.com/dtolnay/serde-yaml @ 2009506d33767dfc88e979d6bc0d53d09f941c94

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Mapping::with_capacity(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_mapping_with_capacity_1cf888::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mapping = Mapping::default();
let _ = mapping.get::&lt;serde_yaml::Value&gt;(Mapping::with_capacity(
    1000000000usize).swap_remove::&lt;String&gt;("".to_string()).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_mapping_get_value_b9d0f2::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = Mapping::with_capacity(1000000000usize).insert(Value::from(0i16),
    Value::Bool(false)).unwrap();
let _ = value.as_mapping();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_value_as_mapping_0fe07c::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Mapping::with_capacity(
    1000000000usize).shift_remove::&lt;serde_yaml::Value&gt;(Value::from(0u64));</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_mapping_shift_remove_value_d71cc8::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Value::Mapping(Mapping::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_value_mapping_966329::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Mapping::from(Mapping::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_mapping_from_serde_yaml_mapping_67fffa::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let value = Value::Tagged(Box::new(TaggedValue { tag: Tag::new("a".to_string()),
    value: Value::from(0i16) }));
let _ = value.get::&lt;serde_yaml::Value&gt;(Mapping::with_capacity(
    1000000000usize).remove::&lt;String&gt;("".to_string()).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_value_get_value_be2ec7::main (src/main.rs:8)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut mapping = Mapping::default();
let _ = mapping.swap_remove_entry::&lt;serde_yaml::Value&gt;(Mapping::with_capacity(
    1000000000usize).remove::&lt;String&gt;("".to_string()).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_mapping_swap_remove_entry_value_bbba70::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TaggedValue { tag: Tag::new("K".to_string()),
    value: Mapping::with_capacity(1000000000usize).remove::&lt;String&gt;(
    "".to_string()).unwrap() };</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_value_taggedvalue_literal_1985f1::main (src/main.rs:6)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mapping = Mapping::with_capacity(1000000000usize);
let _ = mapping.iter();</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity_in&lt;usize, hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:648)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/raw.rs:601)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::with_capacity&lt;usize&gt; (hashbrown/src/table.rs:92)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:122)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity_and_hasher&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:171)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::with_capacity&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/map.rs:156)
  serde_yaml::mapping::Mapping::with_capacity (serde-yaml/src/mapping.rs:29)
  repro_out_of_memory_in_serde_yaml_mapping_iter_3f8d23::main (src/main.rs:4)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut mapping = Mapping::from(Mapping::with_capacity(0usize));
mapping.reserve(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 19327352848 bytes failed
backtrace:
  hashbrown::raw::Fallibility::alloc_err (src/raw.rs:45)
  hashbrown::raw::RawTableInner::new_uninitialized&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1597)
  hashbrown::raw::RawTableInner::fallible_with_capacity&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1634)
  hashbrown::raw::RawTableInner::prepare_resize&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:2696)
  hashbrown::raw::RawTableInner::resize_inner&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:2898)
  hashbrown::raw::RawTableInner::reserve_rehash_inner&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:2784)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::reserve_rehash&lt;usize, hashbrown::alloc::inner::Global, indexmap::inner::get_hash::{closure_env#0}&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;&gt; (hashbrown/src/raw.rs:962)
  hashbrown::raw::RawTable&lt;usize, hashbrown::alloc::inner::Global&gt;::reserve&lt;usize, hashbrown::alloc::inner::Global, indexmap::inner::get_hash::{closure_env#0}&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;&gt; (hashbrown/src/raw.rs:914)
  hashbrown::table::HashTable&lt;usize, hashbrown::alloc::inner::Global&gt;::reserve&lt;usize, hashbrown::alloc::inner::Global, indexmap::inner::get_hash::{closure_env#0}&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;&gt; (hashbrown/src/table.rs:846)
  indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;::reserve&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt; (indexmap/src/inner.rs:233)
  indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt;::reserve&lt;serde_yaml::value::Value, serde_yaml::value::Value, std::hash::random::RandomState&gt; (indexmap/src/map.rs:383)
  serde_yaml::mapping::Mapping::reserve (serde-yaml/src/mapping.rs:42)
  repro_out_of_memory_in_serde_yaml_mapping_reserve_ff1865::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Mapping::with_capacity(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at hashbrown/src/raw.rs:36:40:
Hash table capacity overflow
backtrace:
  &lt;hashbrown::raw::Fallibility&gt;::capacity_overflow (hashbrown/src/raw.rs:36:40)
  &lt;hashbrown::raw::RawTableInner&gt;::fallible_with_capacity::&lt;hashbrown::alloc::inner::Global&gt;::{closure#0} (hashbrown/src/raw.rs:1631:48)
  &lt;hashbrown::raw::RawTableInner&gt;::fallible_with_capacity::&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1631:22)
  &lt;hashbrown::raw::RawTableInner&gt;::with_capacity::&lt;hashbrown::alloc::inner::Global&gt; (hashbrown/src/raw.rs:1661:13)
  &lt;hashbrown::raw::RawTable&lt;usize&gt;&gt;::with_capacity_in (hashbrown/src/raw.rs:648:20)
  &lt;hashbrown::raw::RawTable&lt;usize&gt;&gt;::with_capacity (hashbrown/src/raw.rs:601:9)
  &lt;hashbrown::table::HashTable&lt;usize&gt;&gt;::with_capacity (hashbrown/src/table.rs:92:18)
  &lt;indexmap::inner::Core&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;&gt;::with_capacity (indexmap/src/inner.rs:122:22)
  &lt;indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;&gt;::with_capacity_and_hasher (indexmap/src/map.rs:171:23)
  &lt;indexmap::map::IndexMap&lt;serde_yaml::value::Value, serde_yaml::value::Value&gt;&gt;::with_capacity (indexmap/src/map.rs:156:9)
  &lt;serde_yaml::mapping::Mapping&gt;::with_capacity (serde-yaml/src/mapping.rs:29:18)
  repro_hashbrown_src_raw_rs_36_40_79d5a8::main (src/main.rs:4:13)</pre></td></tr>
</table>
