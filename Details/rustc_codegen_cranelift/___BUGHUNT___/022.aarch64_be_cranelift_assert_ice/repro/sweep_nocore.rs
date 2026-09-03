#![feature(no_core, lang_items)]
#![no_core]
#![no_main]
#![allow(internal_features)]

#[lang = "pointee_sized"]
pub trait PointeeSized {}
#[lang = "meta_sized"]
pub trait MetaSized: PointeeSized {}
#[lang = "sized"]
pub trait Sized: MetaSized {}
#[lang = "copy"]
pub trait Copy {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
