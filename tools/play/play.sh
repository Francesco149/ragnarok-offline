#!/usr/bin/env bash
# Start the playable snapshot of ragnarok-offline (see tools/play/update.sh).
# This file is copied next to the snapshot; the app-launcher entry runs that
# copy, so it works whatever state the source tree is in.
#
#   play.sh               the game, 800x600 in a window
#   play.sh --fullscreen  the game fullscreen (Wine's Wayland driver scales it)
#   play.sh --settings    the client's own Setup.exe
#   play.sh --smoke       headless self-test: server + client + a login, no window
#   play.sh --stop        stop a running game (the server saves first)
#
# Everything runs in its own podman container (`ro-play`, 2 GiB, no swap, own
# loopback network), never on the host, and apart from the workbench.
set -euo pipefail
S="$(cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")" && pwd)"
IMAGE="${RO_PLAY_IMAGE:-localhost/ro-wb:latest}"
MEM="${RO_PLAY_MEM:-2g}"
mode=play; name=ro-play; full=0
case "${1:-}" in
  --fullscreen) full=1 ;;
  --settings) mode=settings ;;
  --smoke) mode=smoke; name=ro-play-smoke ;;
  --stop) podman stop -t 15 ro-play >/dev/null 2>&1 || true; exit 0 ;;
  "") ;;
  *) sed -n '2,13p' "${BASH_SOURCE[0]}"; exit 2 ;;
esac
say() { echo "$*" >&2; [ "$mode" = smoke ] || notify-send -a "Ragnarok Offline" "Ragnarok Offline" "$*" 2>/dev/null || true; }

[ -e "$S/current/ragnarok-offline" ] || { say "No snapshot in $S yet: run tools/wb exec tools/play/update.sh"; exit 1; }
if [ -n "$(podman ps -q --filter "name=^$name\$")" ]; then say "Already running."; exit 0; fi
build="$(basename "$(readlink -f "$S/current")")"

args=(--rm --name "$name" --label ro-play=1 --init
  --memory "$MEM" --memory-swap "$MEM" --pids-limit 2048 --userns keep-id
  -v "$S:/play" -w /play -e RO_MODE="$mode" -e RO_FULLSCREEN="$full")
if [ "$mode" != smoke ]; then
  rt="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}"
  [ -e /dev/dri ] && args+=(--device /dev/dri)
  [ -S "$rt/pulse/native" ] && args+=(-v "$rt/pulse/native:/run/pulse/native" -e PULSE_SERVER=unix:/run/pulse/native)
  if [ "$full" = 1 ] && [ -S "$rt/${WAYLAND_DISPLAY:-wayland-0}" ]; then
    # No DISPLAY: Wine picks its Wayland driver, which shows an 800x600
    # fullscreen mode scaled to the monitor instead of switching modes.
    args+=(-v "$rt/${WAYLAND_DISPLAY:-wayland-0}:/run/wl/wayland-0" -e XDG_RUNTIME_DIR=/run/wl -e WAYLAND_DISPLAY=wayland-0)
  else
    d="${DISPLAY:-:0}"; n="${d#*:}"; n="${n%%.*}"
    [ -S "/tmp/.X11-unix/X$n" ] || { say "No X display ($d) to show the game on."; exit 1; }
    args+=(--ipc host -v /tmp/.X11-unix:/tmp/.X11-unix -e DISPLAY=":$n")
    [ -f "${XAUTHORITY:-$HOME/.Xauthority}" ] && args+=(-v "${XAUTHORITY:-$HOME/.Xauthority}:/tmp/.Xauthority:ro" -e XAUTHORITY=/tmp/.Xauthority)
  fi
fi
out="$S/logs/launch-$mode.log"; mkdir -p "$S/logs"
if podman run "${args[@]}" "$IMAGE" bash "/play/builds/$build/inside.sh" "/play/builds/$build" >"$out" 2>&1; then
  [ "$mode" = smoke ] && tail -1 "$out"; exit 0
else
  rc=$?; say "Ragnarok Offline stopped with an error ($rc): $(tail -2 "$out" | tr '\n' ' ') (log: $out)"; exit $rc
fi
