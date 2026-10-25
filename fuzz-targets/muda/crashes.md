# muda - crashes

Source: https://github.com/tauri-apps/muda @ 33dfdf7c6daceed1dddd73b658e0e4c912685f36

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Icon::from_rgba(vec![], 10000u32, 1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at muda/src/icon.rs:89:31:
attempt to multiply with overflow
backtrace:
  &lt;muda::icon::RgbaIcon&gt;::from_rgba (muda/src/icon.rs:89:31)
  &lt;muda::icon::Icon&gt;::from_rgba (muda/src/icon.rs:137:24)
  repro_muda_src_icon_rs_89_31_56c99a::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Icon::from_rgba(vec![], u32::MAX, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at muda/src/platform_impl/gtk4/icon.rs:35:13:
attempt to multiply with overflow
backtrace:
  &lt;muda::platform_impl::platform::icon::PlatformIcon&gt;::from_rgba (muda/src/platform_impl/gtk4/icon.rs:35:13)
  &lt;muda::icon::Icon&gt;::from_rgba (muda/src/icon.rs:139:24)
  repro_muda_src_platform_impl_gtk4_icon_rs_35_13_79abcd::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Icon::from_rgba(vec![], 0u32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at gdk-pixbuf/src/auto/pixbuf.rs:9:1:
assertion failed: !ptr.is_null()
backtrace:
  &lt;gdk_pixbuf::auto::pixbuf::Pixbuf as glib::translate::FromGlibPtrFull&lt;*mut gdk_pixbuf_sys::GdkPixbuf&gt;&gt;::from_glib_full (glib/src/object.rs:939:17)
  glib::translate::from_glib_full::&lt;*mut gdk_pixbuf_sys::GdkPixbuf, gdk_pixbuf::auto::pixbuf::Pixbuf&gt; (glib/src/translate.rs:1637:14)
  &lt;gdk_pixbuf::auto::pixbuf::Pixbuf&gt;::from_bytes (gdk-pixbuf/src/auto/pixbuf.rs:50:13)
  &lt;muda::platform_impl::platform::icon::PlatformIcon&gt;::from_rgba (muda/src/platform_impl/gtk4/icon.rs:28:22)
  &lt;muda::icon::Icon&gt;::from_rgba (muda/src/icon.rs:139:24)
  repro_gdk_pixbuf_src_auto_pixbuf_rs_9_1_593b57::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;muda::Menu as muda::ContextMenu&gt;::gtk_context_menu(&amp;Menu::new());</pre></td><td valign="top"><pre>thread 'main' panicked at gtk4/src/auto/popover_menu.rs:59:9:
GTK has not been initialized. Call `gtk::init` first.
backtrace:
  &lt;gtk4::auto::popover_menu::PopoverMenu&gt;::from_model_full::&lt;gio::auto::menu::Menu&gt; (gtk4/src/rt.rs:32:17)
  &lt;muda::platform_impl::platform::GtkMenuBar&gt;::new_context (muda/src/platform_impl/gtk4/mod.rs:77:22)
  &lt;muda::platform_impl::platform::PlatformMenu&gt;::ensure_context_menu (muda/src/platform_impl/gtk4/mod.rs:365:20)
  &lt;muda::platform_impl::platform::PlatformMenu&gt;::gtk_context_menu (muda/src/platform_impl/gtk4/mod.rs:352:14)
  &lt;muda::items::menu::Menu as muda::context_menu::ContextMenu&gt;::gtk_context_menu (muda/src/items/menu.rs:633:36)
  repro_gtk4_src_auto_popover_menu_rs_59_9_d91926::main (src/main.rs:4:13)</pre></td></tr>
</table>
