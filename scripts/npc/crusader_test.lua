-- The Crusader test: Prontera Castle takes a Swordman of job level 40 in
-- four parts. Items from the undead, the prison's corridor crossed without a
-- kill, the cathedral knight's questions, and the purification: a hall of
-- the dead cleared in five minutes.
--
-- Behind `jobs2_2` (euRO opened the 2-2 jobs on 2004-06-09).
--
-- Sources:
--   * eAthena 1.0 npc/jobs/2-2/crusader.txt (2004-11-04, Black Dragon,
--     converted by Shin, v1.1, "based on the information at RagnaInfo",
--     "just a temp Job Quest"; data:refs/rathena commit 706e78509): the
--     flow and its quest variable, Man in Pain's cell in the castle prison
--     and the savepoints, the corridor on job_cru with its 38 monsters
--     and the rule that a kill fails it, the
--     knight's twenty yes-or-no questions and his 8 of 10, Harrison's cell
--     and his hall with its 43 monsters, the job level 50 shortcut past the
--     items and the Pike [4] it earns (a Pike otherwise). Tier athena2003
--     (a period emulator, eleven months after our date).
--   * Edge's Knight/Crusader FAQ, GameFAQs 29457, section added in v1.2
--     (2004-03-16, "I have also added the Crusader quest", from GlimmerVoid;
--     research/agy-threads/r30/pages/gamefaqs_knight_29457.html): the four
--     parts, the two item sets (Set 1: 10 Decayed Nail, Danggie, Rotten
--     Scale, Prisoner's Clothing; Set 2: 10 Skel Bone, Orc Nail, Manacles,
--     Danggie), the Rosary "Equip it and talk to the NPC named Man in
--     Pain", "pass through a room full of monsters without hitting the
--     monsters", questions "about Crusaders and Undead monsters" that may be
--     retaken, and "1 Holy Water ... kill a room full of monsters in 5
--     minutes". The nearest witness to our date; where it and eAthena part,
--     Edge is followed.
--   * Tangowr's Crusader guide, GameFAQs 28552 v1.1 (2004-07-01;
--     research/agy-threads/r31/pages/gamefaqs_crusader_tangowr_28552.html):
--     the names Michael Halig ("Senior Crusader", second floor of the
--     castle), Murnak Mijoul ("Man in Anguish") and Gabriel Valentine, "you
--     need to alway wear a rosary", the corridor's monsters ("Mandragoras
--     Floras Greatest Generals Farmiliars"), and nine undead and monster
--     questions "translated from twRO", with their answers.
--   * jAthena //1046 script/npc/job/npc_job_14crusader.txt (2004-09-12,
--     data:refs/jathena commit 6332410, a stub that changes you at once):
--     the Master Crusader (ミケル=レカント=ニキータ) and his guard
--     (ブリッテン=ヒルド) in the castle's west wing, their cells and
--     sprites, and their words, in our English. Edge: "From Prontera go
--     north twice and west twice".
--
-- Where we part from them (docs/DEVIATIONS.md):
--   * the items are Edge's two sets (eAthena shows the first set for both
--     and asks one Ball'n'Chain and ten Mementos);
--   * the Rosary must be worn to enter the corridor (eAthena: carried);
--   * Harrison asks a Holy Water for each try at the hall and gives it five
--     minutes (eAthena: neither; Edge both);
--   * the knight's ten questions: five of eAthena's twenty, five of
--     Tangowr's nine (Edge: "Crusaders and Undead monsters"); the choices
--     are worded as answers, not "Yes" and "No". Read against our data,
--     Tangowr's Holy-against-Fire question answers 25% (a level 2 Undead
--     takes 175% from Holy, 150% from Fire), not his 50%; his "Jirtas", a
--     monster this game lacks, gives way to the Whisper;
--   * the corridor's 38 are Tangowr's four on eAthena's cells (his
--     Khalitzburgs Mandragoras, Injustices Floras, Ghouls Greatest
--     Generals, Requiems and Raydric Archers Familiars). eAthena's undead,
--     up to level 63 and every one aggressive, killed a job level 40
--     Swordman at the door before a step (played, 2026-11-18); Tangowr's
--     stand still but the Familiars, and the corridor's east side is out of
--     their reach. (eAthena's ids, 0.5.2's G_ copies 1423-1468, are not in
--     this client either: its exe names one G_ monster, G_ASSULTER, and
--     drawing them closed the client.)
--   * eAthena's master stood in the east wing as "Lothar" (178,99); here
--     he is jAthena's, in the west wing, under Tangowr's name;
--   * eAthena's slips mended: an eleventh question, the "Raydric Archer"
--     spawned zero times, the hall's count read from a variable a death
--     wipes, the corridor and hall left full of monsters after a failure;
--     the quest variables cleared at the job change;
--   * no waiting for others: nobody else is ever in the test, so "there's
--     already another person there" goes.
--   * 817's Crusader Trainer in prt_in, who changed a Swordman at once,
--     gives way to Harrison.

