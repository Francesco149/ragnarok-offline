-- The Dancer test: Comodo's dance school takes a female Archer's application,
-- a fee and an interview of ten questions, then a one-minute audition on its
-- stage: steps called out by the teacher, a wrong step too late sends you
-- out, and a Poring shot at the end passes you.
--
-- Behind `jobs2_2` (euRO opened the 2-2 jobs on 2004-06-09).
--
-- Sources:
--   * jAthena //1046 script/npc/job/npc_job_20dancer.txt (2004-09-12,
--     data:refs/jathena commit 6332410; Japanese, by Kalen): the flow, the
--     NPCs, their cells and sprites, the quest variable `change_dancer` and
--     its steps, the fee and the items, both sets of questions with their
--     answers and pass marks, the audition's calls and times, its five step
--     sensors and when each opens, the Dance Poring, the reward (a Rope, or
--     a Line at Job Level 50). Tier athena2003 (a period emulator, months
--     after our date). Dialogue here is our English of its Japanese, cut to
--     the window.
--   * jAthena //1046 script/warp/npc_warp_job.txt:360 (from 2f7ba71,
--     2004-03-02): the school's way out to Comodo.
--   * rAthena npc/jobs/2-2/dancer.txt (Aegis-based, tier later): the English
--     names (Bor Robin, Aile, Bijou, Pyorgin), and iRO's wording where its
--     questions are jAthena's first set. Its third set shares six questions
--     with jAthena's second and parts on the others (the casino manager,
--     question 9's point, a jazz quiz for the lighthouses).
--
-- Where we part from jAthena (docs/DEVIATIONS.md):
--   * no waiting room. jAthena seats candidates in an NPC chat room and
--     calls them one at a time; here you tell the usher you are ready, or
--     that you will lounge a bit first (owner, 2026-11-11: as the Wizard
--     test). jAthena's usher says nothing: his lines are ours;
--   * a wrong step ends the audition at once, so you can try again; in
--     jAthena the minute runs on for the next candidate;
--   * question 3 of the first set asks which response is NOT right, as iRO
--     words it: jAthena asks "what do you do", with "point out the mistake"
--     as the one right answer;
--   * at 21 s the "middle" step's sensors open in the same second as the
--     "hold the pose" call that closes them; jAthena leaves the order to
--     its timers. Here they open, and stay open until the next step at
--     29 s: you hold still in the middle;
--   * 817's Performance Master no longer makes a woman a Dancer at once:
--     he sends her to the school. Bards are still his (jAthena's Bard test
--     is a stub).

if not content("jobs2_2") then return end

local SRC = "jathena-1046:script/npc/job/npc_job_20dancer.txt:"
local MAP = "job_duncer"
local OUT = { "comodo", 188, 162 }

local EARTHWORM, BOOTS, ROPE, LINE = 1055, 2405, 1950, 1953

warp { map = MAP, x = 69, y = 43, span = { 1, 1 }, to = { "comodo", 185, 156 } }

