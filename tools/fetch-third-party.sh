#!/usr/bin/env bash
# Fetch pinned third-party binaries the Windows launcher embeds (into third_party/,
# which git ignores). Safe to re-run; every download is checked against its sha256.
#
#   DDrawCompat v0.7.2 (narzoul/DDrawCompat, 0BSD): DirectDraw/Direct3D 1-7 fixes for
#   Windows 10/11. Released 2026-10-14; bump deliberately, never to "latest".
set -euo pipefail
cd "$(dirname "$0")/.."
DDC_VER=v0.7.2
DDC_ZIP_SHA256=a932a4dcb2100552b6918fd4a3cc7de138bb97448bbca2a7051954993a397b39
DDC_DLL_SHA256=94895cf6ede8fd0f74ee1447fdc08b7044a9c65a1a25a49a4df893a0370975a3
d=third_party/ddrawcompat
mkdir -p "$d"
if [ -f "$d/ddraw.dll" ] && echo "$DDC_DLL_SHA256  $d/ddraw.dll" | sha256sum -c --quiet 2>/dev/null; then
  echo "ddrawcompat $DDC_VER: present"; exit 0
fi
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
curl -fsSL -o "$tmp/ddc.zip" "https://github.com/narzoul/DDrawCompat/releases/download/$DDC_VER/DDrawCompat-$DDC_VER.zip"
echo "$DDC_ZIP_SHA256  $tmp/ddc.zip" | sha256sum -c --quiet
python3 -I -c 'import sys, zipfile; zipfile.ZipFile(sys.argv[1]).extract("ddraw.dll", sys.argv[2])' "$tmp/ddc.zip" "$tmp"
echo "$DDC_DLL_SHA256  $tmp/ddraw.dll" | sha256sum -c --quiet
curl -fsSL -o "$tmp/LICENSE.txt" "https://raw.githubusercontent.com/narzoul/DDrawCompat/$DDC_VER/LICENSE.txt"
mv "$tmp/ddraw.dll" "$tmp/LICENSE.txt" "$d/"
echo "$DDC_VER" > "$d/VERSION"
echo "ddrawcompat $DDC_VER -> $d"
