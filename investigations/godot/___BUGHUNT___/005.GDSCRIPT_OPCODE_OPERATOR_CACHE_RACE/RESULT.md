# GDScript VM `OPCODE_OPERATOR` first-run cache: publish-before-payload-complete race

Thread-focused bug-hunt session, picked from `IDEAS.md`'s backlog. Matches a real, open
upstream issue, [godotengine/godot#124010](https://github.com/godotengine/godot/issues/124010),
where the reporter already confirmed a crash (`0xC0000005` access violation on Windows) from
calling the same GDScript function concurrently from multiple threads for the first time.

`OPCODE_OPERATOR` (`modules/gdscript/gdscript_vm.cpp:777-825`) caches the resolved operator
evaluator for an arithmetic/comparison expression (e.g. `a + b`) directly in the function's own
bytecode array (`_code_ptr`), so repeat executions skip the slow, type-dispatching
`Variant::evaluate()` path. The first time a given expression runs, under `initializer_mutex`,
the code writes **three** words to `_code_ptr` in sequence - the return type (`ip+6`), the
function pointer (`ip+7`/`ip+8`), and *last* the "ready" signature (`ip+5`) that other threads
check. The problem: the signature write and the two payload writes before it are three ordinary,
unsynchronized `int` stores with no memory-ordering relationship to each other, and the **read**
side (`op_signature = _code_ptr[ip + 5];`, taken with no lock at all, every single time this
opcode runs) has no ordering relationship to them either. A second thread executing the exact
same bytecode offset concurrently can observe the just-published signature while the return-type/
function-pointer writes are still not visible to it, take the "fast path", and call through a
garbage function pointer.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading + call-graph trace | n/a | Confirmed the three-word publish (`_code_ptr[ip+5]`/`[ip+6]`/`[ip+7..8]`) is unsynchronized plain-`int` stores and the fast-path read is an unsynchronized plain-`int` load with no lock, matching issue #124010's description exactly. Reachable from **any** GDScript binary operator expression (`+`, `-`, `==`, ...) - any function containing one, called concurrently from multiple `Thread`s for the first time |
| GDScript-level repro, naive version (`main.gd` v1, not kept): 32 `Thread`s each calling 400 distinct never-before-run `op_i(a,b): return a+b` methods in the same order, no synchronization beyond `Thread.start()`'s own timing | Real, self-built ASan editor binary (`scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes`), `--headless --path gdscript_repro`, 5 runs, ~15-20s each | **0/5 crashed** - `Thread.start()`'s own scheduling jitter spreads out when each thread first reaches a given opcode too widely to reliably land in the race window |
| GDScript-level repro, barrier-synchronized version (kept as `gdscript_repro/main.gd`): a `Mutex`+counter barrier makes all 32 threads call the **same** never-before-run `op_i` at (as close as possible to) the same instant, repeated across 300 distinct expressions | Same binary/build, `--headless --path gdscript_repro`, 10 runs, ~10-15s each | **Crashed 4/10 runs** (SIGSEGV, signal 11) - see "Status" below |
| Fix verification: same `gdscript_repro/` against the fixed, rebuilt binary | Same binary/build (incremental rebuild, ~48s - only `gdscript_vm.cpp` changed), 8 runs | **Clean 8/8 runs**, "No crash within the time limit" every time |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-004 (same pre-existing `test_ip.cpp` IPv6 environment failure). Only run once, against the fixed binary (see "The fix" below for why a separate "before" run was judged unnecessary) |

## Status

**Confirmed.**

Reproduces **4/10** runs crashing (SIGSEGV) with the barrier-synchronized repro against the
unfixed binary; **0/10 (8 explicitly re-run + 2 earlier in this session's own testing)** crash
with the fix applied, across the same number of runs. All runs: real, self-built
`target=editor use_asan=yes tests=yes debug_symbols=yes` ASan binary, `--headless`, 32 concurrent
`GDScript` `Thread`s, no fixed seed/thread-pinning, 24-core machine. This is a genuinely narrow
race (4/10, not the near-100% hit rates seen in candidates 002-004) - see "Root cause" for why,
and "What's not done" for why the rate isn't higher.

Every crashing run shows the same signature: `signal 11` (SIGSEGV) with a **truncated,
non-symbolized C++ backtrace** (only `handle_crash`/libc frames, no further unwinding) - a strong
indicator of calling through a corrupted/garbage function pointer (the crash destroys the normal
stack-frame chain the unwinder relies on), landing inside whichever `op_N` GDScript method the
barrier happened to release threads into at that moment (different `N` each time: `op_96`,
`op_21`, `op_0`, `op_13` across the four crashing runs) - consistent with the race being timing-
dependent on *which* expression's first run gets contended, not tied to one specific line.

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `GDScriptFunction::call`/`OPCODE_OPERATOR` path
through nothing but public GDScript syntax (`a + b` inside ordinary methods, `Thread.start()`,
`Mutex`). No standalone C++ mirror was built for this candidate (per this project's current repro
policy) - this bug has a real GDScript entry point, so none was needed.

## Class

`memory-safety` (control-flow hijack via an uninitialized/partial function pointer read - a
"publish-before-payload-complete" race, distinct from candidates 002-004's "locked-writer vs.
unlocked-reader on a container" shape)

## Root cause

`modules/gdscript/gdscript_vm.cpp:777-825`. On a cache miss (`op_signature == 0`), under
`initializer_mutex`, the code does, in order:

```cpp
op_func(a, b, dst);                                    // use the result immediately (fine)
if (_code_ptr[ip + 5] == 0) {                           // re-check under lock (fine)
    _code_ptr[ip + 5] = actual_signature;                // (1) publish signature FIRST
    _code_ptr[ip + 6] = static_cast<int>(ret_type);       // (2) ...then ret_type
    *reinterpret_cast<Evaluator *>(&_code_ptr[ip+7]) = op_func; // (3) ...then the func ptr
}
```

Every *other* thread's fast path reads `_code_ptr[ip+5]` with **no lock**, every single time this
opcode executes - not just on a cache miss:

```cpp
uint32_t op_signature = _code_ptr[ip + 5];               // unsynchronized read, always
...
} else if (likely(op_signature == actual_signature)) {
    Variant::Type ret_type = static_cast<Variant::Type>(_code_ptr[ip + 6]);   // (2)
    Evaluator op_func = *reinterpret_cast<Evaluator *>(&_code_ptr[ip + 7]);    // (3)
    op_func(a, b, dst);                                   // CALL THROUGH IT
```

Because writes (1)/(2)/(3) above are plain, unsynchronized `int` stores (no `volatile`, no
atomics, no fence), and the matching read of (1) is likewise a plain unsynchronized load, there
is no guarantee - from the compiler *or* the hardware memory model - that a thread observing (1)'s
new value has also observed (2) and (3)'s new values. A thread that reads the just-published
signature but still sees stale (zero-initialized, i.e. null) `ret_type`/`op_func` words takes the
"fast path" and calls through a null-or-garbage function pointer - exactly the SIGSEGV with a
destroyed call stack seen in every crashing run.

Reachability: trivial. Any two or more threads (`Thread`, `WorkerThreadPool`, or just ordinary
concurrent scene instantiation of the same script class) executing the *same* function containing
*any* binary operator expression for the first time race on this. Real games commonly load/warm
up multiple instances of the same script concurrently (e.g. spawning many enemies that each run
their `_ready()`/`_process()` for the first time close together, or explicit worker threads doing
gameplay math) - this is not a contrived scenario.

## Introduced by

`not determined`. The "check again in case another thread already set it" comment already shows
the original author was aware of *a* multi-threading hazard here and added a partial mitigation
(the re-check under the lock prevents clobbering, and the lock itself prevents two writers from
corrupting each other) - but the mitigation only protects the writer side; readers were never
given a corresponding synchronization point. Not bisected this session.

## The fix

Treat the signature word as the synchronization point with proper acquire/release ordering,
instead of a plain `int`: readers take an **acquire** load of the signature, and the writer does
an **release** store of the signature *last*, after the ret_type/op_func payload is fully written.
Per release-acquire semantics, any reader that observes the new signature value is thereby
guaranteed to also observe the completed payload writes that happened-before it - closing the
race without taking a lock on the hot fast-path (which would defeat the entire point of this
cache: avoiding `Variant::evaluate()`'s slow dispatch on every single arithmetic operation in
GDScript). `_code_ptr` is a plain `int *`; the existing code already reinterpret_casts slices of
it to `Variant::ValidatedOperatorEvaluator *` for the function-pointer field, so reinterpreting
the signature slot as `std::atomic<uint32_t> *` (same size/alignment as `int` on this target) is
consistent with the file's existing style, not a new idiom (`+10/-4` against
`modules/gdscript/gdscript_vm.cpp`, see `patch.diff`):

```cpp
uint32_t op_signature = reinterpret_cast<std::atomic<uint32_t> *>(&_code_ptr[ip + 5])
                             ->load(std::memory_order_acquire);
...
if (reinterpret_cast<std::atomic<uint32_t> *>(&_code_ptr[ip + 5])
        ->load(std::memory_order_relaxed) == 0) {
    _code_ptr[ip + 6] = static_cast<int>(ret_type);
    *reinterpret_cast<Evaluator *>(&_code_ptr[ip + 7]) = op_func;
    reinterpret_cast<std::atomic<uint32_t> *>(&_code_ptr[ip + 5])
        ->store(actual_signature, std::memory_order_release);
}
```

Deliberately **not** a `Mutex`/`MutexLock` fix: this project's general fix-mechanism policy
prefers plain `Mutex` over the experimental `OwnedMutex`, but taking *any* mutex on the fast path
(executed on every GDScript arithmetic/comparison operation in the whole engine) would be a real
performance regression for a cache whose entire purpose is to avoid per-call overhead - this is
the one candidate this session where a lock-free atomic publish is the right tool, not a
deviation from the policy's intent.

Verified: 8/8 clean runs of `gdscript_repro/` against the rebuilt binary (vs. 4/10 crashing
before), identical test-suite results (`1431 | 1430 | 1 | 3`, same baseline).

- **Applied for real** to `modules/gdscript/gdscript_vm.cpp`, rebuilt the ASan editor binary
  (incremental, ~48s - this file is a single translation unit, no header touched, by far the
  fastest rebuild this session), ran `gdscript_repro/` and the test suite against it, then
  reverted (`git checkout -- modules/gdscript/gdscript_vm.cpp`) before finishing this candidate -
  main tree confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan+
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed |
  3 skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6
  failure, unrelated to this change). Only run once, against the fixed binary - the fix only
  changes *how* two already-agreed-upon values become visible across threads, not what value is
  computed in the single-threaded case the test suite exercises, so a separate "before" run was
  judged unnecessary.

## What's not done / open questions

- **The crash rate (4/10) is lower than candidates 002-004's near-100%** - this is expected
  for a pure memory-ordering race (as opposed to a heap-corruption race): it depends on the CPU
  actually reordering or delaying visibility of the three stores relative to the read, which is
  less consistently triggerable than a `memrealloc()`-vs-dereference race. A higher hit rate might
  be achievable with more threads, more distinct expressions per run, or by pinning threads to
  different physical cores (not attempted - the 4/10 rate plus the exactly-matching crash
  signature across all four was judged sufficient for `Confirmed`).
- Did not attempt to reproduce the exact `0xC0000005` Windows access-violation code from issue
  #124010 itself (this session's reproduction is Linux-only, ASan-instrumented) - the crash
  mechanism (call through a not-yet-fully-published function pointer) is the same, just a
  different OS's fault code for the same invalid-memory-access class.
- Not checked whether other VM opcodes have a similar multi-word unsynchronized publish pattern
  (`OPCODE_OPERATOR` was the one already named by issue #124010) - worth a follow-up sweep of
  `gdscript_vm.cpp` for other `static Mutex ...; ... _code_ptr[ip + N] = ...;` patterns.
