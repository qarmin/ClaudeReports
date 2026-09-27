    #![feature(custom_mir, core_intrinsics, lazy_get)]
    use core::intrinsics::mir::*;
    use std::fmt::Debug;
    fn dump_var(
        f: usize,
        var0: usize, val0: impl Debug,
        var1: usize, val1: impl Debug,
        var2: usize, val2: impl Debug,
        var3: usize, val3: impl Debug,
    ) {
        println!("fn{f}:_{var0} = {val0:?}\n_{var1} = {val1:?}\n_{var2} = {val2:?}\n_{var3} = {val3:?}");
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: isize,mut _2: u128) -> u16 {
mir! {
let _3: u128;
let _4: bool;
{
_2 = 35443_u16 as u128;
_4 = true;
RET = !33407_u16;
_3 = !_2;
Call(_2 = fn1(_4, _1, _3, _1, _4, _1, _4, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Goto(bb10)
}
bb10 = {
Goto(bb15)
}
bb15 = {
Goto(bb23)
}
bb19 = {
Goto(bb15)
}
bb23 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: bool,mut _2: isize,mut _3: u128,mut _4: isize,mut _5: bool,mut _6: isize,mut _7: bool,mut _8: bool,mut _9: bool) -> u128 {
mir! {
let _11: Adt29;
let _13: f32;
let _15: f32;
let _17: bool;
let _18: f32;
let _21: (*const ([bool; 2], ([bool; 2], char)), usize, &'static mut u64, i32);
let _22: i8;
let _27: i8;
let _31: ([i8; 2], i128);
let _32: f32;
let _38: *const f32;
let _43: [isize; 2];
let _51: ();
{
_13 = 139_u8 as f32;
_17 = _9;
_18 = _13 * _13;
RET = _3;
_22 = _6 as i8;
_15 = _3 as f32;
_13 = -_15;
Call(_21.1 = core::intrinsics::bswap(10298890547728633553_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_27 = _22 - _22;
_31.1 = _22 as i128;
_31.0 = [_27,_22];
Call(_11.fld1 = fn2(_17, _31.0, _7, _31, _18, _13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_18 = _13 - _15;
_38 = core::ptr::addr_of!(_15);
(*_38) = _18 - _13;
_32 = (*_38) * (*_38);
_4 = _32 as isize;
_43 = [_4,_6];
Goto(bb17)
}
bb10 = {
Call(_21.1 = core::intrinsics::bswap(10298890547728633553_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(1_usize, 43_usize, Move(_43), 27_usize, Move(_27), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: bool,mut _2: [i8; 2],mut _3: bool,mut _4: ([i8; 2], i128),mut _5: f32,mut _6: f32) -> *const *const ([bool; 2], ([bool; 2], char)) {
mir! {
let _9: *const *const ([bool; 2], ([bool; 2], char));
let _39: *const ([bool; 2], ([bool; 2], char));
{
Goto(bb9)
}
bb9 = {
Goto(bb10)
}
bb10 = {
Goto(bb12)
}
bb12 = {
Goto(bb18)
}
bb14 = {
Goto(bb10)
}
bb18 = {
Goto(bb20)
}
bb20 = {
_9 = core::ptr::addr_of!(_39);
RET = core::ptr::addr_of!((*_9));
Goto(bb32)
}
bb26 = {
Goto(bb12)
}
bb32 = {
Goto(bb36)
}
bb34 = {
Goto(bb10)
}
bb36 = {
Goto(bb38)
}
bb37 = {
Goto(bb9)
}
bb38 = {
Goto(bb49)
}
bb43 = {
Goto(bb38)
}
bb49 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19() -> Adt19 {
mir! {
{
Return()
}
}
}
pub fn main() {
                fn0(std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(68816661933224904809568311543406935508_u128));
            }
pub enum Adt19 {
Variant0{
}}
pub struct Adt29 {
fld1: *const *const ([bool; 2], ([bool; 2], char)),
}
pub struct Adt32 {
}
pub enum Adt34 {
Variant0{
}}
pub enum Adt50 {
Variant0{
}}
pub enum Adt62 {
Variant0{
}}
pub enum Adt77 {
Variant0{
}}
pub enum Adt80 {
Variant0{
}}
