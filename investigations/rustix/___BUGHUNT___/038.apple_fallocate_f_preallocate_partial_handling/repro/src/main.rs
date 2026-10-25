// Standalone reproduction for rustix GitHub issue #1682 ("Apple fallocate can return Ok after
// only a partial F_PREALLOCATE allocation"), still present in rustix HEAD 287214b8 (v1.1.5) as
// of 2026-10-01 - confirmed by diffing this repro's extracted logic against the exact lines the
// issue cites (src/backend/libc/fs/syscalls.rs:1797-1825 in that commit), which this session
// re-read and found byte-for-byte identical to what the issue quotes.
//
// WHY THIS DOESN'T ACTUALLY RUN ON macOS (see RESULT.md "Fully reproducible: No" for the full
// account, this is the short version): this session has no macOS hardware, no macOS CI, and no
// way to execute Mach-O code at all in this Linux container.
//   - Cross-COMPILING the real `cfg(apple)` code path in rustix's own source for
//     `x86_64-apple-darwin` *does* work here (`rustup target add x86_64-apple-darwin` ships a
//     prebuilt std, and the vendored `libc` crate's Apple `fstore_t`/`F_PREALLOCATE`/
//     `F_ALLOCATECONTIG`/`F_ALLOCATEALL` definitions are plain Rust, no macOS SDK needed to
//     typecheck) - `cargo build --target x86_64-apple-darwin --features fs,std` against this
//     repo succeeds both before and after the patch. That only confirms the code compiles for
//     that target, not that it behaves as described at runtime.
//   - Actually LINKING a real Mach-O executable that calls it does not work here: a minimal
//     crate calling `rustix::fs::fallocate` and built with `--target x86_64-apple-darwin` fails
//     at the link step with `cc: error: unrecognized command-line option '-arch'` /
//     `'-mmacosx-version-min=...'` - this container's `cc` is plain Linux `gcc`, there is no
//     macOS SDK (`xcrun --sdk macosx --show-sdk-path` fails: "No such file or directory") and no
//     Darwin cross-linker package is installed or available via apt.
//   - Even a successfully linked Mach-O binary could not be *executed* here: `qemu-user`'s
//     `qemu-x86_64`/`qemu-aarch64` etc. (present in this environment) translate Linux-ABI ELF
//     syscalls for a foreign CPU architecture - they do not, and architecturally cannot, emulate
//     XNU/Darwin's own distinct syscall ABI or Mach-O loading, so qemu-user is not a substitute
//     for real Apple hardware here, unlike this project's own sparc/aarch64/riscv64 Linux
//     candidates.
//   - The bug itself is also not a static, compile-time-constant mismatch (unlike candidate
//     028's sparc `NCCS` bug, which could be reproduced by substituting real per-architecture
//     *constants* into rustix's own extracted logic) - it depends on the real XNU kernel's
//     *runtime* decision of how many bytes to actually reserve for a given `F_PREALLOCATE`
//     request on a given filesystem, which is not a value this session can independently compute
//     or observe without the real kernel.
//
// What this repro does instead, per CONFIRMATION.md section 11's hardware exception: it extracts
// rustix's own real control flow (both the pre-fix and this session's patched version) verbatim,
// and drives it with a minimal mock standing in for `libc::fcntl`/`libc::ftruncate` - but the
// mock's *return values* are not invented: they are the exact, real numbers independently
// reported by the issue's own author from an actual macOS 15.7.4/Darwin 24.6.0 arm64 run against
// real APFS volumes (quoted verbatim in the issue body, reproduced in the comments below). This
// demonstrates the real control-flow defect (the unconditional `ftruncate` after a "successful"
// but partial `F_PREALLOCATE`) using real, externally-observed kernel behavior as the test
// fixture, not a guessed or hypothetical response.
//
// Usage: `repro before <scenario>` runs the pre-fix control flow; `repro after <scenario>` runs
// this session's patched control flow. `<scenario>` is `plenty` (issue's first repro: 128 MiB
// request on a volume with enough space) or `scarce` (issue's second repro: 200 MiB request on a
// 128 MiB image, i.e. insufficient real space).

