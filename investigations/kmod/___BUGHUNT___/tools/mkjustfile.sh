#!/usr/bin/env bash
# mkjustfile.sh <candidate_dir> [--no-patch]: instantiate candidate_justfile_c with the main repo's HEAD as base_commit.
set -euo pipefail
here=$(dirname "$(realpath "$0")"); c=$1
base=$(git -C "$here/../.." rev-parse HEAD)
sed "s/FILL_IN_BASE_COMMIT_SHA/$base/" "$here/candidate_justfile_c" > "$c/justfile"
if [ "${2:-}" = --no-patch ]; then
	sed -i -e 's/^test: .*/test: clone reproduce-before/' "$c/justfile"
fi
