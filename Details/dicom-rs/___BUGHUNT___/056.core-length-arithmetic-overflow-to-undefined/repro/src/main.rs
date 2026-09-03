use dicom_core::header::Length;

/// `Length::UNDEFINED` (0xFFFF_FFFF) is a reserved sentinel meaning "no
/// defined length" - used pervasively (SQ elements, encapsulated pixel
/// data) to mean something semantically different from any real byte
/// count. A *defined* length arithmetic result that happens to land
/// exactly on this same bit pattern must never be silently reinterpreted
/// as that sentinel.
fn main() {
    let mut bug = false;

    // Add<Length>: two large-but-defined lengths whose sum equals exactly
    // 0xFFFF_FFFF.
    let sum = Length(0xFFFF_FFFE) + Length(1);
    println!("Length(0xFFFFFFFE) + Length(1) = {sum:?} (is_undefined={})", sum.is_undefined());
    if sum.is_undefined() {
        bug = true;
    }

    // Add<i32>: same idea, adding a plain i32.
    let sum2 = Length(0xFFFF_FFFE) + 1i32;
    println!("Length(0xFFFFFFFE) + 1i32 = {sum2:?} (is_undefined={})", sum2.is_undefined());
    if sum2.is_undefined() {
        bug = true;
    }

    // SubAssign<Length>: 0 - 1 underflows to 0xFFFF_FFFF in u32 arithmetic.
    let mut diff = Length(0);
    diff -= Length(1);
    println!("Length(0) -= Length(1) => {diff:?} (is_undefined={})", diff.is_undefined());
    if diff.is_undefined() {
        bug = true;
    }

    // SubAssign<i32>: same idea, subtracting a plain i32.
    let mut diff2 = Length(0);
    diff2 -= 1i32;
    println!("Length(0) -= 1i32 => {diff2:?} (is_undefined={})", diff2.is_undefined());
    if diff2.is_undefined() {
        bug = true;
    }

    if bug {
        println!(
            "BUG REPRODUCED: a defined-length arithmetic result silently collided with the \
             UNDEFINED sentinel."
        );
        std::process::exit(1);
    }

    println!("No corruption observed: all results stayed correctly defined.");
}
