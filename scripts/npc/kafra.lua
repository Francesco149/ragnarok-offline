-- Kafra Corporation: one service for every town and dungeon Kafra.
--
-- Replaces the eathena-817 Kafras (other/kafra.txt) it lists by `src`. Places,
-- sprites, cut-ins, save points, destinations and the wording are 817's
-- (tier athena2003). What changed, and why (docs/DEVIATIONS.md, "Kafra"):
--   * fares are the ones 817's menus showed; its code charged different ones
--     in places (Prontera->Geffen showed 1400, took 1800). mod0631's Kafra
--     script (conf/npc_town_kafra.txt, archived 2003-12-18) shows and takes
--     the same fares as 817's menus, with Alberta->Prontera at 2100;
--   * what 817 lacked comes from mod0631, file:line in M631 below: the storage
--     fee per place (817: 30z everywhere), Comodo as a destination, Al De
--     Baran's teleport service, the Kafras of Comodo, the Pharos lighthouse,
--     Mt. Mjolnir, Byalan Island and Prontera's east and west gates, and
--     Izlude's save cell;
--   * a cart is charged only when one is given: 817 took 800z, then jumped to
--     a label that didn't exist (merchants paid for nothing) - and took it twice;
--   * storage needs Basic Skill 6, as the client's own skill description says
--     (817 asked for Novice job level 6);
--   * no "Use Kafra Pass" entry: kRO removed the Kafra voucher menu on
--     2003-05-20, and 817's was "Under development";
--   * "Cancel" and "Other Information Check" work (817 pointed them at
--     missing labels); the location tip marks this town's Kafras;
--   * the special reserve (Kafra points): a tenth of every teleport fare, as
--     in mod0631 (`set KF_PIT,KF_PIT+90` after the 900z to Izlude, line 187,
--     and so at every Kafra; storage, carts and saving earn nothing). 817's
--     "Special Reserve Check" said "Under development", and its exchange
--     counter in the Al De Baran head office had a prize list and no code
--     behind it; both work here, the counter at the end of this file. kRO's
--     notice of 2004-02-10 has players carrying saved-up points to that
--     head office (research/notes/round2-review.md, THREADS.md #4).
-- Pre-2004-02-24 fares are period-emulator values (research/THREADS.md #3,
-- research/notes/round1-review.md). kRO's own notice of 2004-02-24 lists only
-- the fares that replaced them.

local STORAGE_FEE = 30 -- 817 kafra.txt, where mod0631 has no Kafra at that place
local CART_FEE = 800 -- 817 kafra.txt and mod0631 (every Kafra that rents one)

local function bye(cut)
  cutin(cut, 255)
  close()
end

local function kafra(k)
  return function()
    local cut = k.cutin or "kafra_03"
    cutin(cut, 2)
    mes "[Kafra]"
    mes "Welcome to Kafra Corp. We will stay with you wherever you go."
    next()
    local opts, acts = {}, {}
    local function add(text, act) opts[#opts + 1] = text; acts[#acts + 1] = act end
    if k.save then add("Save", "save") end
    add("Use Storage", "storage")
    if k.tele then add("Use Teleport Service", "tele") end
    if not k.no_cart then add("Rent a Cart", "cart") end
    add("Other Information Check", "info")
    add("Cancel", "cancel")
    local act = acts[menu(opts)]

    if act == "save" then
      local p = k.save
      if k.save_out then -- gate Kafras (817): a field cell outside, or the town
        p = menu("Save Outside City.", "Save inside City") == 1 and k.save_out or k.save
      end
      save_point(p[1], p[2], p[3])
      mes "[Kafra]"
      mes "Your respawn point has been saved. Thank you."
      return bye(cut)

    elseif act == "storage" then
      if skill_level(1) < 6 then
        mes "[Kafra]"
        mes "I am sorry, but you need Basic Skill level 6 to use the storage."
        return bye(cut)
      end
      local fee = k.storage or STORAGE_FEE
      if player.zeny < fee then
        mes "[Kafra]"
        mes("Dear, you don't have enough money. The storage fee is " .. fee .. " Zeny.")
        return bye(cut)
      end
      player.zeny = player.zeny - fee
      open_storage()
      return bye(cut)

    elseif act == "tele" then
      mes "[Kafra]"
      mes "Please let me know your destination."
      next()
      local list = {}
      for i, t in ipairs(k.tele) do list[i] = t[1] .. " -> " .. t[2] .. " z" end
      list[#list + 1] = "Cancel"
      local i = menu(list)
      local t = k.tele[i]
      if not t then return bye(cut) end
      if player.zeny < t[2] then
        mes "[Kafra]"
        mes "Dear, you don't have enough money. Please check again."
        return bye(cut)
      end
      player.zeny = player.zeny - t[2]
      v.KF_PIT = v.KF_PIT + t[2] // 10 -- mod0631's variable and rule
      cutin(cut, 255)
      warp(t[3], t[4], t[5])
      return

    elseif act == "cart" then
      local c = player.class
      if c ~= Job_Merchant and c ~= Job_Blacksmith and c ~= Job_Alchem then
        mes "[Kafra]"
        mes "I am sorry. The cart service is only provided for Merchants and Blacksmiths."
        return bye(cut)
      end
      mes "[Kafra]"
      mes("The cart fee is " .. CART_FEE .. " Zeny. Do you want to rent a cart?")
      next()
      if menu("Rent a Cart.", "Cancel.") ~= 1 then return bye(cut) end
      if player.zeny < CART_FEE then
        mes "[Kafra]"
        mes("Dear, you don't have enough money. You need " .. CART_FEE .. " Zeny.")
        return bye(cut)
      end
      if check_option(0x0008) == 1 then
        mes "[Kafra]"
        mes "You already have a cart."
        return bye(cut)
      end
      if not set_cart() then
        mes "[Kafra]"
        mes "I am sorry. You need the Pushcart skill to use a cart."
        return bye(cut)
      end
      player.zeny = player.zeny - CART_FEE
      mes "[Kafra]"
      mes "There you go."
      return bye(cut)

    elseif act == "info" then
      -- 817's submenu ("Special Reverse Check" there). mod0631 has the same two
      -- as "積立ポイント確認" and "カプラ職員配置案内" in the main menu
      local c = menu("Special Reserve Check", "Location Tip", "Cancel")
      if c == 1 then
        mes "[Kafra]"
        mes(player.name .. "'s special reserve is " .. v.KF_PIT .. ".")
        next()
        mes "[Kafra]"
        mes "You can exchange special reserve for goods at the head office of Kafra."
        mes "Please use our service for convenience and receive free goods with the special reserve."
        return bye(cut)
      elseif c ~= 2 then
        return bye(cut)
      end
      -- 817: "Location Tip" marks the town's Kafras on the mini-map
      local n = 0
      for _, o in ipairs(KAFRA_LIST) do
        if o.map == k.map then
          n = n + 1
          viewpoint(1, o.x, o.y, n, 0xFF00FF)
        end
      end
      mes "[Kafra]"
      mes "I have marked the Kafra employees of this area on your mini-map."
      return bye(cut)
    end
    bye(cut)
  end
end

-- src, place, sprite, cut-in, save point, storage fee, destinations {name, fare, map, x, y}.
-- `src` is the 817 NPC this one replaces (place, sprite, cut-in and save cells are 817's).
-- `m631` is the line of the same Kafra in mod0631's conf/npc_town_kafra.txt, the
-- source of `storage` and of the destinations 817 did not have. Entries with
-- `m631` and no `src` exist only in mod0631: place, sprite, cut-in and save
-- cells are its own.
KAFRA_LIST = {
  -- Prontera
  { src = "817:other/kafra.txt:2474", m631 = 129, map = "prontera", x = 146, y = 89, dir = 6, sprite = 115, cutin = "kafra_03",
    save = { "prontera", 117, 73 }, storage = 40,
    tele = { { "Izlude", 900, "izlude", 91, 105 }, { "Geffen", 1400, "geffen", 120, 39 }, { "Payon", 1700, "payon", 69, 100 },
             { "Morroc", 1800, "morocc", 156, 46 }, { "Comodo", 2000, "comodo", 209, 143 },
             { "Orc Dungeon", 1700, "gef_fild10", 52, 326 } } },
  { src = "817:other/kafra.txt:2622", m631 = 630, map = "prontera", x = 151, y = 29, dir = 8, sprite = 115, cutin = "kafra_03",
    save_out = { "prt_fild08", 170, 369 }, save = { "prontera", 150, 33 }, storage = 40,
    tele = { { "Izlude", 900, "izlude", 91, 105 }, { "Geffen", 1400, "geffen", 120, 39 }, { "Payon", 1700, "payon", 69, 100 },
             { "Morroc", 1800, "morocc", 156, 46 }, { "Comodo", 2000, "comodo", 209, 143 },
             { "Orc Dungeon", 1700, "gef_fild10", 52, 326 } } },
  -- east and west gates: mod0631 only. 817 had none in the east, and in the west only
  -- the 'FGM Kafra' of a private server (not imported), on mod0631's cell 29,207
  { m631 = 377, map = "prontera", x = 282, y = 200, dir = 2, sprite = 114, cutin = "kafra_04",
    save_out = { "prt_fild06", 33, 192 }, save = { "prontera", 279, 200 }, storage = 40,
    tele = { { "Izlude", 900, "izlude", 91, 105 }, { "Geffen", 1400, "geffen", 120, 39 }, { "Payon", 1700, "payon", 69, 100 },
             { "Morroc", 1800, "morocc", 156, 46 }, { "Comodo", 2000, "comodo", 209, 143 },
             { "Orc Dungeon", 1700, "gef_fild10", 52, 326 } } },
  { m631 = 883, map = "prontera", x = 29, y = 207, dir = 6, sprite = 113, cutin = "kafra_05",
    save_out = { "prt_fild05", 368, 205 }, save = { "prontera", 33, 207 }, storage = 40,
    tele = { { "Izlude", 900, "izlude", 91, 105 }, { "Geffen", 1400, "geffen", 120, 39 }, { "Payon", 1700, "payon", 69, 100 },
             { "Morroc", 1800, "morocc", 156, 46 }, { "Comodo", 2000, "comodo", 209, 143 },
             { "Orc Dungeon", 1700, "gef_fild10", 52, 326 } } },
  -- Izlude: 817's "Save inside City" said so but saved nothing. mod0631:1532 and
  -- jAthena (2004-01-31) save at 94,103; teleports land on 91,105 in all of them
  { src = "817:other/kafra.txt:1667", m631 = 1512, map = "izlude", x = 124, y = 115, dir = 6, sprite = 117, cutin = "kafra_01",
    save_out = { "prt_fild08", 349, 202 }, save = { "izlude", 94, 103 }, storage = 30,
    tele = { { "Prontera", 900, "prontera", 116, 72 }, { "Payon", 1700, "payon", 69, 100 }, { "Geffen", 1800, "geffen", 120, 39 } } },
  -- Geffen
  { src = "817:other/kafra.txt:1381", m631 = 1976, map = "geffen", x = 120, y = 62, dir = 8, sprite = 115, cutin = "kafra_03",
    save = { "geffen", 119, 40 }, storage = 30,
    tele = { { "Prontera", 1400, "prontera", 116, 72 }, { "Izlude", 1800, "izlude", 91, 105 },
             { "Al De Baran", 2100, "aldebaran", 143, 110 }, { "Morroc", 2200, "morocc", 156, 46 } } },
  { src = "817:other/kafra.txt:1519", m631 = 2204, map = "geffen", x = 203, y = 123, dir = 3, sprite = 114, cutin = "kafra_04",
    save_out = { "gef_fild00", 51, 194 }, save = { "geffen", 200, 124 }, storage = 30,
    tele = { { "Prontera", 1400, "prontera", 116, 72 }, { "Izlude", 1800, "izlude", 91, 105 },
             { "Al De Baran", 2100, "aldebaran", 143, 110 }, { "Morroc", 2200, "morocc", 156, 46 } } },
  -- Payon
  { src = "817:other/kafra.txt:2086", m631 = 3671, map = "payon", x = 99, y = 116, dir = 4, sprite = 113, cutin = "kafra_05",
    save_out = { "pay_fild01", 340, 347 }, save = { "payon", 69, 100 }, storage = 40,
    tele = { { "Alberta", 1300, "alberta", 117, 56 }, { "Prontera", 1800, "prontera", 116, 72 }, { "Morroc", 1800, "morocc", 156, 46 } } },
  -- Morroc
  { src = "817:other/kafra.txt:1808", m631 = 2437, map = "morocc", x = 156, y = 97, dir = 4, sprite = 115, cutin = "kafra_03",
    save = { "morocc", 156, 46 }, storage = 40,
    tele = { { "Comodo", 1500, "comodo", 209, 143 }, { "Prontera", 1800, "prontera", 116, 72 }, { "Payon", 1800, "payon", 69, 100 },
             { "Alberta", 2200, "alberta", 117, 56 }, { "Geffen", 2200, "geffen", 120, 39 } } },
  { src = "817:other/kafra.txt:1944", m631 = 2673, map = "morocc", x = 163, y = 260, dir = 4, sprite = 114, cutin = "kafra_04",
    save_out = { "moc_fild07", 212, 30 }, save = { "morocc", 187, 281 }, storage = 40,
    tele = { { "Comodo", 1500, "comodo", 209, 143 }, { "Prontera", 1800, "prontera", 116, 72 }, { "Payon", 1800, "payon", 69, 100 },
             { "Alberta", 2200, "alberta", 117, 56 }, { "Geffen", 2200, "geffen", 120, 39 } } },
  -- Alberta (the two 817 Alberta Kafras showed 1800 and 2100 to Prontera and both took 2100;
  -- mod0631 shows and takes 2100 at both)
  { src = "817:other/kafra.txt:584", m631 = 3070, map = "alberta", x = 28, y = 229, dir = 8, sprite = 115, cutin = "kafra_02",
    save_out = { "pay_fild03", 386, 76 }, save = { "alberta", 31, 231 }, storage = 50,
    tele = { { "Payon", 1300, "payon", 69, 100 }, { "Prontera", 2100, "prontera", 116, 72 }, { "Morroc", 2200, "morocc", 156, 46 } } },
  { src = "817:other/kafra.txt:721", m631 = 3295, map = "alberta", x = 113, y = 60, dir = 5, sprite = 112, cutin = "kafra_06",
    save = { "alberta", 117, 57 }, storage = 50,
    tele = { { "Payon", 1300, "payon", 69, 100 }, { "Prontera", 2100, "prontera", 116, 72 }, { "Morroc", 2200, "morocc", 156, 46 } } },
  -- Al De Baran (817: no teleport service; mod0631:4028 has these two)
  { src = "817:other/kafra.txt:1309", m631 = 4028, map = "aldebaran", x = 143, y = 119, dir = 4, sprite = 113, cutin = "kafra_05",
    save = { "aldebaran", 143, 109 }, storage = 40,
    tele = { { "Geffen", 1600, "geffen", 120, 39 }, { "Mjolnir Dead Pit", 1700, "mjolnir_02", 99, 351 } } },
  { src = "817:other/kafra.txt:1197", map = "aldeba_in", x = 96, y = 181, dir = 4, sprite = 113, cutin = "kafra_05",
    save = { "aldeba_in", 96, 179 } },
  -- Comodo, the beach and the Pharos lighthouse: mod0631 only (sprite 721 is the
  -- client's Comodo Kafra; the two in Comodo rent no cart)
  { m631 = 4598, map = "comodo", x = 195, y = 150, dir = 4, sprite = 721, cutin = "kafra_07",
    save = { "comodo", 180, 151 }, storage = 80, no_cart = true,
    tele = { { "Pharos Lighthouse", 800, "cmd_fild07", 127, 134 }, { "Morroc", 1500, "morocc", 156, 46 } } },
  { m631 = 4216, map = "cmd_in02", x = 146, y = 180, dir = 4, sprite = 721, cutin = "kafra_07",
    save = { "comodo", 209, 143 }, storage = 40, no_cart = true,
    tele = { { "Pharos Lighthouse", 800, "cmd_fild07", 127, 134 }, { "Morroc", 2000, "morocc", 156, 46 } } },
  { m631 = 4407, map = "cmd_fild07", x = 136, y = 134, dir = 4, sprite = 721, cutin = "kafra_07",
    save = { "cmd_fild07", 127, 134 }, storage = 80,
    tele = { { "Comodo", 500, "comodo", 209, 143 }, { "Morroc", 2500, "morocc", 156, 46 } } },
  -- dungeons and fields. kRO's notice of 2004-02-24 ended the save service at the
  -- Mjolnir mine, the Orc Dungeon, the Pyramid, the Sunken Ship, Byalan and the
  -- Archer Village: until then these Kafras saved, as they do here
  { src = "817:other/kafra.txt:2", m631 = 4789, map = "gef_fild10", x = 73, y = 340, dir = 4, sprite = 115, cutin = "kafra_02",
    save = { "gef_fild10", 54, 232 }, storage = 55 },
  { src = "817:other/kafra.txt:284", m631 = 2914, map = "moc_ruins", x = 61, y = 156, dir = 5, sprite = 114, cutin = "kafra_04",
    save = { "moc_ruins", 41, 141 }, storage = 55 },
  { src = "817:other/kafra.txt:375", m631 = 1136, map = "prt_fild01", x = 198, y = 47, dir = 8, sprite = 112, cutin = "kafra_06",
    save = { "prt_fild01", 197, 50 }, storage = 40 },
  { src = "817:other/kafra.txt:481", m631 = 1356, map = "prt_fild05", x = 290, y = 224, dir = 1, sprite = 114, cutin = "kafra_04",
    save = { "prt_fild05", 274, 243 }, storage = 30 },
  { src = "817:other/kafra.txt:853", m631 = 3515, map = "alb2trea", x = 59, y = 69, dir = 1, sprite = 117, cutin = "kafra_03",
    save = { "alb2trea", 92, 64 }, storage = 55 },
  { src = "817:other/kafra.txt:2225", m631 = 3872, map = "pay_arche", x = 55, y = 123, dir = 8, sprite = 117, cutin = "kafra_01",
    save = { "pay_arche", 49, 144 }, storage = 45 },
  { m631 = 4946, map = "mjolnir_02", x = 84, y = 362, dir = 4, sprite = 117, cutin = "kafra_01",
    save = { "mjolnir_02", 98, 351 }, storage = 55 },
}

for _, k in ipairs(KAFRA_LIST) do
  npc {
    map = k.map, x = k.x, y = k.y, dir = k.dir, sprite = k.sprite, name = "Kafra",
    replaces = k.src, src = not k.src and ("m631:conf/npc_town_kafra.txt:" .. k.m631) or nil, on_click = kafra(k),
  }
end

-- The special reserve counter in the Kafra head office (817 other/kafra.txt:995,
-- not imported: its menu labels start with digits, which 817 read as numbers,
-- and every prize branch was a bare `close`). The place, the sprite, the
-- wording and the prize list are 817's. Ours: the points are taken and the
-- goods handed over. The two lottery entries (1000p and 3000p) are left out:
-- 817 names no prize ("Price is + itemName + 1 ea").
local RESERVE_PRIZES = {
  { 516, { { 100, 7 }, { 200, 15 }, { 300, 25 }, { 400, 35 }, { 500, 50 }, { 600, 60 }, { 700, 75 }, { 800, 85 }, { 900, 100 } } },
  { 501, { { 1100, 7 }, { 1300, 15 }, { 1500, 25 }, { 1700, 35 }, { 1900, 50 }, { 2100, 60 }, { 2300, 75 }, { 2500, 85 }, { 2800, 100 } } },
}

npc {
  map = "aldeba_in", x = 79, y = 161, dir = 6, sprite = 115, name = "Kafra",
  src = "817:other/kafra.txt:995",
  on_click = function()
    mes "[Kafra]"
    mes("Welcome, my dearest " .. player.name .. ".")
    mes "Take as many goods as you've got special reserve from Kafra Service~"
    next()
    mes "[Kafra]"
    mes "Please keep in mind that each window requires a different special reserve for your information."
    mes "The window I am at allows you to use special reserve from ^FF00FF100p to 3000p^000000."
    next()
    mes "[Kafra]"
    mes("Your special reserve is ^FF0000" .. v.KF_PIT .. "^000000~")
    mes "Please choose the items you want."
    next()
    local page = 1
    while true do
      local item, tiers = RESERVE_PRIZES[page][1], RESERVE_PRIZES[page][2]
      local list = {}
      for i, t in ipairs(tiers) do
        list[i] = t[1] .. "p = " .. item_name(item):gsub("_", " ") .. " " .. t[2] .. " ea"
      end
      if page == 1 then list[#list + 1] = "Next Articles" end
      list[#list + 1] = "Cancel"
      local i = menu(list)
      local t = tiers[i]
      if t then
        mes "[Kafra]"
        if v.KF_PIT < t[1] then
          mes "Dear~ I hope you will save more special reserve for next time~"
        else
          v.KF_PIT = v.KF_PIT - t[1]
          give_item(item, t[2])
          mes "Here you are, dear~"
          mes "Please come again~~"
        end
        return close()
      elseif page == 1 and i == #tiers + 1 then
        page = 2
      else
        return close()
      end
    end
  end,
}
