#!/usr/bin/env bash
# Build the all-in-one installer from your own CD (run in the workbench):
#   tools/wb exec tools/aio/build.sh [CD_DIR] [OUT_DIR]
# CD_DIR: the disc's files (default /data/iso = ~/data/ragnarok-cd/iso).
# OUT_DIR (default /data/aio/RagnarokOffline-CD) holds Gravity's installer
# files plus ours: it is for your own use - never commit or publish it.
# See docs/WINDOWS.md.
set -euo pipefail
cd "$(dirname "$0")/../.."
CD="${1:-/data/iso}"
OUT="${2:-/data/aio/RagnarokOffline-CD}"
tools/fetch-third-party.sh
cargo build -q --release -p ro-launcher --target x86_64-pc-windows-gnu
python3 -I tools/aio/make_aio.py "$CD" "$OUT" \
  target/x86_64-pc-windows-gnu/release/RagnarokOffline.exe \
  third_party/ddrawcompat/ddraw.dll \
  crates/ro-launcher/assets/DDrawCompat.ini \
  third_party/ddrawcompat/LICENSE.txt
if command -v xorriso >/dev/null; then
  iso="$(dirname "$OUT")/$(basename "$OUT").iso"
  # like the pressed disc: volume id RAGNAROK, ISO 9660 + Joliet, no Rock Ridge
  xorriso -as mkisofs -quiet -V RAGNAROK -J -joliet-long -o "$iso" "$OUT"
  echo "$iso"
fi
