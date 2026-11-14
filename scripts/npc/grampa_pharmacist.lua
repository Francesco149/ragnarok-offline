-- Alberta's Grampa Pharmacist: 817's words (cities/alberta.txt:15) on mod0631's
-- working script (conf/npc_event_potion.txt, 2003-12-14), which has the same six
-- recipes and fees.
--
-- 817's port could not be trusted with a player's herbs. Asked for "as many as I
-- can", the White, Blue and Green Potions took one set of herbs, a bottle and the
-- fee and gave nothing; the Green Potion took Green Potions instead of Green Herbs;
-- its bottle test (`count_item(713) < 0`) never failed, so "as many" went on making
-- potions with no bottles left; and three of its jumps named labels the file lacks
-- (`NeeOther`, `NeetOther`, `inpCon`), ending the talk without a word. mod0631
-- works out how many the herbs, the bottles and the zeny allow, checks the lot,
-- then takes it all at once: so do we. The lines for too little money and too much
-- weight are mod0631's, translated (817 has none); the rest is 817's.
--
-- research/THREADS.md #27, docs/DEVIATIONS.md.

local NAME = "[Grampa Pharmacist]"

-- potion, herbs { id, count } per potion, fee (817's menu order; mod0631 the same)
local RECIPES = {
  { 501, { { 507, 2 } }, 3 },             -- Red Potion: 2 Red Herbs
  { 502, { { 507, 1 }, { 508, 1 } }, 5 }, -- Orange Potion: a Red and a Yellow Herb
  { 503, { { 508, 2 } }, 10 },            -- Yellow Potion: 2 Yellow Herbs
  { 504, { { 509, 2 } }, 20 },            -- White Potion: 2 White Herbs
  { 505, { { 510, 2 } }, 30 },            -- Blue Potion: 2 Blue Herbs
  { 506, { { 511, 2 } }, 3 },             -- Green Potion: 2 Green Herbs
}
local BOTTLE = 713

local function need_other()
  mes "You rascal! What did you expect from me after you even didn't bring all items!"
  mes "Get lost!"
  close()
end

local function make_menu()
  mes(NAME)
  mes "What did you say?! You little punk... Spit spit! Get the Heck out of here!"
  close()
end

-- how many the herbs and the bottles allow (mod0631 L_SUB_1_1)
local function can_make(r)
  local n = count_item(BOTTLE)
  for _, h in ipairs(r[2]) do
    n = math.min(n, count_item(h[1]) // h[2])
  end
  return n
end

local function make(r, n)
  for _, h in ipairs(r[2]) do
    if count_item(h[1]) < h[2] * n then return need_other() end
  end
  if count_item(BOTTLE) < n then return need_other() end
  if check_weight(r[1], n) == 0 then
    mes "You're carrying too much as it is. Off with you!"
    return close()
  end
  if player.zeny < r[3] * n then
    mes "You rascal! You haven't even got the money, and you want me to make you this?"
    mes "Get lost!"
    return close()
  end
  for _, h in ipairs(r[2]) do take_item(h[1], h[2] * n) end
  take_item(BOTTLE, n)
  player.zeny = player.zeny - r[3] * n
  give_item(r[1], n)
  mes "Here you are. Take it."
  mes "But keep in mind! Overdose could kill you."
  close()
end

npc {
  map = "alberta_in", x = 16, y = 28, dir = 4, sprite = 61, name = "Grampa Pharmacist",
  replaces = "817:cities/alberta.txt:15",
  on_click = function()
    mes(NAME)
    mes "Err... Who are you...?"
    next()
    local c = menu("Make Potion", "Talk", "Information", "Cancel")
    if c == 1 then
      mes(NAME)
      if count_item(BOTTLE) < 1 then
        mes "Don't you see I need at least 1 Empty Bottle to fill medicine within it! Why did you come without it! You idiot!!"
        return close()
      end
      mes "Do you prepare all items needed?"
      mes "Which potion do you want?"
      next()
      local r = RECIPES[menu("Red Potion", "Orange Potion", "Yellow Potion", "White Potion", "Blue Potion", "Green Potion", "Nah, I change my mind.")]
      if not r then return make_menu() end
      mes(NAME)
      local most = can_make(r)
      if most == 0 or player.zeny < r[3] then return need_other() end
      mes "How many?"
      next()
      c = menu("As many as I can?", "Um I will set the Numbers.", "Nah, forget about it")
      if c == 1 then
        mes(NAME)
        return make(r, math.min(most, player.zeny // r[3]))
      elseif c == 2 then
        local n = input_number()
        mes(NAME)
        if n <= 0 then
          mes "Deal canceled."
          return close()
        end
        return make(r, n)
      end
      return make_menu()
    elseif c == 2 then
      mes(NAME)
      mes "You can cure disease with medicine."
      mes "But you have to be careful in medicine otherwise you would be addicted..."
      mes "Pssssst... I am too old to talk so long."
      mes "Anyway Potion is a Medicine which can assist you when you are out of physical energy. That's all."
      return close()
    elseif c == 3 then
      mes(NAME)
      mes "Gasp... What an annoying young one... Fine, I will explain how the potions work for you."
      next()
      mes(NAME)
      mes "You can consume nomal Hearbs as themeself, but if you consumethem after proccesing into a potion, you will be able to recover more energy in the same time."
      next()
      mes(NAME)
      mes "I am the one and only who can make potion out of herbs. But there is the few fee for my effort....!"
      mes "need some money for living"
      next()
      mes(NAME)
      mes "Red Potion - ^00FFFF2 Red Herb, 1 Empty Bottle, Fee 3 Zeny.^000000"
      mes "Orange Potion - ^00FFFF1 Red Herb, 1 Yellow Herb, 1 Empty Bottle, Fee 5 Zeny.^000000"
      mes "Yellow Potion - ^00FFFF2 Yellow Herb, 1 Empty Bottle, Fee 10 Zeny.^000000"
      next()
      mes(NAME)
      mes "White Potion - ^00FFFF2 White Herb, 1 Empty Bottle, Fee 20 Zeny.^000000"
      mes "Blue Potion - ^00FFFF2 Blue Herb, 1 Empty Bottle, Fee 30 Zeny.^000000"
      mes "Green Potion - ^00FFFF2 Grren Herb, 1 Empty Bottle, Fee 3 Zeny.^000000"
      return close()
    end
    close()
  end,
}
