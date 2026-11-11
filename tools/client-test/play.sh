#!/usr/bin/env bash
# Playthrough helper on top of rig.sh (runs INSIDE the workbench, client logged in).
#
#   play.sh ev 'lua'            eval on the server (`{player.map, player.x, player.y}`)
#   play.sh pos                 where the character stands
#   play.sh at MAP X Y          warp there (check the cell with research/gat_cells.py first)
#   play.sh cell DX DY          click the cell DX east / DY north of the character
#   play.sh talk X Y            click the NPC standing on map cell X Y (must be on screen)
#   play.sh walk X Y            click along a path to map cell X Y, or to the nearest cell that can be
#                               reached (within NEAR=6 cells of it: a warp tile on the way ends the walk)
#   play.sh snap NAME           800x600 crop of the game -> /data/shots/play/NAME.png
#   play.sh say N               press Return N times, 0.6 s apart (dialogue "next"/"close")
#   play.sh pick N              choose the Nth entry of an NPC menu (1 = first)
#   play.sh adv                 Return through the pages up to the next menu, input box or the end
#   play.sh said [N]            the last N NPC packets, decoded (said.py)
#   play.sh steps 'a; b; c'     several of the above in one call
#
# Screen geometry: the game is the 800x600 area at +115+113 of the 1024x768
# display; at the default zoom and angle the character's feet are near
# (515,425) and one cell is about 34 px wide and 26 px high on screen.
set -euo pipefail
export DISPLAY=:99
R=/work/tools/client-test/rig.sh
S="/work/target/debug/ragnarok-offline -c /work/dev.toml eval"
CX=${CX:-517} CY=${CY:-415} # not 515: the client ignores clicks on that very column
# Pixels per cell differ between outdoor and indoor maps (the camera sits
# nearer indoors); `walk` measures them from how far a click took the
# character and keeps them per map here.
SCALE=/tmp/play.scale
scale() { PX=35 PY=25; [ -f "$SCALE.$1" ] && read -r PX PY < "$SCALE.$1"; return 0; }
settle() { local a b; a=$(pos); while sleep 0.35; b=$(pos); [ "$a" != "$b" ]; do a=$b; done; echo "$b"; }
pos() { (cd /work && $S '{player.map, player.x, player.y}' </dev/null) | tr -d '[]"' | tr ',' ' '; }
cell() { read -r m _ < <(pos); scale "$m"; $R click $((CX + $1 * PX)) $((CY - $2 * PY)); }
one() {
  local c=$1; shift || true
  case "$c" in
    ev) (cd /work && $S "$*" </dev/null) ;;
    pos) pos ;;
    at) (cd /work && $S "warp(\"$1\",$2,$3)" </dev/null) >/dev/null; sleep "${W:-4}" ;;
    cell) cell "$1" "$2" ;;
    talk) read -r m x y < <(pos); scale "$m"
      local sx=$((CX + ($1 - x) * PX)) sy=$((CY - ($2 - y) * PY - 22))
      if [ $sx -lt 120 ] || [ $sx -gt 910 ] || [ $sy -lt 120 ] || [ $sy -gt 700 ]; then echo "talk: $1,$2 is off screen from $x,$y" >&2; return 1; fi
      $R click $sx $sy; sleep 1.2 ;;
    walk) # breadth-first search on the server's cells; click the farthest path cell still on screen
      local lua="local tx,ty=$1,$2 local m,sx,sy=player.map,player.x,player.y local w,h=table.unpack(map_size(m)) local prev={} local q={{sx,sy}} prev[sy*w+sx]=-1 local qi=1 local bx,by,bd=sx,sy,1e9 while q[qi] do local x,y=q[qi][1],q[qi][2] qi=qi+1 local d=math.max(math.abs(x-tx),math.abs(y-ty)) if d<bd then bx,by,bd=x,y,d end if d==0 then break end for _,d in ipairs({{1,0},{-1,0},{0,1},{0,-1},{1,1},{1,-1},{-1,1},{-1,-1}}) do local nx,ny=x+d[1],y+d[2] if nx>=0 and ny>=0 and nx<w and ny<h and prev[ny*w+nx]==nil and walkable(m,nx,ny) and (d[1]==0 or d[2]==0 or (walkable(m,x+d[1],y) and walkable(m,x,y+d[2]))) then prev[ny*w+nx]=y*w+x q[#q+1]={nx,ny} end end end if bd>NEAR then return {99,99} end if bx==sx and by==sy then return {0,0} end local c=by*w+bx local best={bx-sx,by-sy} while c~=-1 do local x,y=c%w,c//w if math.abs(x-sx)<=REACH and y-sy<=math.min(REACH,6) and sy-y<=math.min(REACH,5) then best={x-sx,y-sy} break end c=prev[c] end return best"
      local reach=8 m x y nx ny mx my
      for _ in $(seq 80); do
        read -r dx dy < <( (cd /work && $S "REACH=$reach NEAR=${NEAR:-6} $lua" </dev/null) | tr -d '[]' | tr ',' ' ')
        [ "$dx" = 0 ] && [ "$dy" = 0 ] && break
        [ "$dx" = 99 ] && { echo "walk: no path to $1,$2" >&2; pos; return 1; }
        read -r m x y < <(pos); scale "$m"
        $R click $((CX + dx * PX)) $((CY - dy * PY)); sleep 0.4
        read -r _ nx ny < <(settle); mx=$((nx - x)); my=$((ny - y))
        if [ $mx -eq 0 ] && [ $my -eq 0 ]; then
          reach=$((reach > 2 ? reach - 2 : 1)) # the client would not walk there (a sprite or a wall under the pointer): try nearer
        else
          reach=8 # the click was meant for dx,dy and went mx,my: correct the scale
          [ ${dx#-} -ge 3 ] && [ ${mx#-} -ge 2 ] && PX=$(( (PX * ${dx#-} / ${mx#-} + PX) / 2 ))
          [ ${dy#-} -ge 3 ] && [ ${my#-} -ge 2 ] && PY=$(( (PY * ${dy#-} / ${my#-} + PY) / 2 ))
          [ $PX -lt 25 ] && PX=25; [ $PX -gt 60 ] && PX=60; [ $PY -lt 18 ] && PY=18; [ $PY -gt 50 ] && PY=50
          echo "$PX $PY" > "$SCALE.$m"
        fi
      done; pos ;;
    snap) mkdir -p /data/shots/play; xdotool mousemove 905 420; sleep 0.3; $R shot _p >/dev/null
      magick /data/shots/_p.png -crop 800x600+115+113 +repage "/data/shots/play/$1.png"; echo "play/$1.png" ;;
    say) for _ in $(seq "${1:-1}"); do $R key Return; sleep 0.6; done ;;
    pick) for _ in $(seq 2 "$1"); do $R key Down; sleep 0.2; done; $R key Return; sleep 0.8 ;;
    adv) # press Return through next/close pages until a menu, an input box or the end; then show what was said
      local st n0; n0=$(wc -l < /work/logs/packets.log)
      for _ in $(seq 60); do
        st=$(cd /work && python3 tools/client-test/said.py --state)
        case "$st" in next) $R key Return; sleep 0.7 ;; close) $R key Return; sleep 0.7; break ;; *) break ;; esac
      done
      (cd /work && tail -n +"$n0" logs/packets.log > /tmp/adv.log && python3 tools/client-test/said.py 400 /tmp/adv.log); echo "== $st" ;;
    said) (cd /work && python3 tools/client-test/said.py "${1:-30}") ;;
    sleep) sleep "$1" ;;
    *) sed -n '2,17p' "$0"; exit 2 ;;
  esac
}
if [ "${1:-}" = steps ]; then
  IFS=';' read -ra parts <<< "$2"
  for p in "${parts[@]}"; do read -ra w <<< "$p"; [ ${#w[@]} -gt 0 ] && one "${w[@]}"; done
else
  one "$@"
fi
