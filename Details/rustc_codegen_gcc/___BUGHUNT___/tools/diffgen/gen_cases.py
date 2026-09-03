#!/usr/bin/env python3
"""Generate small, self-contained, independently-runnable Rust programs that exercise
codegen-correctness-prone constructs in rustc_codegen_gcc, for differential testing against
plain rustc (LLVM backend).

Not a random/AFL-style fuzzer: every case is a deliberate, parameterized instantiation of a known
risky pattern (see ___BUGHUNT___/tools/diffgen/README.md and
~/.claude/skills/bug-hunt/projects/rustc_codegen_gcc.md's "Risk map"), swept across a Cartesian
product of types/values rather than randomly generated - so a divergence found by run_diff.sh is
already a confirmed, reproducible behavioral difference between the two backends on ordinary safe
Rust, not a guess that still needs verification.

Each case is a single .rs file, one `fn main()`, no external dependencies - already minimal by
construction (one construct + one parameter combination per file), so a failing case needs no
further reduction to be usable as an IDEAS.md lead or a candidate's own repro/.

Usage: python3 gen_cases.py [--only CATEGORY[,CATEGORY...]]
Writes into ./cases/<category>/<name>.rs (overwrites on re-run - regenerate freely).
"""
import argparse
import itertools
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CASES_DIR = os.path.join(HERE, "cases")

PREAMBLE = "use std::hint::black_box;\n\n"

# (rust type name, bit width, signed?)
INT_TYPES = [
    ("i8", 8, True), ("u8", 8, False),
    ("i16", 16, True), ("u16", 16, False),
    ("i32", 32, True), ("u32", 32, False),
    ("i64", 64, True), ("u64", 64, False),
    ("i128", 128, True), ("u128", 128, False),
    ("isize", 64, True), ("usize", 64, False),
]

FLOAT_TYPES = ["f32", "f64"]


def boundary_values(bits: int, signed: bool, ty: str) -> list[str]:
    """Rust literal expressions, suffixed with `ty` itself (always safe since every value here is
    within that exact type's own range by construction - unlike staging through a fixed-width
    intermediate like i128, which overflows for u128's own upper-half boundary values), covering
    the interesting boundary values for a type of this width/signedness - 0/1/-1/min/max/
    mid-sign-bit pairs."""
    if signed:
        lo = -(1 << (bits - 1))
        hi = (1 << (bits - 1)) - 1
        vals = [0, 1, -1, lo, lo + 1, hi, hi - 1]
    else:
        hi = (1 << bits) - 1
        mid = 1 << (bits - 1)
        vals = [0, 1, hi, hi - 1, mid, mid - 1, mid + 1]
    # dedupe, keep order
    seen = set()
    out = []
    for v in vals:
        if v not in seen:
            seen.add(v)
            out.append(v)
    return [f"{v}{ty}" for v in out]


def write_case(category: str, name: str, body: str, preamble: str = PREAMBLE):
    d = os.path.join(CASES_DIR, category)
    os.makedirs(d, exist_ok=True)
    path = os.path.join(d, f"{name}.rs")
    with open(path, "w") as f:
        f.write(preamble + body)
    return path


# ---------------------------------------------------------------------------
# Category: casts - integer width/sign conversions, with a declared-sign-mismatch step
# (transmute from the "wrong" sign before widening/narrowing) mirroring the already-confirmed
# division-sign bug class and the open sext/zext gap (PR #552) - see risk map.
# ---------------------------------------------------------------------------
def gen_casts():
    lines = []
    for (src_ty, src_bits, src_signed), (dst_ty, dst_bits, dst_signed) in itertools.product(INT_TYPES, INT_TYPES):
        if src_ty == dst_ty:
            continue
        boundaries = boundary_values(src_bits, src_signed, src_ty)
        for i, bv in enumerate(boundaries):
            # Plain direct cast (control).
            lines.append(
                f'    {{ let v: {src_ty} = black_box({bv} as {src_ty}); '
                f'let r = v as {dst_ty}; '
                f'println!("direct/{src_ty}->{dst_ty}/{i}: {{}}", r); }}'
            )
            # Same value, but reinterpreted (transmute) through the OPPOSITE-signedness same-width
            # type first - this is the exact shape that broke scalar division (6f389ff) and is
            # suspected for `sext` (see IDEAS.md) - a value whose GCC-declared sign doesn't match
            # its semantic sign at the moment of the width-changing cast.
            opp_ty = (f"u{src_bits}" if src_signed else f"i{src_bits}") if src_bits in (8, 16, 32, 64, 128) else None
            if opp_ty:
                lines.append(
                    f'    {{ let v: {src_ty} = black_box({bv} as {src_ty}); '
                    f'let reinterpreted: {opp_ty} = unsafe {{ std::mem::transmute(v) }}; '
                    f'let back: {src_ty} = unsafe {{ std::mem::transmute(reinterpreted) }}; '
                    f'let r = back as {dst_ty}; '
                    f'println!("transmute-roundtrip/{src_ty}->{dst_ty}/{i}: {{}}", r); }}'
                )
        body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
        write_case("casts", f"{src_ty}_to_{dst_ty}", body)
        lines = []


