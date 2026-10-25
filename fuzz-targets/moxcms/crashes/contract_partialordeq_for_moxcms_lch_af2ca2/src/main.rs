use moxcms::LCh;

fn main() {
    let l_ch = <moxcms::LCh as ::core::ops::Div<f32>>::div(<moxcms::LCh as ::core::ops::Add<f32>>::add(LCh::default(), 1.0f32), 0.0f32);
    assert_eq!(l_ch == l_ch, l_ch.partial_cmp(&l_ch) == Some(std::cmp::Ordering::Equal), "PartialOrd inconsistent with PartialEq");
}
