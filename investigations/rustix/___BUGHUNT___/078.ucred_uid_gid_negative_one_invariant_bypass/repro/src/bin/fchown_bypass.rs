// Standalone reproduction for candidate 078
// (ucred_uid_gid_negative_one_invariant_bypass).
//
// `Uid::from_raw`/`Gid::from_raw` (src/ugid.rs) document "`raw` must be the value of a valid Unix
// user ID, and not `-1`", but (pre-fix) enforce it only via `debug_assert_ne!(raw, !0)`, which is
// compiled out entirely in a release build. `Uid`/`Gid` wrap a plain `u32` with no niche, so
// constructing `Uid::from_raw(u32::MAX)` is not Undefined Behavior - but
// `src/ugid.rs::translate_fchown_args` uses the exact same bit pattern (`!0`, i.e. `u32::MAX`) as
// its own sentinel for "no change requested" (the `None` case, matching POSIX `chown(2)`'s own
// `-1 = don't change` convention). So a `Uid`/`Gid` holding the one value the type's own
// documentation says must never occur is silently indistinguishable, at the raw-syscall level,
// from "caller passed `None`" - `fchown`'s real kernel permission check for "can this process
// change this file's owner to that id" is never even reached.
//
// This is a genuine, real, single-process use of rustix's own public API (`rustix::fs::fchown`,
// `rustix::fs::Uid::from_raw`) - no second participant (no peer, no network) is faked here, so
// CONFIRMATION.md section 11's bar doesn't apply (there is no second participant to fake in the
// first place: this is a single caller violating a documented precondition on its own input, the
// same way `i.e. slice::get_unchecked(out_of_bounds)` would be).
//
// Exit code convention:
//   0  = FIXED: the invalid `Uid`/`Gid` was rejected (panicked) before ever reaching `fchown`,
//        so the silent permission-check bypass can no longer happen.
//   1  = BUG reproduced: `fchown` silently returned `Ok(())` without actually changing the
//        owner, and without ever consulting the real kernel permission check.
//   2  = unexpected result unrelated to this bug (environment problem) - investigate separately.
//
// Must be run with `cargo run --release` to see the pre-fix bug: in a *debug* build,
// `debug_assert_ne!` already panics at construction time, same observable shape as the fixed
// behavior - the release build is what actually bypasses the check pre-fix (CONFIRMATION.md
// section 2: a bug that needs a specific build to manifest is still real).

use rustix::fs::{fchown, openat, Mode, OFlags, Uid, CWD};
use std::os::unix::fs::MetadataExt;
use std::panic;

fn main() {
    let path = std::env::temp_dir().join(format!("rustix_078_repro_{}", std::process::id()));
    std::fs::write(&path, b"rustix candidate 078 repro").unwrap();
    let fd = openat(CWD, &path, OFlags::RDWR, Mode::empty()).expect("open temp file");

    // Baseline: confirm the real kernel permission check is actually active in this environment -
    // an unprivileged process must NOT be able to chown to an arbitrary different, validly-shaped
    // uid.
    let baseline = fchown(&fd, Some(Uid::from_raw(99_999)), None);
    match &baseline {
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            println!("baseline OK: fchown to a real but foreign uid correctly returned EPERM: {e:?}");
        }
        other => {
            println!(
                "UNEXPECTED environment: fchown to a foreign uid did not EPERM ({other:?}) - \
                 this repro assumes it's run unprivileged without CAP_CHOWN; cannot proceed"
            );
            std::process::exit(2);
        }
    }

    let before = std::fs::metadata(&path).unwrap();
    let (before_uid, before_gid) = (before.uid(), before.gid());

    // The actual suspect construction: a `Uid` holding the exact value the type's own
    // documentation says must never occur.
    let built = panic::catch_unwind(|| Uid::from_raw(u32::MAX));

    match built {
        Err(_) => {
            println!(
                "FIXED: Uid::from_raw(u32::MAX) was rejected (panicked) before ever reaching \
                 fchown - the silent permission-check bypass can no longer happen"
            );
            let _ = std::fs::remove_file(&path);
            std::process::exit(0);
        }
        Ok(bad_uid) => {
            println!(
                "Uid::from_raw(u32::MAX) did NOT panic - got Uid::as_raw() = {}",
                bad_uid.as_raw()
            );

            let result = fchown(&fd, Some(bad_uid), None);
            let after = std::fs::metadata(&path).unwrap();
            let (after_uid, after_gid) = (after.uid(), after.gid());

            println!("fchown(Some(Uid::from_raw(u32::MAX)), None) => {result:?}");
            println!("owner before = ({before_uid}, {before_gid}), after = ({after_uid}, {after_gid})");

            let _ = std::fs::remove_file(&path);

            if result.is_ok() && after_uid == before_uid {
                println!(
                    "BUG: fchown silently returned Ok(()) and left the owner completely \
                     unchanged - the request to chown to uid {} was silently turned into a no-op, \
                     indistinguishable from `owner: None`, bypassing the real kernel permission \
                     check entirely (contrast with the baseline EPERM above)",
                    bad_uid.as_raw()
                );
                std::process::exit(1);
            } else {
                println!(
                    "UNEXPECTED: fchown neither silently no-op'd nor matched the known bug shape - \
                     investigate separately"
                );
                std::process::exit(2);
            }
        }
    }
}
