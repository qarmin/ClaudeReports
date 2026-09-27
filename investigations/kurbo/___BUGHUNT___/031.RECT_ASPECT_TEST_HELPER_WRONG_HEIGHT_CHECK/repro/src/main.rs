// Standalone repro for candidate 031 (RECT_ASPECT_TEST_HELPER_WRONG_HEIGHT_CHECK).
//
// kurbo/src/rect.rs's `contained_rect_with_aspect_ratio`/`inscribed_rect_with_aspect_ratio` unit
// tests share a `case()` helper with a "sanity check" assertion meant to verify the computed
// `expected` rect actually fits inside `outer` on both axes:
//
//     assert!(
//         expected.size().width.abs() <= outer.size().width.abs()
//             && expected.size().width.abs() <= outer.size().height.abs(),
//         "Sanity check {expected} should be smaller than {outer}."
//     );
//
// The second half compares `expected`'s WIDTH against `outer`'s HEIGHT a second time - it never
// examines `expected.size().height` at all. This program reproduces that exact boolean
// expression (copied verbatim, using kurbo's real public `Rect`/`Size` API - no hand-rolled
// stand-in) against a deliberately too-tall `expected` rect and shows:
//
//   1. The ORIGINAL condition from rect.rs, as literally written, returns `true` (i.e. the test's
//      `assert!` would NOT panic) even though `expected`'s height (50) is 5x `outer`'s height (10).
//   2. The CORRECTED condition (comparing `expected`'s height against `outer`'s height, as the
//      sanity check's own doc comment/intent implies) correctly returns `false` for the same
//      input, i.e. the `assert!` WOULD panic and flag the too-tall rect.
//
// This demonstrates the test helper's sanity check is toothless on the height axis: any future
// `expected` value (or, in principle, an actual regression in `inscribed_rect_with_aspect_ratio`
// that produced a too-tall rect matched by an equally wrong hand-written `expected`) would sail
// through this check silently.

use kurbo::Rect;

fn original_buggy_condition(expected: Rect, outer: Rect) -> bool {
    expected.size().width.abs() <= outer.size().width.abs()
        && expected.size().width.abs() <= outer.size().height.abs()
}

fn corrected_condition(expected: Rect, outer: Rect) -> bool {
    expected.size().width.abs() <= outer.size().width.abs()
        && expected.size().height.abs() <= outer.size().height.abs()
}

fn main() {
    // outer: 10 wide, 10 tall.
    let outer = Rect::new(0.0, 0.0, 10.0, 10.0);
    // expected: same width (10) as outer, but height (50) is 5x taller than outer's height.
    // A genuinely "smaller than outer" rect must never have this shape.
    let expected = Rect::new(0.0, 0.0, 10.0, 50.0);

    println!("outer    = {outer} (width={}, height={})", outer.size().width, outer.size().height);
    println!(
        "expected = {expected} (width={}, height={})",
        expected.size().width,
        expected.size().height
    );

    let original = original_buggy_condition(expected, outer);
    let corrected = corrected_condition(expected, outer);

    println!();
    println!(
        "original rect.rs condition (width checked against outer's height twice): {original}"
    );
    println!(
        "corrected condition (height checked against outer's height):            {corrected}"
    );

    assert!(
        original,
        "expected the ORIGINAL buggy condition to accept this too-tall rect (that's the bug)"
    );
    assert!(
        !corrected,
        "expected the CORRECTED condition to reject this too-tall rect"
    );

    println!();
    println!(
        "CONFIRMED: the original assert!() in rect.rs's test helper would NOT have panicked for \
         an `expected` whose height (50) is 5x `outer`'s height (10) - it only ever compares \
         `expected`'s WIDTH against `outer`'s height, never `expected`'s own height."
    );
}
