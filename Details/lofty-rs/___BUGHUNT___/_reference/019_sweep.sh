#!/usr/bin/env bash
set -u
cd /home/rafal/Projekty/Obce/lofty-rs/fuzz

TARGETS="aacfile_read_from aifffile_read_from apefile_read_from filetype_from_buffer flacfile_read_from mp4file_read_from mpcfile_read_from mpegfile_read_from opusfile_read_from picture_from_ape_bytes picture_from_flac_bytes picture_from_reader pictureinformation_from_jpeg pictureinformation_from_png probe_read speexfile_read_from vorbisfile_read_from wavfile_read_from wavpackfile_read_from"

OUT=/home/rafal/Projekty/Obce/lofty-rs/___BUGHUNT___/_reference/019_sweep_results.tsv
echo -e "target\tempty_corpus_result\tseeded_corpus_result" > "$OUT"

for t in $TARGETS; do
    echo "=== $t ===" >&2
    corpus_dir="corpus/$t"
    rm -rf "$corpus_dir"
    mkdir -p "$corpus_dir"

    # --- Test A: empty corpus, sanitizer address ---
    startA=$(date +%s)
    timeout -s KILL 60 cargo +nightly fuzz run "$t" --sanitizer address -- -max_total_time=15 -rss_limit_mb=4096 > "/tmp/019_${t}_empty.log" 2>&1
    rcA=$?
    endA=$(date +%s)
    durA=$((endA - startA))
    if [ $rcA -eq 137 ] || [ $rcA -eq 124 ]; then
        resA="HANG(killed after ${durA}s)"
    elif [ $rcA -eq 0 ]; then
        resA="ok(${durA}s)"
    else
        resA="exit${rcA}(${durA}s)"
    fi
    echo "  empty: $resA" >&2

    # seed corpus with exactly one file: use --sanitizer none for 2s to generate real inputs organically
    rm -rf "$corpus_dir"
    mkdir -p "$corpus_dir"
    timeout -s KILL 30 cargo +nightly fuzz run "$t" --sanitizer none -- -max_total_time=2 -rss_limit_mb=4096 > "/tmp/019_${t}_seedgen.log" 2>&1
    # keep exactly one seed file
    files=("$corpus_dir"/*)
    if [ -e "${files[0]}" ]; then
        keep="${files[0]}"
        for f in "${files[@]:1}"; do rm -f "$f"; done
        nseed=$(ls "$corpus_dir" | wc -l)
    else
        nseed=0
    fi
    echo "  seeded corpus file count before ASAN run: $nseed" >&2

    # --- Test B: seeded corpus (1 file), sanitizer address ---
    startB=$(date +%s)
    timeout -s KILL 60 cargo +nightly fuzz run "$t" --sanitizer address -- -max_total_time=15 -rss_limit_mb=4096 > "/tmp/019_${t}_seeded.log" 2>&1
    rcB=$?
    endB=$(date +%s)
    durB=$((endB - startB))
    if [ $rcB -eq 137 ] || [ $rcB -eq 124 ]; then
        resB="HANG(killed after ${durB}s)"
    elif [ $rcB -eq 0 ]; then
        resB="ok(${durB}s)"
    else
        resB="exit${rcB}(${durB}s)"
    fi
    echo "  seeded(n=$nseed): $resB" >&2

    echo -e "${t}\t${resA}\t${resB}(n=${nseed})" >> "$OUT"
done

echo "DONE" >&2
