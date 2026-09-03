fn main() {
    let _2 = [true, false, true, true, false];
    let _1 = 0;
    let mut _6 = 0.0;
    let RET = 0.0;
    let mut _3 = 51216f32;
    let mut _4 = _3 * _3;
    let _13 = 0;
    let mut _14 = _4;
    let _20 = &mut _14;
    _4 = *_20 * *_20 * _4;
    _6 = *_20;
    *_20 = _4;
    let mut _28 = 0.0;
    let _41 = &mut _28;
    *_41 = *_20;
    let _39 = *_20 = *_41 * *_41;
    let _12 = *_41 = -*_20;
    _3 = *_41;
    let _57 = *_41;
    let _5 = *_41 = _3 * _6 * -_57;
    let val = *_41;
    let tmp = _28 * val;
    let _78 = tmp as u128;
    println!("{_78:?}");
}
