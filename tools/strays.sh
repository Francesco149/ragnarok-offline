#!/usr/bin/env bash
# strays.sh — find (and with --clean, stop) anything this project left running.
# Runs on the host. Start and end every session with it.
#
#   tools/strays.sh            report only; exit 1 if anything stray was found
#   tools/strays.sh --clean    also take workbenches down, kill host strays,
#                              stop the Windows rig's agents
#   tools/strays.sh --no-win   skip the Windows box (offline, or not ours today)
#
# Looks at:
#   * workbench containers (label ro-wb): reported with age and every process
#     beyond PID 1's sleep. --clean takes them all down (`wb down --all`).
#   * ro-play containers (label ro-play): the owner's game. Reported, NEVER
#     touched, not even with --clean.
#   * host processes outside any container that belong to this project: Wine,
#     Ragexe, Xvfb, our server binary, cargo, whose cwd or command line points
#     into the repo or the data folder. Other sessions' processes (another
#     CLAUDE_CODE_SESSION_ID or another project's cwd) are not ours: left alone.
#   * the Windows rig (WIN_HOST): its agents' scheduled tasks and processes,
#     anything running from WIN_DIR, a mounted disc image. --clean runs
#     `win.sh down` and dismounts the image; it never deletes files there.
#     Also reports a drive under win.sh's free-space floor (WIN_MIN_FREE_GB).
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DATA="${WB_DATA:-$HOME/data/ragnarok-cd}"
clean=0 win=1
for a in "$@"; do
  case "$a" in
    --clean) clean=1 ;; --no-win) win=0 ;;
    *) sed -n '2,9p' "$0"; exit 2 ;;
  esac
done
found=0
say() { printf '%s\n' "$*"; }

# --- containers -------------------------------------------------------------
for id in $(podman ps -q --filter label=ro-wb 2>/dev/null); do
  name=$(podman inspect -f '{{.Name}}' "$id"); started=$(podman inspect -f '{{.State.StartedAt}}' "$id")
  age=$(( $(date +%s) - $(date -d "${started%%.*}" +%s) ))
  # everything but podman-init and the lifetime sleep is work someone started
  work=$(podman top "$id" pid args 2>/dev/null | awk 'NR>1 && $2 !~ /podman-init/ && !($2=="sleep" && NF==3)')
  found=1
  say "workbench $name: up $((age / 60)) min$([ -n "$work" ] && printf ', running:\n%s' "$(sed 's/^/    /' <<<"$work")")"
done
for id in $(podman ps -q --filter label=ro-play 2>/dev/null); do
  say "ro-play $(podman inspect -f '{{.Name}}' "$id"): the owner's game, left alone"
done
if [ $clean = 1 ] && [ -n "$(podman ps -q --filter label=ro-wb 2>/dev/null)" ]; then
  "$ROOT/tools/wb" down --all && say "  -> workbenches down"
fi

# --- host processes outside containers --------------------------------------
self=$$
# started by rust-analyzer (its cargo check). Only that: agent sessions and
# terminals descend from the editor too, and their strays are exactly the point.
editor_child() {
  local q=$1 c
  while [ "$q" -gt 1 ] 2>/dev/null; do
    c=$(cat /proc/"$q"/comm 2>/dev/null) || return 1
    case "$c" in rust-analyzer*) return 0 ;; esac
    q=$(awk '{print $4}' /proc/"$q"/stat 2>/dev/null) || return 1
  done
  return 1
}
for p in $(pgrep -f '[Ww]ine|[Rr]agexe|[Xx]vfb|[r]agnarok-offline|[c]argo|[r]ustc' 2>/dev/null); do
  [ "$p" = "$self" ] && continue
  grep -q libpod /proc/"$p"/cgroup 2>/dev/null && continue            # inside a container: covered above
  cwd=$(readlink /proc/"$p"/cwd 2>/dev/null) || continue
  args=$(tr '\0' ' ' < /proc/"$p"/cmdline 2>/dev/null)
  case "$args" in *strays.sh*) continue ;; esac
  if editor_child "$p"; then continue; fi                                # rust-analyzer's cargo check etc.
  case "$cwd $args" in
    *"$ROOT"*|*"$DATA"*|*ragnarok-offline*) ;;
    *) continue ;;
  esac
  found=1
  say "host process $p (cwd $cwd): ${args:0:160}"
  if [ $clean = 1 ]; then kill "$p" 2>/dev/null && say "  -> killed"; fi
done

# --- the Windows rig ---------------------------------------------------------
if [ $win = 1 ]; then
  WDIR="${WIN_DIR:-F:\\ro-test}"
  # read-only, and it must run on a full disk too: past win.sh's disk guard
  out=$(WIN_DISK_OK=1 WIN_HOST="${WIN_HOST:-win-wifi}" timeout 30 "$ROOT/tools/win-test/win.sh" ps "
    \$d='$WDIR'
    Get-ScheduledTask -TaskName 'ro-agent-*' -ErrorAction SilentlyContinue | % { 'task ' + \$_.TaskName + ' ' + \$_.State }
    Get-CimInstance Win32_Process | ? { \$_.CommandLine -like \"*\$d*\" -or \$_.ExecutablePath -like \"\$d*\" -or \$_.Name -match '^(Ragexe|Sakray|setup|ro-launcher)' } |
      % { 'process ' + \$_.ProcessId + ' ' + \$_.Name + ' ' + \$_.CommandLine }
    Get-DiskImage -ImagePath (Get-ChildItem \"\$d\\*.iso\" -ErrorAction SilentlyContinue).FullName -ErrorAction SilentlyContinue |
      ? Attached | % { 'mounted ' + \$_.ImagePath }" 2>&1)
  rc=$?
  if [ $rc = 124 ] || grep -qiE 'could not resolve|timed out|refused|no route' <<<"$out"; then
    say "windows: not reachable (${WIN_HOST:-win-wifi}); not checked"; reach=0
  elif [ -n "$(tr -d '[:space:]' <<<"$out")" ]; then
    found=1; say "windows:"; sed 's/^/    /' <<<"$out"
    if [ $clean = 1 ]; then
      "$ROOT/tools/win-test/win.sh" down >/dev/null
      "$ROOT/tools/win-test/win.sh" ps "Get-CimInstance Win32_Process | ? { \$_.ExecutablePath -like '$WDIR*' -or \$_.Name -match '^(Ragexe|Sakray)' } | % { Stop-Process -Id \$_.ProcessId -Force }
        Get-ChildItem '$WDIR\\*.iso' -ErrorAction SilentlyContinue | % { Dismount-DiskImage -ImagePath \$_.FullName -ErrorAction SilentlyContinue | Out-Null }" >/dev/null
      say "  -> agents down, processes stopped, image dismounted"
    fi
  fi
  # low disk wedged the box's sshd once: say so at every session's start and end
  if [ "${reach:-1}" = 1 ]; then
    disk=$(WIN_HOST="${WIN_HOST:-win-wifi}" "$ROOT/tools/win-test/win.sh" df 2>&1); drc=$?
    if [ $drc = 3 ]; then
      found=1; say "windows: LOW DISK (under ${WIN_MIN_FREE_GB:-20} GB free; win.sh refuses to run until it is freed):"
      sed 's/^/    /' <<<"$disk"
    fi
  fi
fi

[ $found = 0 ] && say "no strays"
[ $clean = 1 ] && exit 0
exit $found
