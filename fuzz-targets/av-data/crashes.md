# av-data - crashes

Source: https://github.com/rust-av/rust-av @ d44bb083ffc543d80397c8244f2e8c534f609308

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Packet::with_capacity(4294967296usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  av_data::packet::Packet::with_capacity (src/packet.rs:32)
  repro_out_of_memory_in_av_data_packet_packet_with_capacity_c74ad3::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Packet::zeroed(2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  av_data::packet::Packet::zeroed (src/packet.rs:44)
  repro_out_of_memory_in_av_data_packet_packet_zeroed_60938b::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut channel_map = ChannelMap::default();
channel_map.add_channels(&amp;[ChannelType::Cs]);
let _ = Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(MediaKind::Audio(
    AudioInfo::new(4294967296usize, 0usize, channel_map.clone(),
    std::sync::Arc::new(S16), None)), None);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8589934592 bytes failed
backtrace:
  bytes::bytes_mut::BytesMut::zeroed (src/bytes_mut.rs:296)
  av_data::frame::DefaultFrameBuffer::new (src/frame.rs:432)
  av_data::frame::Frame::new_default_frame&lt;av_data::frame::MediaKind&gt; (av-data/src/frame.rs:479)
  repro_out_of_memory_in_av_data_frame_frame_new_default_frame_8474e5::main (src/main.rs:11)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>&lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::
    copy_plane_to_buffer(&amp;Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(
    MediaKind::Audio(AudioInfo::new(1000000000usize, 1usize,
    ChannelMap::default_map(1usize), std::sync::Arc::new(S32), None)), None),
    0usize, &amp;mut vec![], 0usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4000000000 bytes failed
backtrace:
  bytes::bytes_mut::BytesMut::zeroed (src/bytes_mut.rs:296)
  av_data::frame::DefaultFrameBuffer::new (src/frame.rs:432)
  av_data::frame::Frame::new_default_frame&lt;av_data::frame::MediaKind&gt; (av-data/src/frame.rs:479)
  repro_out_of_memory_in_av_data_frame_frame_copy_plane_to_buffer_6068fb::main (src/main.rs:8)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let media_kind = MediaKind::Video(VideoInfo::new(1000000000usize, 1usize, false,
    FrameType::OTHER, std::sync::Arc::new(Formaton::new(ColorModel::CMYK,
    &amp;[Chromaton::yuvhb(0u8, 0u8, 1u8, 0u8)], 0u8, false, false, false))));
&lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::
    copy_plane_to_buffer(&amp;Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(
    media_kind.clone(), None), 0usize, &amp;mut vec![66u8], 1usize);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  av_data::frame::copy_plane (src/frame.rs:576)
  av_data::frame::{impl#20}::copy_plane_to_buffer (src/frame.rs:497)
  repro_sigsegv_in_av_data_frame_frame_copy_plane_to_buffer_fe423a::main (src/main.rs:11)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::yuvhb(0u8, 0u8, 1u8, 0u8);
let _ = chromaton.get_data_size(0usize, 0usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:580:5:
attempt to subtract with overflow
backtrace:
  av_data::pixel::align (data/src/pixel.rs:580:5)
  &lt;av_data::pixel::Chromaton&gt;::get_linesize (data/src/pixel.rs:668:9)
  &lt;av_data::pixel::Chromaton&gt;::get_data_size (data/src/pixel.rs:675:14)
  repro_data_src_pixel_rs_580_5_660d6e::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Packet::zeroed(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:51:31)
capacity overflow
backtrace:
  &lt;u8 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:51:31)
  &lt;av_data::packet::Packet&gt;::zeroed (data/src/packet.rs:44:19)
  repro_data_src_packet_rs_44_19_1c8767::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Packet::with_capacity(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from data/src/packet.rs:32:19)
capacity overflow
backtrace:
  &lt;av_data::packet::Packet&gt;::with_capacity (data/src/packet.rs:32:19)
  repro_data_src_packet_rs_32_19_8a8224::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = VideoInfo::new(0usize, 0usize, false, FrameType::B,
    std::sync::Arc::new(Formaton::new(ColorModel::HSV, &amp;[Chromaton::pal8(0u8),
    Chromaton::packrgb(255u8, 0u8, 0u8, 0u8)], 0u8, false, false, false)));</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:778:13:
attempt to add with overflow
backtrace:
  &lt;av_data::pixel::Formaton&gt;::get_total_depth (data/src/pixel.rs:778:13)
  &lt;av_data::frame::VideoInfo&gt;::new (data/src/frame.rs:97:27)
  repro_data_src_pixel_rs_778_13_967bb1::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Formaton::new(ColorModel::Trichromatic(TrichromaticEncodingSystem::RGB),
    &amp;[Chromaton::new(0u8, 0u8, false, 0u8, 0u8, 0u8, 0u8),
    Chromaton::packrgb(0u8, 0u8, 0u8, 0u8), Chromaton::packrgb(0u8, 0u8, 0u8,
    0u8), Chromaton { h_ss: 0u8, v_ss: 0u8, packed: false, depth: 0u8,
    shift: 0u8, comp_offs: 0u8, next_elem: 0u8 }, Chromaton::yuvhb(0u8, 0u8,
    0u8, 0u8), Chromaton::yuv8(0u8, 0u8, 0u8)], 0u8, false, false, false);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:740:13:
too many components
backtrace:
  &lt;av_data::pixel::Formaton&gt;::new (data/src/pixel.rs:740:13)
  repro_data_src_pixel_rs_740_13_a29ca4::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let channel_map = ChannelMap::new();
let _ = channel_map.get_channel(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/audiosample.rs:273:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[av_data::audiosample::ChannelType]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[av_data::audiosample::ChannelType] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;av_data::audiosample::ChannelMap&gt;::get_channel (data/src/audiosample.rs:273:17)
  repro_data_src_audiosample_rs_273_17_f306c7::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>&lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::
    copy_plane_to_buffer(&amp;Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(
    MediaKind::from(AudioInfo::new(0usize, 0usize,
    ChannelMap::default_map(1usize), std::sync::Arc::new(Soniton { bits: 1u8,
    be: false, packed: false, planar: false, float: false, signed: false }),
    None)), None), 1usize, &amp;mut vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/frame.rs:499:13:
not implemented
backtrace:
  &lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::copy_plane_to_buffer (data/src/frame.rs:499:13)
  repro_data_src_frame_rs_499_13_b2267e::main (src/main.rs:8:5)</pre></td></tr>
<tr><td valign="top"><pre>let audio_info = AudioInfo { samples: usize::MAX, sample_rate: 6usize,
    map: ChannelMap::default(),
    format: std::sync::Arc::new(AudioInfo::new(2usize,
    14208578711885967420usize, ChannelMap::new(),
    std::sync::Arc::new(Soniton { bits: 255u8, be: false, packed: true,
    planar: true, float: true, signed: false }), Some(0usize)).get_format()),
    block_len: Some(1usize) };
let _ = audio_info.size(4294967295usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/audiosample.rs:54:27:
attempt to multiply with overflow
backtrace:
  &lt;av_data::audiosample::Soniton&gt;::get_audio_size (data/src/audiosample.rs:54:27)
  &lt;av_data::frame::AudioInfo&gt;::size (data/src/frame.rs:220:21)
  repro_data_src_audiosample_rs_54_27_f6e3a5::main (src/main.rs:7:24)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ChannelMap::default_map(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/audiosample.rs:302:18:
not implemented
backtrace:
  &lt;av_data::audiosample::ChannelMap&gt;::default_map (data/src/audiosample.rs:302:18)
  repro_data_src_audiosample_rs_302_18_b3d1be::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::packrgb(100u8, 0u8, 0u8, 0u8);
let _ = chromaton.get_linesize(0usize, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:580:6:
attempt to add with overflow
backtrace:
  av_data::pixel::align (data/src/pixel.rs:580:6)
  &lt;av_data::pixel::Chromaton&gt;::get_linesize (data/src/pixel.rs:668:9)
  repro_data_src_pixel_rs_580_6_f2715c::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let channel_map = ChannelMap::new();
let usize_v = channel_map.len();
let color_model = ColorModel::LAB;
&lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::
    copy_plane_to_buffer(&amp;Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(
    MediaKind::from(VideoInfo::new(0usize, 0usize, false, FrameType::OTHER,
    std::sync::Arc::new(Formaton::new(color_model, &amp;[], 0u8, false, false,
    false)))), None), 0usize, &amp;mut vec![], usize_v);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/frame.rs:494:60:
called `Result::unwrap()` on an `Err` value: InvalidIndex
backtrace:
  &lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::copy_plane_to_buffer (data/src/frame.rs:494:60)
  repro_data_src_frame_rs_494_60_c4dc08::main (src/main.rs:13:5)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::yuv8(100u8, 0u8, 0u8);
let _ = chromaton.get_linesize(0usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:657:19:
attempt to shift left with overflow
backtrace:
  &lt;av_data::pixel::Chromaton&gt;::get_width (data/src/pixel.rs:657:19)
  &lt;av_data::pixel::Chromaton&gt;::get_linesize (data/src/pixel.rs:668:21)
  repro_data_src_pixel_rs_657_19_6e9df2::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton { h_ss: 0u8, v_ss: 64u8, packed: false, depth: 0u8,
    shift: 0u8, comp_offs: 0u8, next_elem: 0u8 };
let _ = chromaton.get_height(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:662:20:
attempt to shift left with overflow
backtrace:
  &lt;av_data::pixel::Chromaton&gt;::get_height (data/src/pixel.rs:662:20)
  repro_data_src_pixel_rs_662_20_45c67e::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let trichromatic_encoding_system = TrichromaticEncodingSystem::RGB;
let media_kind = MediaKind::Video(VideoInfo { width: 0usize, height: 0usize,
    flipped: false, frame_type: FrameType::B,
    format: std::sync::Arc::new(Formaton::new(ColorModel::Trichromatic(
    trichromatic_encoding_system), &amp;[Chromaton { h_ss: 0u8, v_ss: 100u8,
    packed: false, depth: 0u8, shift: 0u8, comp_offs: 0u8, next_elem: 0u8 }],
    0u8, false, false, false)), bits: 0u8 });
let media_kind2 = MediaKind::from(media_kind.get_video_info().unwrap());
let _ = Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(MediaKind::Video(
    media_kind2.get_video_info().unwrap()), None);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:674:29:
attempt to shift left with overflow
backtrace:
  &lt;av_data::pixel::Chromaton&gt;::get_data_size (data/src/pixel.rs:674:29)
  &lt;av_data::frame::VideoInfo&gt;::size (data/src/frame.rs:143:27)
  &lt;av_data::frame::DefaultFrameBuffer&gt;::new (data/src/frame.rs:412:34)
  &lt;av_data::frame::Frame&gt;::new_default_frame::&lt;av_data::frame::MediaKind&gt; (data/src/frame.rs:479:19)
  repro_data_src_pixel_rs_674_29_c91ed1::main (src/main.rs:14:13)</pre></td></tr>
<tr><td valign="top"><pre>let soniton = F32;
let _ = soniton.get_audio_size(1usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/audiosample.rs:28:20:
attempt to subtract with overflow
backtrace:
  av_data::audiosample::align (data/src/audiosample.rs:28:20)
  &lt;av_data::audiosample::Soniton&gt;::get_audio_size (data/src/audiosample.rs:59:9)
  repro_data_src_audiosample_rs_28_20_961a2f::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let formaton = Formaton::new(ColorModel::CMYK, &amp;[Chromaton::packrgb(0u8, 100u8,
    0u8, 0u8)], 0u8, false, false, false);
let _ = formaton.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:682:24:
attempt to shift left with overflow
backtrace:
  &lt;av_data::pixel::Chromaton as core::fmt::Display&gt;::fmt (data/src/pixel.rs:682:24)
  &lt;&amp;mut alloc::string::String as core::fmt::Write::write_fmt::SpecWriteFmt&gt;::spec_write_fmt (library/core/src/fmt/mod.rs:238:21)
  &lt;av_data::pixel::Formaton as core::fmt::Display&gt;::fmt (data/src/pixel.rs:901:23)
  &lt;av_data::pixel::Formaton as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;av_data::pixel::Formaton as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_data_src_pixel_rs_682_24_2e48e2::main (src/main.rs:7:22)</pre></td></tr>
<tr><td valign="top"><pre>let audio_info = AudioInfo { samples: usize::MAX, sample_rate: 7usize,
    map: ChannelMap::default(), format: std::sync::Arc::new(S16),
    block_len: None };
let _ = audio_info.size(65536usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/audiosample.rs:56:13:
attempt to multiply with overflow
backtrace:
  &lt;av_data::audiosample::Soniton&gt;::get_audio_size (data/src/audiosample.rs:56:13)
  &lt;av_data::frame::AudioInfo&gt;::size (data/src/frame.rs:220:21)
  repro_data_src_audiosample_rs_56_13_e870c5::main (src/main.rs:7:24)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::packrgb(85u8, 0u8, 0u8, 0u8);
let _ = chromaton.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:682:25:
attempt to shift left with overflow
backtrace:
  &lt;av_data::pixel::Chromaton as core::fmt::Display&gt;::fmt (data/src/pixel.rs:682:25)
  &lt;av_data::pixel::Chromaton as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;av_data::pixel::Chromaton as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_data_src_pixel_rs_682_25_b59880::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton { h_ss: 0u8, v_ss: 0u8, packed: false, depth: 0u8,
    shift: 0u8, comp_offs: 0u8, next_elem: 0u8 };
let _ = chromaton.get_linesize(0usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:668:15:
attempt to subtract with overflow
backtrace:
  &lt;av_data::pixel::Chromaton&gt;::get_linesize (data/src/pixel.rs:668:15)
  repro_data_src_pixel_rs_668_15_4f7d62::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let audio_info = AudioInfo::new(1usize, 0usize, ChannelMap::default(),
    std::sync::Arc::new(S32), Some(usize::MAX));
let usize_v = audio_info.get_block_len().unwrap();
let _ = audio_info.size(usize_v);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/audiosample.rs:28:6:
attempt to add with overflow
backtrace:
  av_data::audiosample::align (data/src/audiosample.rs:28:6)
  &lt;av_data::audiosample::Soniton&gt;::get_audio_size (data/src/audiosample.rs:59:9)
  &lt;av_data::frame::AudioInfo&gt;::size (data/src/frame.rs:220:21)
  repro_data_src_audiosample_rs_28_6_be96a8::main (src/main.rs:8:24)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::pal8(0u8);
let _ = chromaton.get_linesize(usize::MAX, 2usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:668:16:
attempt to multiply with overflow
backtrace:
  &lt;av_data::pixel::Chromaton&gt;::get_linesize (data/src/pixel.rs:668:16)
  repro_data_src_pixel_rs_668_16_540f4b::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton { h_ss: 0u8, v_ss: 1u8, packed: false, depth: 1u8,
    shift: 0u8, comp_offs: 1u8, next_elem: 1u8 };
let _ = chromaton.get_data_size(100usize, 1000000000000000000usize,
    1000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:675:9:
attempt to multiply with overflow
backtrace:
  &lt;av_data::pixel::Chromaton&gt;::get_data_size (data/src/pixel.rs:675:9)
  repro_data_src_pixel_rs_675_9_737d0d::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::new(1u8, 1u8, false, 0u8, 0u8, 0u8, 0u8);
let _ = chromaton.get_data_size(usize::MAX, 0usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:657:9:
attempt to add with overflow
backtrace:
  &lt;av_data::pixel::Chromaton&gt;::get_width (data/src/pixel.rs:657:9)
  &lt;av_data::pixel::Chromaton&gt;::get_linesize (data/src/pixel.rs:668:21)
  &lt;av_data::pixel::Chromaton&gt;::get_data_size (data/src/pixel.rs:675:14)
  repro_data_src_pixel_rs_657_9_9a16a4::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::pal8(0u8);
let _ = chromaton.get_linesize(1usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:580:20:
attempt to subtract with overflow
backtrace:
  av_data::pixel::align (data/src/pixel.rs:580:20)
  &lt;av_data::pixel::Chromaton&gt;::get_linesize (data/src/pixel.rs:668:9)
  repro_data_src_pixel_rs_580_20_1a06dc::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let media_kind = MediaKind::Audio(AudioInfo { samples: 0usize,
    sample_rate: 0usize, map: ChannelMap::new(),
    format: std::sync::Arc::new(Soniton { bits: 0u8, be: false, packed: false,
    planar: false, float: false, signed: false }), block_len: None });
let audio_info = media_kind.get_audio_info().unwrap();
let _ = audio_info.size(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/audiosample.rs:28:5:
attempt to subtract with overflow
backtrace:
  av_data::audiosample::align (data/src/audiosample.rs:28:5)
  &lt;av_data::audiosample::Soniton&gt;::get_audio_size (data/src/audiosample.rs:59:9)
  &lt;av_data::frame::AudioInfo&gt;::size (data/src/frame.rs:220:21)
  repro_data_src_audiosample_rs_28_5_bd5dc4::main (src/main.rs:9:24)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::new(0u8, 1u8, false, 0u8, 0u8, 0u8, 0u8);
let _ = chromaton.get_data_size(0usize, usize::MAX, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:674:18:
attempt to add with overflow
backtrace:
  &lt;av_data::pixel::Chromaton&gt;::get_data_size (data/src/pixel.rs:674:18)
  repro_data_src_pixel_rs_674_18_e8a090::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::yuv8(0u8, 1u8, 0u8);
let _ = chromaton.get_height(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/pixel.rs:662:9:
attempt to add with overflow
backtrace:
  &lt;av_data::pixel::Chromaton&gt;::get_height (data/src/pixel.rs:662:9)
  repro_data_src_pixel_rs_662_9_07644c::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(MediaKind::from(
    AudioInfo::new(10000000000000000000usize, 1usize,
    ChannelMap::default_map(2usize), std::sync::Arc::new(U8), None)), None);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/frame.rs:220:9:
attempt to multiply with overflow
backtrace:
  &lt;av_data::frame::AudioInfo&gt;::size (data/src/frame.rs:220:9)
  &lt;av_data::frame::DefaultFrameBuffer&gt;::new (data/src/frame.rs:431:34)
  &lt;av_data::frame::Frame&gt;::new_default_frame::&lt;av_data::frame::MediaKind&gt; (data/src/frame.rs:479:19)
  repro_data_src_frame_rs_220_9_16bc37::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(MediaKind::from(
    AudioInfo::new(usize::MAX, 0usize, ChannelMap::new(),
    std::sync::Arc::new(Soniton::new(1u8, false, true, false, false, false)),
    None)), None);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/audiosample.rs:34:5:
attempt to add with overflow
backtrace:
  av_data::audiosample::round_to_byte (data/src/audiosample.rs:34:5)
  &lt;av_data::audiosample::Soniton&gt;::get_audio_size (data/src/audiosample.rs:54:13)
  &lt;av_data::frame::AudioInfo&gt;::size (data/src/frame.rs:220:21)
  &lt;av_data::frame::DefaultFrameBuffer&gt;::new (data/src/frame.rs:431:34)
  &lt;av_data::frame::Frame&gt;::new_default_frame::&lt;av_data::frame::MediaKind&gt; (data/src/frame.rs:479:19)
  repro_data_src_audiosample_rs_34_5_7156cb::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let chromaton = Chromaton::pal8(1u8);
let channel_map = ChannelMap::new();
let bool_v = channel_map.is_empty();
let video_info = VideoInfo::from(VideoInfo { width: 0usize, height: 0usize,
    flipped: false, frame_type: FrameType::P,
    format: std::sync::Arc::new(Formaton::new(ColorModel::CMYK, &amp;[chromaton],
    0u8, false, false, bool_v)), bits: 0u8 });
&lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::
    copy_plane_to_buffer(&amp;Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(
    MediaKind::Video(video_info.clone()), None), 0usize, &amp;mut vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/frame.rs:569:26:
chunk size must be non-zero
backtrace:
  &lt;[u8]&gt;::chunks_mut (core/src/slice/mod.rs:1199:9)
  av_data::frame::copy_plane (data/src/frame.rs:569:26)
  &lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::copy_plane_to_buffer (data/src/frame.rs:497:13)
  repro_data_src_frame_rs_569_26_e4809a::main (src/main.rs:15:5)</pre></td></tr>
<tr><td valign="top"><pre>let media_kind = MediaKind::from(VideoInfo { width: 1usize,
    height: 531707584965882534usize, flipped: false, frame_type: FrameType::P,
    format: std::sync::Arc::new(Formaton::new(ColorModel::CMYK,
    &amp;[Chromaton::pal8(0u8), Chromaton::packrgb(10u8, 0u8, 0u8, 0u8)], 0u8,
    false, false, false)), bits: 0u8 });
let _ = Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(
    media_kind.clone(), None);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/frame.rs:143:17:
attempt to add with overflow
backtrace:
  &lt;av_data::frame::VideoInfo&gt;::size (data/src/frame.rs:143:17)
  &lt;av_data::frame::DefaultFrameBuffer&gt;::new (data/src/frame.rs:412:34)
  &lt;av_data::frame::Frame&gt;::new_default_frame::&lt;av_data::frame::MediaKind&gt; (data/src/frame.rs:479:19)
  repro_data_src_frame_rs_143_17_5e03a9::main (src/main.rs:11:13)</pre></td></tr>
<tr><td valign="top"><pre>let video_info = VideoInfo { width: 0usize, height: 0usize, flipped: false,
    frame_type: FrameType::OTHER,
    format: std::sync::Arc::new(Formaton { model: ColorModel::LAB,
    primaries: ColorPrimaries::BT2020, xfer: TransferCharacteristic::BT1361E,
    matrix: MatrixCoefficients::BT709, chroma_location: ChromaLocation::Center,
    components: 0u8, comp_info: [None, None, None, None,
    Some(Chromaton::pal8(1u8))], elem_size: 0u8, be: false, alpha: false,
    palette: false }), bits: 0u8 };
&lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::
    copy_plane_to_buffer(&amp;Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(
    MediaKind::Video(video_info.clone()), None), 0usize, &amp;mut vec![], 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at data/src/frame.rs:570:26:
chunk size must be non-zero
backtrace:
  &lt;[u8]&gt;::chunks (core/src/slice/mod.rs:1155:9)
  av_data::frame::copy_plane (data/src/frame.rs:570:26)
  &lt;av_data::frame::Frame as av_data::frame::FrameBufferCopy&gt;::copy_plane_to_buffer (data/src/frame.rs:497:13)
  repro_data_src_frame_rs_570_26_a58274::main (src/main.rs:15:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut channel_map = ChannelMap::new();
channel_map.add_channel(ChannelType::Cs);
let _ = Frame::new_default_frame::&lt;av_data::frame::MediaKind&gt;(MediaKind::from(
    AudioInfo { samples: 8341552203014512160usize, sample_rate: 0usize,
    map: channel_map.clone(), format: std::sync::Arc::new(S16),
    block_len: None }), None);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:51:31)
capacity overflow
backtrace:
  &lt;u8 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:51:31)
  &lt;bytes::bytes_mut::BytesMut&gt;::zeroed (bytes/src/bytes_mut.rs:296:28)
  &lt;av_data::frame::DefaultFrameBuffer&gt;::new (av-data/src/frame.rs:432:27)
  &lt;av_data::frame::Frame&gt;::new_default_frame::&lt;av_data::frame::MediaKind&gt; (av-data/src/frame.rs:479:19)
  repro_bytes_src_bytes_mut_rs_296_28_4be6bb::main (src/main.rs:11:13)</pre></td></tr>
</table>
