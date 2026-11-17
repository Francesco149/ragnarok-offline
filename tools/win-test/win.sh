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
#   win.sh df                 free space on C: and WIN_DIR's drive, WIN_DIR's size
#                             (exit 3 if either drive is under WIN_MIN_FREE_GB)
#
# WIN_HOST (default win-wifi) is an ssh alias for Windows' own OpenSSH with
# PowerShell as the default shell; WIN_DIR (default F:\ro-test) holds
# everything this rig creates on the box. Snippet helpers: see agent.ps1.
#
# Disk guard. A Windows box that ran out of disk once left its sshd sessions
# wedged. Every command but `down` and `df` first checks that C: and WIN_DIR's
# drive each have WIN_MIN_FREE_GB free (default 20; a push needs its own size
# on top) and refuses to start otherwise. An agent that hits the refusal stops
# its Windows work and tells the orchestrator; it does not delete anything to
# get past it. WIN_DISK_OK=1 skips the check: for the orchestrator, once the
# space is dealt with or to clean up.
set -euo pipefail
HOST="${WIN_HOST:-win-wifi}"
DIR="${WIN_DIR:-F:\\ro-test}"
MIN_GB="${WIN_MIN_FREE_GB:-20}"
SSH_OPTS=(-o BatchMode=yes -o LogLevel=ERROR -o ConnectTimeout=15 -o ServerAliveInterval=15 -o ServerAliveCountMax=4)
SHOTS="${WIN_SHOTS:-$HOME/data/ragnarok-cd/shots/win11}"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# PowerShell over ssh without quoting trouble: -EncodedCommand (UTF-16LE base64);
# PS_TIMEOUT=N gives up after N seconds (exit 124)
ps() {
  local b64; b64="$(printf '%s' "\$ProgressPreference='SilentlyContinue'; $1" | iconv -t UTF-16LE | base64 -w0)"
  timeout "${PS_TIMEOUT:-0}" ssh "${SSH_OPTS[@]}" "$HOST" "powershell -NoProfile -NonInteractive -EncodedCommand $b64" 2>&1 \
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

# Free space on the system drive and WIN_DIR's: one line per drive, "LOW" on those under
# the floor (+ $1 extra bytes), then WIN_DIR's size. Exit 3 if any is low, 4 if the box
# did not answer in time.
df_report() {
  local extra="${1:-0}" out rc=0
  out=$(PS_TIMEOUT=60 ps "
    \$need = $MIN_GB * 1GB + $extra
    \$drives = @(\$env:SystemDrive.TrimEnd(':'), '$DIR'.Substring(0,1)) | Select-Object -Unique
    foreach (\$d in \$drives) { \$p = Get-PSDrive \$d
      '{0}: {1:N1} GB free of {2:N1} GB{3}' -f \$d, (\$p.Free/1GB), ((\$p.Free+\$p.Used)/1GB), \$(if (\$p.Free -lt \$need) {'  LOW'} else {''}) }
    if (Test-Path '$DIR') { '$DIR holds {0:N1} GB' -f ((Get-ChildItem '$DIR' -Recurse -Force -File -ErrorAction SilentlyContinue | Measure-Object Length -Sum).Sum/1GB) }") || rc=$?
  if [ $rc = 124 ] || ! grep -q ' GB free of ' <<<"$out"; then
    printf 'no answer from %s within 60 s%s\n' "$HOST" "$( [ -n "$out" ] && printf ':\n%s' "$out")"; return 4
  fi
  printf '%s\n' "$out"
  ! grep -q 'LOW$' <<<"$out" || return 3
}

disk_guard() {
  [ "${WIN_DISK_OK:-}" = 1 ] && return 0
  local stamp="${XDG_RUNTIME_DIR:-/tmp}/ro-win-disk-ok-$HOST" extra="${1:-0}" out rc=0
  # a passing check holds for two minutes (a test loop calls this often); pushes always check
  if [ "$extra" = 0 ] && [ -n "$(find "$stamp" -mmin -2 2>/dev/null)" ]; then return 0; fi
  out=$(df_report "$extra") || rc=$?
  if [ $rc = 0 ]; then touch "$stamp"; return 0; fi
  rm -f "$stamp"
  cat >&2 <<MSG
win.sh: REFUSED to start '$cmd' on $HOST.
$out
$( [ $rc = 3 ] && echo "A drive has less than $MIN_GB GB free (plus what this command would add)." \
   || echo "The box did not answer: it may be out of disk, which has wedged its sshd before.")
Agent: stop the Windows part of this task now. Do not delete files on the box and do not
retry to get past this. Tell the orchestrator (whoever gave you the task) that the Windows
machine needs its disk space checked and freed before Windows runs continue, with the
lines above. Orchestrator: free space (WIN_DIR holds only this rig's files), or set
WIN_DISK_OK=1 for this rig's own clean-up.
MSG
  exit 5
}

cmd="${1:-}"; shift || true
case "$cmd" in
  up|ps|do|doa|shot|pull) disk_guard ;;
  push) disk_guard "$(du -sb "${1:?push LOCAL REMOTE}" | cut -f1)" ;;
esac
case "$cmd" in
  up)
    ps "New-Item -ItemType Directory -Force '$DIR\\agent' | Out-Null; Remove-Item -Force -ErrorAction SilentlyContinue '$DIR\\agent\\q-*\\*'" >/dev/null
    scp -q "${SSH_OPTS[@]}" "$HERE/agent.ps1" "$HOST:${DIR//\\//}/agent/agent.ps1"
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
    scp -q "${SSH_OPTS[@]}" "$HOST:${DIR//\\//}/shots/$name.png" "$SHOTS/$name.png" && echo "$SHOTS/$name.png" ;;
  push) scp -q -r "${SSH_OPTS[@]}" "$1" "$HOST:${2//\\//}" ;;
  pull) scp -q -r "${SSH_OPTS[@]}" "$HOST:${1//\\//}" "$2" ;;
  df)   df_report ;;
  *) sed -n '2,28p' "$0"; exit 2 ;;
esac
