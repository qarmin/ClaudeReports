#!/usr/bin/env bash
# Proves diff_runner.py's detector actually fires, using known-already-confirmed miscompilations
# (canary/) instead of trusting a clean campaign round at face value. See canary/README.md.
#
# UNLIKE every other use of diff_runner.py in this tool: a divergence here is SUCCESS (the
# detector works), and a clean run here is FAILURE (the detector is silently broken, or the
# upstream bug got fixed - see canary/README.md's "Adding a new canary" for how to tell which).
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

FINDINGS_DIR="$(mktemp -d)"
trap 'rm -rf "$FINDINGS_DIR"' EXIT

echo "=== running diff_runner.py against canary/ (known-reproducing cases) ==="
set +e
python3 diff_runner.py --corpus canary/ --findings "$FINDINGS_DIR" --jobs "$(nproc)" "$@"
rc=$?
set -e

divergences=$(find "$FINDINGS_DIR" -mindepth 1 -maxdepth 1 -not -name ".build" | wc -l)
total_canaries=$(ls canary/*.rs 2>/dev/null | wc -l)

echo
if [[ "$divergences" -eq "$total_canaries" && "$total_canaries" -gt 0 ]]; then
  echo "PASS: all $total_canaries canary case(s) produced a divergence - the detector is working."
  exit 0
else
  echo "FAIL: only $divergences/$total_canaries canary case(s) produced a divergence."
  echo "This means diff_runner.py's comparison logic (or the config/rustc it was pointed at) is"
  echo "not catching a KNOWN, already-confirmed miscompilation - do not trust a clean campaign"
  echo "round until this is fixed. Missing cases:"
  for f in canary/*.rs; do
    name="$(basename "$f" .rs)"
    [[ -d "$FINDINGS_DIR/$name" ]] || echo "  - $name (no divergence detected)"
  done
  exit 1
fi
