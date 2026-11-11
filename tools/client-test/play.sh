#!/usr/bin/env bash
# Playthrough helper on top of rig.sh (runs INSIDE the workbench, client logged in).
#
#   play.sh ev 'lua'            eval on the server (`{player.map, player.x, player.y}`)
#   play.sh pos                 where the character stands
#   play.sh at MAP X Y          warp there (check the cell with research/gat_cells.py first)
#   play.sh cell DX DY          click the cell DX east / DY north of the character
#   play.sh talk X Y            click the NPC standing on map cell X Y (must be on screen; UP=px above its feet, 22)
#   play.sh walk X Y            click along a path to map cell X Y, or to the nearest cell that can be
#                               reached (within NEAR=6 cells of it: a warp tile on the way ends the walk)
#                               AVOID='{{x0,y0,x1,y1},...}' keeps off those cells (walkable traps such as the
#                               Swordman course's pits: research/course_check.py --avoid), clicking only where
#                               the whole rectangle up to the clicked cell is clear, so the client's path is too, and only
#                               HOP=3 cells away (farther, a click can land a cell off)
#   play.sh calib               measure how this map's cells lie on the screen (done by itself when needed)
#   play.sh snap NAME           800x600 crop of the game -> /data/shots/play/NAME.png
#   play.sh say N               press Return N times, 0.6 s apart (dialogue "next"/"close")
#   play.sh pick N              choose the Nth entry of an NPC menu (1 = first)
#   play.sh num N | text S      type into the NPC's number or text box and confirm
#   play.sh adv                 Return through the pages up to the next menu, input box or the end
#   play.sh run X Y [N...]      talk to the NPC on X Y and go through its dialogue, picking N... at the menus
#   play.sh said [N]            the last N NPC packets, decoded (said.py)
#   play.sh steps 'a; b; c'     several of the above in one call
#
# Screen geometry: the game is the 800x600 area at +115+113 of the 1024x768
# display, the character's feet near (517,415). How a cell lies on the screen
# depends on the map: outdoors one cell is about 35 px across and 25 px up;
# indoor maps bring the camera nearer (45 px), and some turn it (geffen_in
# is seen from a corner). `calib` finds out with two probe clicks and keeps
# the answer per map; walk and talk run it the first time they see a map.
set -euo pipefail
export DISPLAY=:99
R=/work/tools/client-test/rig.sh
S="/work/target/debug/ragnarok-offline -c /work/dev.toml eval"
CX=${CX:-517} CY=${CY:-415} # not 515: the client ignores clicks on that very column
GEO=/tmp/play.geo # per map "a b c d": a cell dx,dy is drawn a*dx+b*dy px right and c*dx+d*dy px down
pos() { (cd /work && $S '{player.map, player.x, player.y}' </dev/null) | tr -d '[]"' | tr ',' ' '; }
settle() { local a b; a=$(pos); while sleep 0.35; b=$(pos); [ "$a" != "$b" ]; do a=$b; done; echo "$b"; }
calib() {
  local m x0 y0 x1 y1 x2 y2 s1 s2 d
  read -r m x0 y0 < <(pos)
  x1=$x0 y1=$y0
  for d in 210 140 70; do for s1 in 1 -1; do
    $R click $((CX + s1 * d)) $CY; sleep 0.4; read -r _ x1 y1 < <(settle)
    [ "$x1 $y1" != "$x0 $y0" ] && break 2
  done; done
  local d1=$d; x2=$x1 y2=$y1
  for d in 150 100 50; do for s2 in 1 -1; do
    $R click $CX $((CY - s2 * d)); sleep 0.4; read -r _ x2 y2 < <(settle)
    [ "$x2 $y2" != "$x1 $y1" ] && break 2
  done; done
  # Two probes of whole cells are too coarse to solve for the four numbers, so pick the best of
  # the views the client has: the camera turned by a multiple of 45 degrees, a cell 35 px across
  # (outdoors) or 45 px (indoors) and 25 px up.
  awk -v u1=$((x1 - x0)) -v v1=$((y1 - y0)) -v u2=$((x2 - x1)) -v v2=$((y2 - y1)) -v p1=$((s1 * d1)) -v p2=$((-s2 * d)) 'BEGIN {
    pi = atan2(0, -1); best = 1e9
    for (k = 0; k < 8; k++) for (sx = 35; sx <= 45; sx += 10) {
      t = k * pi / 4; sy = 25
      a = sx * cos(t); b = -sx * sin(t); c = -sy * sin(t); d = -sy * cos(t); det = a * d - b * c
      # cells moved by a click p1 px right, and by one p2 px down (p2 < 0 is up)
      e = (d * p1 / det - u1) ^ 2 + (-c * p1 / det - v1) ^ 2 + (-b * p2 / det - u2) ^ 2 + (a * p2 / det - v2) ^ 2
      if (e < best) { best = e; out = sprintf("%.1f %.1f %.1f %.1f", a, b, c, d) }
    }
    if (u1 == 0 && v1 == 0 && u2 == 0 && v2 == 0) exit 1
    print out }' > "$GEO.$m" \
    || { rm -f "$GEO.$m"; echo "calib: the probe clicks did not move the character on $m" >&2; return 1; }
  echo "calib $m: $(cat "$GEO.$m")" >&2
  NEAR=3 one walk "$x0" "$y0" >/dev/null || true # back to where the probes started
}
geo() { [ -f "$GEO.$1" ] || calib || true; A=35 B=0 C=0 D=-25; [ -f "$GEO.$1" ] && read -r A B C D < "$GEO.$1"; return 0; }
# px DX DY: screen coordinates of the cell DX east and DY north of the character (after `geo MAP`).
# A cell higher or lower than the character's is drawn higher or lower: without that, a click on a
# corridor beside a raised block lands on the block (job_sword1 x 78 on, the first stretch). One unit
# of height (5 to a cell's width) is about a seventh of a cell's width in pixels at the period camera.
GRF=${GRF:-/data/play/RagnarokOnline/data.grf}
heights() { [ -s "/tmp/play.h.$1" ] || python3 /work/research/gat_cells.py "$GRF" "$1" --heights > "/tmp/play.h.$1" 2>/dev/null || rm -f "/tmp/play.h.$1"; }
px() {
  local m x y; read -r m x y < <(pos); heights "$m"
  awk -v a="$A" -v b="$B" -v c="$C" -v d="$D" -v dx="$1" -v dy="$2" -v cx="$CX" -v cy="$CY" -v x="$x" -v y="$y" '
    NR == 1 { w = $1; next } NR - 2 == y { h0 = $(x + 1) } NR - 2 == y + dy { h1 = $(x + dx + 1) }
    END { k = sqrt(a * a + b * b) / 7; printf "%d %d\n", cx + a * dx + b * dy, cy + c * dx + d * dy + k * (h1 - h0) }' "/tmp/play.h.$m" 2>/dev/null \
  || awk -v a="$A" -v b="$B" -v c="$C" -v d="$D" -v dx="$1" -v dy="$2" -v cx="$CX" -v cy="$CY" 'BEGIN { printf "%d %d\n", cx + a * dx + b * dy, cy + c * dx + d * dy }'
}
onscreen() { [ "$1" -ge 130 ] && [ "$1" -le 900 ] && [ "$2" -le 630 ] && { [ "$2" -ge 245 ] || { [ "$1" -ge 410 ] && [ "$2" -ge 130 ]; }; }; }
cell() { local m sx sy; read -r m _ < <(pos); geo "$m"; read -r sx sy < <(px "$1" "$2"); $R click "$sx" "$sy"; }
one() {
  local c=$1; shift || true
  case "$c" in
    ev) (cd /work && $S "$*" </dev/null) ;;
    pos) pos ;;
    at) (cd /work && $S "warp(\"$1\",$2,$3)" </dev/null) >/dev/null; sleep "${W:-4}" ;;
    cell) cell "$1" "$2" ;;
    talk) # click the unit on that cell (22 px above its feet) and make sure the client sent a talk or an attack
      local m x y sx sy n0
      read -r m x y < <(pos); geo "$m"; read -r m x y < <(pos)
      read -r sx sy < <(px $(($1 - x)) $(($2 - y))); sy=$((sy - ${UP:-22}))
      onscreen "$sx" "$sy" || { echo "talk: $1,$2 is off screen from $x,$y" >&2; return 1; }
      n0=$(grep -c "< 0090\|< 0089" /work/logs/packets.log || true)
      $R click "$sx" "$sy"; sleep 1.2
      [ "$(grep -c "< 0090\|< 0089" /work/logs/packets.log || true)" != "$n0" ] \
        || { echo "talk: nothing answered at $1,$2 (from $(pos); clicked $sx,$sy)" >&2; return 1; } ;;
    calib) calib ;;
    walk) # breadth-first search on the server's cells; click the farthest path cell that is on screen
      local lua="local tx,ty=$1,$2 local m,sx,sy=player.map,player.x,player.y local w,h=table.unpack(map_size(m)) local AV=${AVOID:-nil} local pit={} for _,r in ipairs(AV or {}) do for x=r[1],r[3] do for y=r[2],r[4] do pit[y*w+x]=true end end end local function ok(x,y) return not pit[y*w+x] and walkable(m,x,y) end local function boxok(x0,y0,x1,y1) if not AV then return true end for x=math.min(x0,x1),math.max(x0,x1) do for y=math.min(y0,y1),math.max(y0,y1) do if not ok(x,y) then return false end end end return true end local prev={} local q={{sx,sy}} prev[sy*w+sx]=-1 local qi=1 local bx,by,bd=sx,sy,1e9 while q[qi] do local x,y=q[qi][1],q[qi][2] qi=qi+1 local d=math.max(math.abs(x-tx),math.abs(y-ty)) if d<bd then bx,by,bd=x,y,d end if d==0 then break end for _,d in ipairs({{1,0},{-1,0},{0,1},{0,-1},{1,1},{1,-1},{-1,1},{-1,-1}}) do local nx,ny=x+d[1],y+d[2] if nx>=0 and ny>=0 and nx<w and ny<h and prev[ny*w+nx]==nil and ok(nx,ny) and (d[1]==0 or d[2]==0 or (ok(x+d[1],y) and ok(x,y+d[2]))) then prev[ny*w+nx]=y*w+x q[#q+1]={nx,ny} end end end if bd>NEAR then return {99,99} end if bx==sx and by==sy then return {0,0} end local c=by*w+bx local best={bx-sx,by-sy} while c~=-1 do local x,y=c%w,c//w if math.max(math.abs(x-sx),math.abs(y-sy))<=REACH and boxok(sx,sy,x,y) then best={x-sx,y-sy} if ALT>0 and ok(x,y+ALT) and boxok(sx,sy,x,y+ALT) then best={x-sx,y+ALT-sy} end break end c=prev[c] end return best"
      local top=${HOP:-$([ -n "${AVOID:-}" ] && echo 3 || echo 8)} m x y nx ny sx sy dx dy; local reach=$top alt=0
      read -r m _ < <(pos); geo "$m"
      for _ in $(seq ${TRIES:-120}); do
        read -r dx dy < <( (cd /work && $S "REACH=$reach ALT=$alt NEAR=${NEAR:-6} $lua" </dev/null) | tr -d '[]' | tr ',' ' ')
        [ "$dx" = 0 ] && [ "$dy" = 0 ] && break
        [ "$dx" = 99 ] && { echo "walk: no path to $1,$2" >&2; pos; return 1; }
        case "$dx" in ''|*[!0-9-]*) echo "walk: $dx $dy" >&2; pos; return 1 ;; esac
        read -r nx x y < <(pos); [ "$nx" = "$m" ] || break # a warp tile took us to another map
        read -r sx sy < <(px "$dx" "$dy")
        if ! onscreen "$sx" "$sy"; then [ $reach -le 1 ] && { echo "walk: cannot click towards $1,$2" >&2; pos; return 1; }; reach=$((reach - 1)); continue; fi
        $R click "$sx" "$sy"; sleep 0.4
        read -r _ nx ny < <(settle)
        if [ "$nx $ny" = "$x $y" ]; then
          # the client would not walk there: a sprite under the pointer, or the cell is hidden behind
          # something nearer the camera (a raised block south of it): try a cell or two north, then nearer
          if [ $alt -lt 2 ]; then alt=$((alt + 1)); else alt=0; reach=$((reach > 2 ? reach - 2 : 1)); fi
        else
          reach=$top alt=0
        fi
      done; pos ;;
    snap) mkdir -p /data/shots/play; xdotool mousemove 905 420; sleep 0.3; $R shot _p >/dev/null
      magick /data/shots/_p.png -crop 800x600+115+113 +repage "/data/shots/play/$1.png"; echo "play/$1.png" ;;
    say) for _ in $(seq "${1:-1}"); do $R key Return; sleep 0.6; done ;;
    pick) for _ in $(seq 2 "$1"); do $R key Down; sleep 0.2; done; $R key Return; sleep 0.8 ;;
    adv) # press Return through next/close pages until a menu, an input box or the end; then show what was said
      local st n0; n0=${N0:-$(wc -l < /work/logs/packets.log)}
      for _ in $(seq 60); do
        st=$(cd /work && python3 tools/client-test/said.py --state)
        case "$st" in next) $R key Return; sleep 0.7 ;; close) $R key Return; sleep 0.7; break ;; *) break ;; esac
      done
      (cd /work && tail -n +"$n0" logs/packets.log > /tmp/adv.log && python3 tools/client-test/said.py 400 /tmp/adv.log); echo "== $st" ;;
    run) # talk X Y, then answer every menu with its first entry (or the given numbers in turn) to the end
      local tx=$1 ty=$2 st a; shift 2
      for _ in $(seq 60); do
        N0=$(wc -l < /work/logs/packets.log); [ -n "$tx" ] && { one talk "$tx" "$ty"; tx=; }
        one adv > /tmp/run.out; unset N0; st=$(tail -1 /tmp/run.out); grep -v "^  \[next\]\|^  > next" /tmp/run.out | grep -v "^==" || true
        case "$st" in "== menu") a=${1:-1}; [ $# -gt 0 ] && shift; N0=$(wc -l < /work/logs/packets.log); one pick "$a" ;; *) break ;; esac
      done; echo "$st"; pos ;;
    said) (cd /work && python3 tools/client-test/said.py "${1:-30}") ;;
    num|text) $R type "$*"; sleep 0.3; $R key Return; sleep 0.8 ;; # fill an NPC's number or text box
    sleep) sleep "$1" ;;
    *) sed -n '2,20p' "$0"; exit 2 ;;
  esac
}
if [ "${1:-}" = steps ]; then
  IFS=';' read -ra parts <<< "$2"
  for p in "${parts[@]}"; do read -ra w <<< "$p"; [ ${#w[@]} -gt 0 ] && one "${w[@]}"; done
else
  one "$@"
fi