if not content("jobs2_2") then return end

local SRC = "eathena-1.0:npc/jobs/2-2/crusader.txt:"
local JA = "jathena-1046:script/npc/job/npc_job_14crusader.txt:"
local MAP = "job_cru"
local ROSARY, HOLY_WATER, PIKE4, PIKE = 2608, 523, 1408, 1409

-- v.crus_q (eAthena's Job_CR): 1 eligible, 3 or 4 the item set given,
-- 5 to the prison, 6 sent into the corridor, 7 crossed it, 8 Man in Pain
-- has seen you out, 9 to the cathedral, 10 failed the questions once,
-- 11 passed them, 12 sent into the hall, 13 cleared it
-- v.crus_jl50 (JLCC): came at job level 50, past the items

local SETS = {
  [3] = { { 957, "Decayed Nail" }, { 901, "Danggie" }, { 959, "Stinky Scale" }, { 1099, "Worn-out Prison Uniform" } },
  [4] = { { 932, "Skel-Bone" }, { 1043, "Orc Claw" }, { 1098, "Manacles" }, { 901, "Danggie" } },
}

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

local function swordman() return player.class == Job_Swordman end
local function rosary_worn()
  for _, slot in ipairs { 7, 8 } do
    local e = equip(slot)
    if e and e.id == ROSARY then return true end
  end
  return false
end

local function list_set(who, set)
  local lines = {}
  for _, it in ipairs(SETS[set]) do lines[#lines + 1] = "^FF000010 " .. it[2] .. "^000000" end
  page(who, table.unpack(lines))
end

local function clear_quest()
  v.crus_q = 0
  v.crus_jl50 = 0
end

-- ------------------------------------------------------------ the castle

local G = "[Britten Hild]"
npc {
  map = "prt_castle", x = 35, y = 151, dir = 4, sprite = 751, name = "Guard",
  src = JA .. "4",
  on_click = function()
    if not swordman() then
      page(G, "...")
      return close()
    end
    page(G, "Welcome. We are the Crusaders,", "who make ready for the holy war", "to come.")
    next()
    page(G, "If the Crusaders interest you,", "you are welcome to join us at", "any time.")
    next()
    page(G, "To become a Crusader, a Swordman", "must reach job level 40. Ask the", "master inside for the rest.")
    next()
    page(G, "May Odin keep you.")
    close()
  end,
}

local M = "[Michael Halig]"

local function to_prison()
  next()
  page(M, "Now the second test. Go down to", "the prison, below the castle,", "and speak to the man there.")
  next()
  page(M, "Before you go, put on a", "^FF0000Rosary^000000. He will not", "send you without one.")
  close()
end

local function job_change_now()
  if player.skill_point > 0 then
    page(M, "Use up all your skill points", "first, then come back to me.")
    return close()
  end
  page(M, "You have passed every test.", "In Odin's name, rise a Crusader.")
  next()
  give_item(v.crus_jl50 == 1 and PIKE4 or PIKE, 1)
  job_change(Job_Crusader)
  clear_quest()
  page(M, "Take this spear as a sign of", "your new calling. Serve the", "righteous, not the powerful.")
  close()
end

local function begin()
  page(M, "Let me look at you first.")
  next()
  if player.job_level == 50 then
    -- eAthena: a Swordman of job level 50 goes straight to the prison
    v.crus_jl50 = 1
    v.crus_q = 5
    page(M, "Job level 50. You have hunted", "enough of the dead already; you", "may pass over the first test.")
    return to_prison()
  end
  if player.job_level < 40 then
    page(M, "I am sorry: you must reach", "job level 40 first. Every second", "calling asks it.")
    return close()
  end
  if player.skill_point > 0 then
    page(M, "Use up all your skill points", "before you begin.")
    return close()
  end
  v.crus_q = 1
  page(M, "Everything seems in order.", "Shall we begin the tests?")
  next()
  if menu("Yes, of course.", "Not yet.") ~= 1 then
    page(M, "Come back when you are ready.")
    return close()
  end
  v.crus_q = 3 + rand(0, 1)
  page(M, "The first test: bring me these,", "taken from the restless dead.")
  next()
  list_set(M, v.crus_q)
  close()
end

local function bring_items()
  local set = SETS[v.crus_q]
  local missing = {}
  for _, it in ipairs(set) do
    if count_item(it[1]) < 10 then missing[#missing + 1] = "^FF000010 " .. it[2] .. "^000000" end
  end
  if #missing > 0 then
    page(M, "You still lack:", table.unpack(missing))
    return close()
  end
  for _, it in ipairs(set) do take_item(it[1], 10) end
  v.crus_q = 5
  page(M, "Good work. You have brought", "everything.")
  to_prison()
end

npc {
  map = "prt_castle", x = 45, y = 169, dir = 4, sprite = 752, name = "Senior Crusader",
  src = JA .. "32",
  on_click = function()
    if player.class == Job_Crusader then
      page(M, "Keep to your training. Nobody", "knows the hour of the holy war.")
      return close()
    end
    if not swordman() then
      page(M, "...")
      return close()
    end
    local q = v.crus_q
    if q == 3 or q == 4 then return bring_items() end
    if q >= 5 and q <= 7 then
      page(M, "Speak to the man in the prison", "below the castle, with a", "^FF0000Rosary^000000 on.")
      return close()
    end
    if q == 8 then
      v.crus_q = 9
      page(M, "You held your hand through the", "corridor: patience, passed.")
      next()
      page(M, "Next, knowledge. Go to the", "cathedral; the knight there will", "question you.")
      return close()
    end
    if q >= 9 and q <= 10 then
      page(M, "The knight in the cathedral is", "waiting to question you.")
      return close()
    end
    if q >= 11 and q <= 12 then
      page(M, "One test left: the purification.", "Harrison, the Guardian Knight,", "will take you. Bring a", "^FF0000Holy Water^000000.")
      return close()
    end
    if q == 13 then return job_change_now() end
    page(M, "We are the Crusaders, who make", "ready for the holy war by Odin's", "will. What brings you here?")
    next()
    if menu("I want to join the holy war.", "Nothing.") ~= 1 then
      page(M, "Keep training, you who carry a", "sword. Nobody knows the hour of", "the holy war.")
      next()
      page(M, "You may not understand it now,", "but one day Odin's voice may", "reach you too.")
      return close()
    end
    if q == 1 then return begin() end
    page(M, "You would be a Crusader? It is no", "easy road to walk with us.")
    next()
    page(M, "We train without rest, held by", "deep trust and loyalty. We take", "the most dependable of the", "Swordmen.")
    next()
    page(M, "A Swordman who carries the", "^3333ffmark of loyalty^000000 and the", "^3333ffstigmata^000000 has the gift from", "heaven. With it, he may pass a", "trial and become one of us.")
    next()
    page(M, "I cannot yet tell what gifts", "you have. Will you take the", "trial?")
    next()
    if menu("Yes, I will try.", "Let me think about it.") ~= 1 then
      page(M, "Then prepare well. When Odin's", "voice calls, come at once.")
      return close()
    end
    begin()
  end,
}

-- ------------------------------------------------------------ the corridor

local PATIENCE = {
  { 10, 43, "Greatest General", 1277 }, { 14, 47, "Mandragora", 1020 }, { 26, 46, "Flora", 1118 },
  { 31, 46, "Familiar", 1005 }, { 16, 52, "Familiar", 1005 }, { 22, 53, "Mandragora", 1020 },
  { 28, 49, "Flora", 1118 }, { 12, 54, "Mandragora", 1020 }, { 17, 55, "Greatest General", 1277 },
  { 21, 57, "Flora", 1118 }, { 30, 58, "Mandragora", 1020 }, { 11, 64, "Familiar", 1005 },
  { 19, 67, "Flora", 1118 }, { 24, 62, "Mandragora", 1020 }, { 13, 70, "Greatest General", 1277 },
  { 18, 69, "Flora", 1118 }, { 29, 67, "Mandragora", 1020 }, { 32, 68, "Familiar", 1005 },
  { 16, 75, "Familiar", 1005 }, { 21, 74, "Mandragora", 1020 }, { 27, 76, "Flora", 1118 },
  { 14, 78, "Mandragora", 1020 }, { 19, 76, "Greatest General", 1277 }, { 24, 79, "Familiar", 1005 },
  { 14, 83, "Familiar", 1005 }, { 18, 85, "Mandragora", 1020 }, { 22, 84, "Flora", 1118 },
  { 33, 86, "Mandragora", 1020 }, { 13, 89, "Greatest General", 1277 }, { 17, 92, "Flora", 1118 },
  { 23, 90, "Mandragora", 1020 }, { 26, 88, "Greatest General", 1277 }, { 17, 97, "Mandragora", 1020 },
  { 23, 100, "Flora", 1118 }, { 31, 101, "Greatest General", 1277 }, { 13, 103, "Familiar", 1005 },
  { 22, 105, "Flora", 1118 }, { 30, 107, "Mandragora", 1020 },
}
local PATIENCE_EV = "crus_corridor::OnKilled"
local in_corridor = false

local function clear_corridor()
  in_corridor = false
  kill_monster(MAP, PATIENCE_EV)
end

local function enter_corridor()
  clear_corridor()
  v.crus_q = 6
  save_point("prt_castle", 162, 27)
  for _, m in ipairs(PATIENCE) do monster(MAP, m[1], m[2], m[3], m[4], 1, PATIENCE_EV) end
  in_corridor = true
  warp("job_cru", 23, 42)
  map_announce(MAP, "Endure. Reach the far end without striking down a single one.", 0)
end

npc {
  map = "-", name = "crus_corridor", sprite = -1, events = { "OnKilled" },
  src = SRC .. "999",
  on_click = function(label)
    if label ~= "OnKilled" or not in_corridor then return end
    clear_corridor()
    warp("prt_castle", 162, 27)
  end,
}

npc {
  map = MAP, x = 23, y = 109, dir = 0, sprite = 45, name = "corridor exit", id = "crus_corridor_exit",
  touch = { 1, 1 }, src = SRC .. "1069",
  on_click = function()
    if in_corridor and v.crus_q == 6 then v.crus_q = 7 end
    clear_corridor()
    warp("prt_castle", 162, 28)
  end,
}

local P = "[Murnak Mijoul]"
npc {
  map = "prt_castle", x = 163, y = 33, dir = 0, sprite = 107, name = "Man in Pain",
  src = SRC .. "357",
  on_click = function()
    local q = v.crus_q
    if q == 7 then
      v.crus_q = 8
      page(P, "So you came through. Good work.", "Go back to the Senior Crusader.")
      return close()
    end
    if q > 7 then
      page(P, "There is nothing more you need", "from me. Let me be...")
      return close()
    end
    page(P, "Who... who are you?")
    next()
    if not swordman() or q < 5 then
      page(P, "What do you want?", "Leave me alone!")
      return close()
    end
    if not rosary_worn() then
      page(P, "Leave me alone! Can't you leave", "a man in pain alone?")
      return close()
    end
    if q == 6 then
      page(P, "You must go again, I'm afraid.")
    else
      page(P, "I see, you are here for the test.")
      next()
      page(P, "It's torture. They send me into", "that room time and time again.", "I've grown awfully weak.")
      next()
      page(P, "But if it is your wish, I will", "send you there.")
    end
    next()
    if menu("Yes.", "On second thought...") ~= 1 then
      page(P, "Then do not disturb me!")
      return close()
    end
    page(P, "Then go. Cross to the far end,", "and do not kill.")
    close()
    enter_corridor()
  end,
}

-- ------------------------------------------------------------ the questions

-- eAthena's twenty, each with the choice it counts right
local VIRTUE = {
  { { "If someone asks your help in a", "good cause, will you oblige?" }, { "I will help.", "I will refuse." }, 1 },
  { { "Would you let a robber get away", "with stolen goods?" }, { "I'd let him go.", "I'd stop him." }, 2 },
  { { "Do you wish the world were in", "harmony?" }, { "I wish it were.", "I don't care." }, 1 },
  { { "Would you ever take your own", "life?" }, { "I might.", "Never." }, 2 },
  { { "Will you get drunk at parties?" }, { "Of course.", "I keep sober." }, 2 },
  { { "Would you slaughter openly?" }, { "If I must.", "I would not." }, 2 },
  { { "Will you hold your temper in", "an argument?" }, { "I hold it.", "I lose it." }, 1 },
  { { "Would you let Alchemists carry", "out forbidden experiments?" }, { "Let them.", "Forbid them." }, 2 },
  { { "If someone threw a stone at you,", "would you be furious?" }, { "Furious.", "I'd stay calm." }, 2 },
  { { "If you were offered money that", "belongs to the poor, would you", "take it?" }, { "I'd take it.", "I'd refuse it." }, 2 },
  { { "Do you think highly of yourself?" }, { "Very highly.", "Not too highly." }, 2 },
  { { "Are you humble before others?" }, { "I am humble.", "I am proud." }, 1 },
  { { "Do you answer people with", "respect and kindness?" }, { "With kindness.", "As I please." }, 1 },
  { { "Do you fight when you feel", "like it?" }, { "When I like.", "Only for cause." }, 2 },
  { { "Do you go to the pub every", "night?" }, { "Every night.", "Seldom." }, 2 },
  { { "Do you believe in Odin and the", "gods of Asgard?" }, { "I believe.", "I doubt." }, 1 },
  { { "Should the Devil, evil and", "cruel, be treated cruelly too?" }, { "Cruelly, yes.", "Not with cruelty." }, 2 },
  { { "Is talk for the weak, and action", "the only way to make peace?" }, { "Action only.", "Talk comes first." }, 2 },
  { { "If every other way to keep the", "peace fails, may you then", "resort to force?" }, { "Then I may.", "Never at all." }, 1 },
  { { "Will you risk your life to save", "someone else's?" }, { "I will risk it.", "My life first." }, 1 },
}

-- Tangowr's nine (twRO), read against our data (see the header)
local UNDEAD = {
  { { "When fighting the undead, which", "element is best?" }, { "Neutral", "Earth", "Undead", "Holy" }, 4 },
  { { "Against a level 2 Undead, how", "much more does Holy deal than", "Fire?" }, { "25%", "50%", "7%", "100%" }, 1 },
  { { "What can you NOT get from an", "Evil Druid?" }, { "Monk Hat", "Yggdrasil Leaf", "White Herb", "Amulet" }, 1 },
  { { "Which of these undead has the", "most HP?" }, { "Ghoul", "Skeleton Prisoner", "Wraith", "Zombie Prisoner" }, 4 },
  { { "Which of these undead differs", "from the others in size?" }, { "Wraith", "Khalitzburg", "Drake", "Evil Druid" }, 3 },
  { { "Which card makes you tougher", "against the undead?" }, { "Orc Skeleton Card", "Orc Zombie Card", "Ghoul Card", "Skeleton Worker Card" }, 2 },
  { { "What were Munak and Bongun to", "each other in life?" },
    { "Sister and brother", "Girl and boy next door", "No relation", "Strangers" }, 2 },
  { { "Which of these demons has the", "most HP?" }, { "Whisper", "Magnolia", "Dokebi", "Marionette" }, 4 },
  { { "Which of these demons differs", "from the others in size?" }, { "Ghostring", "Whisper", "Deviruchi", "Baphomet Jr." }, 1 },
}

local function pick(pool, n)
  local idx = {}
  for i = 1, #pool do idx[i] = i end
  for i = #idx, 2, -1 do
    local j = rand(1, i)
    idx[i], idx[j] = idx[j], idx[i]
  end
  local out = {}
  for i = 1, n do out[i] = pool[idx[i]] end
  return out
end

local K = "[Gabriel Valentine]"
local function questions()
  local asked = pick(VIRTUE, 5)
  for _, q in ipairs(pick(UNDEAD, 5)) do asked[#asked + 1] = q end
  local right = 0
  for i, q in ipairs(asked) do
    page(K, "Question " .. i .. ".", table.unpack(q[1]))
    next()
    if menu(table.unpack(q[2])) == q[3] then right = right + 1 end
  end
  page(K, "You answered " .. right .. " of 10", "rightly.")
  next()
  if right < 8 then
    v.crus_q = 10
    page(K, "Not enough, I'm afraid. Come", "back and try again when you", "feel ready; I will want at", "least 8 right.")
    return close()
  end
  v.crus_q = 11
  page(K, "Excellent! You pass the test of", "knowledge.")
  next()
  page(K, "One test is left, the", "purification. Harrison, the", "Guardian Knight, will take you.", "Bring a ^FF0000Holy Water^000000.")
  close()
end

npc {
  map = "prt_church", x = 94, y = 116, dir = 0, sprite = 56, name = "Cathedral Knight",
  src = SRC .. "434",
  on_click = function()
    local q = v.crus_q
    if not swordman() or q < 9 then
      page(K, "Good day to you. May you find", "your path.")
      return close()
    end
    if q > 10 then
      page(K, "Go to Harrison, the Guardian", "Knight, for the purification.")
      return close()
    end
    page(K, q == 10 and "Back to try again?" or "On your way to become a Crusader?", "Then answer me ten questions,", "of Crusaders and of the dead.")
    next()
    if menu("Take the test.", "Not now.") ~= 1 then return close() end
    questions()
  end,
}

-- ------------------------------------------------------------ the hall

local PURIFY = {
  { 163, 42, "Bongun", 1188 }, { 171, 66, "Bongun", 1188 }, { 163, 89, "Bongun", 1188 },
  { 171, 115, "Bongun", 1188 }, { 166, 141, "Bongun", 1188 }, { 168, 166, "Bongun", 1188 },
  { 164, 170, "Munak", 1026 }, { 173, 153, "Munak", 1026 }, { 164, 138, "Munak", 1026 },
  { 166, 128, "Munak", 1026 }, { 171, 117, "Munak", 1026 }, { 163, 102, "Munak", 1026 },
  { 172, 84, "Munak", 1026 }, { 162, 76, "Munak", 1026 }, { 167, 65, "Munak", 1026 },
  { 171, 58, "Munak", 1026 }, { 162, 48, "Munak", 1026 }, { 170, 33, "Munak", 1026 },
  { 170, 26, "Skeleton", 1076 }, { 166, 37, "Skeleton", 1076 }, { 171, 47, "Skeleton", 1076 },
  { 165, 61, "Skeleton", 1076 }, { 172, 65, "Skeleton", 1076 }, { 163, 77, "Skeleton", 1076 },
  { 164, 86, "Skeleton", 1076 }, { 165, 98, "Skeleton", 1076 }, { 170, 101, "Skeleton", 1076 },
  { 163, 110, "Skeleton", 1076 }, { 164, 117, "Skeleton", 1076 }, { 166, 129, "Skeleton", 1076 },
  { 165, 139, "Skeleton", 1076 }, { 166, 151, "Skeleton", 1076 }, { 168, 160, "Skeleton", 1076 },
  { 162, 169, "Skeleton", 1076 }, { 171, 171, "Skeleton", 1076 }, { 162, 32, "Zombie", 1015 },
  { 165, 51, "Zombie", 1015 }, { 172, 73, "Zombie", 1015 }, { 170, 92, "Zombie", 1015 },
  { 173, 111, "Zombie", 1015 }, { 170, 124, "Zombie", 1015 }, { 171, 147, "Zombie", 1015 },
  { 163, 162, "Zombie", 1015 },
}
local PURIFY_EV = "crus_hall::OnKilled"
local OUT = { "prt_in", 81, 100 }
local in_hall = false

local function clear_hall()
  in_hall = false
  npc_timer_stop("crus_hall")
  kill_monster(MAP, PURIFY_EV)
end

local function enter_hall()
  clear_hall()
  v.crus_q = 12
  save_point(OUT[1], OUT[2], OUT[3])
  for _, m in ipairs(PURIFY) do monster(MAP, m[1], m[2], m[3], m[4], 1, PURIFY_EV) end
  in_hall = true
  npc_timer_start("crus_hall")
  warp("job_cru", 167, 175)
  map_announce(MAP, "Purify this hall: defeat every one of the dead within 5 minutes.", 0)
end

local CLOCK = {
  [60000] = "4 minutes remaining.",
  [120000] = "3 minutes remaining.",
  [180000] = "2 minutes remaining.",
  [240000] = "1 minute remaining.",
  [270000] = "30 seconds remaining.",
}
local hall_events = { "OnKilled", "OnTimer300000" }
for ms in pairs(CLOCK) do hall_events[#hall_events + 1] = "OnTimer" .. ms end

npc {
  map = "-", name = "crus_hall", sprite = -1, events = hall_events,
  src = SRC .. "1087",
  on_click = function(label)
    if not in_hall then return end
    if label == "OnKilled" then
      local left = mob_count(MAP, PURIFY_EV)
      if left > 0 then return end
      clear_hall()
      if v.crus_q == 12 then v.crus_q = 13 end
      return map_announce(MAP, "The hall is purified. You may leave by the exit to the south.", 0)
    end
    if label == "OnTimer300000" then
      map_announce(MAP, "Time is up. Please try again.", 0)
      clear_hall()
      return area_warp(MAP, 150, 10, 185, 185, OUT[1], OUT[2], OUT[3])
    end
    local ms = tonumber(label:match("OnTimer(%d+)"))
    if ms and CLOCK[ms] then map_announce(MAP, CLOCK[ms], 0) end
  end,
}

npc {
  map = MAP, x = 167, y = 17, dir = 0, sprite = 45, name = "hall exit", id = "crus_hall_exit",
  touch = { 2, 2 }, src = SRC .. "1161",
  on_click = function()
    clear_hall()
    warp(OUT[1], OUT[2], OUT[3])
  end,
}

local H = "[Harrison]"
npc {
  map = "prt_in", x = 83, y = 97, dir = 0, sprite = 734, name = "Guardian Knight",
  replaces = "817:jobs/crusader.txt:4", src = SRC .. "836",
  on_click = function()
    local q = v.crus_q
    if not swordman() or q < 11 then
      page(H, "How goes your training? Are you", "strong enough to become a", "Crusader yet?")
      next()
      page(H, "If you are, you will find", "yourself coming back to me for", "one of the tests.")
      return close()
    end
    if q == 13 then
      page(H, "Well done! Go back to the Senior", "Crusader in the castle; he will", "make you a Crusader.")
      return close()
    end
    if q == 12 then
      page(H, "Do you want to try again?")
    else
      page(H, "I see you have passed so far.", "Don't let me down: this is the", "purification.")
      next()
      page(H, "Purify the hall of the dead:", "defeat every one of them within", "5 minutes. A ^FF0000Holy Water^000000 for", "the rite, and I take you there.")
    end
    next()
    if menu("I want to go now.", "Just a moment.") ~= 1 then
      page(H, "No hurry. Take as long as you", "need.")
      return close()
    end
    if count_item(HOLY_WATER) < 1 then
      page(H, "You have no ^FF0000Holy Water^000000.", "Bring one, and I will take you.")
      return close()
    end
    take_item(HOLY_WATER, 1)
    page(H, "Good. Off you go.")
    close()
    enter_hall()
  end,
}
