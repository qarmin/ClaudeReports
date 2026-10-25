# gdk4 - crashes

Source: https://github.com/gtk-rs/gtk4-rs @ 472c2f879e01d7e675b80f29e753fddb374bb5ac

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Paintable::new_empty(-1000000000i32, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at gdk4/src/auto/paintable.rs:14:1:
assertion failed: !ptr.is_null()
backtrace:
  &lt;gdk4::auto::paintable::Paintable as glib::translate::FromGlibPtrFull&lt;*mut gdk4_sys::GdkPaintable&gt;&gt;::from_glib_full (glib/src/object.rs:939:17)
  glib::translate::from_glib_full::&lt;*mut gdk4_sys::GdkPaintable, gdk4::auto::paintable::Paintable&gt; (glib/src/translate.rs:1637:14)
  &lt;gdk4::auto::paintable::Paintable&gt;::new_empty (gdk4/src/auto/paintable.rs:30:13)
  repro_gdk4_src_auto_paintable_rs_14_1_b09b51::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let cursor_builder = Cursor::builder();
let cursor_builder2 = cursor_builder.hotspot_y(-1000000000i32);
let _ = cursor_builder2.build();</pre></td><td valign="top"><pre>thread 'main' panicked at gdk4/src/auto/cursor.rs:142:22:
property 'hotspot-y' of type 'GdkCursor' can't be set from given value, it is invalid or out of range
backtrace:
  glib::object::validate_property_type (glib/src/object.rs:3311:13)
  &lt;glib::object::Object&gt;::new_internal (glib/src/object.rs:1514:21)
  &lt;glib::object::Object&gt;::with_mut_values (glib/src/object.rs:1454:18)
  &lt;glib::object::ObjectBuilder&lt;gdk4::auto::cursor::Cursor&gt;&gt;::build (glib/src/object.rs:1666:22)
  &lt;gdk4::auto::cursor::CursorBuilder&gt;::build (gdk4/src/auto/cursor.rs:142:22)
  repro_gdk4_src_auto_cursor_rs_142_22_0b376a::main (src/main.rs:6:29)</pre></td></tr>
<tr><td valign="top"><pre>let color_state = ColorState::oklab();
let _ = color_state.equivalent(&amp;ColorState::oklch());</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  gdk_color_state_equivalent (/usr/lib/x86_64-linux-gnu/libgtk-4.so.1:/usr/lib/x86_64-linux-gnu/libgtk-4.so.1)
  gdk4::auto::color_state::ColorState::equivalent (src/auto/color_state.rs:44)
  repro_sigsegv_in_gdk4_colorstate_equivalent_bb513e::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Texture::from_resource("");</pre></td><td valign="top"><pre>abort: Program received signal SIGTRAP, Trace/breakpoint trap
backtrace:
  ??? (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0:/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0)
  g_log_writer_default (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0:/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0)
  g_log_structured_array (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0:/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0)
  g_log_structured_standard (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0:/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0)
  gdk_texture_new_from_resource (/usr/lib/x86_64-linux-gnu/libgtk-4.so.1:/usr/lib/x86_64-linux-gnu/libgtk-4.so.1)
  gdk4::auto::texture::Texture::from_resource (src/auto/texture.rs:91)
  repro_abort_in_gdk4_texture_from_resource_c1beab::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let _ = DisplayManager::get();</pre></td><td valign="top"><pre>abort: Program received signal SIGTRAP, Trace/breakpoint trap
backtrace:
  ??? (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0:/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0)
  g_log_writer_default (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0:/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0)
  g_log_structured_array (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0:/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0)
  g_log_structured_standard (/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0:/usr/lib/x86_64-linux-gnu/libglib-2.0.so.0)
  gdk_display_manager_get (/usr/lib/x86_64-linux-gnu/libgtk-4.so.1:/usr/lib/x86_64-linux-gnu/libgtk-4.so.1)
  gdk4::auto::display_manager::DisplayManager::get (src/auto/display_manager.rs:68)
  repro_abort_in_gdk4_displaymanager_get_4aa5f9::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let dmabuf_texture_builder = DmabufTextureBuilder::new();
let dmabuf_texture_builder2 = dmabuf_texture_builder.clone().set_stride(0u32,
    0u32);
let dmabuf_texture_builder3 = dmabuf_texture_builder2.clone().set_stride(0u32,
    1u32);
let _ = dmabuf_texture_builder3.display();</pre></td><td valign="top"><pre>thread 'main' panicked at gdk4/src/auto/display.rs:23:1:
assertion failed: !ptr.is_null()
backtrace:
  &lt;gdk4::auto::display::Display as glib::translate::FromGlibPtrNone&lt;*mut gdk4_sys::GdkDisplay&gt;&gt;::from_glib_none (glib/src/object.rs:911:17)
  glib::translate::from_glib_none::&lt;*mut gdk4_sys::GdkDisplay, gdk4::auto::display::Display&gt; (glib/src/translate.rs:1628:14)
  &lt;gdk4::auto::dmabuf_texture_builder::DmabufTextureBuilder&gt;::display (gdk4/src/auto/dmabuf_texture_builder.rs:44:13)
  repro_gdk4_src_auto_display_rs_23_1_eb3cce::main (src/main.rs:7:37)</pre></td></tr>
<tr><td valign="top"><pre>let memory_texture_builder = MemoryTextureBuilder::new();
let _ = memory_texture_builder.build();</pre></td><td valign="top"><pre>thread 'main' panicked at gdk4/src/auto/texture.rs:15:1:
assertion failed: !ptr.is_null()
backtrace:
  &lt;gdk4::auto::texture::Texture as glib::translate::FromGlibPtrFull&lt;*mut gdk4_sys::GdkTexture&gt;&gt;::from_glib_full (glib/src/object.rs:939:17)
  glib::translate::from_glib_full::&lt;*mut gdk4_sys::GdkTexture, gdk4::auto::texture::Texture&gt; (glib/src/translate.rs:1637:14)
  &lt;gdk4::auto::memory_texture_builder::MemoryTextureBuilder&gt;::build (gdk4/src/auto/memory_texture_builder.rs:26:18)
  repro_gdk4_src_auto_texture_rs_15_1_a2a2b1::main (src/main.rs:5:36)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Texture::from_resource("\0");</pre></td><td valign="top"><pre>thread 'main' panicked at glib/src/translate.rs:564:56:
called `Result::unwrap()` on an `Err` value: GStrInteriorNulError(0)
backtrace:
  &lt;str as glib::translate::ToGlibPtr&lt;*const i8&gt;&gt;::to_glib_none (glib/src/translate.rs:564:56)
  &lt;gdk4::auto::texture::Texture&gt;::from_resource (gdk4/src/auto/texture.rs:92:31)
  repro_glib_src_translate_rs_564_56_ed2e26::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;gdk4::RGBA as ::core::str::FromStr&gt;::from_str("&lt;svg \
    xmlns=\"http://www.w3.org/2000/svg\" width=\"NaN\" height=\"NaN\" \
    viewBox=\"0 0 114 0\"&gt;&lt;linearGradient" /* ... 285 more chars */);</pre></td><td valign="top"><pre>thread 'main' panicked at glib/src/gstring.rs:1954:42:
called `Result::unwrap()` on an `Err` value: GStrInteriorNulError(274)
backtrace:
  &lt;glib::gstring::GString as core::convert::From&lt;&amp;str&gt;&gt;::from (glib/src/gstring.rs:1954:42)
  &lt;&amp;str as glib::gstring::IntoGStr&gt;::run_with_gstr::&lt;core::result::Result&lt;gdk4::auto::rgba::RGBA, glib::error::BoolError&gt;, &lt;gdk4::auto::rgba::RGBA&gt;::parse&lt;&amp;str&gt;::{closure#0}&gt; (glib/src/gstring.rs:2516:15)
  &lt;gdk4::auto::rgba::RGBA&gt;::parse::&lt;&amp;str&gt; (gdk4/src/rgba.rs:205:15)
  &lt;gdk4::auto::rgba::RGBA as core::str::traits::FromStr&gt;::from_str (gdk4/src/rgba.rs:244:9)
  repro_glib_src_gstring_rs_1954_42_18290a::main (src/main.rs:2:13)</pre></td></tr>
</table>