/// Mirrors `libc::fstore_t` (libc crate 0.2.189, `src/unix/bsd/apple/mod.rs:363-369`) - the exact
/// field set and order rustix's own `fallocate` constructs and reads.
#[derive(Debug, Clone, Copy)]
struct FstoreT {
    fst_flags: u32,
    fst_posmode: i32,
    fst_offset: i64,
    fst_length: i64,
    fst_bytesalloc: i64,
}

const F_ALLOCATECONTIG: u32 = 0x02;
const F_ALLOCATEALL: u32 = 0x04;

/// Stand-in for the real XNU kernel's `fcntl(fd, F_PREALLOCATE, &store)`, parameterized by a
/// scenario whose numbers are taken verbatim from the issue's own real-hardware output (see each
/// scenario's own comment for the exact quoted figures). Mutates `store.fst_bytesalloc` and
/// returns the `fcntl` return value (0 on success, -1 on failure), exactly mirroring what real
/// `libc::fcntl` returns - rustix's own code never inspects anything else.
fn mock_fcntl_preallocate(store: &mut FstoreT, scenario: Scenario) -> i32 {
    match (scenario, store.fst_flags) {
        // Issue body, "On the normal APFS data volume": requested_bytes=134217728,
        // fallocate_result=Ok(()), after_fallocate allocated_size=3088384. F_ALLOCATECONTIG
        // "succeeds" (returns 0, not -1) despite reserving only ~2.3% of the request.
        (Scenario::Plenty, F_ALLOCATECONTIG) => {
            store.fst_bytesalloc = 3_088_384;
            0
        }
        // Issue body, "F_ALLOCATEALL control", sufficient-space case: "a 128 MiB request returns
        // success with fst_bytesalloc = 134217728". This is what the fixed code's retry sees.
        (Scenario::Plenty, F_ALLOCATEALL) => {
            store.fst_bytesalloc = store.fst_length;
            0
        }
        // Issue body, "Insufficient-space reproduction": requested_bytes=209715200,
        // fallocate_result=Ok(()), after_fallocate allocated_size=104857600 (on a 128 MiB image
        // with 132767744 bytes available before the call). F_ALLOCATECONTIG again "succeeds"
        // while reserving only about half the request.
        (Scenario::Scarce, F_ALLOCATECONTIG) => {
            store.fst_bytesalloc = 104_857_600;
            0
        }
        // Issue body, "F_ALLOCATEALL control", insufficient-space case: "a 200 MiB request
        // returns -1 with ENOSPC, fst_bytesalloc = 0". This is what the fixed code's retry sees.
        (Scenario::Scarce, F_ALLOCATEALL) => {
            store.fst_bytesalloc = 0;
            -1
        }
        _ => unreachable!("rustix's fallocate only ever sets fst_flags to these two values"),
    }
}

