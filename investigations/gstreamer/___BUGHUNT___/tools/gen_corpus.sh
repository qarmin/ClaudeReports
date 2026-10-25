#!/usr/bin/env bash
# Generate a broad, synthetic seed corpus for the ci/fuzzing/ targets using ffmpeg's own lavfi
# test sources (testsrc/sine/anoisesrc) - no external sample files needed. Sweeps container,
# codec, resolution, framerate, pixel format, sample rate and channel count independently (a full
# cartesian product across all axes would be tens of thousands of files) so the corpus has real
# structural diversity without taking forever to generate. Parallelized across all cores via xargs.
#
# Usage: tools/gen_corpus.sh [output-dir]   (default: ___BUGHUNT___/__corpus)
set -uo pipefail

OUTDIR="${1:-$(dirname "$0")/../corpus}"
mkdir -p "$OUTDIR"
JOBS="$(nproc)"

JOBLIST="$(mktemp)"
trap 'rm -f "$JOBLIST"' EXIT

emit() {
    # emit <output-relative-name> <full ffmpeg args...>
    local name="$1"; shift
    echo "ffmpeg -y -loglevel error -nostdin $* \"$OUTDIR/$name\"" >> "$JOBLIST"
}

# ---- 1. base container/codec pairs, one "normal" file each (320x240, 25fps, yuv420p, 44.1kHz stereo) ----
emit base_mp4_h264_aac.mp4        -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libx264 -pix_fmt yuv420p -c:a aac -shortest
emit base_mp4_h265_aac.mp4        -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libx265 -pix_fmt yuv420p -c:a aac -shortest
emit base_mp4_mpeg4_mp3.mp4       -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v mpeg4 -pix_fmt yuv420p -c:a mp3 -shortest
emit base_mkv_vp8_vorbis.mkv      -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libvpx -pix_fmt yuv420p -c:a libvorbis -shortest
emit base_mkv_vp9_opus.mkv        -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libvpx-vp9 -pix_fmt yuv420p -c:a libopus -shortest
emit base_mkv_h264_flac.mkv       -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libx264 -pix_fmt yuv420p -c:a flac -shortest
emit base_mkv_av1_opus.mkv        -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libsvtav1 -pix_fmt yuv420p -c:a libopus -shortest
emit base_avi_mjpeg_mp3.avi       -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v mjpeg -pix_fmt yuvj420p -c:a mp3 -shortest
emit base_avi_h264_pcm.avi        -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libx264 -pix_fmt yuv420p -c:a pcm_s16le -shortest
emit base_flv_h264_mp3.flv        -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libx264 -pix_fmt yuv420p -c:a mp3 -shortest
emit base_flv_flv1_mp3.flv        -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v flv -pix_fmt yuv420p -c:a mp3 -shortest
emit base_ts_h264_aac.ts          -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libx264 -pix_fmt yuv420p -c:a aac -shortest
emit base_ts_mpeg2_mp2.ts         -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v mpeg2video -pix_fmt yuv420p -c:a mp2 -shortest
emit base_ogg_theora_vorbis.ogg   -f lavfi -i testsrc=size=320x240:rate=25:duration=1 -f lavfi -i "sine=frequency=440:duration=1" -c:v libtheora -pix_fmt yuv420p -c:a libvorbis -shortest
emit base_wav_pcm.wav             -f lavfi -i "sine=frequency=440:duration=1:sample_rate=44100"
emit base_ogg_opus.opus           -f lavfi -i "sine=frequency=440:duration=1:sample_rate=48000" -c:a libopus
emit base_ogg_flac.flac           -f lavfi -i "sine=frequency=440:duration=1:sample_rate=44100" -c:a flac
emit base_mp3.mp3                 -f lavfi -i "sine=frequency=440:duration=1:sample_rate=44100" -c:a mp3
emit base_id3_mp3.mp3             -f lavfi -i "sine=frequency=440:duration=1:sample_rate=44100" -c:a mp3 -id3v2_version 3 -metadata title="Fuzz Title" -metadata artist="Fuzz Artist" -metadata album="Fuzz Album/With\\Slashes" -metadata comment="A very very very very very very very very very very very very very very long comment field to stress id3demux text frame handling"