-- v.change_dancer: 1 applied, 2 sent for the fee and items, 5 to the
-- interview, 6 interview failed, 7 passed, 8 sent to the audition,
-- 10 danced (jAthena's numbers; 3 and 4 are its unused other errands)

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

local function candidate() return player.class == Job_Archer and player.sex == 0 end

-- 817's Performance Master: Bards only now
wrap_npc("817:jobs/dancer-bard.txt:5", function(orig)
  return function(at)
    if candidate() then
      page("[Performance Master]", "A Dancer? Comodo's dance", "school trains its own now.",
        "Bor Robin, by the stage, will", "take you there.")
      return close()
    end
    return orig(at)
  end
end)

npc {
  map = "comodo", x = 193, y = 151, dir = 4, sprite = 86, name = "Bor Robin",
  src = SRC .. "4",
  on_click = function()
    page("[Bor Robin]", "Ahh... the prima donnas who", "draw every eye in town!",
      "And I get to watch them become", "Dancers right before me!")
    next()
    page("[Bor Robin]", "How lucky to be born now.", "Hurray for the Comodo Dancers!")
    next()
    page("[Bor Robin]", "Hm? You want to watch the", "Dancer job change test too?")
    next()
    if menu("Go to the Dancer school", "Cancel") ~= 1 then
      page("[Bor Robin]", "Boo, no fun...")
      return close()
    end
    page("[Bor Robin]", "Yeah! Let's go!")
    close()
    warp("job_duncer", 70, 49)
  end,
}

npc {
  map = MAP, x = 43, y = 93, dir = 4, sprite = 724, name = "Aile",
  src = SRC .. "31",
  on_click = function()
    if player.class == Job_Dancer then
      page("[Aile]", "Hello~", "How are things lately?", "Are you keeping people", "entertained?")
      return close()
    end
    if not candidate() then
      page("[Aile]", "Excuse me, only staff", "may come in here.")
      next()
      page("[Aile]", "Not just anyone can enter", "the dance school, you know.")
      next()
      page("[Aile]", "If you'd like to watch, go", "to the dance stage in the", "village.")
      return close()
    end
    local q = v.change_dancer
    if q == 1 then
      page("[Aile]", "Yes, you applied a moment ago.", "Now you'll need to gather", "what the lessons call for.")
      next()
      page("[Aile]", "We're short of a few things...", "They're for your own lessons,", "so sorry for the trouble, but", "please bring them yourself.")
      next()
      page("[Aile]", "First the ^0000FFlesson fee of", "10000 Zeny^000000. Then", "^0000FF5 Earthworm Peelings^000000", "to keep your shoes from", "slipping.")
      next()
      page("[Aile]", "And a pair of ^0000FFBoots^000000,", "so you don't hurt your feet.")
      next()
      page("[Aile]", "Talk to me again when you", "have it all ready.")
      v.change_dancer = 2
      return close()
    end
    if q >= 2 and q <= 4 then
      -- jAthena takes the fee only: the peelings and the boots are yours
      -- for the lessons (rAthena's iRO script the same)
      if count_item(EARTHWORM) < 5 or count_item(BOOTS) < 1 or player.zeny < 10000 then
        page("[Aile]", "Hmm...", "Something seems to be missing.", "Here's the list again,", "so please bring it all.")
        next()
        page("[Aile]", "^0000FFLesson fee 10000 Zeny^000000", "^0000FF5 Earthworm Peelings^000000", "^0000FF1 pair of Boots^000000")
        next()
        page("[Aile]", "That's all.", "I'll be waiting.")
        return close()
      end
      page("[Aile]", "Oh, you brought everything.", "Then I'll take the lesson fee.")
      player.zeny = player.zeny - 10000
      next()
      q = 5
    end
    if q == 5 or q == 6 then
      page("[Aile]", "Go and see ^0000FFMadam Bijou^000000,", "who holds the interviews.", "It's just a few simple", "questions, so relax.")
      if v.change_dancer < 5 then v.change_dancer = 5 end
      return close()
    end
    if q >= 7 then
      page("[Aile]", "Practise hard and show us", "a lovely dance!")
      return close()
    end
    page("[Aile]", "Welcome~", "This is the Comodo Dance", "School, where we teach", "all sorts of dances.")
    next()
    page("[Aile]", "We give the tourists who come", "from everywhere lovely dances,")
    next()
    page("[Aile]", "and we train dancers who", "become famous all over", "Rune-Midgard.")
    next()
    page("[Aile]", "How about it? Dancing under", "bright lights before a crowd!", "Isn't that wonderful?")
    next()
    page("[Aile]", "But not just anyone gets into", "our school. That's what makes", "us different.")
    next()
    page("[Aile]", "Those who could never be", "Dancers are turned away.")
    next()
    page("[Aile]", "You, though, look like you", "have it in you.", "Why not take our lessons?")
    next()
    page("[Aile]", "You only need to fill in", "the application form there.")
    next()
    page("[Aile]", "Well? Won't you apply?", "Even just to see what", "it's like?")
    next()
    if menu("Yes, I'll try", "Ignore her") ~= 1 then
      page("[Aile]", "Oh... how cold...", "Do come again, won't you?")
      return close()
    end
    if player.job_level < 40 then
      page("[Aile]", "Um...", "What a pity, you don't meet", "the conditions for lessons...")
      next()
      page("[Aile]", "We can only take applications", "from Job Level 40 up.", "I look forward to seeing", "you again then.")
      return close()
    end
    page("[Aile]", "Good decision!", "Please fill in the form there.")
    next()
    mes("......")
    next()
    mes("...scribble, scribble...")
    next()
    mes("......")
    next()
    page("[Aile]", "Your name is...", player.name .. "...", "What a lovely name!", "One moment please, I'll take", "this to the headmistress.")
    v.change_dancer = 1
    close()
  end,
}

-- Bijou's two sets of questions: { question lines, choices, right choice };
-- a choice of `false` is the candidate's own name, worth -100
local SETS = {
  { -- jAthena's first set, iRO's second (rAthena)
    { { "1. What does the Bard and", "Dancer ensemble Loki's Veil do?" },
      { "Raises Level 4 weapons' ATK", "Doubles damage", "No skills or magic in its area", "Raises attack speed" }, 3 },
    { { "2. Which is bad manners", "on the dance floor", "after a dance?" },
      { "Thank your partner", "Praise the dance", "Ask for another dance", "Say they're no match for you" }, 4 },
    { { "3. Which is not a right", "response when someone makes", "a mistake while you dance?" },
      { "Keep smiling and dancing", "Point out the mistake", "Pretend not to notice", "Give them a quiet smile" }, 2 },
    { { "4. In which town can you", "become a Dancer?" }, { "Cocomo", "Sandarman", "Comudo", "Comodo" }, 4 },
    { { "5. How many caves lead", "straight out of Comodo?" }, { "1", "2", "3", "4" }, 3 },
    { { "6. Which of these monsters", "cannot be a pet?" }, { "Isis", "Argiope", "Dokebi", "Deviruchi" }, 2 },
    { { "7. Who dances best of all?" }, { false, "Madam Bijou", "Isis", "Mercy" }, 2 },
    { { "8. What is the Kafra office", "in Comodo called?" },
      { "Kafra Headquarters", "Kafra West District", "Kafra Service", "Kafra Western Branch" }, 4 },
    { { "9. What is my name?" }, { "Bonsoir", "Bourgeois", "Bijou", "Bonjour" }, 3, "[......]" },
    { { "10. What does the skill", "Lullaby do?" },
      { "Blinds those around", "Puts those around to sleep", "Keeps those around awake", "Freezes those around" }, 2 },
  },
  { -- jAthena's second set
    { { "1. What does the Dancer's", "skill Dancing Lesson do?" },
      { "Raises INT", "Strengthens dance skills", "Raises whip damage", "Stuns those around" }, 2 },
    { { "2. In special shoes that", "click, you step and beat", "a lively rhythm. What is", "that dance called?" },
      { "Tap dance", "Improve Concentration", "Tango", "Double Strafing" }, 1 },
    { { "3. Which is furthest from", "what makes a Dancer?" },
      { "Mastering dance skills", "Attacking from afar", "Using whips", "Using two-handed swords" }, 4 },
    { { "4. Which town has the most", "Dancers?" }, { "Al De Baran", "Juno", "Morroc", "Comodo" }, 4 },
    { { "5. Who gives the most", "beautiful dance?" }, { false, "Madam Bijou", "Isis", "Emeral Handas" }, 2 },
    { { "6. Where does a Dancer", "outdo every other job?" }, { "Stamina", "Acting", "Dancing", "Magic" }, 3 },
    { { "7. Who manages the Comodo", "casino?" }, { "Roo", "Moo", "Martine", "Loyar" }, 3 },
    { { "8. Which can a Dancer", "not wear?" }, { "Kitty Band", "Two-Handed Sword", "Sandals", "Earring" }, 2 },
    { { "9. Is this test boring?" }, { "Yes", "No" }, 2 },
    { { "10. How many lighthouses", "are there on Pharos", "Lighthouse Island?" }, { "1", "2", "3" }, 1 },
  },
}

local function interview()
  local score = 0
  for _, q in ipairs(SETS[math.random(1, 2)]) do
    page(q[4] or "[Bijou]", table.unpack(q[1]))
    next()
    local choices = {}
    for i, c in ipairs(q[2]) do choices[i] = c or player.name end
    local c = menu(table.unpack(choices))
    if q[2][c] == false then
      score = score - 100
      if q[1][1]:find("^7%.") then
        mes("......")
        next()
      end
    elseif c == q[3] then
      score = score + 10
    end
  end
  page("[Bijou]", "There, well done~")
  next()
  local pass = v.change_dancer == 5 and 80 or 70
  if score < pass then
    page("[Bijou]", "Your interview score is", score .. " points...", "You fail.", "This won't do at all.")
    next()
    page("[Bijou]", "Was it a little hard?", "I knew far more at your age.", "Go and study, then come back.")
    v.change_dancer = 6
    return close()
  end
  if score == 100 then
    page("[Bijou]", "Your interview score is", score .. " points...", "Splendid. You pass.")
  else
    page("[Bijou]", "Your interview score is", score .. " points...", "A few things worried me,", "but that will do.")
  end
  next()
  page("[Bijou]", "This much is fine.", "The dance practical is", "another matter, but with work", "you'll manage.")
  v.change_dancer = 7
  close()
end

npc {
  map = MAP, x = 95, y = 93, dir = 4, sprite = 101, name = "Bijou",
  src = SRC .. "227",
  on_click = function()
    if player.class == Job_Dancer then
      page("[Bijou]", "Oh, hello~")
      next()
      page("[Bijou]", "How are things? Are you", "moving many people?", "Never forget the heart", "of a Dancer.")
      return close()
    end
    if not candidate() then
      page("[Bijou]", "Oh, from outside, are you?", "You came all this way to", "see my lovely dancing...")
      next()
      page("[Bijou]", "Sadly I've retired. Now I put", "my strength into training", "the next ones.")
      next()
      page("[Bijou]", "Go to the dance stage in the", "middle of the village: a girl", "who learnt my dance well", "should be dancing there.")
      return close()
    end
    if player.skill_point ~= 0 then
      page("[Bijou]", "You can't change jobs with", "skill points left over.", "Go and use them.")
      return close()
    end
    local q = v.change_dancer
    if q == 5 or q == 6 then
      if q == 5 then
        page("[Bijou]", "Oh, a Dancer hopeful.")
        next()
        page("[Bijou]", "...You'll need to tighten", "that body up a little.")
        next()
        page("[Bijou]", "Don't look so worried.", "It shapes up during lessons,", "and there are figure lessons", "too. Hohoho...")
        next()
        page("[Bijou]", "Now, I'd like to know your", "heart as a Dancer, so I'll", "ask a few things. It's an", "interview, but simple: relax.")
      else
        page("[Bijou]", "Chin up!")
      end
      next()
      return interview()
    end
    if q == 7 then
      page("[Bijou]", "Now for the practical.", "Shall I first tell you", "a few points to mind?")
      next()
      if menu("Hear the points", "Go to the audition") == 1 then
        page("[Bijou]", "This is an audition, really.", "You have ^0000FFone minute^000000,")
        next()
        page("[Bijou]", "and dance ^0000FFone at a time^000000.", "Don't worry if you have", "never danced before.")
        next()
        page("[Bijou]", "On the stage, first stand", "where you can ^0000FFsee the whole", "stage^000000.")
        next()
        page("[Bijou]", "^0000FFSo front, back, left and right", "are clear^000000. Double-click the", "right mouse button to turn", "the view back to the start...", "I'm no good with camera talk.")
        next()
        page("[Bijou]", "Until the start, wait in the", "^0000FFwaiting room^000000.")
        next()
        page("[Bijou]", "When it starts, I call out", "which way to move from", "where you first stood.", "Just move along with it.")
        next()
        page("[Bijou]", "^0000FFMiss the timing and you are", "out at once^000000.", "I'll be strict!")
        return close()
      end
    elseif q == 8 then -- luck wished again, and back to the waiting room
    elseif q == 10 then
      page("[Bijou]", "Oh, that dance just now", "was lovely~")
      next()
      page("[Bijou]", "Dancing like that, you are", "Dancer enough.", "Come, I'll change your job.")
      next()
      page("[Bijou]", "Thou, now blessed by", "the goddess...")
      next()
      page("[Bijou]", "become a Dancer who", "brings joy to all...")
      next()
      local line = player.job_level == 50
      page("[Bijou]", "and wherever thou dancest...")
      next()
      job_change(Job_Dancer)
      v.change_dancer = 0
      page("[Bijou]", "may smiles overflow.")
      next()
      page("[Bijou]", "This is a present from me.", "Congratulations!")
      give_item(line and LINE or ROPE, 1)
      return close()
    else
      page("[Bijou]", "Oh, a girl who wants to dance.")
      next()
      page("[Bijou]", "If you want to be a Dancer,", "apply to the school first.", "Aile, left of the stage,", "will tell you all about it.")
      return close()
    end
    page("[Bijou]", "Good luck, then!")
    v.change_dancer = 8
    close()
    warp("job_duncer", 104, 109)
  end,
}

-- The audition. Five sensors cover the stage: the middle (68-70,109-111)
-- and its four arms. Each call hides them all; a few seconds later the
-- four that are not the called step open, and stepping on one of them
-- before the next call puts you out. "Back" is up the screen: the stage
-- seen with the camera as it starts.
local STEPS = { -- cell, jAthena line
  up = { 69, 113, 851 }, left = { 66, 110, 864 }, middle = { 69, 110, 877 }, right = { 72, 110, 890 }, down = { 69, 107, 903 },
}
local function sensor(step) return "#dance_" .. step end

-- the calls: { ms, words, closes, step to judge, ms until the others open }.
-- `closes`: the call hides the sensors the last one opened (jAthena's
-- donpcevent "::OnDE1"). jAthena's jdtN timers fire at 5000 after
-- setnpctimer N: the others open 5000 - N after the call
local CALLS = {
  { 1000, "Bijou: Let's begin. Relax. You have one minute.", false },
  { 3000, "Bijou: Back! : [^]", false, "up", 3000 },
  { 7000, "Bijou: Forward~ : [v]", true, "down", 3000 },
  { 11000, "Bijou: Left~ : [<]", true, "left", 3000 },
  { 15000, "Bijou: Turn right : [>]", true, "right", 3000 },
  { 19000, "Bijou: Back to the middle : [*]", true, "middle", 2000 },
  { 21000, "Bijou: Hold the pose!", true },
  { 26000, "Bijou: After the pose, ad-lib something like Improve Concentration!", false },
  { 28000, "Bijou: Hold it~", false },
  { 29000, "Bijou: To the left~ : [<]", true, "left", 3000 },
  { 33000, "Bijou: Forward~ : [v]", true }, -- jAthena: not judged ("the right after it is hard")
  { 35000, "Bijou: Right away, right! : [>]", false, "right", 2000 },
  { 38000, "Bijou: And pose!", true },
  { 41000, "Bijou: Left, middle, right, back~ : [<][*][>][^]", false, "up", 5000 },
  { 47000, "Bijou: [>]", true, "right", 2000 },
  { 50000, "Bijou: Left, middle, forward, back : [<][*][v][^]", true, "up", 5000 },
  { 56000, "Bijou: Once more, left, middle, forward, back : [<][*][v][^]", true, "up", 5000 },
  { 62000, "Bijou: Forward~ : [v]", true, "down", 2500 },
  { 65000, "Bijou: Left : [<]", true, "left", 2500 },
  { 68000, "Bijou: Middle : [*]", true, "middle", 2500 },
  { 71000, "Bijou: Finish with an Arrow Shower!", true },
}
local LATE = 80000

-- every OnTimer label: the calls, the openings, the Poring's leaving
local AT = {}
for _, c in ipairs(CALLS) do
  AT[c[1]] = AT[c[1]] or {}
  AT[c[1]].call = c
  if c[4] then
    local open = c[1] + c[5]
    AT[open] = AT[open] or {}
    AT[open].open = c[4]
  end
end
AT[76000] = { clear = true }
AT[LATE] = { late = true }
local LABELS = {}
for ms in pairs(AT) do LABELS[#LABELS + 1] = "OnTimer" .. ms end

local function hide_all()
  for step in pairs(STEPS) do disable_npc(sensor(step)) end
end

local function backdancers(label)
  for i = 1, 4 do do_event("dance_back" .. i .. "::" .. label) end
end

local function reset()
  npc_timer_stop("dance_audition")
  hide_all()
  kill_monster(MAP, "dance_poring::OnKilled")
end

local function out()
  -- the stage: the middle row and the middle column
  area_warp(MAP, 68, 106, 70, 114, OUT[1], OUT[2], OUT[3])
  area_warp(MAP, 65, 109, 73, 111, OUT[1], OUT[2], OUT[3])
end

npc {
  map = "-", name = "dance_audition", sprite = -1, events = LABELS, src = SRC .. "610",
  on_click = function(label)
    local ms = tonumber(label:match("^OnTimer(%d+)$"))
    local at = ms and AT[ms]
    if not at then return end
    if at.call then
      local c = at.call
      -- a step hides the other sensors too (jAthena's L_SUBn), which
      -- the closing calls have hidden already
      if c[3] then backdancers("OnDE1") end
      if c[3] or c[4] then hide_all() end
      map_announce(MAP, c[2], 0)
      if ms == 71000 then
        monster(MAP, 69, 106, "Dance Poring", 1002, 1, "dance_poring::OnKilled")
      end
    end
    if at.open then
      for step in pairs(STEPS) do
        if step ~= at.open then enable_npc(sensor(step)) end
      end
    end
    if at.clear then kill_monster(MAP, "dance_poring::OnKilled") end
    if at.late then
      hide_all()
      map_announce(MAP, "dance: Your timing was late!", 0)
      out()
    end
  end,
}

for step, cell in pairs(STEPS) do
  npc {
    map = MAP, x = cell[1], y = cell[2], dir = 4, sprite = 139, name = sensor(step),
    touch = { 1, 1 }, events = { "OnInit" }, src = SRC .. cell[3],
    on_click = function(label)
      if label == "OnInit" then return disable_npc(sensor(step)) end
      if v.change_dancer ~= 8 then return end
      map_announce(MAP, "dance: " .. player.name .. "~ your timing was late!", 0)
      reset()
      backdancers("OnDE2")
      warp(OUT[1], OUT[2], OUT[3])
    end,
  }
end

npc {
  map = "-", name = "dance_poring", sprite = -1, events = { "OnKilled" }, src = SRC .. "763",
  on_click = function()
    if v.change_dancer ~= 8 then return end
    map_announce(MAP, "dance: Well done! Madam Bijou will have noticed you~", 0)
    v.change_dancer = 10
    reset()
    warp(OUT[1], OUT[2], OUT[3])
  end,
}

-- the backdancers nod along (emotion 21) and wince at a wrong step (23);
-- jAthena's cells, on the stage's blocked wings
for i, cell in ipairs { { 63, 110 }, { 66, 113 }, { 72, 113 }, { 75, 110 } } do
  npc {
    map = MAP, x = cell[1], y = cell[2], dir = 4, sprite = 724, name = "Backdancer#" .. i, id = "dance_back" .. i,
    events = { "OnDE1", "OnDE2" }, src = SRC .. (772 + 17 * (i - 1)),
    on_click = function(label)
      if label == "OnDE1" then return npc_emotion(21) end
      if label == "OnDE2" then return npc_emotion(23) end
    end,
  }
end

-- the usher of the waiting room (ours from here on, owner 2026-11-11: as
-- the Wizard test's Helper). jAthena's 案内要員 holds a chat room and says
-- nothing; rAthena names him Pyorgin
npc {
  map = MAP, x = 32, y = 152, dir = 4, sprite = 69, name = "Pyorgin",
  src = SRC .. "597",
  on_click = function()
    if v.change_dancer ~= 8 or not candidate() then
      page("[Pyorgin]", "Candidates wait here for", "their audition.", "Those who are curious may", "watch from backstage.")
      return close()
    end
    if tmp.dance_lounge == 1 then
      page("[Pyorgin]", "Rested? Nobody is in front", "of you. Shall I take you", "to the stage?")
    else
      page("[Pyorgin]", "Candidates wait their turn", "here, and dance one by one.")
      next()
      page("[Pyorgin]", "...But there is nobody in", "front of you today.", "Shall I take you to the stage?")
    end
    next()
    if menu("I am ready.", "I'd like to lounge a bit first.") ~= 1 then
      tmp.dance_lounge = 1
      page("[Pyorgin]", "Take your time. Tell me", "when you are ready.")
      return close()
    end
    tmp.dance_lounge = 0
    close()
    reset()
    warp("job_duncer", 70, 112)
    npc_timer_start("dance_audition")
  end,
}
