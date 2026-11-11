#!/usr/bin/env bash
# Refresh the playable snapshot behind the app-launcher entry "Ragnarok Offline".
# Run inside the workbench at the end of a session, from a committed tree:
#
#   tools/wb exec tools/play/update.sh      # build + snapshot
#   tools/play/install.sh                   # host, once: the .desktop entry
#   ~/data/ragnarok-cd/stable/play.sh --smoke   # host: headless check of the snapshot
#
# The snapshot lives in /data/stable (= ~/data/ragnarok-cd/stable) and shares
# nothing with the development rig that changes: its own server binary,
# scripts and database (builds/<stamp>, `current` points at the newest), its
# own client folder (hard links to the play copy, own data.grf pointing at
# its own port), its own saves, Wine prefix and config. A game started from
# the launcher keeps the build it started with; updating swaps `current`
# under it without touching that build.
set -euo pipefail
cd "$(dirname "$0")/../.."
S="${RO_STABLE:-/data/stable}"
LOGIN_PORT=6910 CHAR_PORT=6131 MAP_PORT=5131 CONSOLE_PORT=6133

cargo build -q --release -p ro-server --target x86_64-unknown-linux-musl
bin=target/x86_64-unknown-linux-musl/release/ragnarok-offline

mkdir -p "$S/builds" "$S/saves" "$S/logs"
stamp="$(date +%Y%m%d-%H%M%S)-$(git rev-parse --short HEAD)$(git diff --quiet HEAD -- scripts data crates 2>/dev/null || echo -dirty)"
b="$S/builds/$stamp"
mkdir -p "$b/data"
cp "$bin" "$b/ragnarok-offline"
cp -r scripts "$b/scripts"
cp -r data/db "$b/data/db"
cp tools/play/inside.sh tools/play/settings.reg "$b/"
git log -1 --format='%h %ad %s' --date=iso > "$b/VERSION"

# The client folder, once: hard links to the rig's play copy (those files are
# never changed in place), then its own data.grf for its own port.
if [ ! -f "$S/client/Ragexe.exe" ]; then
  src=/data/play/RagnarokOnline
  mkdir -p "$S/client"
  cp -al "$src/." "$S/client/"
  rm -f "$S/client/data.grf" "$S/client/Play Offline.bat"
fi
if ! "$b/ragnarok-offline" grf cat "$S/client/data.grf" 'data\clientinfo.xml' 2>/dev/null | grep -q "<port>$LOGIN_PORT</port>"; then
  "$b/ragnarok-offline" setup --client "$S/client" --address 127.0.0.1 --port "$LOGIN_PORT" --name Offline
fi

# The config, once: yours to edit afterwards (rates, start items...).
if [ ! -f "$S/ragnarok-offline.toml" ]; then
  cat > "$S/ragnarok-offline.toml" <<EOF
# The launcher's own server config (tools/play). Paths are relative to this
# file; \`current\` is the newest snapshot of scripts and database.
client_dir = "client"
bind = "127.0.0.1"
login_port = $LOGIN_PORT
console_port = $CONSOLE_PORT
watch_scripts = false
packet_log = false

[game]
char_port = $CHAR_PORT
map_port = $MAP_PORT
save_dir = "saves"
script_dir = "current/scripts"
db_dir = "current/data/db"
EOF
fi

ln -sfn "builds/$stamp" "$S/current.new" && mv -T "$S/current.new" "$S/current"
cp tools/play/play.sh "$S/play.sh" && chmod +x "$S/play.sh"

# The launcher icon, from the CD's own (stays in /data, Gravity's).
if [ ! -f "$S/ragnarok.png" ] && [ -f /data/iso/ragnarok.ico ]; then
  magick /data/iso/ragnarok.ico "$S/ico.png" 2>/dev/null || true
  big="$(ls -S "$S"/ico*.png 2>/dev/null | head -1)"
  [ -n "$big" ] && mv "$big" "$S/ragnarok.png"; rm -f "$S"/ico*.png
fi

# Keep the three newest builds (a running game may still sit on an older one).
ls -1d "$S"/builds/* | sort | head -n -3 | xargs -r rm -rf
echo "stable -> $stamp"
cat "$b/VERSION"
