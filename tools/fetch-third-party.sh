#!/usr/bin/env bash
# Fetch pinned third-party binaries the Windows launcher embeds (into third_party/,
# which git ignores). Safe to re-run; every download is checked against its sha256.
#
#   DDrawCompat v0.7.2 (narzoul/DDrawCompat, 0BSD): DirectDraw/Direct3D 1-7 fixes for
#   Windows 10/11. Released 2026-10-14; bump deliberately, never to "latest".
#   euRO patch server snapshot: the files euRO's wpatch.euro-ro.net served to
#   this CD's patcher, and its front page (the same site as www.euro-ro.net,
#   the client's registrationweb), as the Wayback Machine archived them
#   (closest captures to the disc; each sha256 = the capture whose Wayback
#   digest matched).
set -euo pipefail
cd "$(dirname "$0")/.."
DDC_VER=v0.7.2
DDC_ZIP_SHA256=a932a4dcb2100552b6918fd4a3cc7de138bb97448bbca2a7051954993a397b39
DDC_DLL_SHA256=94895cf6ede8fd0f74ee1447fdc08b7044a9c65a1a25a49a4df893a0370975a3
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT

# --- euRO patch server snapshot (web.archive.org, http://wpatch.euro-ro.net/patch/)
e=third_party/euro-patch
mkdir -p "$e"
while read -r ts path name sha; do
  out="$e/$name.$ts"
  if [ -f "$out" ] && echo "$sha  $out" | sha256sum -c --quiet 2>/dev/null; then continue; fi
  curl -fsSL -o "$tmp/$name" "https://web.archive.org/web/${ts}id_/http://wpatch.euro-ro.net:80$path"
  echo "$sha  $tmp/$name" | sha256sum -c --quiet
  mv "$tmp/$name" "$out"
  echo "euro-patch: $path ($ts)"
done <<'LIST'
20040221010205 /patch/patch2.txt patch2.txt c828ff54029fcc7a6e2296dee601ace659d62b0bf6c94b1937603ff6a7ba4dd6
20040718192102 /patch/patch_allow.txt patch_allow.txt 410083735735a10e658a19edd1704e606c9dd112e225825b63fafeded766c8b9
20040718124248 /patch/notice_patch.txt notice_patch.txt 8b0b747398b4059df98554da803600100d2a95e2ff7089d017f7385b51ea634d
20040718054701 /patch/notice_close.txt notice_close.txt 9984d393188de690e7a6494652d2ebff9ee87b7e0eb818a76bcf290d680acc05
20040221010012 / index.html d8a2a8e1692f0e4bb5c8bbc5c81d216c53d36382044a7420e16d412da648b717
LIST
echo "euro-patch: present"

# --- DDrawCompat
d=third_party/ddrawcompat
mkdir -p "$d"
if [ -f "$d/ddraw.dll" ] && echo "$DDC_DLL_SHA256  $d/ddraw.dll" | sha256sum -c --quiet 2>/dev/null; then
  echo "ddrawcompat $DDC_VER: present"; exit 0
fi
curl -fsSL -o "$tmp/ddc.zip" "https://github.com/narzoul/DDrawCompat/releases/download/$DDC_VER/DDrawCompat-$DDC_VER.zip"
echo "$DDC_ZIP_SHA256  $tmp/ddc.zip" | sha256sum -c --quiet
python3 -I -c 'import sys, zipfile; zipfile.ZipFile(sys.argv[1]).extract("ddraw.dll", sys.argv[2])' "$tmp/ddc.zip" "$tmp"
echo "$DDC_DLL_SHA256  $tmp/ddraw.dll" | sha256sum -c --quiet
curl -fsSL -o "$tmp/LICENSE.txt" "https://raw.githubusercontent.com/narzoul/DDrawCompat/$DDC_VER/LICENSE.txt"
mv "$tmp/ddraw.dll" "$tmp/LICENSE.txt" "$d/"
echo "$DDC_VER" > "$d/VERSION"
echo "ddrawcompat $DDC_VER -> $d"
