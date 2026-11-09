#!/usr/bin/env bash
# Build release archives into dist/ (run inside the workbench: tools/wb exec tools/package.sh)
#   dist/ragnarok-offline-<ver>-linux-x86_64.tar.gz   static (musl) server + launcher (runs the client in Wine)
#   dist/ragnarok-offline-<ver>-windows-x86_64.zip    native server CLI + RagnarokOffline.exe (Windows 10+)
#   dist/RagnarokOffline-<ver>.exe                    the Windows launcher alone: drop it into the game folder
# The archives contain the binaries, scripts/, data/db/, docs and README.
set -euo pipefail
cd "$(dirname "$0")/.."
ver=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
tools/fetch-third-party.sh
cargo build -q --release -p ro-server -p ro-launcher --target x86_64-unknown-linux-musl
cargo build -q --release -p ro-server -p ro-launcher --target x86_64-pc-windows-gnu
rm -rf dist && mkdir -p dist
for os in linux windows; do
  d="dist/ragnarok-offline-$ver-$os-x86_64"
  mkdir -p "$d/data"
  cp -r scripts "$d/"; cp -r data/db "$d/data/"; cp -r docs README.md "$d/"
  if [ $os = linux ]; then
    cp target/x86_64-unknown-linux-musl/release/{ragnarok-offline,RagnarokOffline} "$d/"
    (cd dist && tar czf "$(basename "$d").tar.gz" "$(basename "$d")")
  else
    cp target/x86_64-pc-windows-gnu/release/{ragnarok-offline,RagnarokOffline}.exe "$d/"
    cp third_party/ddrawcompat/LICENSE.txt "$d/DDrawCompat-LICENSE.txt"
    (cd dist && zip -qr "$(basename "$d").zip" "$(basename "$d")")
  fi
done
cp target/x86_64-pc-windows-gnu/release/RagnarokOffline.exe "dist/RagnarokOffline-$ver.exe"
ls -la dist/*.tar.gz dist/*.zip dist/*.exe
