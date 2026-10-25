# widestring - crashes

Source: https://github.com/VoidStarKat/widestring-rs @ 5c65b30447e5cf4beb824eb2d6464b63110bce76

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = U32String::with_capacity(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::ustring::U32String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u32string_with_capacity_d61b51::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Utf32String::from_ustring(U32String::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::ustring::U32String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_utf32string_from_ustring_786d12::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = U16String::with_capacity(2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::ustring::U16String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u16string_with_capacity_1bb32b::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut utf16_string = Utf16String::new();
utf16_string.reserve_exact(4294967295usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8589934590 bytes failed
backtrace:
  widestring::utfstring::Utf16String::reserve_exact (widestring-rs/src/utfstring.rs:180)
  repro_out_of_memory_in_widestring_utf16string_reserve_exact_187c71::main (src/main.rs:5)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let u16_string = U16String::with_capacity(2147483648usize);
let _ = u16_string.as_ustr();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::ustring::U16String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u16string_as_ustr_a312ca::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let u16_string = U16String::with_capacity(2147483648usize);
let _ = u16_string.as_vec();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::ustring::U16String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u16string_as_vec_b3da1b::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Utf16String::with_capacity(1000000000000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2000000000000000000 bytes failed
backtrace:
  widestring::utfstring::Utf16String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_utf16string_with_capacity_386b53::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Utf16String::try_from(U16String::with_capacity(4294967296usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 8589934592 bytes failed
backtrace:
  widestring::ustring::U16String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_utf16string_try_from_widestring__e439c1::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut u16_string = U16String::new();
u16_string.reserve(1000000000000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2000000000000000000 bytes failed
backtrace:
  widestring::ustring::U16String::reserve (widestring-rs/src/ustring.rs:147)
  repro_out_of_memory_in_widestring_u16string_reserve_b8710b::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Utf32String::from(Utf32String::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::utfstring::Utf32String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_utf32string_from_widestring_utf3_26f999::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = U16String::from(U16String::with_capacity(4294967295usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 8589934590 bytes failed
backtrace:
  widestring::ustring::U16String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u16string_from_widestring_u16str_829581::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Utf32String::with_capacity(100000000000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 400000000000000000 bytes failed
backtrace:
  widestring::utfstring::Utf32String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_utf32string_with_capacity_771947::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let utf16_string = Utf16String::with_capacity(2147483648usize);
let _ = utf16_string.as_utfstr();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::utfstring::Utf16String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_utf16string_as_utfstr_178a59::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut u16_string = U16String::from(vec![]);
u16_string.reserve_exact(2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::ustring::U16String::reserve_exact (widestring-rs/src/ustring.rs:158)
  repro_out_of_memory_in_widestring_u16string_reserve_exact_0c03f0::main (src/main.rs:5)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut utf32_string = Utf32String::from(Utf32String::default());
utf32_string.reserve_exact(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::utfstring::Utf32String::reserve_exact (widestring-rs/src/utfstring.rs:180)
  repro_out_of_memory_in_widestring_utf32string_reserve_exact_0be5da::main (src/main.rs:5)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = U32String::from(Utf32String::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::utfstring::Utf32String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_u32string_from_widestring_utf32s_069176::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = U32String::from(U32String::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::ustring::U32String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u32string_from_widestring_u32str_0529b5::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut u32_string = U32String::new();
u32_string.reserve_exact(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::ustring::U32String::reserve_exact (widestring-rs/src/ustring.rs:158)
  repro_out_of_memory_in_widestring_u32string_reserve_exact_5f7bde::main (src/main.rs:5)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let utf32_string = Utf32String::with_capacity(1000000000usize);
let _ = Deref::deref(&amp;utf32_string);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::utfstring::Utf32String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_utf32string_deref_ed6567::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let u16_string = U16String::with_capacity(4294967296usize);
let _ = Deref::deref(&amp;u16_string);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8589934592 bytes failed
backtrace:
  widestring::ustring::U16String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u16string_deref_b6c79e::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut utf16_string = Utf16String::with_capacity(0usize);
utf16_string.reserve(2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::utfstring::Utf16String::reserve (widestring-rs/src/utfstring.rs:167)
  repro_out_of_memory_in_widestring_utf16string_reserve_9b5f92::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let u32_string = U32String::with_capacity(1000000000000000000usize);
let _ = Deref::deref(&amp;u32_string);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000000000000 bytes failed
backtrace:
  widestring::ustring::U32String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u32string_deref_821cde::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let utf16_string = Utf16String::with_capacity(2147483648usize);
let _ = utf16_string.as_ustr();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::utfstring::Utf16String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_utf16string_as_ustr_873a31::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = U16String::from(Utf16String::with_capacity(2147483648usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::utfstring::Utf16String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_u16string_from_widestring_utf16s_04167b::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let utf16_string = Utf16String::with_capacity(4294967295usize);
let _ = Deref::deref(&amp;utf16_string);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8589934590 bytes failed
backtrace:
  widestring::utfstring::Utf16String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_utf16string_deref_32d36b::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Utf16String::from(Utf16String::from_ustring(U16String::with_capacity(
    2147483648usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::ustring::U16String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_utf16string_from_widestring_utf1_b31de7::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = U16String::try_from(U16String::with_capacity(4294967295usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 8589934590 bytes failed
backtrace:
  widestring::ustring::U16String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u16string_try_from_widestring_u1_0fef3c::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let utf32_string = Utf32String::with_capacity(1000000000usize);
let _ = utf32_string.as_ustr();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::utfstring::Utf32String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_utf32string_as_ustr_1f7d74::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = U32String::try_from(U32String::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::ustring::U32String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u32string_try_from_widestring_u3_d91102::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let utf32_string = Utf32String::with_capacity(1000000000usize);
let _ = utf32_string.as_utfstr();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::utfstring::Utf32String::with_capacity (widestring-rs/src/utfstring.rs:88)
  repro_out_of_memory_in_widestring_utf32string_as_utfstr_f6a593::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Utf16String::from_ustring(U16String::with_capacity(2147483648usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  widestring::ustring::U16String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_utf16string_from_ustring_746758::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let u32_string = U32String::with_capacity(1000000000usize);
let _ = u32_string.as_vec();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::ustring::U32String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u32string_as_vec_3480f5::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Utf32String::try_from(U32String::with_capacity(1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::ustring::U32String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_utf32string_try_from_widestring__5056f2::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let u32_string = U32String::with_capacity(1000000000usize);
let _ = u32_string.as_ustr();</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  widestring::ustring::U32String::with_capacity (widestring-rs/src/ustring.rs:124)
  repro_out_of_memory_in_widestring_u32string_as_ustr_6d6ba6::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut u16_string = U16String::new();
u16_string.reserve_exact(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from widestring-rs/src/ustring.rs:158:28)
capacity overflow
backtrace:
  &lt;widestring::ustring::U16String&gt;::reserve_exact (widestring-rs/src/ustring.rs:158:28)
  repro_widestring_rs_src_ustring_rs_158_28_73f08e::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Utf16String::with_capacity(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from widestring-rs/src/utfstring.rs:88:28)
capacity overflow
backtrace:
  &lt;widestring::utfstring::Utf16String&gt;::with_capacity (widestring-rs/src/utfstring.rs:88:28)
  repro_widestring_rs_src_utfstring_rs_88_28_9e12ba::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = U32String::from(U32String::with_capacity(10000000000000000000usize));</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from widestring-rs/src/ustring.rs:124:28)
capacity overflow
backtrace:
  &lt;widestring::ustring::U32String&gt;::with_capacity (widestring-rs/src/ustring.rs:124:28)
  repro_widestring_rs_src_ustring_rs_124_28_20e4f7::main (src/main.rs:4:29)</pre></td></tr>
<tr><td valign="top"><pre>let mut u32_string = U32String::from_chars(vec![]);
u32_string.reserve(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from widestring-rs/src/ustring.rs:147:28)
capacity overflow
backtrace:
  &lt;widestring::ustring::U32String&gt;::reserve (widestring-rs/src/ustring.rs:147:28)
  repro_widestring_rs_src_ustring_rs_147_28_ef7a62::main (src/main.rs:5:16)</pre></td></tr>
</table>
