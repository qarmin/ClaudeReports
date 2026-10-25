# alsa - crashes

Source: https://github.com/diwic/alsa-rs @ ffc02b01730d2f78d2fdebd26e8ef492bae9f276

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = alsa::poll::poll(&amp;mut vec![], -1000000000i32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  alsa::poll::poll (src/poll.rs:45)
  repro_timeout_in_alsa_poll_poll_03cbeb::main (src/main.rs:2)
  (3 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Output::local_error_handler();
let _ = Mixer::new("(\\)".repeat(82) + "(\\", false);</pre></td><td valign="top"><pre>abort: Program received signal SIGABRT, Aborted
backtrace:
  __libc_message_wrapper (../include/stdio.h:203)
  alsa::io::our_error_handler::{closure#0} (src/io.rs:87)
  alsa::io::our_error_handler (src/io.rs:83)
  alsa::mixer::Mixer::attach (src/error.rs:27)
  alsa::mixer::Mixer::new (src/mixer.rs:27)
  repro_abort_in_alsa_mixer_new_8b7d73::main (src/main.rs:6)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let format = Format::dsd_u32();
let _ = format.silence_16();</pre></td><td valign="top"><pre>abort: Program received signal SIGABRT, Aborted
backtrace:
  __libc_message_wrapper (../include/stdio.h:203)
  alsa::pcm::Format::silence_16 (src/pcm.rs:809)
  repro_abort_in_alsa_pcm_format_silence_16_e66391::main (src/main.rs:5)
  (8 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Output::local_error_handler();
let _ = Ctl::new("a\u{301}\u{301}\u{301}".repeat(35), false);</pre></td><td valign="top"><pre>abort: Program received signal SIGABRT, Aborted
backtrace:
  __libc_message_wrapper (../include/stdio.h:203)
  alsa::io::our_error_handler::{closure#0} (src/io.rs:87)
  alsa::io::our_error_handler (src/io.rs:83)
  alsa::ctl_int::Ctl::open (src/error.rs:27)
  alsa::ctl_int::Ctl::new (src/ctl_int.rs:94)
  repro_abort_in_alsa_ctl_new_700101::main (src/main.rs:6)
  (15 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Output::local_error_handler();
let _ = Ump::new(
    "\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{\
    feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff\
    }\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{\
    feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff\
    }\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{\
    feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff\
    }\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{\
    feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff\
    }\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}", Direction::Capture, false);</pre></td><td valign="top"><pre>abort: Program received signal SIGABRT, Aborted
backtrace:
  __libc_message_wrapper (../include/stdio.h:203)
  alsa::io::our_error_handler::{closure#0} (src/io.rs:87)
  alsa::io::our_error_handler (src/io.rs:83)
  alsa::ump::Ump::open (src/error.rs:27)
  alsa::ump::Ump::new (src/ump.rs:39)
  repro_abort_in_alsa_ump_new_2fd1f7::main (src/main.rs:7)
  (16 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Mixer::new("\0", false);</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/mixer.rs:27:42:
called `Result::unwrap()` on an `Err` value: NulError(0, [0])
backtrace:
  &lt;alsa::mixer::Mixer&gt;::new (alsa-rs/src/mixer.rs:27:42)
  repro_alsa_rs_src_mixer_rs_27_42_26ddf6::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rawmidi::new("\0", Direction::output(), false);</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/rawmidi.rs:229:40:
called `Result::unwrap()` on an `Err` value: NulError(0, [0])
backtrace:
  &lt;alsa::rawmidi::Rawmidi&gt;::new (alsa-rs/src/rawmidi.rs:229:40)
  repro_alsa_rs_src_rawmidi_rs_229_40_36cead::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;alsa::mixer::MilliBel as ::core::ops::Sub&lt;alsa::mixer::MilliBel&gt;&gt;::sub(
    MilliBel(i64::MIN), MilliBel::from_db(1.0f32));</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/mixer.rs:123:48:
attempt to subtract with overflow
backtrace:
  &lt;alsa::mixer::MilliBel as core::ops::arith::Sub&gt;::sub (alsa-rs/src/mixer.rs:123:48)
  repro_alsa_rs_src_mixer_rs_123_48_5fda4b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;alsa::mixer::MilliBel as ::core::ops::Add&lt;alsa::mixer::MilliBel&gt;&gt;::add(
    MilliBel(i64::MAX), MilliBel(1i64));</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/mixer.rs:114:48:
attempt to add with overflow
backtrace:
  &lt;alsa::mixer::MilliBel as core::ops::arith::Add&gt;::add (alsa-rs/src/mixer.rs:114:48)
  repro_alsa_rs_src_mixer_rs_114_48_93b797::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = HCtl::new("\0", false);</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/hctl.rs:58:37:
called `Result::unwrap()` on an `Err` value: NulError(0, [0])
backtrace:
  &lt;alsa (alsa-rs/src/hctl.rs:58:37)
  repro_alsa_rs_src_hctl_rs_58_37_849423::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let v = ();
let _ = Event::new::&lt;()&gt;(EventType::Bounce, &amp;v);</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/seq.rs:691:9:
event type must not carry variable-length data
backtrace:
  &lt;alsa::seq::Event&gt;::new::&lt;()&gt; (alsa-rs/src/seq.rs:691:9)
  repro_alsa_rs_src_seq_rs_691_9_95e2ad::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let v = ();
let _ = Event::new::&lt;()&gt;(EventType::Pgmchange, &amp;v);</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/seq.rs:695:9:
assertion failed: D::has_data(t)
backtrace:
  &lt;alsa::seq::Event&gt;::new::&lt;()&gt; (alsa-rs/src/seq.rs:695:9)
  repro_alsa_rs_src_seq_rs_695_9_390b6d::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Ump::new("\0", Direction::Capture, false);</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/ump.rs:39:40:
called `Result::unwrap()` on an `Err` value: NulError(0, [0])
backtrace:
  &lt;alsa::ump::Ump&gt;::new (alsa-rs/src/ump.rs:39:40)
  repro_alsa_rs_src_ump_rs_39_40_2f1691::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PCM::new("\0", Direction::Capture, false);</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/pcm.rs:154:40:
called `Result::unwrap()` on an `Err` value: NulError(0, [0])
backtrace:
  &lt;alsa::pcm::PCM&gt;::new (alsa-rs/src/pcm.rs:154:40)
  repro_alsa_rs_src_pcm_rs_154_40_788705::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Ctl::new("\0", false);</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/ctl_int.rs:94:37:
called `Result::unwrap()` on an `Err` value: NulError(0, [0])
backtrace:
  &lt;alsa::ctl_int::Ctl&gt;::new (alsa-rs/src/ctl_int.rs:94:37)
  repro_alsa_rs_src_ctl_int_rs_94_37_e10ce7::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = SelemId::new("\0", 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at alsa-rs/src/mixer.rs:174:40:
called `Result::unwrap()` on an `Err` value: NulError(0, [0])
backtrace:
  &lt;alsa::mixer::SelemId&gt;::new (alsa-rs/src/mixer.rs:174:40)
  repro_alsa_rs_src_mixer_rs_174_40_3de505::main (src/main.rs:4:13)</pre></td></tr>
</table>
