#!/usr/bin/env bash
# hunt.sh NAME [KILLS]: walk to the nearest monster of that name, click it until it is dead, pick up what it drops
P=/work/tools/client-test/play.sh
name=$1; kills=${2:-1}
for k in $(seq "$kills"); do
  read -r id mx my < <($P ev "local me=player local best,bd for _,u in ipairs(nearby(14)) do if u.kind=='mob' and u.name=='$name' then local d=math.max(math.abs(u.x-me.x),math.abs(u.y-me.y)) if not bd or d<bd then best,bd=u,d end end end return best and {best.id,best.x,best.y} or {0,0,0}" | tr -d '[]' | tr ',' ' ')
  [ "$id" = 0 ] && { echo "no $name near $($P pos)"; exit 1; }
  NEAR=3 $P walk "$mx" $((my - 2)) >/dev/null
  for _ in $(seq 25); do
    read -r hp x y < <($P ev "for _,u in ipairs(nearby(14)) do if u.id==$id then return {u.info.hp,u.x,u.y} end end return {0,0,0}" | tr -d '[]' | tr ',' ' ')
    [ "$hp" = 0 ] && break
    $P talk "$x" "$y" 2>/dev/null || NEAR=3 $P walk "$x" $((y - 2)) >/dev/null
    sleep 1
  done
  echo "kill $k: $name $id hp=$hp; me $($P pos) hp $($P ev 'player.hp')"
  # pick up: click each item lying within 3 cells
  for _ in 1 2 3; do
    read -r ix iy < <($P ev "local me=player for _,u in ipairs(nearby(4)) do if u.kind=='item' then return {u.x,u.y} end end return {0,0}" | tr -d '[]' | tr ',' ' ')
    [ "$ix" = 0 ] && break
    read -r _ px py < <($P pos)
    PYOFF=0 $P cell $((ix - px)) $((iy - py)); sleep 1.5
  done
done
