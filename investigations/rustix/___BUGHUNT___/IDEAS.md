# Ideas

Unchecked suspects only - see `SUMMARY.md` for what's already been investigated.

Last GitHub issue/PR checked for inspiration: **#1693** (repo: bytecodealliance/rustix)

## Backlog (not yet investigated)

Consumed top-to-bottom by default when a session isn't given a specific target - see `SKILL.md`.

| Added | Suspect | Why it looks suspicious | Source |
|---|---|---|---|
| 2026-10-01 | build-failure: loongarch64 missing MADV_SOFT_OFFLINE | Incomplete lead carried over from batch triage - the note that produced this row cut off mid-sentence ("GitHub issue #1518... wait placeholder") and its source field literally says "placeholder", so there is no real description or confirmed issue number here yet. Needs the actual GitHub issue tracked down (search `MADV_SOFT_OFFLINE` + `loongarch64` in bytecodealliance/rustix issues) before this is actionable - not claimed this batch for that reason. | batch triage (incomplete/placeholder note, needs follow-up before it can be worked) |
| 2026-10-01 | **[pre-assigned #108 - do not reassign]** apple_fcntl_rdadvise_offset_len_silent_ignore (`src/backend/libc/fs/syscalls.rs:2230-2245`) | Apple `fcntl_rdadvise` silently returns `Ok(())` when `offset.try_into()` or `len.try_into()` fails converting the caller's u64 offset/len into the platform's off_t/c_int radvisory fields, swallowing the failed-conversion case instead of surfacing it or clamping. Lower confidence: surrounding comments suggest this is intentional (F_RDADVISE is just a perf hint), but it matches the error-swallowing shape (Err path converted to Ok and dropped) exactly. | parallel batch (claimed, candidate dispatch never started/completed - folder `108.*` was never created, so this number is still free but must not be handed to a different idea) |
