#!/bin/bash
t=$1
FR=/home/rafal/Projekty/Obce/pic-scale/___BUGHUNT___/_reference/fuzzruns
cd /home/rafal/Projekty/Obce/pic-scale/fuzz
mkdir -p "$FR/corpus_$t"
cargo +nightly fuzz run --sanitizer none --features avx,sse,nightly_avx512,colorspaces "$t" "$FR/corpus_$t" -- -max_total_time=150 -rss_limit_mb=2048 > "$FR/log_$t.txt" 2>&1
echo "DONE $t exit=$?" >> "$FR/status.txt"
