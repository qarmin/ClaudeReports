# czkawka_core - crashes

Source: https://github.com/qarmin/czkawka @ eb8b91dbb4d25dff416202674ee0de84ee4b5e5c

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let u8_v = DEFAULT_THUMBNAIL_GRID_TILES_PER_SIDE;
let _ = VideoCropParams::with_custom_params(VideoCroppingMechanism::BlackBars,
    255u8, 0u8, 0usize, 0u32, false, 0u8, false, u8_v);</pre></td><td valign="top"><pre>thread 'main' panicked at czkawka_core/src/tools/video_optimizer/mod.rs:166:9:
black_pixel_threshold must be 0-128, got 255
backtrace:
  &lt;czkawka_core::tools::video_optimizer::VideoCropParams&gt;::with_custom_params (czkawka_core/src/tools/video_optimizer/mod.rs:166:9)
  repro_czkawka_core_src_tools_video_optimizer_mod_rs_166_9_cdd9c4::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = czkawka_core::common::config_cache_path::get_config_cache_path();</pre></td><td valign="top"><pre>thread 'main' panicked at czkawka_core/src/common/config_cache_path.rs:20:29:
Cannot fail if set_config_cache_path was called before
backtrace:
  czkawka_core::common::config_cache_path::get_config_cache_path (czkawka_core/src/common/config_cache_path.rs:20:29)
  repro_czkawka_core_src_common_config_cache_path_rs_20_29_8cf25d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = VideoCropParams::with_custom_params(VideoCroppingMechanism::BlackBars,
    0u8, 0u8, 0usize, 0u32, false, 0u8, false, 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at czkawka_core/src/tools/video_optimizer/mod.rs:167:9:
black_bar_min_percentage must be 50-100, got 0
backtrace:
  &lt;czkawka_core::tools::video_optimizer::VideoCropParams&gt;::with_custom_params (czkawka_core/src/tools/video_optimizer/mod.rs:167:9)
  repro_czkawka_core_src_tools_video_optimizer_mod_rs_167_9_f54a95::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = SimilarVideosParameters::new(1000000000i32, false, false, 0u32, 0u32,
    false, 0u32, 0.0f64, 0.0f64, 0.0f64, false, 0u8, false, 0u8, false, 0.0f64,
    0.0f64, 1.0f64, 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at czkawka_core/src/tools/similar_videos/mod.rs:182:9:
assertion failed: (0..=MAX_TOLERANCE).contains(&amp;tolerance)
backtrace:
  &lt;czkawka_core::tools::similar_videos::SimilarVideosParameters&gt;::new (czkawka_core/src/tools/similar_videos/mod.rs:182:9)
  repro_czkawka_core_src_tools_similar_videos_mod_rs_182_9_ea7b63::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;czkawka_core::tools::duplicate::DuplicateFinder as czkawka_core::
    common::traits::PrintResults&gt;::print_results_to_writer::&lt;Vec&lt;u8&gt;&gt;(
    &amp;DuplicateFinder::new(DuplicateFinderParameters::new(
    CheckingMethod::VideoAudioContent, HashType::default(), false, 0u64, 0u64,
    false)), &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at czkawka_core/src/tools/duplicate/traits.rs:347:18:
explicit panic
backtrace:
  &lt;czkawka_core::tools::duplicate::DuplicateFinder as czkawka_core::common::traits::PrintResults&gt;::write_results::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (czkawka_core/src/tools/duplicate/traits.rs:347:18)
  &lt;czkawka_core::tools::duplicate::DuplicateFinder as czkawka_core::common::traits::PrintResults&gt;::print_results_to_writer::&lt;alloc::vec::Vec&lt;u8&gt;&gt;::{closure#0} (czkawka_core/src/common/traits.rs:78:14)
  &lt;czkawka_core::tools::duplicate::DuplicateFinder as czkawka_core::common::traits::PrintResults&gt;::print_results_to_writer::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (czkawka_core/src/common/traits.rs:76:5)
  repro_czkawka_core_src_tools_duplicate_traits_rs_347_18_fb2e8b::main (src/main.rs:7:13)</pre></td></tr>
</table>
