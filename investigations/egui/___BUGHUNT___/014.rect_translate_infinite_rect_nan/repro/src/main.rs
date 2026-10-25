use emath::{Rect, vec2};

fn main() {
    let delta = vec2(1.0, 2.0);
    for (name, rect) in [("Rect::EVERYTHING", Rect::EVERYTHING), ("Rect::NOTHING", Rect::NOTHING)] {
        let moved = rect.translate(delta);
        println!("{name}.translate({delta:?}) = {moved:?}   (any_nan: {})", moved.any_nan());
    }
    // control: a finite rect
    let finite = Rect::from_min_max(emath::pos2(0.0, 0.0), emath::pos2(10.0, 10.0));
    println!("finite.translate = {:?}", finite.translate(delta));
    // a half-infinite rect, e.g. "everything to the right of x = 5"
    let half = Rect::from_min_max(emath::pos2(5.0, f32::NEG_INFINITY), emath::pos2(f32::INFINITY, f32::INFINITY));
    println!("half-infinite.translate = {:?}", half.translate(delta));
}