/// Stand-in for `libc::ftruncate` - real XNU `ftruncate` always succeeds here (both of the
/// issue's own repros show it does: "after_sync: logical_size=... " matches the requested size
/// in both scenarios), so it unconditionally "succeeds" in this mock too. The point of this bug
/// is entirely in whether rustix reaches this call at all with an under-allocated `store`, not in
/// `ftruncate` itself misbehaving.
fn mock_ftruncate(logical_size: &mut i64, new_len: i64) -> i32 {
    *logical_size = new_len;
    0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Scenario {
    Plenty,
    Scarce,
}

impl Scenario {
    fn parse(s: &str) -> Self {
        match s {
            "plenty" => Scenario::Plenty,
            "scarce" => Scenario::Scarce,
            other => {
                eprintln!("unknown scenario {other:?}, expected 'plenty' or 'scarce'");
                std::process::exit(2);
            }
        }
    }

    fn new_len(self) -> i64 {
        match self {
            Scenario::Plenty => 134_217_728, // 128 MiB, issue's first repro
            Scenario::Scarce => 209_715_200, // 200 MiB, issue's second repro
        }
    }
}

/// Verbatim extraction of the PRE-FIX control flow, rustix HEAD 287214b8,
/// src/backend/libc/fs/syscalls.rs:1817-1824 (`fcntl`/`ftruncate` calls replaced by the mocks
/// above; everything else - the branch condition, the flag mutation, the unconditional
/// `ftruncate` - is the same shape as the real source):
///
///     if c::fcntl(borrowed_fd(fd), c::F_PREALLOCATE, &store) == -1 {
///         // Unable to allocate contiguous disk space; attempt to allocate non-contiguously.
///         store.fst_flags = c::F_ALLOCATEALL;
///         let _ = ret_c_int(c::fcntl(borrowed_fd(fd), c::F_PREALLOCATE, &store))?;
///     }
///     ret(c::ftruncate(borrowed_fd(fd), new_len))
fn fallocate_before(scenario: Scenario) -> Result<(i64, i64), i32> {
    let new_len = scenario.new_len();
    let mut store = FstoreT {
        fst_flags: F_ALLOCATECONTIG,
        fst_posmode: 0,
        fst_offset: 0,
        fst_length: new_len,
        fst_bytesalloc: 0,
    };
    let mut logical_size: i64 = 0;

    if mock_fcntl_preallocate(&mut store, scenario) == -1 {
        store.fst_flags = F_ALLOCATEALL;
        if mock_fcntl_preallocate(&mut store, scenario) == -1 {
            return Err(-1);
        }
    }
    let rc = mock_ftruncate(&mut logical_size, new_len);
    if rc == -1 {
        return Err(rc);
    }
    Ok((logical_size, store.fst_bytesalloc))
}

/// Verbatim extraction of this session's PATCHED control flow (patch.diff), same real source
/// location:
///
///     if c::fcntl(borrowed_fd(fd), c::F_PREALLOCATE, &store) == -1
///         || store.fst_bytesalloc < new_len
///     {
///         store.fst_flags = c::F_ALLOCATEALL;
///         let _ = ret_c_int(c::fcntl(borrowed_fd(fd), c::F_PREALLOCATE, &store))?;
///     }
///     ret(c::ftruncate(borrowed_fd(fd), new_len))
fn fallocate_after(scenario: Scenario) -> Result<(i64, i64), i32> {
    let new_len = scenario.new_len();
    let mut store = FstoreT {
        fst_flags: F_ALLOCATECONTIG,
        fst_posmode: 0,
        fst_offset: 0,
        fst_length: new_len,
        fst_bytesalloc: 0,
    };
    let mut logical_size: i64 = 0;

    let first_rc = mock_fcntl_preallocate(&mut store, scenario);
    if first_rc == -1 || store.fst_bytesalloc < new_len {
        store.fst_flags = F_ALLOCATEALL;
        if mock_fcntl_preallocate(&mut store, scenario) == -1 {
            return Err(-1);
        }
    }
    let rc = mock_ftruncate(&mut logical_size, new_len);
    if rc == -1 {
        return Err(rc);
    }
    Ok((logical_size, store.fst_bytesalloc))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: repro <before|after> <plenty|scarce>");
        std::process::exit(2);
    }
    let mode = args[1].as_str();
    let scenario = Scenario::parse(&args[2]);

    match mode {
        "before" => match fallocate_before(scenario) {
            Ok((logical_size, bytesalloc)) => {
                println!("PRE-FIX fallocate({:?}): Ok(()), logical_size={}, actual_bytes_allocated={}", scenario, logical_size, bytesalloc);
                if bytesalloc < scenario.new_len() {
                    println!("BUG REPRODUCED: reported success but only reserved {bytesalloc} of {} requested bytes - file is logically {logical_size} bytes but under-backed by real storage", scenario.new_len());
                } else {
                    println!("bug not reproduced in this scenario (fully allocated)");
                }
            }
            Err(e) => println!("PRE-FIX fallocate({:?}): Err({e}) (ENOSPC)", scenario),
        },
        "after" => match fallocate_after(scenario) {
            Ok((logical_size, bytesalloc)) => {
                println!("PATCHED fallocate({:?}): Ok(()), logical_size={}, actual_bytes_allocated={}", scenario, logical_size, bytesalloc);
                assert_eq!(bytesalloc, scenario.new_len(), "patched code returned Ok(()) despite under-allocation - fix did not hold");
                println!("OK: success implies full allocation, as expected");
            }
            Err(e) => println!("PATCHED fallocate({:?}): Err({e}) (ENOSPC) - no ftruncate performed, file left untouched", scenario),
        },
        _ => {
            eprintln!("usage: repro <before|after> <plenty|scarce>");
            std::process::exit(2);
        }
    }
}