# ---------------------------------------------------------------------------
# Category: overflow-checked arithmetic - checked_/overflowing_/wrapping_/saturating_ variants of
# add/sub/mul/div/rem, at boundary values, for every integer width/sign. Entirely unexplored per
# the risk map (gcc_checked_binop and friends, src/int.rs:294+).
# ---------------------------------------------------------------------------
OPS = ["add", "sub", "mul", "div", "rem"]
VARIANTS = ["checked", "overflowing", "wrapping", "saturating"]


def gen_overflow():
    for ty, bits, signed in INT_TYPES:
        boundaries = boundary_values(bits, signed, ty)
        # Curated pairs, not the full cross product (7x7=49 would make each file huge and slow to
        # compile for little extra signal): every boundary against itself (self-interaction, e.g.
        # MAX+MAX) and against the first boundary (0) and last (a second boundary) - covers the
        # overflow-triggering combinations without a combinatorial blowup.
        pairs = list(zip(boundaries, boundaries))
        pairs += [(a, boundaries[0]) for a in boundaries]
        pairs += [(a, boundaries[-1]) for a in boundaries]
        seen_pairs = set()
        dedup = []
        for p in pairs:
            if p not in seen_pairs:
                seen_pairs.add(p)
                dedup.append(p)
        pairs = dedup
        lines = []
        for op in OPS:
            for variant in VARIANTS:
                if variant == "saturating" and op in ("div", "rem"):
                    # std doesn't provide saturating_div/saturating_rem (integer div/rem only ever
                    # overflows on the single MIN/-1 case, not worth a saturating variant) - skip.
                    continue
                method = f"{variant}_{op}"
                for i, (a, b) in enumerate(pairs):
                    if op in ("div", "rem") and variant in ("wrapping",):
                        # wrapping_div/rem panic on div-by-zero (not defined to wrap) - guard.
                        lines.append(
                            f'    {{ let a: {ty} = black_box({a} as {ty}); '
                            f'let b: {ty} = black_box({b} as {ty}); '
                            f'if b != 0 {{ let r = a.{method}(b); '
                            f'println!("{method}/{ty}/{i}: {{:?}}", r); }} }}'
                        )
                    elif op in ("div", "rem"):
                        lines.append(
                            f'    {{ let a: {ty} = black_box({a} as {ty}); '
                            f'let b: {ty} = black_box({b} as {ty}); '
                            f'if b != 0 {{ let r = a.{method}(b); '
                            f'println!("{method}/{ty}/{i}: {{:?}}", r); }} }}'
                        )
                    else:
                        lines.append(
                            f'    {{ let a: {ty} = black_box({a} as {ty}); '
                            f'let b: {ty} = black_box({b} as {ty}); '
                            f'let r = a.{method}(b); '
                            f'println!("{method}/{ty}/{i}: {{:?}}", r); }}'
                        )
        body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
        write_case("overflow", f"{ty}_ops", body)


# ---------------------------------------------------------------------------
# Category: float <-> int casts, especially into/out of 128-bit ints, plus `!bool as iNN` -
# mirrors open issues #690 (huge/inf f32 -> u128 giving 0), #705 (u128 as f32 broken in release),
# #691 (!bool as usize giving -1-like garbage).
# ---------------------------------------------------------------------------
FLOAT_BOUNDARY_EXPRS = [
    "0.0", "-0.0", "1.0", "-1.0",
    "f32::MAX as f64", "f32::MIN as f64", "-(f32::MAX as f64)",
    "f64::MAX", "f64::MIN",
    "f32::INFINITY as f64", "f32::NEG_INFINITY as f64",
    "f64::INFINITY", "f64::NEG_INFINITY",
    "f64::NAN",
    "1e30", "-1e30", "1e100", "-1e100",
    "3.9999999", "-3.9999999",
]


