use std::hint::black_box;

trait Op { fn id(&self) -> i32; fn compute(&self, x: i32) -> i32; }
struct S0;
impl Op for S0 { fn id(&self) -> i32 { 0 } fn compute(&self, x: i32) -> i32 { x * 0 + 0 } }
struct S1;
impl Op for S1 { fn id(&self) -> i32 { 1 } fn compute(&self, x: i32) -> i32 { x * 1 + 1 } }
struct S2;
impl Op for S2 { fn id(&self) -> i32 { 2 } fn compute(&self, x: i32) -> i32 { x * 2 + 2 } }
struct S3;
impl Op for S3 { fn id(&self) -> i32 { 3 } fn compute(&self, x: i32) -> i32 { x * 3 + 3 } }
struct S4;
impl Op for S4 { fn id(&self) -> i32 { 4 } fn compute(&self, x: i32) -> i32 { x * 4 + 4 } }
struct S5;
impl Op for S5 { fn id(&self) -> i32 { 5 } fn compute(&self, x: i32) -> i32 { x * 5 + 5 } }
struct S6;
impl Op for S6 { fn id(&self) -> i32 { 6 } fn compute(&self, x: i32) -> i32 { x * 6 + 6 } }
struct S7;
impl Op for S7 { fn id(&self) -> i32 { 7 } fn compute(&self, x: i32) -> i32 { x * 7 + 7 } }
struct S8;
impl Op for S8 { fn id(&self) -> i32 { 8 } fn compute(&self, x: i32) -> i32 { x * 8 + 8 } }
struct S9;
impl Op for S9 { fn id(&self) -> i32 { 9 } fn compute(&self, x: i32) -> i32 { x * 9 + 9 } }
struct S10;
impl Op for S10 { fn id(&self) -> i32 { 10 } fn compute(&self, x: i32) -> i32 { x * 10 + 10 } }
struct S11;
impl Op for S11 { fn id(&self) -> i32 { 11 } fn compute(&self, x: i32) -> i32 { x * 11 + 11 } }
fn main() {
    let boxed: Vec<Box<dyn Op>> = vec![Box::new(S0), Box::new(S1), Box::new(S2), Box::new(S3), Box::new(S4), Box::new(S5), Box::new(S6), Box::new(S7), Box::new(S8), Box::new(S9), Box::new(S10), Box::new(S11)];
    for (idx, b) in boxed.iter().enumerate() {
        println!("vtable_dispatch/{}: id={} compute={}", idx, b.id(), b.compute(black_box(10)));
    }
    let refs: [&dyn Op; 12] = [&S0, &S1, &S2, &S3, &S4, &S5, &S6, &S7, &S8, &S9, &S10, &S11];
    for (idx, r) in refs.iter().enumerate() {
        println!("vtable_ref_dispatch/{}: id={} compute={}", idx, r.id(), r.compute(black_box(7)));
    }
    fn generic_double<T: std::ops::Add<Output = T> + Copy + std::fmt::Debug>(x: T) -> T { x + x }
    println!("generic_double/i8: {:?}", generic_double(black_box(5i8)));
    println!("generic_double/u8: {:?}", generic_double(black_box(5u8)));
    println!("generic_double/i64: {:?}", generic_double(black_box(5i64)));
    println!("generic_double/u64: {:?}", generic_double(black_box(5u64)));
    println!("generic_double/f32: {:?}", generic_double(black_box(5.5f32)));
    println!("generic_double/f64: {:?}", generic_double(black_box(5.5f64)));
    let mult = black_box(3i32);
    let fns: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(move |x| x + mult),
        Box::new(|x| x * 2),
        Box::new(move |x| x - mult),
    ];
    for (idx, f) in fns.iter().enumerate() {
        println!("closure_dispatch/{}: {}", idx, f(black_box(10)));
    }
}