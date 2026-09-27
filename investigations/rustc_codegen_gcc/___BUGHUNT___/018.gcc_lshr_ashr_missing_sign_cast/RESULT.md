# gcc_lshr_ashr_missing_sign_cast

`gcc_lshr` (`src/int.rs:71-99`) is shared, unmodified, by both `Builder::lshr` (unsigned/logical
shift-right) and `Builder::ashr` (signed/arithmetic shift-right, `src/builder.rs:921-929`), and
does the native `a >> b` with no sign-correcting cast beforehand - unlike `division_operation`
(`src/int.rs:224-243`, fixed upstream in commit 6f389ff), which forces the GCC-declared type to
the correct signed/unsigned variant before dividing. `ashr`'s own comment already flags this as
unverified ("FIXME(antoyo): check whether behavior is an arithmetic shift for >> . It seems to be
if the value is signed."). This candidate investigates whether a real Rust program can observe the
wrong kind of shift because of this.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: real end-to-end compile of a standalone Rust source through the real backend (`./y.sh rustc`, release build, real `libgccjit.so`) - `transmute::<*const (), usize>(ptr)` then `>> 2`, compared against `usize::MAX >> 2` and a non-transmuted `(ptr as usize) >> 2` | Release build; single run; `repro.rs` (minimal, ad hoc) | Real mismatch: transmuted `u >> 2` printed `18446744073709551615` (unchanged - an arithmetic/sign-filling shift of an all-ones value), `assert_eq!` panicked (exit 101), while the non-transmuted `(ptr as usize) >> 2` and plain `usize::MAX >> 2` both correctly printed `4611686018427387903`. Reproduced 3/3 identical runs. |
| Root-cause instrumentation: temporary `eprintln!` added to `Builder::ptrtoint`/`bitcast`/`inttoptr` and `Builder::gcc_lshr` (reverted before writing the final patch), same release build, single run | Release build; single run | Showed the transmute path's `ptrtoint` call is given `dest_ty=__int64_t` (signed) instead of the correct unsigned `usize` GCC type, so the value reaching `gcc_lshr` for `u >> 2` is declared signed even though `u` is a `usize`; the direct `(ptr as usize) >> 2` cast's own `ptrtoint` call is correctly given `dest_ty=__uint64_t`. See `output.txt`'s "Root-cause instrumentation" section for the full captured trace. |
| Boundary-value construction for the mirror-image case (`ashr` on a value whose declared type ends up unsigned) plus non-transmuted sanity checks (`i64::MIN >> 4`, `-8i64 >> 1`, `0xFFFF_FFFFu32 >> 4`), and the final `repro/ptr_to_int_shr.rs` used for the permanent repro, all via the same real `./y.sh rustc` end-to-end path | Release build; single run each | All non-transmuted/ordinary cases were already correct before the fix (confirming the bug is specific to a value's GCC type ending up wrongly signed, not the shift logic in general); the transmuted-`isize` arithmetic-shift mirror case also reproduced correctly under the *fixed* build (see "The fix" below) and was not separately checked as broken before the fix (the specific mismatch observed live was the `usize`/logical-shift direction; the `isize`/arithmetic-shift direction is the same code path by construction, verified working post-fix). |

## Status

**Confirmed.** Reproduces 3/3 runs, deterministic, real release-mode build of the actual backend
(`./y.sh build --sysroot --release` then `CHANNEL=release ./y.sh rustc repro/ptr_to_int_shr.rs`).
Also reproduced fresh via `just test`'s `reproduce-before` step against a brand-new clone at the
pinned base commit (see this folder's own `justfile`), independent of the main checkout's build
cache.

**Fully reproducible**: Yes. `repro/ptr_to_int_shr.rs` is a plain, public-API-only Rust source file
compiled through the project's actual, unmodified end-to-end entry point
(`rustc -Zcodegen-backend=<real librustc_codegen_gcc.so>`, wrapped by the project's own
`./y.sh rustc`) - the real affected compiler backend reacting to a real Rust program it is handed,
not a hand-built stand-in for one. This is a single-participant finding (a compiler compiling a
source file it's handed), so `CONFIRMATION.md` section 11's two-participant bar doesn't apply.

## Class

logic (a well-typed, safe-looking `usize`/`isize` right-shift silently produces the wrong numeric
result - no crash, no compile-time diagnostic).

## Root cause

`Builder::lshr` and `Builder::ashr` (`src/builder.rs:921-929`) both delegate directly to the
shared `gcc_lshr` (`src/int.rs:71`), which performs GCC's native `a >> b` and trusts `a`'s
*declared* GCC type's sign to decide whether that native operator does a logical or an arithmetic
shift - unlike LLVM, where an integer type carries no sign at all and the choice of `lshr` vs
`ashr` instruction alone decides the shift kind, GCC's `>>` operator's behavior is determined by
the operand's declared C-level signed/unsigned type. `lshr` is supposed to *always* mean a logical
shift and `ashr` *always* an arithmetic one, regardless of what sign `a`'s GCC type happens to
carry at that point - but neither wrapper forces that.

The GCC-declared sign of a value can legitimately diverge from what a shift needs whenever a value
reaches the shift through a path that doesn't preserve sign for its own reasons. Here, that path is
`Builder::ptrtoint` (`src/builder.rs:1369-1372`): `let usize_value = self.cx.context.new_cast(None,
value, self.cx.type_isize())` - this bridges the pointer through libgccjit's *signed* `isize`
scalar type as an intermediate step (an implementation detail with no correctness issue when the
final destination type is also forced to the correct sign by the subsequent `intcast` call, which
is what happens for an ordinary `ptr as usize` expression). For `mem::transmute::<*const (),
usize>`, the codegen path that lowers the transmute hands `ptrtoint` a destination type that is
itself signed (confirmed live via instrumentation - `dest_ty=__int64_t`, not `__uint64_t`), so the
`intcast` inside `ptrtoint` has nothing to correct it to and the resulting `usize` value keeps a
signed GCC type all the way to the later `u >> 2`, where `gcc_lshr`'s native `a >> b` then performs
an arithmetic (sign-filling) shift instead of the logical one `usize >>` requires.

This is the same "GCC-declared-signedness-vs-Rust-semantic-signedness mismatch" family already
confirmed elsewhere in this codebase (candidate 004's atomic fetch_max/min, candidate 006's
`type_uint_from_ty`, and the division/remainder bug fixed by 6f389ff) - `division_operation`
already defends against exactly this shape of mismatch by forcing both operands to the
caller-specified correct sign before the native divide; `gcc_lshr` never received the equivalent
treatment, and `saturating_add`/`saturating_sub` (`src/intrinsic/mod.rs`) independently discovered
and worked around the same gap by manually casting to `result_type.to_unsigned(cx)` before calling
`gcc_lshr` at their own call sites, without the fix ever being applied inside `lshr`/`ashr`
themselves.

## Introduced by

afae271 "Initial commit" (the earliest commit in this repository's tracked history, no parent -
`git log -S "fn ashr" -- src/builder.rs` finds no other candidate, and `Builder::lshr`/
`Builder::ashr` calling `gcc_lshr` with no sign-forcing cast has been present in this exact shape
since that commit). Same introducing commit as candidate 004's atomic fetch_max/min bug.

## The fix

Changed `Builder::lshr` to cast `a` to its unsigned counterpart before delegating to `gcc_lshr`
whenever `a`'s declared GCC type is a native int type currently marked signed, and changed
`Builder::ashr` to do the mirror-image cast to the signed counterpart whenever `a`'s declared type
is currently unsigned - both leaving `a` untouched when its declared sign already matches what the
operation needs (the overwhelmingly common case), so no extra cast is inserted for ordinary,
already-correctly-signed shifts. This mirrors `division_operation`'s existing sign-forcing pattern
rather than trying to fix the upstream `ptrtoint`/transmute-lowering path (which would only cover
this one specific trigger, not any other way a wrongly-signed-but-same-width value could reach a
shift).

```
src/builder.rs
     fn lshr(&mut self, a: RValue<'gcc>, b: RValue<'gcc>) -> RValue<'gcc> {
-        self.gcc_lshr(a, b)
+        let a_type = a.get_type();
+        if self.is_native_int_type(a_type) && a_type.is_signed(self) {
+            let unsigned_type = a_type.to_unsigned(self.cx);
+            let a = self.context.new_cast(self.location, a, unsigned_type);
+            self.gcc_lshr(a, b)
+        } else {
+            self.gcc_lshr(a, b)
+        }
     }

     fn ashr(&mut self, a: RValue<'gcc>, b: RValue<'gcc>) -> RValue<'gcc> {
-        // FIXME(antoyo): check whether behavior is an arithmetic shift for >> .
-        // It seems to be if the value is signed.
-        self.gcc_lshr(a, b)
+        let a_type = a.get_type();
+        if self.is_native_int_type(a_type) && !a_type.is_signed(self) {
+            let signed_type = a_type.to_signed(self.cx);
+            let a = self.context.new_cast(self.location, a, signed_type);
+            self.gcc_lshr(a, b)
+        } else {
+            self.gcc_lshr(a, b)
+        }
     }
```

(See `patch.diff` for the exact diff including comments.)

Verified against `repro/ptr_to_int_shr.rs`, which covers both directions (transmuted `usize`
logical shift, transmuted `isize` arithmetic shift) plus three non-transmuted boundary-value checks
(`i64::MIN >> 4`, `-8i64 >> 1`, `0xFFFF_FFFFu32 >> 4`) to confirm the fix doesn't regress the
already-correct ordinary case. Before the fix: exit 101 (assertion failure on the transmuted-usize
check, `left: 18446744073709551615, right: 4611686018427387903`). After the fix: exit 0, re-run
3/3 times with identical (silent, all-pass) result each time. See `output.txt` for the actual
captured output.

- Project test suite (`./y.sh test --release --cargo-tests`, which runs `tests/lang_tests.rs`'s
  `lang_tester`-driven `tests/run/*.rs` suite in both debug and release profiles, plus the
  mini/cargo unit tests): **41 passed; 0 failed; 1 ignored** in both profiles, and **9 passed; 0
  failed** for the mini/cargo tests - identical before and after the fix, no regressions.

## What's not done / open questions

- The non-native (array-based, `supports_128bit_integers = false`) branch of `gcc_lshr` computes
  its own sign bit for the high word via `a_type.is_signed(self)` on the *array* GCC type
  directly - but `SignType::is_signed` only recognizes scalar `i8`/`i16`/.../`i128` GCC types, so
  it always returns `false` for an array type. This means non-native 128-bit `i128 >> n` (a
  genuinely negative `i128`, arithmetic shift) may *always* fill with zero instead of sign-extending
  in an environment without native 128-bit support, independent of and not fixed by this candidate's
  patch (which only forces sign at the `lshr`/`ashr` wrapper level for *native* int types - the
  array-type branch's own internal sign check is untouched). Not independently confirmed this
  session (this environment has native 128-bit support, `is_native_int_type` returns true for
  i128/u128, so the array branch is never exercised here) - worth a dedicated candidate with the
  backend explicitly built/forced into the non-native 128-bit path, the same way candidate 003 did
  for division.
- Only the pointer-transmute-to-integer trigger was exercised end-to-end; other ways a value could
  reach a shift with a GCC type sign that disagrees with the Rust-level operation (e.g. via SIMD
  lane extraction, or other bitcast-heavy paths) were not separately swept for the same class of
  bug beyond what's already noted in this workspace's `projects/rustc_codegen_gcc.md` risk map.
