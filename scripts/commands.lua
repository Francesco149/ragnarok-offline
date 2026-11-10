-- @commands typed in chat. command(name, fn(args_string)) - return a string to
-- show it to the player. Accounts with gm_level > 0 may use them (the
-- default config gives your auto-registered account 99).

local function words(s) local t = {} for w in s:gmatch("%S+") do t[#t + 1] = w end return t end

command("help", function()
  return "@warp <map> [x y], @go <town>, @where, @item <id|name> [n], @zeny <n>, @lvup [n], @joblvup [n], " ..
         "@job <id>, @heal, @monster <id|name> [n], @save, @stats, @reload, @allskill, @useskill <id|NAME> [lv]"
end)

-- Every skill of the current job at its top level (quest skills too).
command("allskill", function()
  local n = 0
  for _, s in ipairs(SKILL_DB) do
    if not s.npc and s.id < 10000 and _rt("skill_in_tree", s.id) then
      give_skill(s.id, s.max)
      n = n + 1
    end
  end
  return n .. " skills"
end)

-- Use a skill the way the client would: on the nearest monster, on yourself,
-- or on the nearest monster's cell, as the skill's target type says.
command("useskill", function(args)
  local a = words(args)
  local id = tonumber(a[1]) or SK[(a[1] or ""):upper()]
  local s = id and skill_db[id]
  if not s then return "usage: @useskill <id|NAME> [level]" end
  local lv = tonumber(a[2]) or math.max(1, skill_level(id))
  local target, tx, ty = player.id, player.x, player.y
  local best
  for _, u in ipairs(nearby(14)) do
    if u.kind == "mob" then
      local d = math.max(math.abs(u.x - player.x), math.abs(u.y - player.y))
      if not best or d < best then best, target, tx, ty = d, u.id, u.x, u.y end
    end
  end
  if (s.inf & 2) ~= 0 then
    _rt("skill_request", player.id, id, lv, 0, tx, ty)
  elseif (s.inf & 1) ~= 0 then
    if not best then return "no monster in sight" end
    _rt("skill_request", player.id, id, lv, target, 0, 0)
  else
    _rt("skill_request", player.id, id, lv, player.id, 0, 0)
  end
end)

command("where", function()
  return string.format("%s %d,%d", player.map, player.x, player.y)
end)

command("warp", function(args)
  local a = words(args)
  if not a[1] then return "usage: @warp <map> [x y]" end
  warp(a[1], tonumber(a[2]) or 0, tonumber(a[3]) or 0)
end)

-- Town list as the era's @go (Athena atcommand.c, 2003).
local towns = {
  prontera = { "prontera", 156, 191 }, morocc = { "morocc", 156, 93 }, geffen = { "geffen", 119, 59 },
  payon = { "payon", 162, 233 }, alberta = { "alberta", 192, 147 }, izlude = { "izlude", 128, 114 },
  aldebaran = { "aldebaran", 140, 131 }, xmas = { "xmas", 147, 134 }, comodo = { "comodo", 188, 161 },
  yuno = { "yuno", 157, 51 },
}
command("go", function(args)
  local t = towns[(args or ""):lower()]
  if not t then
    local names = {} for k in pairs(towns) do names[#names + 1] = k end
    table.sort(names)
    return "towns: " .. table.concat(names, ", ")
  end
  warp(t[1], t[2], t[3])
end)

command("item", function(args)
  local a = words(args)
  if not a[1] then return "usage: @item <id|name> [amount]" end
  local id = tonumber(a[1]) or a[1]
  if not give_item(id, tonumber(a[2]) or 1) then return "could not give " .. a[1] end
end)

command("zeny", function(args)
  player.zeny = player.zeny + (tonumber(args) or 0)
  return "zeny: " .. player.zeny
end)

command("lvup", function(args)
  local n = tonumber(args) or 1
  for _ = 1, n do
    if player.base_level >= 99 then break end
    player.base_level = player.base_level + 1
    player.status_point = player.status_point + math.floor((player.base_level + 14) / 5)
  end
  heal(99999, 99999)
end)

command("joblvup", function(args)
  local n = tonumber(args) or 1
  player.job_level = player.job_level + n
  player.skill_point = player.skill_point + n
end)

command("job", function(args)
  local j = tonumber(args)
  if not j then return "usage: @job <id> (0 Novice, 1 Swordsman, 2 Magician, 3 Archer, 4 Acolyte, 5 Merchant, 6 Thief, 7..)" end
  job_change(j)
end)

command("heal", function() heal(99999, 99999) end)

command("monster", function(args)
  local a = words(args)
  if not a[1] then return "usage: @monster <id|name> [count]" end
  local ids = spawn_mob(player.map, player.x, player.y, tonumber(a[1]) or a[1], tonumber(a[2]) or 1)
  return #ids .. " spawned"
end)

command("save", function()
  save_point(player.map, player.x, player.y)
  return "save point set"
end)

command("stats", function()
  return string.format("Lv %d/%d job %d  HP %d/%d SP %d/%d  zeny %d  str %d agi %d vit %d int %d dex %d luk %d",
    player.base_level, player.job_level, player.job, player.hp, player.max_hp, player.sp, player.max_sp,
    player.zeny, player.str, player.agi, player.vit, player.int, player.dex, player.luk)
end)
