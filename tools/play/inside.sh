#!/usr/bin/env bash
# Runs inside the ro-play container (tools/play/play.sh starts it): the
# snapshot's server, then the client in Wine; saves and stops the server when
# the client exits. $1 = the build directory this game stays on.
set -uo pipefail
B="$1"; S=/play; MODE="${RO_MODE:-play}"
export WINEDEBUG=-all WINEDLLOVERRIDES="mscoree,mshtml=" HOME=/tmp/home
mkdir -p "$HOME" "$S/logs"
cfg="$S/ragnarok-offline.toml"
export WINEPREFIX="$S/wine"
if [ "$MODE" = smoke ]; then
  # Headless self-test: throwaway saves and prefix, the same ports (this
  # container has its own loopback).
  export WINEPREFIX=/tmp/home/wine DISPLAY=:98
  sed -e 's|^save_dir.*|save_dir = "/tmp/saves"|' -e "s|\"current/|\"$B/|g" -e 's|^client_dir.*|client_dir = "/play/client"|' "$cfg" > /tmp/smoke.toml
  cfg=/tmp/smoke.toml
  Xvfb :98 -screen 0 1024x768x24 -nolisten tcp >/dev/null 2>&1 &
  sleep 1
fi
log="$S/logs/server-$MODE.log"

# The server reads scripts and database through `current`; pin this game to
# the build it started with by giving it a config of its own.
pinned="/tmp/pinned.toml"
sed -e "s|\"current/|\"$B/|g" -e "s|^client_dir = \"client\"|client_dir = \"$S/client\"|" -e "s|^save_dir = \"saves\"|save_dir = \"$S/saves\"|" "$cfg" > "$pinned"

"$B/ragnarok-offline" -c "$pinned" run > "$log" 2>&1 &
srv=$!
for _ in $(seq 100); do grep -q "ready:" "$log" 2>/dev/null && break; kill -0 $srv 2>/dev/null || break; sleep 0.2; done
if ! grep -q "ready:" "$log"; then echo "server did not start:"; tail -5 "$log"; exit 3; fi

if [ ! -f "$WINEPREFIX/.ro-settings" ]; then
  wineboot -i >/dev/null 2>&1
  wine regedit /S "Z:${B//\//\\}\\settings.reg" 2>/dev/null
  touch "$WINEPREFIX/.ro-settings"
fi
# The client asks for Arial; without it Wine substitutes a wider font.
[ -d "$S/fonts" ] && cp -n "$S"/fonts/*.TTF "$WINEPREFIX/drive_c/windows/Fonts/" 2>/dev/null
key='HKLM\Software\Wow6432Node\Gravity Soft\Ragnarok'
[ "$MODE" = smoke ] && wine reg add "$key" /v SOUNDMODE /t REG_DWORD /d 0 /f >/dev/null 2>&1
if [ "$MODE" = play ]; then
  wine reg add "$key" /v ISFULLSCREENMODE /t REG_DWORD /d "${RO_FULLSCREEN:-0}" /f >/dev/null 2>&1
fi

cd "$S/client"
if [ "$MODE" = settings ]; then
  wine Setup.exe
elif [ "$MODE" = smoke ]; then
  (wine Ragexe.exe 1rag1 >/dev/null 2>&1 &)
  for _ in $(seq 60); do xdotool search --name '^Ragnarok$' >/dev/null 2>&1 && break; sleep 0.5; done
  sleep 20
  focus() { local w; w=$(xdotool search --name '^Ragnarok$' | head -1); [ -n "$w" ] && xdotool windowfocus "$w" 2>/dev/null; true; }  # no window manager on Xvfb
  click() { focus; xdotool mousemove "$1" "$2" sleep 0.3 mousedown 1 sleep 0.15 mouseup 1; }
  typ() { focus; xdotool type --delay 90 "$1"; }
  # ID, password, Enter: a new account logs in and is shown the server list.
  click 535 525; typ smoke; click 535 557; typ smoke1; focus; xdotool key --delay 120 Return; sleep 4
  import -window root "$S/logs/smoke.png"
  wineserver -k
else
  wine Ragexe.exe 1rag1 > "$S/logs/client.log" 2>&1
  wineserver -w
fi
kill -INT $srv 2>/dev/null; wait $srv 2>/dev/null
if [ "$MODE" = smoke ]; then
  if grep -q "login: smoke ok" "$log"; then echo "smoke: ok (login reached the server; shot: logs/smoke.png)"; else echo "smoke: FAILED (no login in $log)"; exit 4; fi
fi
