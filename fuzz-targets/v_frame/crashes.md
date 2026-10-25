# v_frame - crashes

Source: https://github.com/rust-av/v_frame @ d8643c808d96f0cbe30d7f03c0f76a7936a2fe19

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let frame_builder = FrameBuilder::new(4294967295usize, 4294967295usize,
    ChromaSubsampling::Monochrome, 10u8);
let _ = frame_builder.build::&lt;u16&gt;();</pre></td><td valign="top"><pre>thread 'main' panicked at v_frame/src/plane/aligned.rs:48:14:
allocation size does not overflow usize
backtrace:
  &lt;v_frame::plane::aligned::AlignedData&lt;u16&gt;&gt;::layout (v_frame/src/plane/aligned.rs:48:14)
  &lt;v_frame::plane::aligned::AlignedData&lt;u16&gt;&gt;::new (v_frame/src/plane/aligned.rs:99:22)
  &lt;v_frame::plane::Plane&lt;u16&gt;&gt;::new (v_frame/src/plane.rs:220:19)
  &lt;v_frame::frame::FrameBuilder&gt;::build::&lt;u16&gt; (v_frame/src/frame.rs:281:22)
  repro_v_frame_src_plane_aligned_rs_48_14_e2d10f::main (src/main.rs:6:27)</pre></td></tr>
<tr><td valign="top"><pre>let plane_geometry = PlaneGeometry::new(1usize, 1usize, 0usize, 0usize,
    usize::MAX, 0usize, 1u8, 1u8).unwrap();
let _ = plane_geometry.alloc_height();</pre></td><td valign="top"><pre>thread 'main' panicked at v_frame/src/plane/geometry.rs:218:9:
attempt to add with overflow
backtrace:
  &lt;v_frame::plane::geometry::PlaneGeometry&gt;::alloc_height (v_frame/src/plane/geometry.rs:218:9)
  repro_v_frame_src_plane_geometry_rs_218_9_6313cc::main (src/main.rs:5:28)</pre></td></tr>
<tr><td valign="top"><pre>let frame_builder = FrameBuilder::new(100usize, 10000000000000000000usize,
    ChromaSubsampling::Yuv444, 16u8);
let _ = frame_builder.build::&lt;u16&gt;();</pre></td><td valign="top"><pre>thread 'main' panicked at v_frame/src/plane/geometry.rs:225:9:
attempt to multiply with overflow
backtrace:
  &lt;v_frame::plane::geometry::PlaneGeometry&gt;::alloc_size (v_frame/src/plane/geometry.rs:225:9)
  &lt;v_frame::plane::Plane&lt;u16&gt;&gt;::new (v_frame/src/plane.rs:217:31)
  &lt;v_frame::frame::FrameBuilder&gt;::build::&lt;u16&gt; (v_frame/src/frame.rs:275:34)
  repro_v_frame_src_plane_geometry_rs_225_9_5176ca::main (src/main.rs:6:27)</pre></td></tr>
<tr><td valign="top"><pre>let frame_builder = FrameBuilder::new(1usize, 1000000000usize,
    ChromaSubsampling::Yuv444, 8u8);
let _ = frame_builder.build::&lt;u8&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000 bytes failed
backtrace:
  v_frame::plane::aligned::AlignedData&lt;u8&gt;::new&lt;u8&gt; (v_frame/src/plane/aligned.rs:107)
  v_frame::plane::Plane&lt;u8&gt;::new&lt;u8&gt; (v_frame/src/plane.rs:220)
  v_frame::frame::FrameBuilder::build&lt;u8&gt; (v_frame/src/frame.rs:281)
  repro_out_of_memory_in_v_frame_frame_framebuilder_build_u8_ded5fe::main (src/main.rs:6)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let chroma_subsampling = ChromaSubsampling::Yuv422;
let frame_builder = FrameBuilder::new(1000000000usize, 1usize,
    chroma_subsampling, 10u8);