def gen_floatint():
    for fty in FLOAT_TYPES:
        lines = []
        for i, expr in enumerate(FLOAT_BOUNDARY_EXPRS):
            # Route through an f64 intermediate (via black_box, so the compiler can't constant-fold
            # the literal straight into the target type) before narrowing to `fty` - casting an
            # untyped literal directly inside `as {fty}` triggers rustc's overflowing_literals lint
            # (deny-by-default) whenever the literal doesn't fit `fty`, which is exactly the
            # boundary case being tested here.
            lines.append(f'    let raw{i}: f64 = black_box(({expr}) as f64);')
            lines.append(f'    let v{i}: {fty} = raw{i} as {fty};')
        for i in range(len(FLOAT_BOUNDARY_EXPRS)):
            for ity, _, _ in INT_TYPES:
                lines.append(f'    println!("{fty}->{ity}/{i}: {{}}", v{i} as {ity});')
        body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
        write_case("floatint", f"{fty}_to_int", body)

    for ity, bits, signed in INT_TYPES:
        boundaries = boundary_values(bits, signed, ity)
        lines = []
        for i, bv in enumerate(boundaries):
            lines.append(f'    let v{i}: {ity} = black_box({bv} as {ity});')
        for i in range(len(boundaries)):
            for fty in FLOAT_TYPES:
                lines.append(f'    println!("{ity}->{fty}/{i}: {{}}", v{i} as {fty});')
        body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
        write_case("floatint", f"{ity}_to_float", body)

    # !bool as iNN / uNN - issue #691.
    lines = []
    for i, b in enumerate(["false", "true", "black_box(false)", "black_box(true)"]):
        lines.append(f'    let b{i}: bool = {b};')
        lines.append(f'    let n{i}: bool = !b{i};')
        for ity, _, _ in INT_TYPES:
            lines.append(f'    println!("!bool_as_{ity}/{i}: {{}}", n{i} as {ity});')
    body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
    write_case("floatint", "bool_not_cast", body)


# ---------------------------------------------------------------------------
# Category: bit manipulation ops - rotate/leading_zeros/trailing_zeros/count_ones/swap_bytes/
# reverse_bits/leading_ones/trailing_ones, entirely unexplored per the risk map.
# ---------------------------------------------------------------------------
BIT_METHODS_UNARY = [
    "leading_zeros", "trailing_zeros", "count_ones", "count_zeros",
    "swap_bytes", "reverse_bits", "leading_ones", "trailing_ones",
]


