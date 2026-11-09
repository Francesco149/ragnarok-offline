#!/usr/bin/env bash
# Headless client test rig (runs INSIDE the workbench: tools/wb exec tools/client-test/rig.sh ...).
#
#   rig.sh server-start | server-stop | server-log [N]
#   rig.sh client-start | client-stop
#   rig.sh shot <name>             screenshot -> /data/shots/<name>.png
#   rig.sh click X Y | type TEXT | key KEY...
#
# Display :99 (Xvfb 1024x768), client = /data/play/RagnarokOnline (after `setup`),
# Wine prefix /data/wine, settings from tools/client-test/ragnarok-settings.reg.
set -euo pipefail
export DISPLAY=:99 WINEDLLOVERRIDES="mscoree,mshtml=" WINEPREFIX=/data/wine
CLIENT=/data/play/RagnarokOnline
cmd="${1:-}"; shift || true
focus() { local w; w=$(xdotool search --name '^Ragnarok$' | head -1); [ -n "$w" ] && xdotool windowactivate --sync "$w" 2>/dev/null || true; }
xvfb() { pgrep -x Xvfb >/dev/null || { Xvfb :99 -screen 0 1024x768x24 -nolisten tcp >/tmp/xvfb.log 2>&1 & sleep 1; }; }
case "$cmd" in
  server-start)
    pkill -f "ragnarok-offline -c dev.toml run" 2>/dev/null || true
    cd /work && nohup ./target/debug/ragnarok-offline -c dev.toml run > /data/server.log 2>&1 &
    for _ in $(seq 50); do grep -q "ready:" /data/server.log 2>/dev/null && break; sleep 0.2; done
    tail -3 /data/server.log ;;
  server-stop) pkill -INT -f "ragnarok-offline -c dev.toml run" || true; sleep 1 ;;
  server-log) tail -n "${1:-30}" /data/server.log ;;
  client-start)
    xvfb
    [ -f /data/wine/system.reg ] || wineboot -i >/dev/null 2>&1
    wine regedit /S 'Z:\work\tools\client-test\ragnarok-settings.reg' 2>/dev/null; wineserver -w
    cd "$CLIENT" && (wine Ragexe.exe 1rag1 >/tmp/ragexe.log 2>&1 &)
    for _ in $(seq 60); do xdotool search --name '^Ragnarok$' >/dev/null 2>&1 && break; sleep 0.5; done
    sleep 6; echo "client up" ;;
  client-stop) wineserver -k 2>/dev/null || true ;;
  shot) mkdir -p /data/shots; import -window root "/data/shots/${1:-shot}.png"; echo "/data/shots/${1:-shot}.png" ;;
  click) focus; xdotool mousemove "$1" "$2" sleep 0.3 mousedown 1 sleep 0.15 mouseup 1 ;;
  dclick) focus; xdotool mousemove "$1" "$2" sleep 0.3 mousedown 1 sleep 0.08 mouseup 1 sleep 0.08 mousedown 1 sleep 0.08 mouseup 1 ;;
  rclick) focus; xdotool mousemove "$1" "$2" sleep 0.3 mousedown 3 sleep 0.15 mouseup 3 ;;
  type) focus; xdotool type --delay 90 "$*" ;;
  key) focus; xdotool key --delay 120 "$@" ;;
  *) sed -n '2,12p' "$0"; exit 2 ;;
esac
