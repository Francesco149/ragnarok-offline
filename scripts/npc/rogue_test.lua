-- The Rogue test: the guild under the Pharos lighthouse field (in_rogue).
-- Markie's interview, Mr. Smith's errand and fee, a guildsman found in
-- Comodo's fields who opens his door to the right password, and the run
-- through the guild's underground passage among the undead, back to Markie.
--
-- Behind `jobs2_2` (euRO opened the 2-2 jobs on 2004-06-09).
--
-- Sources:
--   * eAthena 1.0 npc/jobs/2-2/rogue.txt (2004-11-04, kobra_k88, v1.0,
--     "Rogue job quest based off of official iRO Rogue quest. There are some
--     differences from official quest due to scripting issues"; the
--     Assassin test's author; data:refs/rathena commit 706e78509): the NPCs,
--     their cells and sprites, the words, both sets of questions with their
--     answers and the 90 to pass, the three errands and the 10,000 Zeny fee,
--     the three guildsmen, their hiding places, passwords and houses, the
--     run's start, its thirteen ambushes and its end, the Gladius [2] (or
--     [3] at job level 50). Tier athena2003 (eleven months after our date).
--   * eAthena 1.0 npc/warps/other/jobquests.txt:123-128: the ways in from
--     cmd_fild07 and between the guild's rooms (jAthena //1046's
--     npc_warp_job.txt:352-355 has the same doors).
--   * jAthena //1046's Rogue test is a stub (npc_job_17rogue.txt: a guild
--     member who changes you at once, and スミスラト counting his trading
--     partners on Mr. Smith's cell).
--   * No player's account of the test before September 2004 is on disk.
--
-- Where we part from eAthena (docs/DEVIATIONS.md):
--   * the first set's monster that sees through Hiding is the Argos (an
--     insect here); eAthena's Worm Tail is a plant in our data, and our
--     monsters see through Hiding as mod0631's: insects, demons, bosses;
--   * an ambush springs once a run, not each time its line is crossed;
--     eAthena cleared the overspawned map every ten minutes instead;
--   * the run's end passes only a candidate on the run (eAthena: anyone
--     stepping on it), and nothing is left behind after a run;
--   * 817's Rogue's Lady near Morroc, who changed a Thief at once, sends
--     Thieves to the guild instead;
--   * words cut to the dialogue window, typos mended.

if not content("jobs2_2") then return end

local SRC = "eathena-1.0:npc/jobs/2-2/rogue.txt:"
local MAP = "in_rogue"
local GLADIUS2, GLADIUS3 = 1219, 1220

-- eAthena's variables:
--   v.rogue_q (ROGUE_Q): 1 interviewed (or failed), 2 passed, 3 errand
--     done, 4 on the run, 5 through
--   v.rogue_q2 (ROGUE_Q2): which errand, then which guildsman (1-3)
--   v.rogue_jl (JBLVL): the job level on the form, 40 or 50 (which Gladius)

warp { map = "cmd_fild07", x = 193, y = 117, span = { 1, 1 }, to = { MAP, 378, 46 } }
warp { map = MAP, x = 375, y = 46, span = { 1, 1 }, to = { "cmd_fild07", 195, 116 } }
warp { map = MAP, x = 376, y = 34, span = { 1, 1 }, to = { MAP, 378, 125 } }
warp { map = MAP, x = 375, y = 125, span = { 1, 1 }, to = { MAP, 379, 34 } }
warp { map = "cmd_fild07", x = 352, y = 275, span = { 1, 1 }, to = { MAP, 265, 122 } }
warp { map = MAP, x = 265, y = 118, span = { 1, 1 }, to = { "cmd_fild07", 349, 275 } }

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

local function thief() return player.class == Job_Thief end

-- 817's Rogue's Lady: Thieves go to the guild now
wrap_npc("817:jobs/rogue.txt:6", function(orig)
  return function(at)
    if thief() then
      page("[Rogue's Lady]", "A Rogue? We don't hand that out", "in the desert, sweetie.")
      next()
      page("[Rogue's Lady]", "The guild is in Comodo's north", "field, the one with the Pharos", "lighthouse. Ask for Markie.")
      return close()
    end
    return orig(at)
  end
end)

-- ------------------------------------------------------------ Markie

