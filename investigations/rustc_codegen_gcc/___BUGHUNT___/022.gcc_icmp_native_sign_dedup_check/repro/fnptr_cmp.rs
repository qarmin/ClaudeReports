// Compiler:
//
// Run-time:
//   status: 0

// Escalation for candidate 022: ordering comparison (`<`/`>`) on function pointers exercises
// gcc_icmp's native path with a_type/b_type being function-pointer GCC types (not caught by the
// `get_pointee().is_some()` data-pointer branch in src/int.rs). Checking whether the
// IntULT/IntUGT sign-cast (`a_type.to_unsigned(self.cx)`) copes with a function-pointer-typed
// `a_type`, since to_unsigned/to_signed are presumably meant for integer types only.

use std::hint::black_box;

fn f1() {}
fn f2() {}

fn main() {
    let fp1: fn() = black_box(f1);
    let fp2: fn() = black_box(f2);
    // Don't crash / miscompile; the actual ordering result is unspecified by Rust for fn ptrs,
    // but the codegen must not panic, and the result must be self-consistent (irreflexive < / >).
    let lt = fp1 < fp2;
    let gt = fp1 > fp2;
    let eq = fp1 == fp2;
    assert!(!(lt && gt), "fn ptr cannot be both < and > the other");
    assert!(!(lt && eq), "fn ptr cannot be both < and == the other");
    assert!(fp1 <= fp1);
    assert!(fp1 >= fp1);
    assert!(!(fp1 < fp1));
    assert!(!(fp1 > fp1));
    println!("fn ptr checks passed: lt={lt} gt={gt} eq={eq}");
}
