#!/usr/bin/env bash
# Host, once: add "Ragnarok Offline" to the app launcher. It starts the
# snapshot kept by tools/play/update.sh, not the source tree.
set -euo pipefail
S="${RO_STABLE_HOST:-$HOME/data/ragnarok-cd/stable}"
apps="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
mkdir -p "$apps"
icon="$S/ragnarok.png"   # made by update.sh from the client's icon when it can be
[ -f "$icon" ] || icon=applications-games
cat > "$apps/ragnarok-offline.desktop" <<EOD
[Desktop Entry]
Type=Application
Name=Ragnarok Offline
Comment=The December 2003 euRO client on the single-player server (latest session snapshot)
Exec=$S/play.sh
Icon=$icon
Terminal=false
Categories=Game;RolePlaying;
Actions=fullscreen;settings;stop;

[Desktop Action fullscreen]
Name=Fullscreen
Exec=$S/play.sh --fullscreen

[Desktop Action settings]
Name=Client settings (Setup.exe)
Exec=$S/play.sh --settings

[Desktop Action stop]
Name=Stop (save and quit)
Exec=$S/play.sh --stop
EOD
update-desktop-database "$apps" 2>/dev/null || true
echo "installed $apps/ragnarok-offline.desktop -> $S/play.sh"