let frame_builder2 = frame_builder.luma_padding_top(0usize);
let _ = frame_builder2.build::&lt;u16&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2000000000 bytes failed
backtrace:
  v_frame::plane::aligned::AlignedData&lt;u16&gt;::new&lt;u16&gt; (v_frame/src/plane/aligned.rs:107)
  v_frame::plane::Plane&lt;u16&gt;::new&lt;u16&gt; (v_frame/src/plane.rs:220)
  v_frame::frame::FrameBuilder::build&lt;u16&gt; (v_frame/src/frame.rs:281)
  repro_out_of_memory_in_v_frame_frame_framebuilder_build_u16_31d806::main (src/main.rs:8)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let plane_geometry = PlaneGeometry::new(1usize, 1usize, 0usize,
    9499363980073713412usize, 10usize, 0usize, 1u8, 1u8).unwrap();
let _ = plane_geometry.data_origin();</pre></td><td valign="top"><pre>thread 'main' panicked at v_frame/src/plane/geometry.rs:211:9:
attempt to multiply with overflow
backtrace:
  &lt;v_frame::plane::geometry::PlaneGeometry&gt;::data_origin (v_frame/src/plane/geometry.rs:211:9)
  repro_v_frame_src_plane_geometry_rs_211_9_86fbdf::main (src/main.rs:5:28)</pre></td></tr>
<tr><td valign="top"><pre>let frame = FrameBuilder::new(10usize, 1000000000usize,
    ChromaSubsampling::Yuv422, 10u8).build::&lt;u16&gt;().unwrap();
let _ = frame.plane(1usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 10000000000 bytes failed
backtrace:
  v_frame::plane::aligned::AlignedData&lt;u16&gt;::new&lt;u16&gt; (v_frame/src/plane/aligned.rs:107)
  v_frame::plane::Plane&lt;u16&gt;::new&lt;u16&gt; (v_frame/src/plane.rs:220)
  v_frame::frame::FrameBuilder::build&lt;u16&gt; (v_frame/src/frame.rs:275)
  repro_out_of_memory_in_v_frame_frame_frame_u16_plane_ec6699::main (src/main.rs:5)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let chroma_subsampling = ChromaSubsampling::Yuv420;
let frame_builder = FrameBuilder::new(16usize, 1440161638835863816usize,
    chroma_subsampling, 10u8);
let frame_builder2 = frame_builder.luma_padding_right(0usize);
let _ = frame_builder2.build::&lt;u16&gt;();</pre></td><td valign="top"><pre>thread 'main' panicked at v_frame/src/plane/aligned.rs:52:18:
invalid layout
backtrace:
  &lt;v_frame::plane::aligned::AlignedData&lt;u16&gt;&gt;::layout (v_frame/src/plane/aligned.rs:52:18)
  &lt;v_frame::plane::aligned::AlignedData&lt;u16&gt;&gt;::new (v_frame/src/plane/aligned.rs:99:22)
  &lt;v_frame::plane::Plane&lt;u16&gt;&gt;::new (v_frame/src/plane.rs:220:19)
  &lt;v_frame::frame::FrameBuilder&gt;::build::&lt;u16&gt; (v_frame/src/frame.rs:275:34)
  repro_v_frame_src_plane_aligned_rs_52_18_2749d9::main (src/main.rs:8:28)</pre></td></tr>
<tr><td valign="top"><pre>let chroma_subsampling = ChromaSubsampling::Yuv444;
let frame_builder = FrameBuilder::new(1000000000usize, 1usize,
    chroma_subsampling, 8u8);
let frame = frame_builder.build::&lt;u8&gt;().unwrap();
let _ = frame.plane(1usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000 bytes failed
backtrace:
  v_frame::plane::aligned::AlignedData&lt;u8&gt;::new&lt;u8&gt; (v_frame/src/plane/aligned.rs:107)
  v_frame::plane::Plane&lt;u8&gt;::new&lt;u8&gt; (v_frame/src/plane.rs:220)
  v_frame::frame::FrameBuilder::build&lt;u8&gt; (v_frame/src/frame.rs:281)
  repro_out_of_memory_in_v_frame_frame_frame_u8_plane_6427ce::main (src/main.rs:7)
  (13 std/libc frames omitted)</pre></td></tr>
</table>
