# oom-sweep-compressed-modules: allocation-failure sweep over module loading (raw, zst, xz, gz) and related modprobe flags

Companion to 019: the same LD_PRELOAD allocation-failure shim (`tools/failmalloc.c`, `tools/failmalloc_sweep.sh`) applied to the module file pipeline, where error paths of decompressors and the ELF parser are most intricate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `modinfo <real 7.0.0-38 module>` as raw `.ko`, `.ko.zst`, `.ko.xz`, `.ko.gz` (about 3500 allocations each), every 3rd allocation failed in turn | plain debug build of HEAD, `timeout 20` per run | no signal exits for any of the four. Three sampled allocations (#67 zst, #82 xz, #73 gz) never return |
| Investigation of the three hangs | gdb backtrace | all three are the process parked in `pthread_once` inside `OPENSSL_init_crypto()` -> `CRYPTO_THREAD_lock_new()` -> `CRYPTO_zalloc()` -> `ERR_new()`: a failing allocation inside libcrypto's own initialization re-enters the init lock (self-deadlock in OpenSSL 3.x), triggered when libkmod loads the signature code (`dlopen` of libcrypto). Not in libkmod's code |
| `depmod -b` over a 15-module real compressed tree (10489 allocations, every 8th failed) and `depmod -n -F System.map -e` (43002 allocations, every 144th failed) | same | no signals; one hang at allocation #271 of the first (same kind: parked in OpenSSL init, not separately debugged) |
| `modinfo -F vermagic` (xz), `modprobe --show-modversions` (zst), `modprobe --show-exports` (zst) | same | no crashes or hangs |

## Status

**Not reproducible** as a libkmod defect - the only misbehavior is inside OpenSSL's init-on-OOM, which libkmod cannot guard. libkmod's own error handling for the decompression and ELF paths held under about 11000 injected failures.

## Not done

- `depmod` on a large tree and the config/install paths were swept only on the tiny synthetic tree (see 019 and the `oom_sweep` testbed component).
