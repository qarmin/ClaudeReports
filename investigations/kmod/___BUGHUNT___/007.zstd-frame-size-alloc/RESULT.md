# zstd-frame-size-alloc: a tiny .ko.zst claiming a huge frame content size

`kmod_file_load_zstd()` (libkmod/libkmod-file-zstd.c) `malloc()`s `ZSTD_getFrameContentSize()` bytes straight from the untrusted frame header before decompressing, so a 13-byte file can ask for up to 2^64 bytes. The testbed's `fuzz_modfile` flagged it as `asan:requested:kmod_file_load_zstd` / `libfuzzer:out-of-memory`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `fuzz_modfile` (testbed), HEAD ASAN | libFuzzer, default ASAN allocator limits | ASAN aborts: "requested allocation size ... exceeds maximum" / "out of memory" - an allocator-policy abort, not a libkmod defect |
| Real `modinfo` on `repro/{huge,big,mid}.ko.zst` (claimed 2^62 / 40 GiB / 3 GiB) with `allocator_may_return_null=1` | HEAD ASAN build | `huge`/`big`: `malloc` returns NULL, libkmod returns -ENOMEM, `modinfo` prints "Cannot allocate memory" and exits 1; `mid`: `malloc` succeeds lazily (overcommit, pages untouched), `ZSTD_decompress` fails with "Data corruption detected", -EINVAL, buffer freed |

## Status

**Not reproducible** as a product defect: with the default allocator (glibc) every case ends in a graceful error. The only abort is ASAN's `allocator_may_return_null=0` policy. Residual note: an input can make libkmod attempt a multi-GiB allocation (and commit it on strict-overcommit systems) before any data is decoded, but this is a resource-limit design choice rather than a memory-safety bug and no bounded fix was attempted.

## Not done

- xz and zlib size handling was not checked here (separate backlog item on compressed decoders).