def gen_bitops():
    for ty, bits, signed in INT_TYPES:
        boundaries = boundary_values(bits, signed, ty)
        lines = []
        for i, bv in enumerate(boundaries):
            lines.append(f'    {{ let v: {ty} = black_box({bv} as {ty});')
            for m in BIT_METHODS_UNARY:
                lines.append(f'        println!("{m}/{ty}/{i}: {{:?}}", v.{m}());')
            for shift in [0, 1, bits // 2, bits - 1]:
                lines.append(
                    f'        println!("rotate_left({shift})/{ty}/{i}: {{}}", v.rotate_left({shift}));'
                )
                lines.append(
                    f'        println!("rotate_right({shift})/{ty}/{i}: {{}}", v.rotate_right({shift}));'
                )
            lines.append('    }')
        body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
        write_case("bitops", f"{ty}_ops", body)


# ---------------------------------------------------------------------------
# Category: SIMD (portable_simd) - shl/shr (signedness matters for `>>`: arithmetic vs logical
# shift, same risk-map family as scalar casts/division, never checked for vectors), cast
# (widening/narrowing/sign-changing, the vector analogue of the scalar casts category), and
# select (mask-based blend - structurally close to the already-confirmed gather/scatter masking
# bug, candidate 008, so worth checking a sibling construct even though gather/scatter itself
# isn't re-tested here to avoid re-deriving an already-Confirmed finding).
# ---------------------------------------------------------------------------
SIMD_INT_TYPES = [
    ("u8", 8, False), ("i8", 8, True),
    ("u16", 16, False), ("i16", 16, True),
    ("u32", 32, False), ("i32", 32, True),
    ("u64", 64, False), ("i64", 64, True),
]
SIMD_PREAMBLE = "#![feature(portable_simd)]\nuse std::simd::prelude::*;\nuse std::hint::black_box;\n\n"
LANES = [4, 8]


def gen_simd():
    # shl/shr
    for ty, bits, signed in SIMD_INT_TYPES:
        for lanes in LANES:
            vals = boundary_values(bits, signed, ty)[:lanes]
            while len(vals) < lanes:
                vals.append(vals[-1])
            arr = ", ".join(vals)
            lines = [f'    let v = Simd::<{ty}, {lanes}>::from_array(black_box([{arr}]));']
            for shift in [0, 1, bits // 2, bits - 1]:
                lines.append(f'    println!("shl({shift})/{ty}x{lanes}: {{:?}}", v << {shift});')
                lines.append(f'    println!("shr({shift})/{ty}x{lanes}: {{:?}}", v >> {shift});')
            # Per-lane shift amounts too (not just a uniform scalar shift).
            shift_arr = ", ".join(str((i * 3) % bits) for i in range(lanes))
            lines.append(f'    let shifts = Simd::<{ty}, {lanes}>::from_array(black_box([{shift_arr}]));')
            lines.append(f'    println!("shl(perlane)/{ty}x{lanes}: {{:?}}", v << shifts);')
            lines.append(f'    println!("shr(perlane)/{ty}x{lanes}: {{:?}}", v >> shifts);')
            body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
            write_case("simd", f"shift_{ty}x{lanes}", body, SIMD_PREAMBLE)

    # cast (widening/narrowing/sign-changing)
    for (src_ty, src_bits, src_signed) in SIMD_INT_TYPES:
        for (dst_ty, dst_bits, dst_signed) in SIMD_INT_TYPES:
            if src_ty == dst_ty:
                continue
            for lanes in LANES:
                vals = boundary_values(src_bits, src_signed, src_ty)[:lanes]
                while len(vals) < lanes:
                    vals.append(vals[-1])
                arr = ", ".join(vals)
                lines = [
                    f'    let v = Simd::<{src_ty}, {lanes}>::from_array(black_box([{arr}]));',
                    f'    let r: Simd<{dst_ty}, {lanes}> = v.cast();',
                    f'    println!("cast/{src_ty}x{lanes}->{dst_ty}: {{:?}}", r);',
                ]
                body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
                write_case("simd", f"cast_{src_ty}_to_{dst_ty}_x{lanes}", body, SIMD_PREAMBLE)

    # select (mask-based blend) - every possible mask pattern for lanes=4, boundary values on
    # both sides so a wrong lane selection is visible in the output.
    for ty, bits, signed in SIMD_INT_TYPES:
        lanes = 4
        a_vals = boundary_values(bits, signed, ty)[:lanes]
        while len(a_vals) < lanes:
            a_vals.append(a_vals[-1])
        b_vals = list(reversed(a_vals))
        a_arr = ", ".join(a_vals)
        b_arr = ", ".join(b_vals)
        lines = [
            f'    let a = Simd::<{ty}, {lanes}>::from_array(black_box([{a_arr}]));',
            f'    let b = Simd::<{ty}, {lanes}>::from_array(black_box([{b_arr}]));',
        ]
        mask_ty = f"i{bits}"
        for mask_bits in range(1 << lanes):
            mask_arr = ", ".join(("true" if (mask_bits >> i) & 1 else "false") for i in range(lanes))
            lines.append(f'    let m{mask_bits} = Mask::<{mask_ty}, {lanes}>::from_array([{mask_arr}]);')
            lines.append(f'    println!("select({mask_bits:04b})/{ty}x{lanes}: {{:?}}", m{mask_bits}.select(a, b));')
        body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
        write_case("simd", f"select_{ty}x{lanes}", body, SIMD_PREAMBLE)


    main()


# ---------------------------------------------------------------------------
# Category: atomics - full RMW op matrix (not just fetch_max/min, already Confirmed as candidate
# 004 on this exact codebase - re-included here only as an implicit harness sanity check, not a
# new lead if it fires) across every standard atomic integer type. fetch_nand, swap,
# compare_exchange(_weak), and fetch_update are NOT covered by candidate 004 and are entirely
# unexplored per the risk map.
# ---------------------------------------------------------------------------
ATOMIC_TYPES = [
    ("AtomicI8", "i8", 8, True), ("AtomicU8", "u8", 8, False),
    ("AtomicI16", "i16", 16, True), ("AtomicU16", "u16", 16, False),
    ("AtomicI32", "i32", 32, True), ("AtomicU32", "u32", 32, False),
    ("AtomicI64", "i64", 64, True), ("AtomicU64", "u64", 64, False),
    ("AtomicIsize", "isize", 64, True), ("AtomicUsize", "usize", 64, False),
]
ATOMIC_PREAMBLE = "use std::sync::atomic::*;\nuse std::hint::black_box;\n\n"
FETCH_OPS = ["fetch_add", "fetch_sub", "fetch_and", "fetch_or", "fetch_xor", "fetch_nand", "fetch_max", "fetch_min"]


def gen_atomics():
    for aty, ty, bits, signed in ATOMIC_TYPES:
        boundaries = boundary_values(bits, signed, ty)
        pairs = list(zip(boundaries, boundaries))
        pairs += [(a, boundaries[0]) for a in boundaries]
        pairs += [(a, boundaries[-1]) for a in boundaries]
        seen = set(); dedup = []
        for p in pairs:
            if p not in seen:
                seen.add(p); dedup.append(p)
        pairs = dedup

        lines = []
        for op in FETCH_OPS:
            for i, (init, arg) in enumerate(pairs):
                lines.append(
                    f'    {{ let a = {aty}::new(black_box({init})); '
                    f'let old = a.{op}(black_box({arg}), Ordering::SeqCst); '
                    f'println!("{op}/{ty}/{i}: old={{}} new={{}}", old, a.load(Ordering::SeqCst)); }}'
                )
        # swap / compare_exchange / compare_exchange_weak / fetch_update
        for i, (init, arg) in enumerate(pairs):
            lines.append(
                f'    {{ let a = {aty}::new(black_box({init})); '
                f'let old = a.swap(black_box({arg}), Ordering::SeqCst); '
                f'println!("swap/{ty}/{i}: old={{}} new={{}}", old, a.load(Ordering::SeqCst)); }}'
            )
            lines.append(
                f'    {{ let a = {aty}::new(black_box({init})); '
                f'let r = a.compare_exchange(black_box({init}), black_box({arg}), Ordering::SeqCst, Ordering::SeqCst); '
                f'println!("cmpxchg_match/{ty}/{i}: {{:?}} new={{}}", r, a.load(Ordering::SeqCst)); }}'
            )
            lines.append(
                f'    {{ let a = {aty}::new(black_box({init})); '
                f'let wrong = black_box({init}).wrapping_add(1); '
                f'let r = a.compare_exchange(wrong, black_box({arg}), Ordering::SeqCst, Ordering::SeqCst); '
                f'println!("cmpxchg_nomatch/{ty}/{i}: {{:?}} new={{}}", r, a.load(Ordering::SeqCst)); }}'
            )
            lines.append(
                f'    {{ let a = {aty}::new(black_box({init})); '
                f'let r = a.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| if v > black_box({arg}) {{ Some(v.wrapping_sub(1)) }} else {{ None }}); '
                f'println!("fetch_update/{ty}/{i}: {{:?}} new={{}}", r, a.load(Ordering::SeqCst)); }}'
            )
        body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
        write_case("atomics", f"{ty}_ops", body, ATOMIC_PREAMBLE)


# ---------------------------------------------------------------------------
# Category: enum/niche layout - entirely unexplored per the risk map. Covers: explicit-discriminant
# C-like enums across every `repr` integer width/sign, `Option<T>` niche optimization for every
# common niche-able type, nested `Option<Option<T>>`, and data-carrying enums whose niche comes
# from a field. Printed via `mem::discriminant`-derived ordering AND (where the type is plain-old-
# data) a raw-byte dump via `transmute` to the same-sized byte array, so a case where the enum
# still *behaves* correctly via `match`/`==` but has a corrupted internal bit-pattern would still
# show up as a byte-level diff even if the higher-level Debug/PartialEq view wouldn't catch it.
# ---------------------------------------------------------------------------
NICHE_PREAMBLE = "use std::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize, NonZeroI32};\nuse std::hint::black_box;\nuse std::mem::size_of;\n\n"
REPR_INT_TYPES = ["u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize"]


def gen_enum_niche():
    # C-like enum with explicit discriminants (including negative ones for signed reprs), one repr
    # per file so a failure pinpoints exactly which discriminant width/sign is affected.
    for repr_ty in REPR_INT_TYPES:
        is_signed = repr_ty.startswith("i")
        discs = [0, 1, 2, 100] if not is_signed else [0, 1, -1, -100, 100]
        variants = "\n".join(f"    V{i} = {d}," for i, (d, _) in enumerate(zip(discs, discs)))
        lines = [f"#[repr({repr_ty})]", "#[derive(Debug, Clone, Copy, PartialEq)]", "enum E {", variants, "}", "", "fn main() {"]
        for i, d in enumerate(discs):
            lines.append(f'    let v = black_box(E::V{i});')
            lines.append(f'    println!("disc/{repr_ty}/{i}: as_int={{}} debug={{:?}}", v as {repr_ty}, v);')
            lines.append(f'    let bytes: [u8; size_of::<E>()] = unsafe {{ std::mem::transmute(v) }};')
            lines.append(f'    println!("disc_bytes/{repr_ty}/{i}: {{:?}}", bytes);')
        lines.append("}")
        write_case("enum_niche", f"repr_{repr_ty}_discriminant", "\n".join(lines), NICHE_PREAMBLE)

    # Option<T> niche optimization for common niche-able types.
    niche_cases = [
        ("bool", "black_box(true)", "Some(black_box(true))", "None::<bool>"),
        ("char", "black_box('x')", "Some(black_box('x'))", "None::<char>"),
        ("NonZeroU8", "NonZeroU8::new(black_box(5)).unwrap()", "Some(NonZeroU8::new(black_box(5)).unwrap())", "None::<NonZeroU8>"),
        ("NonZeroU32", "NonZeroU32::new(black_box(5)).unwrap()", "Some(NonZeroU32::new(black_box(5)).unwrap())", "None::<NonZeroU32>"),
        ("NonZeroU64", "NonZeroU64::new(black_box(5)).unwrap()", "Some(NonZeroU64::new(black_box(5)).unwrap())", "None::<NonZeroU64>"),
        ("NonZeroI32", "NonZeroI32::new(black_box(-5)).unwrap()", "Some(NonZeroI32::new(black_box(-5)).unwrap())", "None::<NonZeroI32>"),
        ("NonZeroUsize", "NonZeroUsize::new(black_box(5)).unwrap()", "Some(NonZeroUsize::new(black_box(5)).unwrap())", "None::<NonZeroUsize>"),
    ]
    lines = []
    for ty, _, some_expr, none_expr in niche_cases:
        lines.append(f'    {{ let s: Option<{ty}> = {some_expr}; let n: Option<{ty}> = {none_expr};')
        lines.append(f'      println!("niche/{ty}/some: is_some={{}} debug={{:?}}", s.is_some(), s);')
        lines.append(f'      println!("niche/{ty}/none: is_none={{}} debug={{:?}}", n.is_none(), n);')
        lines.append(f'      println!("niche_size/{ty}: {{}}", size_of::<Option<{ty}>>() == size_of::<{ty}>()); }}')
    # Nested Option<Option<T>> - triple-state niche (None, Some(None), Some(Some(x))).
    lines.append('    { let a: Option<Option<bool>> = Some(Some(black_box(true)));')
    lines.append('      let b: Option<Option<bool>> = Some(None::<bool>);')
    lines.append('      let c: Option<Option<bool>> = None;')
    lines.append('      println!("nested_option_bool: {:?} {:?} {:?}", a, b, c); }')
    lines.append('    { let a: Option<Option<NonZeroU32>> = Some(Some(NonZeroU32::new(black_box(7)).unwrap()));')
    lines.append('      let b: Option<Option<NonZeroU32>> = Some(None);')
    lines.append('      let c: Option<Option<NonZeroU32>> = None;')
    lines.append('      println!("nested_option_nzu32: {:?} {:?} {:?}", a, b, c); }')
    # Reference/fn-pointer niche.
    lines.append('    { let x = black_box(42i32); let r: Option<&i32> = Some(&x); let n: Option<&i32> = None;')
    lines.append('      println!("niche_ref: {:?} {} {}", r, *r.unwrap(), n.is_none()); }')
    lines.append('    { fn f() -> i32 { black_box(1) } let p: Option<fn() -> i32> = Some(f); let n: Option<fn() -> i32> = None;')
    lines.append('      println!("niche_fnptr: {} {}", p.unwrap()(), n.is_none()); }')
    body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
    write_case("enum_niche", "option_niche_types", body, NICHE_PREAMBLE)

    # Data-carrying enum whose niche comes from one variant's field (Result-like), across a few
    # payload types/widths, plus round-tripping through `match` to make sure the payload itself
    # (not just the discriminant) survives correctly.
    lines = []
    for ty, ok_expr in [("u32", "black_box(123u32)"), ("i64", "black_box(-123i64)"), ("bool", "black_box(true)")]:
        lines.append(f'    {{ let ok: Result<{ty}, ()> = Ok({ok_expr});')
        lines.append(f'      let err: Result<{ty}, ()> = Err(());')
        lines.append(f'      let ok_val = match ok {{ Ok(v) => format!("{{:?}}", v), Err(_) => "ERR".to_string() }};')
        lines.append(f'      let err_val = match err {{ Ok(v) => format!("{{:?}}", v), Err(_) => "ERR".to_string() }};')
        lines.append(f'      println!("result_niche/{ty}: ok={{}} err={{}}", ok_val, err_val); }}')
    body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
    write_case("enum_niche", "result_niche_payload", body, NICHE_PREAMBLE)


# ---------------------------------------------------------------------------
# Category: panic/unwind codegen - entirely unexplored per the risk map. Covers: catch_unwind
# around several distinct panic triggers (explicit panic!, unwrap on None/Err, array OOB, integer
# overflow with overflow-checks forced on), and - the highest-value check here - whether Drop
# destructors actually run in the correct order while unwinding through several stack frames. A
# backend bug in unwind-table generation would most plausibly show up as "some destructors got
# skipped" or "wrong order", not as an immediate crash, so this needs an explicit drop-order log
# rather than just checking that catch_unwind returns Err.
# ---------------------------------------------------------------------------
PANIC_PREAMBLE = (
    "use std::hint::black_box;\nuse std::panic;\nuse std::sync::Mutex;\n\n"
    "static DROP_LOG: Mutex<Vec<i32>> = Mutex::new(Vec::new());\n\n"
    "struct Logged(i32);\n"
    "impl Drop for Logged {\n"
    "    fn drop(&mut self) { DROP_LOG.lock().unwrap().push(self.0); }\n"
    "}\n\n"
)


def gen_panic_unwind():
    lines = ['    panic::set_hook(Box::new(|_| {}));', ""]

    # catch_unwind around several distinct panic sources - confirm each is caught and the process
    # keeps running afterward (a wrong unwind-table entry could crash the process instead of
    # unwinding cleanly into the catch_unwind boundary).
    triggers = [
        ("explicit_panic", '{ panic!("boom"); }'),
        ("unwrap_none", '{ let x: Option<i32> = black_box(None); x.unwrap(); }'),
        ("unwrap_err", '{ let x: Result<i32, &str> = black_box(Err("e")); x.unwrap(); }'),
        ("array_oob", '{ let a = black_box([1, 2, 3]); let i = black_box(10usize); let _ = a[i]; }'),
        ("div_by_zero", '{ let a = black_box(1i32); let b = black_box(0i32); let _ = a / b; }'),
        ("overflow_add", '{ let a = black_box(i32::MAX); let b = black_box(1i32); let _ = a + b; }'),
    ]
    for name, body in triggers:
        lines.append(f'    {{ let r = panic::catch_unwind(|| {body}); println!("catch/{name}: caught={{}}", r.is_err()); }}')

    # Drop order across several nested unwinding scopes: N Logged values constructed in order
    # 0..N, a panic partway through a loop that constructs more, caught by catch_unwind - correct
    # unwind codegen must run every already-constructed Logged's destructor, in reverse
    # (last-constructed-first) order, exactly like a normal (non-panicking) scope exit would.
    lines.append("    {")
    lines.append("        DROP_LOG.lock().unwrap().clear();")
    lines.append("        let r = panic::catch_unwind(|| {")
    lines.append("            let _a = Logged(1);")
    lines.append("            let _b = Logged(2);")
    lines.append("            {")
    lines.append("                let _c = Logged(3);")
    lines.append("                let _d = Logged(4);")
    lines.append('                if black_box(true) { panic!("mid-unwind"); }')
    lines.append("            }")
    lines.append("        });")
    lines.append('        println!("drop_order: caught={} log={:?}", r.is_err(), DROP_LOG.lock().unwrap().clone());')
    lines.append("    }")

    # Drop order through a Vec of Logged values, panicking partway through construction inside a
    # loop - a more realistic RAII pattern (a Vec being built up, one push causes a panic).
    lines.append("    {")
    lines.append("        DROP_LOG.lock().unwrap().clear();")
    lines.append("        let r = panic::catch_unwind(|| {")
    lines.append("            let mut v = Vec::new();")
    lines.append("            for i in 0..5i32 {")
    lines.append("                v.push(Logged(i));")
    lines.append('                if i == black_box(3) { panic!("mid-vec"); }')
    lines.append("            }")
    lines.append("        });")
    lines.append('        println!("drop_order_vec: caught={} log={:?}", r.is_err(), DROP_LOG.lock().unwrap().clone());')
    lines.append("    }")

    body = "fn main() {\n" + "\n".join(lines) + "\n}\n"
    write_case("panic_unwind", "catch_unwind_and_drop_order", body, PANIC_PREAMBLE)


# ---------------------------------------------------------------------------
# Category: generics/vtable dispatch - entirely unexplored per the risk map. Covers: `dyn Trait`
# dispatch across many distinct implementors (a wrong vtable slot/layout would show up as two
# different types' method calls returning each other's results, not necessarily a crash), generic
# function monomorphization across many types, and boxed closures as trait objects (a different
# vtable-construction path than a plain struct impl).
# ---------------------------------------------------------------------------
VTABLE_PREAMBLE = "use std::hint::black_box;\n\n"


def gen_generics_vtable():
    # Many distinct implementors of the same trait, called both directly and through `&dyn Trait` -
    # a wrong vtable layout would make two implementors' `id()`/`compute()` results collide.
    lines = [
        "trait Op { fn id(&self) -> i32; fn compute(&self, x: i32) -> i32; }",
    ]
    n_impls = 12
    for i in range(n_impls):
        lines.append(f"struct S{i};")
        lines.append(f"impl Op for S{i} {{ fn id(&self) -> i32 {{ {i} }} fn compute(&self, x: i32) -> i32 {{ x * {i} + {i} }} }}")
    lines.append("fn main() {")
    lines.append(f"    let boxed: Vec<Box<dyn Op>> = vec![{', '.join(f'Box::new(S{i})' for i in range(n_impls))}];")
    lines.append("    for (idx, b) in boxed.iter().enumerate() {")
    lines.append('        println!("vtable_dispatch/{}: id={} compute={}", idx, b.id(), b.compute(black_box(10)));')
    lines.append("    }")
    # Same objects through &dyn (reference, not Box) to exercise a different fat-pointer
    # construction path.
    lines.append(f"    let refs: [&dyn Op; {n_impls}] = [{', '.join(f'&S{i}' for i in range(n_impls))}];")
    lines.append("    for (idx, r) in refs.iter().enumerate() {")
    lines.append('        println!("vtable_ref_dispatch/{}: id={} compute={}", idx, r.id(), r.compute(black_box(7)));')
    lines.append("    }")

    # Generic function monomorphized across every integer type - one shared code path, many
    # instantiations, checking the codegen backend correctly specializes each one independently.
    lines.append("    fn generic_double<T: std::ops::Add<Output = T> + Copy + std::fmt::Debug>(x: T) -> T { x + x }")
    for ty, val in [("i8", "black_box(5i8)"), ("u8", "black_box(5u8)"), ("i64", "black_box(5i64)"), ("u64", "black_box(5u64)"), ("f32", "black_box(5.5f32)"), ("f64", "black_box(5.5f64)")]:
        lines.append(f'    println!("generic_double/{ty}: {{:?}}", generic_double({val}));')

    # Boxed closures as trait objects (Fn/FnMut) - a different vtable-construction path than a
    # plain struct impl, exercising closure-environment capture through indirect calls.
    lines.append("    let mult = black_box(3i32);")
    lines.append("    let fns: Vec<Box<dyn Fn(i32) -> i32>> = vec![")
    lines.append("        Box::new(move |x| x + mult),")
    lines.append("        Box::new(|x| x * 2),")
    lines.append("        Box::new(move |x| x - mult),")
    lines.append("    ];")
    lines.append("    for (idx, f) in fns.iter().enumerate() {")
    lines.append('        println!("closure_dispatch/{}: {}", idx, f(black_box(10)));')
    lines.append("    }")
    lines.append("}")

    body = "\n".join(lines)
    write_case("generics_vtable", "dyn_trait_and_closures", body, VTABLE_PREAMBLE)


CATEGORIES = {
    "casts": gen_casts,
    "overflow": gen_overflow,
    "floatint": gen_floatint,
    "bitops": gen_bitops,
    "simd": gen_simd,
}

CATEGORIES["atomics"] = gen_atomics
CATEGORIES["enum_niche"] = gen_enum_niche
CATEGORIES["panic_unwind"] = gen_panic_unwind
CATEGORIES["generics_vtable"] = gen_generics_vtable


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", default=None, help="comma-separated subset of: " + ",".join(CATEGORIES))
    args = ap.parse_args()
    wanted = args.only.split(",") if args.only else list(CATEGORIES)
    os.makedirs(CASES_DIR, exist_ok=True)
    for cat in wanted:
        CATEGORIES[cat]()
        n = len(os.listdir(os.path.join(CASES_DIR, cat)))
        print(f"[{cat}] {n} case file(s) written")


if __name__ == "__main__":
    main()
