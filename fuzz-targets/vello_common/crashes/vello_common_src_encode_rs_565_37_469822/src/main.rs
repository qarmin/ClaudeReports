use vello_common::paint::IndexedPaint;
use vello_common::paint::Paint;

fn main() {
    let paint = Paint::Indexed(IndexedPaint::new(0usize));
    let _ = paint.may_have_transparency(&[]);
}