# ---- 2. resolution sweep (mp4/h264, fixed everything else) ----
for res in 2x2 6x4 15x17 33x33 64x64 129x65 320x240 640x480 1280x720 1920x1080 4096x2160; do
    emit "res_${res}.mp4" -f lavfi -i "testsrc=size=${res}:rate=25:duration=1" -c:v libx264 -pix_fmt yuv420p
done

# ---- 3. framerate sweep (mp4/h264, fixed resolution) ----
for fr in "1" "5" "15" "24000/1001" "24" "25" "30000/1001" "30" "50" "60" "90" "120" "240" "1000"; do
    safe="$(echo "$fr" | tr '/' '_')"
    emit "fps_${safe}.mp4" -f lavfi -i "testsrc=size=320x240:rate=${fr}:duration=1" -c:v libx264 -pix_fmt yuv420p
done

# ---- 4. pixel format sweep (mp4/mkv, whichever codec supports it) ----
for pf in yuv420p yuv422p yuv444p yuv420p10le yuv444p10le gray gray16le rgba bgra nv12 nv21 monow yuyv422; do
    emit "pixfmt_${pf}.mkv" -f lavfi -i "testsrc=size=320x240:rate=25:duration=1" -pix_fmt "$pf" -c:v ffv1
done

# ---- 5. audio sample-rate sweep (wav, no video) ----
for sr in 1 8 8000 11025 16000 22050 24000 32000 44100 48000 88200 96000 192000; do
    emit "samplerate_${sr}.wav" -f lavfi -i "sine=frequency=440:duration=1:sample_rate=${sr}"
done

# ---- 6. channel-count sweep (wav, no video) ----
for ch in 1 2 3 4 6 8 16; do
    emit "channels_${ch}.wav" -f lavfi -i "sine=frequency=440:duration=1:sample_rate=44100" -ac "$ch"
done

# ---- 7. duration/edge-case sweep (mp4/h264) ----
emit dur_singleframe.mp4   -f lavfi -i "testsrc=size=320x240:rate=25:duration=1" -frames:v 1 -c:v libx264 -pix_fmt yuv420p
emit dur_veryshort.mp4     -f lavfi -i "testsrc=size=320x240:rate=25:duration=0.04" -c:v libx264 -pix_fmt yuv420p
emit dur_zero_fps_odd.mp4  -f lavfi -i "testsrc=size=321x241:rate=25:duration=1" -c:v libx264 -pix_fmt yuv420p

# ---- 8. B-frames / GOP structure variants (mp4/h264) ----
emit gop_bframes.mp4       -f lavfi -i "testsrc=size=320x240:rate=25:duration=2" -c:v libx264 -bf 3 -g 10 -pix_fmt yuv420p
emit gop_nobframes.mp4     -f lavfi -i "testsrc=size=320x240:rate=25:duration=2" -c:v libx264 -bf 0 -g 300 -pix_fmt yuv420p
emit gop_allintra.mp4      -f lavfi -i "testsrc=size=320x240:rate=25:duration=2" -c:v libx264 -g 1 -pix_fmt yuv420p

echo "Generating $(wc -l < "$JOBLIST") files into $OUTDIR using $JOBS parallel jobs..."
xargs -P "$JOBS" -I{} bash -c '{}' < "$JOBLIST"

echo "Done. $(find "$OUTDIR" -maxdepth 1 -type f | wc -l) files generated (some may have failed - missing encoders etc, that's fine, check for zero-byte files below)."
find "$OUTDIR" -maxdepth 1 -type f -size 0 -print -delete
echo "$(find "$OUTDIR" -maxdepth 1 -type f | wc -l) valid non-empty files remain."

# ---- 9. truncated variants of a handful of valid files - deep-parser-reaching seeds for cheap ----
TRUNCDIR="$OUTDIR/truncated"
mkdir -p "$TRUNCDIR"
i=0
for f in "$OUTDIR"/base_*.* "$OUTDIR"/gop_*.*; do
    [ -f "$f" ] || continue
    i=$((i + 1))
    [ "$i" -gt 15 ] && break
    size=$(stat -c%s "$f")
    base="$(basename "$f")"
    for pct in 10 25 50 75 90; do
        n=$((size * pct / 100))
        [ "$n" -lt 1 ] && n=1
        head -c "$n" "$f" > "$TRUNCDIR/trunc${pct}_${base}"
    done
done
echo "$(find "$TRUNCDIR" -type f | wc -l) truncated variants generated into $TRUNCDIR"
