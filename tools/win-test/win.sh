#!/usr/bin/env bash
# win.sh — drive a real Windows machine over ssh for client/installer tests.
# Runs on the host (it only talks ssh/scp; nothing heavy runs locally).
#
#   win.sh up                 copy agent.ps1, start the desktop agents (user + admin)
#   win.sh down               stop the agents, remove their scheduled tasks
#   win.sh ps  'code'         PowerShell in the ssh session (session 0, elevated, no desktop)
#   win.sh do  'code'         PowerShell inside the desktop session, medium integrity
#   win.sh doa 'code'         same, elevated (drives elevated windows, e.g. msiexec UI)
#   win.sh shot NAME [admin]  screenshot of the desktop -> $WIN_SHOTS/NAME.png
#   win.sh push LOCAL REMOTE  scp to the box;  win.sh pull REMOTE LOCAL
#
# WIN_HOST (default win-wifi) is an ssh alias for Windows' own OpenSSH with
# PowerShell as the default shell; WIN_DIR (default F:\ro-test) holds
# everything this rig creates on the box. Snippet helpers: see agent.ps1.
set -euo pipefail
HOST="${WIN_HOST:-win-wifi}"
DIR="${WIN_DIR:-F:\\ro-test}"
SHOTS="${WIN_SHOTS:-$HOME/data/ragnarok-cd/shots/win11}"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# PowerShell over ssh without quoting trouble: -EncodedCommand (UTF-16LE base64)
ps() {
  local b64; b64="$(printf '%s' "\$ProgressPreference='SilentlyContinue'; $1" | iconv -t UTF-16LE | base64 -w0)"
  ssh -o BatchMode=yes -o LogLevel=ERROR "$HOST" "powershell -NoProfile -NonInteractive -EncodedCommand $b64" 2>&1 \
    | { grep -v -e 'post-quantum' -e 'store now, decrypt' -e 'server may need to be upgraded' || true; } | tr -d '\r'
}

# queue a snippet for the desktop agent and wait for its output
agent() {
  local name="$1" code="$2" timeout="${WIN_TIMEOUT:-120}" b64
  b64="$(printf '%s' "$code" | base64 -w0)"
  ps "\$q='$DIR\\agent\\q-$name'; if (-not (Test-Path \$q)) { 'agent $name not running (win.sh up)'; exit 1 }
      \$id=(Get-Date -Format 'yyyyMMddHHmmssfff'); \$c=[Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('$b64'))
      [IO.File]::WriteAllText(\"\$q\\\$id.cmd\", \$c)
      \$t=0; while (-not (Test-Path \"\$q\\\$id.out\") -and \$t -lt $timeout*10) { Start-Sleep -Milliseconds 100; \$t++ }
      if (Test-Path \"\$q\\\$id.out\") { Get-Content -Raw \"\$q\\\$id.out\"; Remove-Item \"\$q\\\$id.out\" } else { 'TIMEOUT (agent busy or dead?)' }"
}

cmd="${1:-}"; shift || true
case "$cmd" in
  up)
    ps "New-Item -ItemType Directory -Force '$DIR\\agent' | Out-Null; Remove-Item -Force -ErrorAction SilentlyContinue '$DIR\\agent\\q-*\\*'" >/dev/null
    scp -q -o LogLevel=ERROR "$HERE/agent.ps1" "$HOST:${DIR//\\//}/agent/agent.ps1"
    ps "foreach (\$n in 'user','admin') {
          \$t = \"ro-agent-\$n\"; Unregister-ScheduledTask -TaskName \$t -Confirm:\$false -ErrorAction SilentlyContinue
          \$a = New-ScheduledTaskAction -Execute powershell.exe -Argument \"-NoProfile -WindowStyle Hidden -ExecutionPolicy Bypass -File $DIR\\agent\\agent.ps1 -Name \$n\"
          \$lvl = if (\$n -eq 'admin') { 'Highest' } else { 'Limited' }
          \$p = New-ScheduledTaskPrincipal -UserId \$env:USERNAME -LogonType Interactive -RunLevel \$lvl
          \$s = New-ScheduledTaskSettingsSet -ExecutionTimeLimit (New-TimeSpan -Hours 8) -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries
          Register-ScheduledTask -TaskName \$t -Action \$a -Principal \$p -Settings \$s | Out-Null
          Start-ScheduledTask -TaskName \$t }
        Start-Sleep 3; Get-Content '$DIR\\agent\\agent-*.log'" ;;
  down)
    ps "'user','admin' | % { \$q=\"$DIR\\agent\\q-\$_\"; if (Test-Path \$q) { [IO.File]::WriteAllText(\"\$q\\zz-quit.cmd\", 'quit') } }" >/dev/null
    ps "'user','admin' | % { Unregister-ScheduledTask -TaskName \"ro-agent-\$_\" -Confirm:\$false -ErrorAction SilentlyContinue }; 'down'" ;;
  ps)  ps "$*" ;;
  do)  agent user "$*" ;;
  doa) agent admin "$*" ;;
  shot)
    name="${1:-shot}"; who="${2:-user}"; mkdir -p "$SHOTS"
    agent "$who" "Shot '$DIR\\shots\\$name.png'" >/dev/null
    scp -q -o LogLevel=ERROR "$HOST:${DIR//\\//}/shots/$name.png" "$SHOTS/$name.png" && echo "$SHOTS/$name.png" ;;
  push) scp -q -r -o LogLevel=ERROR "$1" "$HOST:${2//\\//}" ;;
  pull) scp -q -r -o LogLevel=ERROR "$HOST:${1//\\//}" "$2" ;;
  *) sed -n '2,16p' "$0"; exit 2 ;;
esac