-- each question: its lines, its choices, the choices that score
local SET1 = {
  { { "1) What flee rate does a Thief", "gain with ^5533FFImprove Dodge^000000", "mastered?" }, { "30", "40", "160", "20" }, { 1 } },
  { { "2) Choose a monster that can", "see a hidden or cloaked", "character." }, { "Worm Tail", "Argos", "Mummy", "Soldier Skeleton" }, { 2 } },
  { { "3) Where is the Rogue guild?" }, { "Comodo", "Kokomo Beach", "Pharos Lighthouse", "Morroc" }, { 3 } },
  { { "4) In which town can you", "become a Thief?" }, { "Comodo", "Lutie", "Alberta", "Morroc" }, { 4 } },
  { { "5) Choose the card that does", "^FF3355not^000000 touch a player's", "^5533FFDEX^000000." }, { "Rocker Card", "Mummy Card", "Zerom Card", "Drops Card" }, { 2 } },
  { { "6) What is cool about being", "a Rogue?" }, { "Being a bad ass", "Being a hypocrite", "Being shameless", "Excellent attack strength" }, { 4 } },
  { { "7) At what job level can a", "Thief become a Rogue?" }, { "Job level 30", "Job level 35", "Job level 40", "Job level 50" }, { 3, 4 } },
  { { "8) Where would you go to have", "your hair dyed?" },
    { "Southwest Morroc", "Southwest Prontera", "Southeast Morroc", "Northeast Prontera" }, { 2 } },
  { { "9) Which mushrooms must you", "steal to become a Thief?" },
    { "Orange Gooey Mushrooms", "Red Hairy Mushrooms", "Orange Net Mushrooms", "Orange Sticky Mushrooms" }, { 1, 3 } },
  { { "10) Which of these cards is", "^FF3355useless^000000 to a Rogue?" }, { "Whisper Card", "Elder Willow Card", "Zerom Card", "Matyr Card" }, { 2 } },
}
local SET2 = {
  { { "1) What must you learn before", "^5533FFStalk^000000?" }, { "Hiding", "Steal", "Improve Dodge", "Bash" }, { 1 } },
  { { "2) How much more discount does", "a Rogue's ^5533FFHaggle^000000 give than", "a Merchant's ^5533FFDiscount^000000?" }, { "3%", "2%", "1%", "0%" }, { 3 } },
  { { "3) What does ^5533FFMug^000000 do?" },
    { "Steal items from players", "Steal items from monsters", "Steal Zeny from monsters", "Steal Zeny from players" }, { 3 } },
  { { "4) How many Rogues does", "^5533FFSlyness^000000 need?" },
    { "1 Rogue + 2 Assassins", "2 Rogues + 1 Thief", "4 Thieves", "2 or more Rogues" }, { 4 } },
  { { "5) ^5533FFDivest Helm^000000 at level 5 opens", "which skill?" }, { "Envenom", "Strip Tease", "Venom Splasher", "Divest Shield" }, { 4 } },
  { { "6) Choose a skill that lets", "you move while hidden." }, { "Hiding", "Back Slide", "Stalk", "Sand Attack" }, { 3 } },
  { { "7) Choose the card that raises", "its owner's ^5533FFaccuracy^000000." }, { "Andre Card", "Familiar Card", "Mummy Card", "Marina Card" }, { 3 } },
  { { "8) A Vadon card adds 20% against", "Fire. Which monster takes more", "from it?" }, { "Vadon", "Deviruchi", "Elder Willow", "Baphomet" }, { 3 } },
  { { "9) How much SP does ^5533FFDouble", "Attack^000000 take with a dagger?" }, { "15 SP", "No SP needed", "10 SP", "54 SP" }, { 2 } },
  { { "10) Choose a good dagger for", "the Byalan Dungeon." },
    { "Wind Main-Gauche", "Ice Main-Gauche", "Earth Main-Gauche", "Fire Main-Gauche" }, { 1 } },
}

local K = "[Markie]"

local function interview()
  page(K, "I'll ask you a few questions.", "Listen carefully and choose", "the best answer, okay?")
  next()
  save_point(MAP, 366, 114)
  v.rogue_q = 1
  local score = 0
  for _, q in ipairs(rand(0, 1) == 1 and SET2 or SET1) do
    page(K, table.unpack(q[1]))
    next()
    local c = menu(table.unpack(q[2]))
    for _, ok in ipairs(q[3]) do
      if c == ok then score = score + 10 end
    end
  end
  page(K, "That's all of my questions.", "Not so bad, was it sweetie?", "I've been grading your answers,", "I hope you don't mind.")
  next()
  page(K, "You got ^FF3355" .. score .. "^000000/100...")
  next()
  if score < 90 then
    page(K, "*sigh*... not the score I was", "hoping for. With that, you're", "not cut out to be a Rogue.")
    emotion(4, true)
    next()
    page(K, "Go do some more training, shuga.", "You need it... *mumbles* what", "a waste of my time...")
    return close()
  end
  if score == 100 then
    page(K, "A perfect score! I knew you had", "what it takes. But don't get", "ahead of yourself, honey.")
  else
    page(K, "Good, good. You did well, shuga.", "But there's more, so don't get", "excited just yet...")
  end
  next()
  v.rogue_q = 2
  page(K, "Your second test will be given", "by ^5533FFMr. Smith^000000. He's hard to", "please, so mind how you act", "around him.")
  close()
