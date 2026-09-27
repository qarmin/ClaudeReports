//! Differential check of rustc's backend-provided `--print` requests: plain rustc (LLVM) vs
//! `rustc-clif` (cg_clif).
//!
//! rustc routes six print requests through `CodegenBackend::print`
//! (`rustc_driver_impl/src/lib.rs`: `RelocationModels | CodeModels | TlsModels | TargetCPUs |
//! StackProtectorStrategies | TargetFeatures => codegen_backend.print(..)`). The trait's default
//! body is empty, so a backend that does not override it answers every one of them with an empty
//! string and exit code 0.
//!
//! Env vars:
//!   RUSTC_LLVM  - path to plain rustc            (default: `rustc`)
//!   RUSTC_CLIF  - path to `dist/rustc-clif`      (required)
//!   TARGET      - target tuple to ask about      (default: host default)

use std::path::PathBuf;
use std::process::Command;

/// Print kinds that rustc dispatches to `CodegenBackend::print`, and a substring that any
/// non-degenerate answer for an x86_64 target has to contain.
const KINDS: &[(&str, &str)] = &[
    // The upstream `tests/run-make/print-to-output` test asserts exactly this substring.
    ("relocation-models", "dynamic-no-pic"),
    ("code-models", "small"),
    ("tls-models", "global-dynamic"),
    ("stack-protector-strategies", "strong"),
    ("target-cpus", "native"),
    ("target-features", "sse2"),
];

fn run(rustc: &str, args: &[String]) -> (String, String, i32) {
    let out = Command::new(rustc)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {rustc}: {e}"));
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

fn main() {
    let llvm = std::env::var("RUSTC_LLVM").unwrap_or_else(|_| "rustc".to_string());
    let clif = std::env::var("RUSTC_CLIF").expect("set RUSTC_CLIF to dist/rustc-clif");
    let target = std::env::var("TARGET").ok();

    let tmp = PathBuf::from(std::env::var("OUT").unwrap_or_else(|_| ".".to_string()));

    let mut failures = Vec::new();

    println!("LLVM rustc: {llvm}");
    println!("clif rustc: {clif}");
    if let Some(t) = &target {
        println!("target    : {t}");
    }
    println!();

    for (kind, needle) in KINDS {
        let mut args = vec!["--print".to_string(), kind.to_string()];
        if let Some(t) = &target {
            args.push(format!("--target={t}"));
        }

        let (l_out, _, l_code) = run(&llvm, &args);
        let (c_out, c_err, c_code) = run(&clif, &args);

        println!("=== --print {kind} ===");
        println!(
            "  LLVM: exit {l_code}, {} bytes, {} lines, contains {:?}: {}",
            l_out.len(),
            l_out.lines().count(),
            needle,
            l_out.contains(needle)
        );
        println!(
            "  clif: exit {c_code}, {} bytes, {} lines, contains {:?}: {}",
            c_out.len(),
            c_out.lines().count(),
            needle,
            c_out.contains(needle)
        );
        if !c_err.trim().is_empty() {
            println!("  clif stderr: {}", c_err.trim());
        }

        if !l_out.contains(needle) {
            println!("  SKIP (LLVM oracle itself lacks the needle - not a cg_clif finding)");
            println!();
            continue;
        }
        if !c_out.contains(needle) {
            failures.push(format!(
                "--print {kind}: clif printed {} bytes and does not contain {:?} (LLVM printed {} bytes)",
                c_out.len(),
                needle,
                l_out.len()
            ));
        }

        // Second form the upstream run-make test exercises: `--print KIND=PATH`.
        let path = tmp.join(format!("{kind}.txt"));
        let _ = std::fs::remove_file(&path);
        let mut file_args = vec!["--print".to_string(), format!("{kind}={}", path.display())];
        if let Some(t) = &target {
            file_args.push(format!("--target={t}"));
        }
        let (_, _, fc) = run(&clif, &file_args);
        let written = std::fs::read_to_string(&path).unwrap_or_default();
        println!(
            "  clif --print {kind}=<file>: exit {fc}, file is {} bytes",
            written.len()
        );
        if written.trim().is_empty() {
            failures.push(format!(
                "--print {kind}=<file>: clif wrote an empty file"
            ));
        }
        println!();
    }

    if failures.is_empty() {
        println!("OK: cg_clif answers every backend print request non-degenerately.");
    } else {
        println!("MISMATCH ({} findings):", failures.len());
        for f in &failures {
            println!("  - {f}");
        }
        std::process::exit(1);
    }
}
