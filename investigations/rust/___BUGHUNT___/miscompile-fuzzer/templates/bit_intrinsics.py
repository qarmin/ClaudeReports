"""Category: integer bit-manipulation intrinsics chained across inlining
boundaries (`rotate_left`/`rotate_right`, `swap_bytes`, `reverse_bits`,
`count_ones`, `leading_zeros`, `trailing_zeros`).

Targets LLVM's bit-twiddling combine/known-bits lowering for the `fshl`/
`fshr`/`bswap`/`bitreverse`/`ctpop`/`ctlz`/`cttz` intrinsics these methods
compile to - a different MIR/LLVM surface from every existing category here:
`arithmetic.py` only chains `wrapping_*`/`checked_*`/`saturating_*` (plain
arithmetic, no bit permutation), and `simd_autovec.py` targets the loop
vectorizer, not scalar bit-intrinsic combine/known-bits reasoning. Each
generated program chains several of these ops through helper functions with
randomly assigned `#[inline(always)]`/`#[inline(never)]`, interleaved with
`wrapping_add`/`^` steps so the optimizer sees a realistic mix rather than a
single intrinsic in isolation.

UB-freedom, by construction: every method used here is a total, panic-free
function of its input for *every* bit pattern of its declared integer type -
`rotate_left`/`rotate_right` take the shift amount modulo the type's bit
width internally (documented behavior, never panics regardless of the amount
passed, which is why this template deliberately picks amounts larger than
the type's width too), `swap_bytes`/`reverse_bits` are pure bit permutations,
`count_ones`/`leading_zeros`/`trailing_zeros` are pure bit counts (including
the well-defined all-zero case), and bitwise `^`/`wrapping_add` can't
overflow/panic by construction. No unsafe, no pointers, no indexing. The
initial accumulator and every operand are read through
`std::hint::black_box` so the whole chain can't be constant-folded away at
compile time.
"""

from __future__ import annotations

from . import register
from .common import INT_TYPES, Program, ident, rand_value, wrap

_OPS = ["rotate_left", "rotate_right", "swap_bytes", "reverse_bits", "xor_mask", "wrapping_add"]


def _bits_signed(ty_tuple):
    _, lo, hi = ty_tuple
    span = hi - lo + 1
    bits = span.bit_length() - 1
    return bits, lo < 0


def _to_unsigned(v: int, bits: int, signed: bool) -> int:
    if signed and v < 0:
        return v + (1 << bits)
    return v


def _from_unsigned(u: int, bits: int, signed: bool) -> int:
    u &= (1 << bits) - 1
    if signed and u >= (1 << (bits - 1)):
        return u - (1 << bits)
    return u


def _rotate_left(v: int, k: int, ty_tuple) -> int:
    bits, signed = _bits_signed(ty_tuple)
    u = _to_unsigned(v, bits, signed)
    k %= bits
    mask = (1 << bits) - 1
    rotated = u if k == 0 else ((u << k) | (u >> (bits - k))) & mask
    return _from_unsigned(rotated, bits, signed)


def _rotate_right(v: int, k: int, ty_tuple) -> int:
    bits, signed = _bits_signed(ty_tuple)
    u = _to_unsigned(v, bits, signed)
    k %= bits
    mask = (1 << bits) - 1
    rotated = u if k == 0 else ((u >> k) | (u << (bits - k))) & mask
    return _from_unsigned(rotated, bits, signed)


def _swap_bytes(v: int, ty_tuple) -> int:
    bits, signed = _bits_signed(ty_tuple)
    u = _to_unsigned(v, bits, signed)
    n_bytes = bits // 8
    b = u.to_bytes(n_bytes, "big")
    swapped = int.from_bytes(bytes(reversed(b)), "big")
    return _from_unsigned(swapped, bits, signed)


def _reverse_bits(v: int, ty_tuple) -> int:
    bits, signed = _bits_signed(ty_tuple)
    u = _to_unsigned(v, bits, signed)
    rb = int(format(u, f"0{bits}b")[::-1], 2)
    return _from_unsigned(rb, bits, signed)


def _count_ones(v: int, ty_tuple) -> int:
    bits, signed = _bits_signed(ty_tuple)
    u = _to_unsigned(v, bits, signed)
    return bin(u).count("1")


def _leading_zeros(v: int, ty_tuple) -> int:
    bits, signed = _bits_signed(ty_tuple)
    u = _to_unsigned(v, bits, signed)
    if u == 0:
        return bits
    return bits - u.bit_length()