end

npc {
  map = MAP, x = 363, y = 123, dir = 3, sprite = 747, name = "Markie",
  src = SRC .. "22",
  on_click = function()
    if player.class == Job_Rogue then
      page(K, "Nice to see you again, sweetie.", "Having fun as a Rogue? Tee hee...")
      return close()
    end
    if not thief() then
      page(K, "I don't know how you found this", "place, but you'd better leave...")
      return close()
    end
    if player.job_level < 40 then
      page(K, "Oh hi there. If you want to be", "a Rogue, come back after some", "more training.")
      next()
      page(K, "Only Thieves of ^5533FFjob level 40^000000", "or more can become Rogues.")
      return close()
    end
    local q = v.rogue_q
    if q == 1 then
      page(K, "You again? Sure you're ready", "this time? Okay honey, just", "relax...")
      next()
      return interview()
    end
    if q == 2 then
      page(K, "Your second test is with", "^5533FFMr. Smith^000000. Careful how you act", "around him.")
      return close()
    end
    if q == 3 then
      page(K, "Those guys are hard to find, I", "know. Be patient and keep", "looking. And get the password", "right.")
      return close()
    end
    if q == 4 then
      page(K, "What are you doing here? Not", "trying to skip the last test,", "I hope?")
      emotion(1, true)
      next()
      page(K, "You're almost there, so keep", "trying. I know you can do it.")
      return close()
    end
    if q == 5 then
      page(K, "You're back, and in one piece!", "You've passed every test, sweetie.", "You're ready to be a Rogue!")
      emotion(5, true)
      next()
      set_look(7, 0)
      job_change(Job_Rogue)
      emotion(46, true)
      give_item(v.rogue_jl == 50 and GLADIUS3 or GLADIUS2, 1)
      v.rogue_q, v.rogue_q2, v.rogue_jl = 0, 0, 0
      page(K, "Tada! Congratulations, Rogue.", "You worked hard for this; here's", "a little reward.")
      next()
      page(K, "Now you're free to go wherever", "you like and do whatever you", "like.")
      next()
      page(K, "Just remember that freedom", "needs responsibility. Treat", "people the way you want to be", "treated.")
      next()
      page(K, "Have fun, shuga! See ya around!")
      return close()
    end
    page(K, "Hmm? What brings you down here?", "Oh, I see... you want to be a", "Rogue, don't you, sweetie?")
    next()
    page(K, "Nice to meet you. I'm Markie,", "and you are...?")
    next()
    page(K, player.name .. "... Tee hee, a nice", "name, honey.")
    next()
    page(K, "Since you gave me your real", "name... a Rogue never tells", "anybody who he really is. Rule", "number one, shuga. Remember it.")
    next()
    page(K, "Now, here's an application", "form. Fill it all in, please...")
    next()
    mes "(You fill in the form and hand it back.)"
    v.rogue_jl = player.job_level == 50 and 50 or 40
    next()
    page(K, "Looks good: application", "accepted. Now a short interview,", "to see how much you know about", "Rogues. Shall we begin?")
    next()
    if menu("Okay.", "Hold on... I need some time.") ~= 1 then
      page(K, "Take your time, honey. Come back", "when you're ready.")
      return close()
    end
    interview()
  end,
}

-- ------------------------------------------------------------ Mr. Smith

local ERRANDS = {
  { { 510, 6, "Blue Herbs" }, { 932, 10, "Skel-Bones" }, { 957, 10, "Decayed Nails" }, { 958, 10, "Horrendous Mouths" } },
  { { 511, 10, "Green Herbs" }, { 910, 10, "Garlets" }, { 926, 10, "Snake Scales" }, { 964, 10, "Crab Shells" } },
  { { 508, 10, "Yellow Herbs" }, { 948, 10, "Bear's Footskins" }, { 935, 10, "Shells" }, { 940, 10, "Grasshopper's Legs" } },
}

