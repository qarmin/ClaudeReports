# wgpu - crashes

Source: https://github.com/gfx-rs/wgpu @ efcf445784dd31537f09bc61bad27431de713fce

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let arena = &lt;naga::Arena&lt;naga::diagnostic_filter::DiagnosticFilterNode&gt;&gt;::new();
let _ = arena.range_from(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at naga/src/arena/range.rs:132:9:
assertion failed: inner.start &lt;= inner.end
backtrace:
  &lt;naga::arena::range::Range&lt;naga::diagnostic_filter::DiagnosticFilterNode&gt;&gt;::from_index_range (naga/src/arena/range.rs:132:9)
  &lt;naga::arena::Arena&lt;naga::diagnostic_filter::DiagnosticFilterNode&gt;&gt;::range_from (naga/src/arena/mod.rs:191:9)
  repro_naga_src_arena_range_rs_132_9_669e2c::main (src/main.rs:3:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = TypeInner::Struct { members: vec![],
    span: 1u32 }.component_type(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at naga/src/proc/type_methods.rs:460:79:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[naga::ir::StructMember]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[naga::ir::StructMember] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;naga::ir::TypeInner&gt;::component_type (naga/src/proc/type_methods.rs:460:79)
  repro_naga_src_proc_type_methods_rs_460_79_3ca6e8::main (src/main.rs:4:63)</pre></td></tr>
<tr><td valign="top"><pre>let _ = wgpu_hal::vulkan::conv::map_surface_color_space(
    SurfaceColorSpace::Auto);</pre></td><td valign="top"><pre>thread 'main' panicked at wgpu-hal/src/vulkan/conv.rs:199:22:
internal error: entered unreachable code: wgpu-core resolves `Auto` before configuring the surface
backtrace:
  wgpu_hal::vulkan::conv::map_surface_color_space (wgpu-hal/src/vulkan/conv.rs:199:22)
  repro_wgpu_hal_src_vulkan_conv_rs_199_22_ffaddf::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = wgpu_hal::vulkan::conv::map_present_mode(PresentMode::AutoVsync);</pre></td><td valign="top"><pre>thread 'main' panicked at wgpu-hal/src/vulkan/conv.rs:547:13:
internal error: entered unreachable code: Cannot create swapchain with Auto PresentationMode
backtrace:
  wgpu_hal::vulkan::conv::map_present_mode (wgpu-hal/src/vulkan/conv.rs:547:13)
  repro_wgpu_hal_src_vulkan_conv_rs_547_13_f430cf::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = FormatAspects::all().map();</pre></td><td valign="top"><pre>thread 'main' panicked at wgpu-hal/src/lib.rs:2086:18:
internal error: entered unreachable code
backtrace:
  &lt;wgpu_hal::FormatAspects&gt;::map (wgpu-hal/src/lib.rs:2086:18)
  repro_wgpu_hal_src_lib_rs_2086_18_55f664::main (src/main.rs:4:34)</pre></td></tr>
<tr><td valign="top"><pre>let module_info = ModuleInfo::default();
let _ = module_info.get_entry_point(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at naga/src/valid/analyzer.rs:1320:27:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[naga::valid::analyzer::FunctionInfo]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[naga::valid::analyzer::FunctionInfo] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;naga::valid::ModuleInfo&gt;::get_entry_point (naga/src/valid/analyzer.rs:1320:27)
  repro_naga_src_valid_analyzer_rs_1320_27_99391a::main (src/main.rs:5:25)</pre></td></tr>
</table>
