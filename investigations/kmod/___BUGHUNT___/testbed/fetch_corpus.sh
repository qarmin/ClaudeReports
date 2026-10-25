#!/usr/bin/env bash
# Real-world corpus: Ubuntu's linux-modules-7.0.0-38-generic (6901 signed .ko.zst, System.map,
# modules.builtin.modinfo), fetched without root via a private apt state dir.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
st="$here/__corpus-data/apt"
mkdir -p "$st/lists/partial" "$st/cache/archives/partial" "$here/__corpus-data/realroot"
A=(-o Dir::State::Lists="$st/lists" -o Dir::Cache="$st/cache" -o Debug::NoLocking=1)
apt-get "${A[@]}" update >/dev/null
cd "$st" && apt-get "${A[@]}" download linux-modules-7.0.0-38-generic
dpkg -x "$st"/linux-modules-7.0.0-38-generic_*.deb "$here/__corpus-data/realroot"
rm -f "$st"/*.deb