local S = "[Mr. Smith]"
local function errand_list()
  local lines = {}
  for _, it in ipairs(ERRANDS[v.rogue_q2]) do lines[#lines + 1] = "^5533FF" .. it[2] .. " " .. it[3] .. "^000000" end
  page(S, table.unpack(lines))
end

-- the three guildsmen: who, where, and the password
local GUILDSMEN = {
  { "Aragham Junior",
    { "He lives in a little house", "^5533FFsouthwest^000000 of the Fortress of", "Sandarman, one map ^5533FFeast^000000 of", "here, on the southwestern land." },
    { "He collects unpaid debts, and", "well. People wanted him dead for", "something his father did, so he", "keeps his head down." },
    { "Aragham never hoarded", "upgrade items." } },
  { "Hollgrehen Junior",
    { "He lives in a little house", "^5533FFsoutheast^000000 of the Fortress of", "Sandarman, one map ^5533FFeast^000000 of", "here, on the eastern land." },
    nil,
    { "My father never hoarded", "upgrade items." } },
  { "Antonio Junior",
    { "He lives in a little hut at the", "^5533FFeastern end of Kokomo Beach^000000,", "one map north and one map west", "of here." },
    nil,
    { "Antonio doesn't like breaking", "refining materials." } },
}

local function guildsman_brief()
  local g = GUILDSMEN[v.rogue_q2]
  page(S, "Go and find ^5533FF" .. g[1] .. "^000000.")
  next()
  page(S, table.unpack(g[2]))
  next()
  if g[3] then
    page(S, table.unpack(g[3]))
    next()
  end
  page(S, "Only the guild knows where he", "is. So you'll need the password", "to talk to him:")
  next()
  page(S, "'^5533FF" .. g[4][1] .. "^000000", "^5533FF" .. g[4][2] .. "^000000'")
end

npc {
  map = MAP, x = 376, y = 23, dir = 3, sprite = 57, name = "Mr. Smith",
  src = SRC .. "370",
  on_click = function()
    if player.class == Job_Rogue then
      page(S, "Looking good there, my Rogue", "friend.")
      return close()
    end
    if not thief() then
      page(S, "Go away! I'm busy...")
      return close()
    end
    local q = v.rogue_q
    if q > 3 then
      page(S, "I've no more business with you...")
      return close()
    end
    if q == 2 and v.rogue_q2 == 0 then
      page(S, "So you're here for the second", "test... good...")
      next()
      page(S, "First, the application fee. Our", "guild does a lot of *cough*", "'business', and always needs", "funds.")
      next()
      page(S, "So the Rogue tests cost ^5533FF10,000", "Zeny^000000. I take it when you finish", "this test; no need to have it", "now.")
      next()
      page(S, "Now your test: bring me these.")
      next()
      v.rogue_q2 = rand(1, 3)
      errand_list()
      next()
      page(S, "Come back when you have ALL of", "them and 10,000 Zeny. Got it?", "Good.")
      return close()
    end
    if q == 2 then
      local set = ERRANDS[v.rogue_q2]
      local short = player.zeny < 10000
      for _, it in ipairs(set) do
        if count_item(it[1]) < it[2] then short = true end
      end
      if short then
        page(S, "...What the f@&#!? You don't", "have it all? Then why come back?", "Don't tell me you forgot...")
        emotion(1, true)
        next()
        page(S, "Write this down! ^5533FF10,000 Zeny^000000,", "and:")
        next()
        errand_list()
        next()
        page(S, "Got it? DON'T come back until", "you have everything...")
        emotion(32, true)
        return close()
      end
      for _, it in ipairs(set) do take_item(it[1], it[2]) end
      player.zeny = player.zeny - 10000
      v.rogue_q, v.rogue_q2 = 3, 0
      page(S, "Let's see... fee, check. Items,", "check. All in order: you pass", "the second test.")
      next()
      page(S, "Tell me when you're ready for", "the next one.")
      return close()
    end
    if q == 3 and v.rogue_q2 == 0 then
      page(S, "Who should I have you find...", "hmm...")
      emotion(20, true)
      next()
      page(S, "Ah, I know! Last time you found", "items; this time you find a", "person.")
      emotion(5, true)
      next()
      v.rogue_q2 = rand(1, 3)
      guildsman_brief()
      next()
      page(S, "Good luck. Be quick about it:", "getting things done fast is a", "Rogue's way.")
      return close()
    end
    if q == 3 then
      page(S, "What!? You forgot where to go?")
      emotion(1, true)
      next()
      page(S, "GRRR! That's why you WRITE", "important things down! Geez...")
      emotion(6, true)
      next()
      guildsman_brief()
      return close()
    end
    page(S, "One... two... three... where do", "you do business... Morroc...", "Geffen... Comodo...?")
    next()
    page(S, "Ugh... this is giving me a", "headache...")
    next()
    page(S, "Huh? Who are you? If you want to", "be a Rogue, talk to ^5533FFMarkie^000000.")
    next()
    page(S, "...DAMN! I forgot what I was", "counting!")
    emotion(16, true)
    close()
  end,
}

npc {
  map = MAP, x = 272, y = 136, dir = 3, sprite = 85, name = "Hermanthorn Jr.",
  src = SRC .. "599",
  on_click = function()
    local H = "[Hermanthorn Jr.]"
    page(H, "Huh? Wha... who are you? You're", "not from the guild! Get out!")
    emotion(1, true)
    next()
    page(H, "You're... I see! You're one of", "them! You came to kill me!", "No... NO! I'm not ready to die!")
    emotion(0, true)
    next()
    page(H, "Aaaaaahhh!! Go away! Get lost!", "Or I'll KILL YOU!")
    emotion(16, true)
    close()
  end,
}

-- ------------------------------------------------------------ the doors

local WORDS = {
  { "My father", "Aragham", "Antonio", "Legolas" },
  { "did not", "doesn't like", "never", "ever" },
  { "hoard", "hoarded", "hide", "took", "breaking" },
  { "upgrade items.", "forging items.", "refining materials.", "upgrade equipment." },
}

local DOORS = {
  { "cmd_fild09", 106, 195, { "Aragham", "never", "hoarded", "upgrade items." }, { 244, 24 }, 616 },
  { "cmd_fild09", 335, 143, { "My father", "never", "hoarded", "upgrade items." }, { 168, 34 }, 621 },
  { "cmd_fild04", 304, 179, { "Antonio", "doesn't like", "breaking", "refining materials." }, { 165, 104 }, 626 },
}

for i, d in ipairs(DOORS) do
  npc {
    map = d[1], x = d[2], y = d[3], dir = 0, sprite = 45, name = "Hidden House", id = "rogue_door_" .. i,
    touch = { 1, 1 }, src = SRC .. d[6],
    on_click = function()
      page("[???]", "Who's there?!", "Who dares trespass on my", "territory?")
      if not thief() or v.rogue_q < 3 or v.rogue_q2 == 0 then return close() end
      local said = {}
      for k = 1, 4 do
        next()
        said[k] = WORDS[k][menu(table.unpack(WORDS[k]))]
        page("[^5533FF" .. player.name .. "^000000]", table.unpack(said))
      end
      next()
      local right = v.rogue_q2 == i
      for k = 1, 4 do
        if said[k] ~= d[4][k] then right = false end
      end
      if not right then
        page("[???]", "Get lost!!")
        return close()
      end
      mes "~ creeeak ~"
      mes "You hear the door begin to open..."
      close()
      warp(MAP, d[5][1], d[5][2])
    end,
  }
end

-- ------------------------------------------------------------ the run

-- each ambush: where its line is ({ x, y, half-width, half-height }), and
-- who springs out
local AMBUSHES = {
  { { 15, 185, 8, 0 }, { { 14, 188, 1015 }, { 15, 188, 1015 }, { 16, 188, 1015 }, { 14, 189, 1015 }, { 15, 189, 1015 }, { 16, 189, 1015 } } },
  { { 15, 245, 8, 0 }, { { 14, 247, 1041 }, { 15, 247, 1041 }, { 16, 247, 1041 }, { 14, 248, 1041 }, { 15, 248, 1041 }, { 16, 248, 1041 } } },
  { { 15, 328, 8, 0 }, { { 14, 331, 1015 }, { 15, 331, 1015 }, { 16, 331, 1015 }, { 14, 332, 1015 }, { 15, 332, 1015 }, { 16, 332, 1015 } } },
  { { 35, 342, 0, 8 }, { { 38, 340, 1041 }, { 37, 342, 1219 }, { 39, 342, 1036 }, { 38, 344, 1041 } } },
  { { 57, 306, 4, 0 }, { { 59, 301, 1132 } } },
  { { 96, 333, 0, 4 }, { { 92, 325, 1132 } } },
  { { 139, 313, 4, 0 }, { { 139, 309, 1219 } } },
  { { 135, 246, 0, 4 }, { { 139, 245, 1015, 2 }, { 139, 246, 1015, 2 }, { 139, 247, 1015, 2 } } },
  { { 62, 302, 0, 4 }, { { 57, 305, 1132 } } },
  { { 199, 218, 0, 8 }, { { 157, 218, 1015, 10 }, { 157, 218, 1219 }, { 157, 218, 1036 }, { 157, 218, 1016 } } },
  { { 86, 187, 0, 8 }, { { 88, 186, 1041 }, { 88, 187, 1041 }, { 88, 188, 1041 } } },
  { { 225, 187, 0, 8 }, { { 232, 191, 1219 }, { 233, 190, 1219 }, { 234, 189, 1219 } } },
  { { 252, 320, 0, 8 }, { { 252, 319, 1041 }, { 252, 321, 1041 }, { 252, 319, 1036 }, { 252, 321, 1036 }, { 257, 320, 1016 }, { 248, 320, 1016 } } },
}
local AMBUSH_LINES = { 817, 827, 837, 847, 855, 860, 865, 870, 877, 882, 890, 897, 904 }
local RUN_EV = "rogue_run::OnKilled"
local sprung = {}

local function clear_run()
  sprung = {}
  kill_monster(MAP, RUN_EV)
end

for i, a in ipairs(AMBUSHES) do
  local at = a[1]
  npc {
    map = MAP, x = at[1], y = at[2], dir = 0, sprite = -1, name = "rogue_mob" .. i, id = "rogue_mob" .. i,
    touch = { at[3], at[4] }, src = SRC .. AMBUSH_LINES[i],
    on_click = function()
      if v.rogue_q ~= 4 or sprung[i] then return end
      sprung[i] = true
      for _, m in ipairs(a[2]) do monster(MAP, m[1], m[2], mob_name(m[3]), m[3], m[4] or 1, RUN_EV) end
    end,
  }
end

-- nothing to do when one falls: the event names the run's monsters
npc { map = "-", name = "rogue_run", sprite = -1, events = { "OnKilled" }, src = SRC .. "929", on_click = function() end }

npc {
  map = MAP, x = 370, y = 320, dir = 0, sprite = 45, name = "rogue04", id = "rogue_run_end",
  touch = { 1, 1 }, src = SRC .. "918",
  on_click = function()
    if v.rogue_q == 4 then v.rogue_q = 5 end
    clear_run()
    warp(MAP, 378, 113)
  end,
}

local HOUSES = {
  { 244, 39, 99, "Aragham Junior", { 245, 27 }, 742 },
  { 152, 27, 85, "Hollgrehen Junior", { 160, 32 }, 747 },
  { 183, 105, 88, "Antonio Junior", { 175, 107 }, 752 },
}

for _, h in ipairs(HOUSES) do
  local G = "[" .. h[4] .. "]"
  npc {
    map = MAP, x = h[1], y = h[2], dir = 4, sprite = h[3], name = h[4],
    src = SRC .. h[6],
    on_click = function()
      if not thief() or v.rogue_q < 3 then
        page(G, "...")
        return close()
      end
      if v.rogue_q == 4 then
        page(G, "...Looks like you got creamed...")
        emotion(4, true)
        next()
        page(G, "Up for it? I'll send you back in.", "Failure has a way of teaching", "success... yada yada yada...")
        percent_heal(100, 100)
      else
        page(G, "Hello there... from the guild,", "are you? I'm " .. h[4] .. ",", "the Rogue of the Desert...")
        next()
        page(G, "Aw, who am I kidding. Let's cut", "the crap. Ready to become a", "Rogue?")
        emotion(4, true)
        next()
        page(G, "The Rogue motto is... ^5533FF'Avoid", "the strong! Be malicious to the", "weak!'^000000 It goes for monsters too.")
        next()
        page(G, "The very last thing: walk an", "underground passage back to the", "Rogue guild.")
        next()
        page(G, "Easy, huh? It's not! But stay", "true to the motto, hide a lot,", "run a lot, and you'll be fine.")
      end
      next()
      page(G, "Okay, ready to go?")
      next()
      if menu("As ready as I'll ever be.", "Actually I'm kinda scared...") ~= 1 then
        page(G, "...")
        next()
        page(G, "Well, take your time then...", "I guess...")
        return close()
      end
      page(G, "Good luck, then.")
      close()
      v.rogue_q = 4
      clear_run()
      save_point(MAP, h[5][1], h[5][2])
      warp(MAP, 15, 105)
    end,
  }
end
