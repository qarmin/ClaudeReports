# compressed-modfile-fuzz: whole-file pipeline (decompress, ELF, signature) under libFuzzer

Backlog items "compressed module decoders" and "libkmod-signature.c trailer parsing". The harness feeds arbitrary bytes to `kmod_module_new_from_path()` through the way `modinfo`/`modprobe` use a module: decompression, ELF parsing, signature trailer parsing, info/versions/symbols.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| libFuzzer `testbed/fuzz/fuzz_modfile.c` with seeds from real signed `.ko.zst` and xz/gz re-compressions | clang ASAN+UBSAN, `-max_len=262144`, 10 minutes, `allocator_may_return_null=1`, `-rss_limit_mb=0 -malloc_limit_mb=0` under `memcap 3000` | 995,256 execs, peak RSS 258 MB, no crash/leak/timeout. The earlier runs aborted on libFuzzer's own allocation limit for the zstd frame-size claim (candidate 007), which is why the limits were lifted |
| 75-second runs inside `just testbed` | previous refreshes | only the 007 allocator-policy signature |
| Differential `compress_diff` and `signature_diff` | HEAD | only the gzip trailer observation (008) |

## Status

**Not reproducible** - no memory-safety or logic finding in decompression handling or signature parsing beyond 007/008.

## Not done

- A `libFuzzer` run with a PKCS#7 structure-aware mutator (valid CMS blobs with mutated lengths) would reach deeper into the OpenSSL-facing code than random bytes do.
