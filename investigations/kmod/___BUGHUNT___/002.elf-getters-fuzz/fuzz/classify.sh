#!/usr/bin/env bash
# classify.sh <fuzz_bin> <crash>: prints "<error kind> | <top 3 libkmod frames>"
ulimit -c 0
out=$("$1" "$2" 2>&1)
kind=$(grep -m1 -oE "Assertion .*failed|AddressSanitizer: [a-z-]+|runtime error: [a-z ]+|LeakSanitizer|deadly signal" <<<"$out")
frames=$(grep -E '^ +#[0-9]+ ' <<<"$out" | grep -oE 'in [A-Za-z_0-9]+' | grep -v -E 'pthread|raise|abort|__assert|LLVMFuzzer|fuzzer|_start|libc|main$|__sanitizer|__asan|ubsan|__interceptor|strlen|strncmp|strcmp|memcmp|touch|use_versions' | head -3 | tr '\n' ' ')
echo "$kind | $frames"
