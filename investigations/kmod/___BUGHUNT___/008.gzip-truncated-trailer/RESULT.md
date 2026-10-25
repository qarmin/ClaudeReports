# gzip-truncated-trailer: a .ko.gz missing its CRC32/ISIZE trailer is accepted

`kmod_file_load_zlib()` (libkmod/libkmod-file-zlib.c) only treats a negative `gzread()` as an error. `gzread()` reports a truncated stream (`Z_BUF_ERROR`, "unexpected end of file") as a short read followed by EOF, so a gzip file whose last bytes are cut off is loaded as if complete. Flagged by the `compress_diff` testbed component (`diff:compress-truncated-trailer-accepted-gz`, 4 of 30 sampled modules).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `compress_diff` (testbed): 30 real 7.0.0-38 modules, each as raw / xz / gz / zst, plus variants with the last 5-9 bytes cut and with the stream cut in half | `kinfo` driver over libkmod (HEAD ASAN) | trailer-truncated `.gz` returns module info (4 modules, only the ones with a small tail), trailer-truncated `.xz`/`.zst` and every half-cut variant of all three formats are rejected |
| Analysis of the accepted case | cut = the 8-byte gzip trailer (CRC32 + ISIZE) only | the deflate payload is complete, so the decompressed bytes are identical to the original: only the integrity check is skipped, no wrong data reaches the ELF parser |
| Cuts inside the payload | half-cut `.gz` | rejected later by the ELF parser: the section header table sits at the end of a `.ko`, so a payload cut can never yield a valid module |

## Status

**Not reproducible** as a defect with observable wrong behavior: the accepted input decodes to exactly the intended module. What is real is that zlib's trailer CRC is not enforced for truncated files (`gzerror()` is never consulted after `gzread()` returns 0); it would matter only for a corruption that damages the trailer and nothing else.

## Not done

- No patch drafted; a fix would check `gzerror()` / `gzeof()` after the loop and fail on `Z_BUF_ERROR`.
