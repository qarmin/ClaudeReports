# nu-ansi-term - crashes

Source: https://github.com/nushell/nu-ansi-term @ a23b71dae8efc17ca89d6bfe86134e15c3b157f5

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let string = nu_ansi_term::gradient::build_all_gradient_text("(\\)".repeat(871),
    Gradient::from_color_rgb(Color::LightGray, Color::from(Color::Blue)),
    Gradient::from_color_rgb(Color::Black, Color::DarkGray));
let string2 = nu_ansi_term::gradient::build_all_gradient_text(&amp;string,
    Gradient::new(
    &lt;nu_ansi_term::Rgb as ::core::ops::Add&lt;nu_ansi_term::Rgb&gt;&gt;::add(
    Rgb { r: 239u8, g: 15u8, b: 8u8 }, Rgb::from_f32(244.0f32, 8.4f32,
    762.0f32)), Rgb { r: 15u8, g: 244u8, b: 1u8 }).reverse(),
    Gradient::new(Rgb::from_f32(65536.0f32, -47.5f32, f32::NEG_INFINITY),
    Rgb::from_f32(f32::INFINITY, 4.9f32, 1e-40f32)));
let string3 = nu_ansi_term::gradient::build_all_gradient_text(&amp;string2,
    Gradient::from_color_rgb(Color::Red, Color::Default),
    Gradient::from_color_rgb(Color::LightRed, Color::Magenta));
let gradient = Gradient { start: Rgb::from_f32(0.9999999f32, 0.277f32,
    0.297f32), end: Rgb::from_f32(-91.9f32, -1.1754944e-38f32, 1e-40f32) };
let _ = gradient.build(&amp;string3, TargetGround::Background);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  __GI_memcpy (../string/bits/string_fortified.h:29)
  nu_ansi_term::gradient::{impl#0}::build::{closure#0} (src/gradient.rs:51)
  nu_ansi_term::gradient::Gradient::build (src/gradient.rs:50)
  repro_timeout_in_nu_ansi_term_gradient_gradient_build_8d2174::main (src/main.rs:11)
  (26 std/libc frames omitted)</pre></td></tr>
</table>