def _trailing_zeros(v: int, ty_tuple) -> int:
    bits, signed = _bits_signed(ty_tuple)
    u = _to_unsigned(v, bits, signed)
    if u == 0:
        return bits
    count = 0
    while u & 1 == 0:
        count += 1
        u >>= 1
    return count


@register("bit_intrinsics_chain")
def generate(rng, idx) -> Program:
    ty_tuple = rng.choice(INT_TYPES)
    ty, lo, hi = ty_tuple
    bits, _signed = _bits_signed(ty_tuple)

    init = rand_value(rng, ty_tuple)
    n_ops = rng.randint(5, 10)
    chosen_ops = [rng.choice(_OPS) for _ in range(n_ops)]
    inline_attrs = [rng.choice(["#[inline(always)]", "#[inline(never)]", "#[inline]"]) for _ in range(n_ops)]

    # Rotate amounts deliberately range well past the type's own bit width,
    # to exercise the documented "amount is taken modulo the bit width"
    # wraparound behavior, not just small in-range shifts.
    rotate_amounts = [rng.randint(0, 4 * bits) for _ in range(n_ops)]
    operands = [rand_value(rng, ty_tuple) for _ in range(n_ops)]

    fns = []
    calls = []
    for k, op in enumerate(chosen_ops):
        fn_name = ident("bit_step", idx, k)
        if op == "rotate_left":
            fns.append(f"{inline_attrs[k]}\nfn {fn_name}(a: {ty}, k: u32) -> {ty} {{ a.rotate_left(k) }}")
            calls.append(f"    acc = {fn_name}(acc, {rotate_amounts[k]}u32);")
        elif op == "rotate_right":
            fns.append(f"{inline_attrs[k]}\nfn {fn_name}(a: {ty}, k: u32) -> {ty} {{ a.rotate_right(k) }}")
            calls.append(f"    acc = {fn_name}(acc, {rotate_amounts[k]}u32);")
        elif op == "swap_bytes":
            fns.append(f"{inline_attrs[k]}\nfn {fn_name}(a: {ty}) -> {ty} {{ a.swap_bytes() }}")
            calls.append(f"    acc = {fn_name}(acc);")
        elif op == "reverse_bits":
            fns.append(f"{inline_attrs[k]}\nfn {fn_name}(a: {ty}) -> {ty} {{ a.reverse_bits() }}")
            calls.append(f"    acc = {fn_name}(acc);")
        elif op == "xor_mask":
            fns.append(f"{inline_attrs[k]}\nfn {fn_name}(a: {ty}, m: {ty}) -> {ty} {{ a ^ m }}")
            calls.append(f"    acc = {fn_name}(acc, {operands[k]});")
        elif op == "wrapping_add":
            fns.append(f"{inline_attrs[k]}\nfn {fn_name}(a: {ty}, x: {ty}) -> {ty} {{ a.wrapping_add(x) }}")
            calls.append(f"    acc = {fn_name}(acc, {operands[k]});")
        else:
            raise ValueError(op)

    source = f"""#![allow(dead_code)]

{chr(10).join(fns)}

fn main() {{
    let mut acc: {ty} = std::hint::black_box({init});
{chr(10).join(calls)}
    println!("{{}}", acc);
    println!("{{}}", acc.count_ones());
    println!("{{}}", acc.leading_zeros());
    println!("{{}}", acc.trailing_zeros());
}}
"""

    acc = init
    for k, op in enumerate(chosen_ops):
        if op == "rotate_left":
            acc = _rotate_left(acc, rotate_amounts[k], ty_tuple)
        elif op == "rotate_right":
            acc = _rotate_right(acc, rotate_amounts[k], ty_tuple)
        elif op == "swap_bytes":
            acc = _swap_bytes(acc, ty_tuple)
        elif op == "reverse_bits":
            acc = _reverse_bits(acc, ty_tuple)
        elif op == "xor_mask":
            acc = acc ^ operands[k]
        elif op == "wrapping_add":
            acc = wrap(acc + operands[k], ty_tuple)

    expected = [
        str(acc),
        str(_count_ones(acc, ty_tuple)),
        str(_leading_zeros(acc, ty_tuple)),
        str(_trailing_zeros(acc, ty_tuple)),
    ]
    expected_stdout = "".join(line + "\n" for line in expected)

    return Program(
        name=f"bit_intrinsics_chain_{idx}",
        source=source,
        expected_stdout=expected_stdout,
        category="bit_intrinsics_chain",
    )
