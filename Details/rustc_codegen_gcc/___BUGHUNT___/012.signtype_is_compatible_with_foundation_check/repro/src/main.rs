// Standalone repro: queries libgccjit's gcc_jit_compatible_types() (via gccjit::Type::is_compatible_with,
// the exact same public API rustc_codegen_gcc's src/common.rs uses to implement SignType::is_signed/
// is_unsigned/to_signed/to_unsigned) directly, for every same-width signed/unsigned integer pair.
//
// Candidate 012 asks: does gcc_jit_compatible_types(int8_t, uint8_t) return false (as required for
// SignType's sign dispatch to be sound), or does it return true (which would silently break every
// sign-sensitive decision built on top of it in src/int.rs)?

use std::ffi::CString;

fn main() {
    let lib_path = std::env::var("LIBGCCJIT_PATH")
        .unwrap_or_else(|_| "libgccjit.so".to_string());
    let cpath = CString::new(lib_path.clone()).unwrap();
    gccjit::load(&cpath).unwrap_or_else(|e| panic!("failed to load {lib_path}: {e}"));

    let ctx = gccjit::Context::default();

    use gccjit::CType;

    // Mirrors src/context.rs's `create_type` calls exactly: CodegenCx's i8_type/u8_type/etc
    // (the ones SignType::is_i8/is_u8/... actually compare against via is_compatible_with) are
    // built with context.new_c_type(CType::IntNt/UIntNt) -- NOT context.new_type::<iN>() (which
    // maps to a different libgccjit entry point, gcc_jit_context_get_int_type, rather than
    // gcc_jit_context_get_type). Test the exact construction path the real backend uses.
    let pairs: Vec<(&str, gccjit::Type, gccjit::Type)> = vec![
        ("i8/u8 (CType)", ctx.new_c_type(CType::Int8t), ctx.new_c_type(CType::UInt8t)),
        ("i16/u16 (CType)", ctx.new_c_type(CType::Int16t), ctx.new_c_type(CType::UInt16t)),
        ("i32/u32 (CType)", ctx.new_c_type(CType::Int32t), ctx.new_c_type(CType::UInt32t)),
        ("i64/u64 (CType)", ctx.new_c_type(CType::Int64t), ctx.new_c_type(CType::UInt64t)),
        ("i128/u128 (CType)", ctx.new_c_type(CType::Int128t), ctx.new_c_type(CType::UInt128t)),
        ("i8/u8 (Typeable)", ctx.new_type::<i8>(), ctx.new_type::<u8>()),
        ("i16/u16 (Typeable)", ctx.new_type::<i16>(), ctx.new_type::<u16>()),
        ("i32/u32 (Typeable)", ctx.new_type::<i32>(), ctx.new_type::<u32>()),
        ("i64/u64 (Typeable)", ctx.new_type::<i64>(), ctx.new_type::<u64>()),
        // Cross-construction: does the CType-built i8 compare correctly against the
        // Typeable/new_int_type-built u8 (mixing construction paths, as could plausibly happen
        // when a value flows from one codegen call site to another)?
        ("i8(CType)/u8(Typeable)", ctx.new_c_type(CType::Int8t), ctx.new_type::<u8>()),
    ];

    let mut any_unexpected = false;
    for (name, signed, unsigned) in &pairs {
        let fwd = signed.is_compatible_with(*unsigned);
        let rev = unsigned.is_compatible_with(*signed);
        let self_signed = signed.is_compatible_with(*signed);
        let self_unsigned = unsigned.is_compatible_with(*unsigned);
        println!(
            "{name}: signed.is_compatible_with(unsigned)={fwd} unsigned.is_compatible_with(signed)={rev} self_signed={self_signed} self_unsigned={self_unsigned}"
        );
        if fwd || rev {
            any_unexpected = true;
        }
        if !self_signed || !self_unsigned {
            any_unexpected = true;
        }
    }

    // Also check int128 pair via new_int_type, since i128/u128 in rustc_codegen_gcc's CodegenCx
    // are constructed with new_int_type(16, signed) rather than context.new_type::<iN>().
    let i128_t = ctx.new_int_type(16, true);
    let u128_t = ctx.new_int_type(16, false);
    let fwd128 = i128_t.is_compatible_with(u128_t);
    let rev128 = u128_t.is_compatible_with(i128_t);
    println!(
        "i128/u128 (new_int_type): signed.is_compatible_with(unsigned)={fwd128} unsigned.is_compatible_with(signed)={rev128}"
    );
    if fwd128 || rev128 {
        any_unexpected = true;
    }

    if any_unexpected {
        println!("RESULT: UNEXPECTED - gcc_jit_compatible_types treats a signed/unsigned pair of the same width as compatible, or a type as incompatible with itself.");
        std::process::exit(1);
    } else {
        println!("RESULT: AS EXPECTED - gcc_jit_compatible_types distinguishes signed from unsigned for every same-width pair checked, and every type is compatible with itself.");
    }
}
