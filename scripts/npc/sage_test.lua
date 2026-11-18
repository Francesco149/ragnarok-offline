-- The Sage test: Juno's Schweicherbil Magic Academy takes a Mage of Job
-- Level 40 as a student: enrolment and its fee, a written test of twenty
-- questions, a practical test of three fights against the clock, a course
-- of study with one of two professors (monsters or elemental magic), a
-- thesis, and the Dean's verdict on it.
--
-- Behind `jobs2_2` (euRO opened the 2-2 jobs on 2004-06-09).
--
-- Sources:
--   * jAthena //1046 script/npc/job/npc_job_16sage.txt (2004-09-12,
--     data:refs/jathena commit 6332410; Japanese): the flow, the NPCs, their
--     cells and sprites, the quest variables `job2nd_sage`, `job2nd_sage2`
--     and `sagecheck` with their steps, the fees and items, the three sets
--     of twenty questions with their answers and pass mark, the arena's
--     three fights (monsters, names, cells, time limits, announcements),
--     how the course is chosen, both courses' errands and lectures, both
--     theses, the Book. Tier athena2003 (a period emulator, months after
--     our date). Dialogue here is our English of its Japanese, cut to the
--     window.
--   * jAthena //1071 (2004-12-04, commit 789add7): Metheus turns away a
--     Mage with skill points unspent.
--   * rAthena npc/jobs/2-2/sage.txt (Aegis-based, tier later): the English
--     names (Kayron Grik, Metheus Sylphe, Claytos Verdo, Hermes Tris,
--     Saphien Layless, Lucius Celsus, Aebecee George; "Grade F"/"Grade D"
--     for the first fight's monsters). Its test differs (three courses,
--     the History Professor's among them); we keep jAthena's.
--   * eAthena 1.0 npc/jobs/2-2/sage.txt (2004-11-04, a translation of
--     jAthena's): a second reading of the Japanese, nothing taken from its
--     reworked questions.
--
-- Where we part from jAthena (docs/DEVIATIONS.md):
--   * no waiting room. jAthena seats candidates in an NPC chat room and
--     sends them in one at a time; here the Test Helper sends you in when
--     you are ready, or lets you lounge first (owner, 2026-11-11);
--   * the fee waiver asks for both the Old Magic Book and the Necklace of
--     Wisdom it names (jAthena's second check takes either, then deletes
--     both); a student already enrolled is not told at signing that Job
--     Level 50 waives the fee they just paid;
--   * the third fight's "too slow" mark reads its own clock (jAthena reads
--     the second fight's stopped one);
--   * the elemental thesis' seventh line writes what you chose (jAthena
--     repeats the sixth's lines);
--   * the quest variables are cleared at the job change;
--   * 817's academy staff (cities/yuno.txt, behind `juno`) and its Cleitos
--     Berdo, who made a Mage a Sage at once (jobs/sage.txt), give way to
--     jAthena's.
-- jAthena marks how Hermes picks the course as unknown ("判定条件不明"):
-- `sagecheck` starts at 10 with the written test, gains a point for full
-- marks and for each fight won fast, loses one or two for each won late;
-- 12 or more sends you to elemental magic, else to monsters.

if not content("jobs2_2") then return end

local SRC = "jathena-1046:script/npc/job/npc_job_16sage.txt:"
local IN = "yuno_in03"
local ARENA = "job_sage"

local STONE, BOOK = 7049, 1550
local CRYSTAL_BLUE, WIND_OF_VERDURE, GREEN_LIVE = 991, 992, 993
local CRYSTAL_ARROW, ARROW_OF_WIND, STONE_ARROW = 1754, 1755, 1756
local HOLY_WATER = 523
local OLD_MAGIC_BOOK, NECKLACE_OF_WISDOM = 1006, 1007
-- the thesis: a quill, a page, a binding, the ink and its bottle
local THESIS = { { 916, "for a quill" }, { 919, "to write on" }, { 1019, "for the binding" }, { 1024, "for ink" }, { 713, "for the ink" } }
-- the cheaper fee: 30,000 Zeny and fifty each of these
local FEE_ITEMS = { 916, 914, 705, 949 }

local function page(who, ...)
  mes(who)
  for _, line in ipairs({ ... }) do mes(line) end
end

local function blue(n, id) return "^3051FD" .. item_name(id) .. "^000000 " .. n end

local function has_all(list, n)
  for _, it in ipairs(list) do
    if count_item(type(it) == "table" and it[1] or it) < n then return false end
  end
  return true
end

local function take_all(list, n)
  for _, it in ipairs(list) do take_item(type(it) == "table" and it[1] or it, n) end
end

local function thesis_list(who)
  page(who, blue(1, 916) .. " for a quill,", blue(1, 919) .. " to write on,", blue(1, 1019) .. " to bind it,",
    blue(1, 1024) .. " for ink,", blue(1, 713) .. " to hold it.")
end

local function mage() return player.class == Job_Mage end

-- a replaced 817 NPC: the academy staff live in cities/yuno.txt, behind `juno`
local function r817(src) return content("juno") and src or nil end

local function dots()
  mes("......") next()
  mes("..........") next()
  mes("...............") next()
end

-- ---------------------------------------------------------------- the Dean

local K = "[Kayron Grik]"
npc {
  map = "yuno_in02", x = 38, y = 61, dir = 4, sprite = 743, name = "Kayron Grik",
  src = SRC .. "4",
  on_click = function()
    if player.class == Job_Sage then
      page(K, "Well, what is it? Being a", "Sage is no reason to stop", "studying.")
      next()
      page(K, "Our knowledge does much for", "this continent. Keep up your", "research and your records,", "for the good of everyone.")
      return close()
    end
    if not mage() then
      page(K, "Hmm... we study this world", "day after day, and still one", "thing is lacking: the", "knowledge that comes of other", "callings' experience...")
      next()
      page(K, "If you know a Sage, tell them", "what you have lived through.", "They will help you in turn.")
      return close()
    end
    if v.job2nd_sage == 11 then
      if count_item(BOOK) < 1 then
        page(K, "Hm? Where is your thesis?", "If you want me to read it,", "bring it here.")
        next()
        page(K, "You haven't forgotten it,", "surely? Well, I shall wait.")
        return close()
      end
      page(K, "So, you have worked hard on", "your thesis. Let me see it.")
      next()
      page(K, "Hm...") next()
      page(K, "Hmm...") next()
      page(K, "Hmmm...") next()
      page(K, "Your style is still rough...", "but your thesis shows a real", "zeal for research.", "Very well.")
      next()
      job_change(Job_Sage)
      v.job2nd_sage, v.job2nd_sage2, v.sagecheck = 0, 0, 0
      page(K, "You graduate!", "Live as a Sage from now on,", "and never lose your zeal", "for learning.")
      next()
      page(K, "Keep your thesis safe: it is", "the one book you have written.", "It may serve you one day.")
      next()
      page(K, "Now go, and become a", "treasury of knowledge", "yourself!")
      return close()
    end
    if v.job2nd_sage >= 3 then
      page(K, "Ha ha, the road to Sage is", "never easy. Using magic well", "is not enough.")
      next()
      page(K, "Work your way to your thesis.", "I shall be waiting.")
      return close()
    end
    page(K, "I welcome you, who have a", "passion for magic.", "What brings you here?")
    next()
    local c = menu("I want to change to Sage.", "Tell me about becoming a Sage.", "Nothing.")
    if c == 1 then
      page(K, "Hm... change to Sage, you say?", "Then that idea of a 'change'", "is the first thing to go.")
      next()
      page(K, "A Sage is one who serves the", "continent by endless study and", "record of the world. Changing", "your clothes and your looks", "means nothing at all.")
      next()
      page(K, "First, apply to the", "Schweicherbil Magic Academy.", "They will tell you all there.")
      return close()
    elseif c == 2 then
      page(K, "Becoming a Sage... Nobody", "calls it a 'job change'.")
      next()
      page(K, "To be a Sage, first apply to", "the Schweicherbil Magic", "Academy and sit its entrance", "examination.")
      next()
      page(K, "Then you study your subject", "and, at the end, hand in", "a thesis.")
      next()
      page(K, "I read the theses and judge", "them. Pass, and you earn the", "right to live as a Sage.")
      next()
      page(K, "Go to the Academy first, and", "apply to a young Sage there", "named Metheus Sylphe.")
      return close()
    end
    page(K, "Hm... if you have time to", "spare, read a book. Most", "things in this world can be", "learned from books.")
    next()
    page(K, "Not everything, of course.", "Still, we Sages treasure books", "as storehouses of knowledge.")
    close()
  end,
}

-- --------------------------------------------------------- the enrolment

local M = "[Metheus Sylphe]"
local function fee_items_page()
  page(M, "Bring me the following:", blue(50, 916), blue(50, 914), blue(50, 705), blue(50, 949))
end

-- jAthena //1071 (2004-12-04): no enrolment with skill points unspent
local function points_left()
  if player.skill_point == 0 then return false end
  page(M, "You seem to have skill points", "left over. Please use them all", "up and come back.")
  close()
  return true
end

-- the student signs; then, unless enrolled already, the fee
local function sign()
  page(M, "Then I'll draw up your", "application. Sign here,", "please.")
  next()
  menu(player.name)
  page(M, "Your name is...", player.name .. ".", "A good name.")
  next()
  if v.job2nd_sage ~= 3 then
    if player.job_level == 50 then
      v.job2nd_sage = 3
      page(M, "Oh, you have trained your Mage", "to the highest level! Then the", "fee is waived, as a special", "case.")
      next()
      page(M, "There, that's all done.", "Now go and sit the entrance", "examination.")
      next()
      page(M, "Go to Professor Claytos, in", "the room at the back on the", "left.")
      return close()
    end
    if count_item(OLD_MAGIC_BOOK) >= 1 and count_item(NECKLACE_OF_WISDOM) >= 1 then
      take_item(OLD_MAGIC_BOOK, 1)
      take_item(NECKLACE_OF_WISDOM, 1)
      page(M, "I have received the items.", "You are enrolled.")
    elseif player.zeny >= 70000 then
      player.zeny = player.zeny - 70000
      page(M, "I have received 70,000 Zeny.", "You are enrolled.")
    else
      page(M, "...The fee is not enough.", "Please come back with it.")
      return close()
    end
    v.job2nd_sage = 3
    next()
  end
  page(M, "First you will sit Professor", "Claytos' written test.")
  next()
  page(M, "Professor Claytos is in the", "Academy's library: the room", "at the back on the upper left.", "Please go there.")
  next()
  page(M, "Good luck.")
  close()
end

npc {
  map = IN, x = 154, y = 35, dir = 4, sprite = 742, name = "Metheus Sylphe",
  replaces = r817("817:cities/yuno.txt:266"), src = SRC .. "1078",
  on_click = function()
    local q = v.job2nd_sage
    if player.class == Job_Sage then
      page(M, "Welcome. It's been a while.", "Is your research going well?")
      next()
      page(M, "Books and papers are fine, but", "now and then you must face", "monsters yourself, out there.")
      next()
      page(M, "If you meet a Mage who wants", "to be a Sage, teach them well.", "And give my regards to your", "colleagues.")
      return close()
    end
    if not mage() then
      page(M, "Welcome. This is the", "Schweicherbil Magic Academy.")
      next()
      page(M, "We train Sages here, and do", "research on monsters and", "magic.")
      next()
      page(M, "Any Mage of Job Level 40 or", "above may enrol, and become a", "Sage by our course. If you", "know such a Mage, tell them.")
      next()
      page(M, "Have a good day.")
      return close()
    end
    if q == 11 then
      page(M, "Oh, your thesis is done?", "Then take it to Dean Kayron.")
      next()
      page(M, "I hope your work bears fruit.", "Have a good day.")
      return close()
    end
    if q >= 4 then
      page(M, "Ah, I'm so sorry, we're rather", "busy at the moment...", "Please come back later.")
      return close()
    end
    if q == 3 then
      page(M, "Hm? What are you doing?", "The test is starting. Go to", "Professor Claytos, in the room", "at the back on the left.")
      return close()
    end
    if q == 2 then
      page(M, "Welcome. You are here to", "enrol?")
      next()
      if player.job_level < 40 then
        page(M, "Your Job Level is not enough", "yet. Please study a little", "more and come again.")
        return close()
      end
      if points_left() then return end
      if count_item(OLD_MAGIC_BOOK) >= 1 and count_item(NECKLACE_OF_WISDOM) >= 1 then
        take_item(OLD_MAGIC_BOOK, 1)
        take_item(NECKLACE_OF_WISDOM, 1)
        v.job2nd_sage = 3
        page(M, "I have received the items.", "You are enrolled.")
        next()
        return sign()
      end
      if player.zeny >= 70000 then
        player.zeny = player.zeny - 70000
        v.job2nd_sage = 3
        page(M, "I have received 70,000 Zeny.", "You are enrolled.")
        next()
        return sign()
      end
      if not has_all(FEE_ITEMS, 50) then
        page(M, "But you are not ready yet.", "Let me go over the items", "again.")
        next()
        fee_items_page()
        next()
        page(M, "Bring those, and the fee is", "30,000 Zeny. Do your best.", "Have a good day.")
        return close()
      end
      if player.zeny < 30000 then
        page(M, "...The fee is not enough.", "Please come back with it.")
        return close()
      end
      take_all(FEE_ITEMS, 50)
      player.zeny = player.zeny - 30000
      v.job2nd_sage = 3
      page(M, "I have received the items and", "30,000 Zeny.", "You are enrolled.")
      next()
      return sign()
    end
    page(M, "Welcome. This is the", "Schweicherbil Magic Academy.", "You are a Mage, I see.", "What brings you here?")
    next()
    local c = menu("Tell me about becoming a Sage.", "I want to enrol.", "Nothing.")
    if c == 1 then
      page(M, "You want to become a Sage?", "But we don't change anyone's", "job here.")
      next()
      page(M, "Those who enrol and finish the", "set course are recognised", "as researchers.")
      next()
      page(M, "So we don't call it a job", "change, but graduation. Enrol", "and pay the fee, and you may", "sit the tests.")
      next()
      page(M, "As a special case, those who", "bring an " .. item_name(OLD_MAGIC_BOOK), "and a " .. item_name(NECKLACE_OF_WISDOM), "pay no fee.")
      next()
      page(M, "Once enrolled, you sit the", "tests. Pass them, and you", "study a subject; at the end", "you write a thesis on it.")
      next()
      page(M, "The Dean reads the thesis and", "decides. Pass, and you may", "work as a Sage.")
      next()
      page(M, "Come and enrol any time.", "Have a good day.")
      return close()
    elseif c == 3 then
      page(M, "I see. Do look around as long", "as you like. Have a good day.")
      return close()
    end
    page(M, "You wish to enrol. Welcome to", "the Schweicherbil Magic", "Academy.")
    next()
    page(M, "You must be a Mage of Job", "Level 40 or more. And you pay", "the costs of the course as", "an enrolment fee.")
    next()
    page(M, "The fee is 70,000 Zeny. Or", "hand in an " .. item_name(OLD_MAGIC_BOOK), "and a " .. item_name(NECKLACE_OF_WISDOM), "and it is waived.")
    next()
    page(M, "Will you enrol now?")
    next()
    c = menu("I'll enrol.", "The fee is too high.", "I'll come back later.")
    if c == 1 then
      if player.job_level < 40 then
        page(M, "Your Job Level is not enough", "yet. Please study a little", "more and come again.")
        return close()
      end
      if points_left() then return end
      return sign()
    elseif c == 3 then
      page(M, "Of course, take your time to", "get ready. Have a good day.")
      return close()
    end
    page(M, "Hm... not enough money?", "The rules say 70,000 Zeny...")
    next()
    if menu("Is there no other way?", "I'll come back later.") ~= 1 then
      page(M, "Of course, take your time to", "get ready. Have a good day.")
      return close()
    end
    v.job2nd_sage = 2
    page(M, "Then I'll bend a little: the", "fee comes down to 30,000 Zeny,", "if you bring some items.")
    next()
    fee_items_page()
    next()
    page(M, "Bring those, and the fee is", "30,000 Zeny. That should be", "possible. Do your best.")
    next()
    page(M, "Of course, if you have 70,000", "Zeny before then, you pay", "that. Rules are rules.")
    next()
    page(M, "Good luck with them, then.", "Have a good day.")
    close()
  end,
}

-- ------------------------------------------------------- the written test

local C = "[Claytos Verdo]"
-- SETS: filled in below, three sets of twenty { lines, choices, right }
local SETS

local function written_test()
  local set = SETS[rand(1, #SETS)]
  local score = 0
  for i, q in ipairs(set) do
    mes(i .. ".")
    for _, line in ipairs(q[1]) do mes(line) end
    next()
    if menu(table.unpack(q[2])) == q[3] then score = score + 5 end
  end
  page(C, "Right, that's enough.", "Let's mark it.")
  next()
  page(C, "Let's see...", "Hm...")
  next()
  if score < 80 then
    v.sagecheck = v.sagecheck - 1
    page(C, "Done. You scored " .. score .. ".", "...", "I'm sorry, you have failed.")
    next()
    page(C, "You may sit it again. Learn a", "little more about the world", "first.")
    return close()
  end
  if score == 100 then
    v.sagecheck = v.sagecheck + 1
    page(C, "Done. You scored " .. score .. ".", "I am impressed. With that,", "you have all it takes to be", "a Sage.")
  else
    page(C, "Done. You scored " .. score .. ".", "Hm. That will do.")
  end
  next()
  v.job2nd_sage = 5
  page(C, "You have passed the written", "test. Go to Professor Hermes", "for the practical one.")
  close()
end

npc {
  map = IN, x = 105, y = 177, dir = 4, sprite = 754, name = "Claytos Verdo",
  replaces = "817:jobs/sage.txt:6", src = SRC .. "1699",
  on_click = function()
    local q = v.job2nd_sage
    if player.class == Job_Sage then
      page(C, "Hm? What does a graduate want", "here? To enrol again?")
      next()
      page(C, "We passed you: be sure of", "yourself. Missing your school", "is fine, but don't linger.", "Go on a dangerous journey", "now and then.")
      next()
      page(C, "Keep a record of all you see,", "and teach what is useful", "to others.")
      return close()
    end
    if not mage() then
      page(C, "Hm... one quite taken with", "magic. That is not so bad...")
      next()
      page(C, "But remember: no one lives", "alone. What you have learned", "shows its worth when you", "teach it to others.")
      return close()
    end
    if q == 11 then
      page(C, "Ah, you finished your thesis.", "But I don't mark theses.")
      next()
      page(C, "Show it to Dean Kayron. He", "decides whether you graduate.")
      return close()
    end
    if q >= 6 then
      page(C, "I'm marking the other", "students' tests, I'm busy...", "Sorry, come back later.")
      return close()
    end
    if q == 5 then
      page(C, "Hm? You want to sit it again?", "You have passed the written", "test.")
      next()
      page(C, "Go to Professor Hermes for the", "practical test. Don't dawdle.")
      return close()
    end
    if q == 4 then
      page(C, "Then let's have the retest.", "Have you studied properly", "this time?")
      next()
      page(C, "The pass mark is the same:", "twenty questions at five", "points each, 80 to pass.")
      next()
      page(C, "Let's begin. Answer at once,", "or the test is over.", "Good luck.")
      next()
      return written_test()
    end
    if q == 3 then
      v.job2nd_sage = 4
      v.sagecheck = 10
      page(C, "Welcome to the Schweicherbil", "Academy. You have enrolled,", "I take it?")
      next()
      page(C, "Let me see. Your name is...", player.name .. ",", "is it? Good, let's begin", "the test at once.")
      next()
      page(C, "My test is on everything in", "this world. Twenty questions,", "five points each; 80 or more", "passes.")
      next()
      page(C, "Answer at once, or the test is", "over. Let's begin.")
      next()
      return written_test()
    end
    page(C, "Hm... you want to be a Sage.", "It's written on your face.")
    next()
    page(C, "For that you must enrol in our", "Academy. Come back when you", "have done so.")
    close()
  end,
}

-- ------------------------------------------------------ the practical test

local H = "[Hermes Tris]"
local function to_arena()
  page(H, "Good, let's start at once.", "Do your best.")
  close()
  warp("job_sage", 50, 154)
end

npc {
  map = IN, x = 169, y = 180, dir = 4, sprite = 755, name = "Hermes Tris",
  replaces = r817("817:cities/yuno.txt:281"), src = SRC .. "1453",
  on_click = function()
    local q = v.job2nd_sage
    if player.class == Job_Sage then
      page(H, "Hello, how are you? You look", "as if you've been through", "a good deal.")
      next()
      page(H, "Experience matters. Knowing a", "thing and living it are worlds", "apart.")
      next()
      page(H, "But take friends you can trust", "into the dangerous dungeons.")
      return close()
    end
    if not mage() then
      page(H, "This is the Schweicherbil", "Magic Academy.")
      next()
      page(H, "We Sages are more scholars", "than sorcerers. Handy in a", "party too, I'd say.")
      next()
      page(H, "Take a Sage on your next", "journey and see. They will", "do their part.")
      return close()
    end
    if q == 5 then
      v.job2nd_sage = 6
      page(H, "Ah, a student who has passed", "the written test? Then you", "sit the practical one now.")
      next()
      page(H, "The rules are simple: defeat", "every monster that appears", "within the time limit.")
      next()
      page(H, "Better to try once than hear", "a hundred times, as they say.", "Will you sit it now?")
      next()
      if menu("I'll take the test.", "Let me get ready.") == 1 then return to_arena() end
      page(H, "Of course. Get ready properly.")
      return close()
    end
    if q == 6 then
      page(H, "So, are you ready this time?", "The practical test is not that", "hard. Do your best.")
      next()
      page(H, "Will you sit it now?")
      next()
      if menu("I'll take the test.", "Give me time to get ready.") == 1 then return to_arena() end
      page(H, "Of course. Get ready properly.")
      return close()
    end
    if q == 7 then
      page(H, "Well done. You have passed the", "practical test too. Welcome", "to the Academy.")
      next()
      page(H, "Now, your field of research...", "Wait a moment. Let me see your", "written score and the time", "the fights took you.")
      next()
      page(H, "Hm...", "Good, this will do.")
      next()
      if v.sagecheck >= 12 then
        v.job2nd_sage = 9
        page(H, "You will research elemental", "magic: a deeper grasp of the", "basic magic we all use.")
        next()
        page(H, "A good field for seeing what", "magic really is. Hear", "Professor Aebecee George at", "the Biotechnology Laboratory.")
      else
        v.job2nd_sage = 8
        page(H, "You will research monsters:", "the monsters found all over", "the continent.")
        next()
        page(H, "You will learn what you need", "to stay alive out there. Hear", "Professor Lucius Celsus at", "the Monster Museum.")
      end
      next()
      page(H, "I wish you good results.")
      return close()
    end
    if q == 8 then
      page(H, "Hm? Weren't you listening?", "I said you would research", "monsters.")
      next()
      page(H, "Go to Professor Lucius Celsus", "at the Monster Museum.")
      return close()
    end
    if q == 9 then
      page(H, "Hm? Weren't you listening?", "I said you would research", "elemental magic.")
      next()
      page(H, "Go to Professor Aebecee George", "at the Biotechnology", "Laboratory.")
      return close()
    end
    if q == 11 then
      page(H, "What are you idling here for?", "If your thesis is done, show", "it to the Dean. I'd read it,", "but he decides who graduates.")
      next()
      page(H, "Go on, off with you.")
      return close()
    end
    page(H, "I am Hermes, in charge of the", "practical test. A student who", "wants to be a Sage?")
    next()
    if q >= 3 then
      page(H, "Pass Professor Claytos'", "written test first. Then you", "sit mine.")
    else
      page(H, "Finish your enrolment first,", "then sit the written test.")
    end
    close()
  end,
}

-- The arena: three fights in the ring (100-131, 82-113), each against the
-- clock. This client draws `job_sage` with `job_wiz`'s map (resnametable);
-- its ring is the same sealed room, every spawn cell floor. A fight won in under a minute (20 s for the last) raises
-- `sagecheck`; one won late lowers it.
local RING = { 100, 82, 131, 113 }
local FAIL = { "yuno", 324, 258 }

local function arena_say(text) map_announce(ARENA, text, 0) end

local function ring_warp(map, x, y) area_warp(ARENA, RING[1], RING[2], RING[3], RING[4], map, x, y) end

-- the one in the ring, for the timers' announcements (no player attached)
local candidate = ""

local FIGHTS = {
  { -- jAthena 2364: eight Chonchons and eight Fabres, "credits", 3 minutes
    id = "sage_fight1", src = 2364, limit = 180000, fast = 60000, slow = 120000, late = 170000,
    spawn = function(ev)
      for _, c in ipairs { { 115, 106 }, { 120, 102 }, { 124, 98 }, { 120, 93 }, { 115, 90 }, { 111, 93 }, { 107, 98 }, { 111, 102 } } do
        monster(ARENA, c[1], c[2], "Grade F", 1183, 1, ev)
      end
      for _, c in ipairs { { 128, 110 }, { 124, 106 }, { 124, 89 }, { 128, 85 }, { 107, 89 }, { 103, 85 }, { 107, 106 }, { 103, 110 } } do
        monster(ARENA, c[1], c[2], "Grade D", 1184, 1, ev)
      end
      return 16
    end,
    say = { [1000] = "Arena: The practical entrance test begins.", [2500] = "Arena: First test - time limit 3 minutes",
      [4000] = "Arena: Defeat every monster within the time limit." },
  },
  { -- jAthena 2445: twenty-four Lunatics named for the sciences, 3 minutes
    id = "sage_fight2", src = 2445, limit = 180000, fast = 60000, slow = 120000, late = 170000,
    spawn = function(ev)
      for _, g in ipairs {
        { 120, 102, "Geography", "Statistics", "Meteorology", "Astronomy", "Linguistics", "Urban Studies" },
        { 124, 98, "Forestry", "Hygiene", "Psychology" },
        { 120, 93, "Anthropology", "Biology", "Ethics" },
        { 111, 93, "Architecture", "Design", "Nutrition", "Food Science", "Control", "Measurement" },
        { 107, 98, "Economics", "Magic", "Politics" },
        { 111, 102, "Mathematics", "History", "Physics" },
      } do
        for i = 3, #g do monster(ARENA, g[1], g[2], g[i], 1063, 1, ev) end
      end
      return 24
    end,
    say = { [1500] = "Arena: Second test - time limit 3 minutes", [3000] = "Arena: Defeat every monster within the time limit." },
  },
  { -- jAthena 2533: the Bachelor's Degree, a Whisper among four others, 1 minute
    id = "sage_fight3", src = 2533, limit = 60000, fast = 20000, slow = 30000, late = 50000,
    spawn = function(ev)
      monster(ARENA, 116, 98, "Bachelor's Degree", 1179, 1, ev)
      monster(ARENA, 124, 106, "Lateness", 1185, 1, "sage_ring::OnNone")
      monster(ARENA, 124, 89, "Overdue Books", 1185, 1, "sage_ring::OnNone")
      monster(ARENA, 107, 89, "Cribbing", 1185, 1, "sage_ring::OnNone")
      monster(ARENA, 107, 106, "Absence", 1185, 1, "sage_ring::OnNone")
      return 1
    end,
    say = { [1500] = "Arena: Third test - time limit 1 minute" },
  },
}
-- the countdown: "N remaining" at jAthena's times
for _, f in ipairs { FIGHTS[1], FIGHTS[2] } do
  f.say[30000] = "Arena: 2 minutes 30 seconds left"
  f.say[60000] = "Arena: 2 minutes left"
  f.say[90000] = "Arena: 1 minute 30 seconds left"
  f.say[120000] = "Arena: 1 minute left"
  f.say[150000] = "Arena: 30 seconds left"
  f.say[170000] = "Arena: 10 seconds left"
end
FIGHTS[3].say[30000] = "Arena: 30 seconds left"
FIGHTS[3].say[50000] = "Arena: 10 seconds left"

local function stop_all()
  for _, f in ipairs(FIGHTS) do npc_timer_stop(f.id) end
  npc_timer_stop("sage_passed")
  kill_monster(ARENA, "All")
end

local start = function(n) end

for n, f in ipairs(FIGHTS) do
  local labels = { "OnKilled" }
  for ms in pairs(f.say) do labels[#labels + 1] = "OnTimer" .. ms end
  labels[#labels + 1] = "OnTimer" .. f.limit
  labels[#labels + 1] = "OnTimer" .. (f.limit + 1500)
  f.spawn_event = f.id .. "::OnKilled"
  npc {
    map = "-", name = f.id, sprite = -1, events = labels, src = SRC .. f.src,
    on_click = function(label)
      if label == "OnKilled" then
        if v.job2nd_sage ~= 6 then return end
        tmp.sage_left = tmp.sage_left - 1
        if tmp.sage_left > 0 then return end
        local took = _rt("now") - tmp.sage_started
        npc_timer_stop(f.id)
        if took < f.fast then
          v.sagecheck = v.sagecheck + 1
        elseif took > f.late then
          v.sagecheck = v.sagecheck - 2
        elseif took >= f.slow then
          v.sagecheck = v.sagecheck - 1
        end
        if n < #FIGHTS then
          arena_say("Arena: " .. player.name .. " has defeated every monster.")
          return start(n + 1)
        end
        kill_monster(ARENA, "All")
        v.job2nd_sage = 7
        arena_say("Arena: Congratulations, " .. player.name .. ", you have passed the practical test.")
        return npc_timer_start("sage_passed")
      end
      local ms = tonumber(label:match("^OnTimer(%d+)$"))
      if not ms then return end
      if f.say[ms] then arena_say(f.say[ms]) end
      if ms == f.limit then
        kill_monster(ARENA, "All")
        arena_say("Arena: Time is up.")
      elseif ms == f.limit + 1500 then
        npc_timer_stop(f.id)
        arena_say("Arena: " .. candidate .. " has failed the test.")
        ring_warp(FAIL[1], FAIL[2], FAIL[3])
      end
    end,
  }
end

start = function(n)
  local f = FIGHTS[n]
  kill_monster(ARENA, "All")
  candidate = player.name
  tmp.sage_left = f.spawn(f.spawn_event)
  tmp.sage_started = _rt("now")
  npc_timer_start(f.id)
end

-- the Whispers that are not the degree: nothing happens when one dies
npc { map = "-", name = "sage_ring", sprite = -1, events = { "OnNone" }, src = SRC .. "2539", on_click = function() end }

-- passed: 6 s later, back to the Academy (jAthena 2587)
npc {
  map = "-", name = "sage_passed", sprite = -1, events = { "OnTimer3000", "OnTimer6000" }, src = SRC .. "2587",
  on_click = function(label)
    if label == "OnTimer3000" then
      arena_say("Examiner: Do your best in the rest of the course.")
    elseif label == "OnTimer6000" then
      npc_timer_stop("sage_passed")
      arena_say("Examiner: That ends the test.")
      ring_warp(IN, 163, 180)
    end
  end,
}

-- the Test Helper by the entrance (jAthena's 実技試験担当, who keeps a
-- waiting room; ours from "Nobody is waiting" on, as the Wizard test's
-- Helper)
local TH = "[Test Helper]"
npc {
  map = ARENA, x = 50, y = 165, dir = 4, sprite = 700, name = "Test Helper",
  src = SRC .. "2326",
  on_click = function()
    page(TH, "Welcome to the Sage practical", "test.")
    next()
    if v.job2nd_sage ~= 6 or not mage() then
      page(TH, "Each candidate takes five to", "ten minutes. Students wait", "their turn here.")
      next()
      page(TH, "If you have no test today,", "I'll send you back to the", "Academy.")
      next()
      if menu("Back to the Academy.", "I'll stay a while.") == 1 then
        close()
        return warp("yuno_in03", 163, 180)
      end
      return close()
    end
    if tmp.sage_lounge == 1 then
      page(TH, "Rested? Nobody is in front of", "you. Shall I send you in?")
    else
      page(TH, "Candidates wait their turn", "here and fight one at a time,", "five to ten minutes each.")
      next()
      page(TH, "...But nobody is waiting", "today. Shall I send you in?")
    end
    next()
    local c = menu("I am ready.", "I'd like to lounge a bit first.", "Back to the Academy.")
    if c == 3 then
      close()
      return warp("yuno_in03", 163, 180)
    elseif c ~= 1 then
      tmp.sage_lounge = 1
      page(TH, "Take your time. Tell me when", "you are ready.")
      return close()
    end
    tmp.sage_lounge = 0
    close()
    stop_all()
    warp("job_sage", 118, 99)
    start(1)
  end,
}

-- --------------------------------------------------- the monster course

local L = "[Lucius Celsus]"
-- the three errands of each lesson: { item, count } x3, picked at random
local SEA = { -- jAthena 719-737
  { { 1050, 5 }, { 960, 5 }, { 963, 5 } },
  { { 966, 5 }, { 960, 5 }, { 950, 5 } },
  { { 1052, 5 }, { 962, 5 }, { 1023, 5 } },
}
local INSECTS = { -- jAthena 857-875
  { { 1025, 5 }, { 935, 5 }, { 928, 5 } },
  { { 1057, 5 }, { 946, 5 }, { 947, 5 } },
  { { 1031, 5 }, { 955, 5 }, { 1013, 5 } },
}
local function errand(set)
  for _, it in ipairs(set) do mes(blue(it[2], it[1])) end
end
local function errand_done(set)
  for _, it in ipairs(set) do if count_item(it[1]) < it[2] then return false end end
  return true
end
local function errand_take(set)
  for _, it in ipairs(set) do take_item(it[1], it[2]) end
end

npc {
  map = IN, x = 32, y = 102, dir = 0, sprite = 755, name = "Lucius Celsus",
  replaces = r817("817:cities/yuno.txt:242"), src = SRC .. "631",
  on_click = function()
    local q, q2 = v.job2nd_sage, v.job2nd_sage2
    if player.class == Job_Sage then
      page(L, "Something unclear? Book a", "question a week ahead. I'm", "busy, you know.")
      next()
      page(L, "You know how busy I am. I'd", "have thought a Sage would.")
      next()
      page(L, "If you have time, go into a", "dungeon. Even lying on the", "floor staring at the ceiling", "teaches you something.")
      return close()
    end
    if not mage() then
      page(L, "Hm? Come to look at the", "monsters?")
      next()
      page(L, "Look all you like, but don't", "touch anything. Some of it", "is dangerous.")
      next()
      page(L, "And if you catch a rare", "monster, do tell me. I pay", "well for the information.")
      return close()
    end
    if q == 11 then
      page(L, "What are you doing? I told you", "to show the Dean your thesis.", "Stop loafing about here.")
      return close()
    end
    if q2 >= 1 and q2 <= 3 then
      local set = SEA[q2]
      if not errand_done(set) then
        page(L, "Come on, pull yourself", "together. I'll say it once", "more, so listen.")
        next()
        mes(L) errand(set)
        return close()
      end
      page(L, "Ah, well done. Whether you", "picked them up or bought them,", "I can't tell.")
      next()
      page(L, "Anyway, the monsters that drop", "these have something in", "common. Do you know what?")
      next()
      local c = menu("They are Water.", "They are fish and shellfish.", "They are aggressive.", "They are monsters.")
      if c <= 2 then
        page(L, "Yes: Water, and fish and", "shellfish too. Most fish and", "shellfish live in the water,", "so they are Water.")
      else
        page(L, "...No. What they share is", "that they are Water, and fish", "and shellfish. Most of them", "live in the water, and grew", "Water to suit it.")
      end
      next()
      page(L, "Not every fish is Water, but", "most are. Now, which spell", "works on these fish and", "shellfish?")
      next()
      c = menu("Lightning Bolt.", "Fire Bolt.", "Thunderstorm.", "Frost Diver.")
      if c == 1 or c == 3 then
        page(L, "Right. Water falls to Wind", "magic, " .. (c == 1 and "Lightning Bolt." or "Thunderstorm."),
          c == 1 and "Mind the monsters that answer" or "And it strikes an area: it can",
          c == 1 and "a cast, though." or "do a lot, at the right time.")
        next()
        page(L, "But monsters like Penomena or", "Star Fish are fish and", "shellfish of another element.", "Watch out for them.")
      else
        page(L, "...No. Water falls to Wind", "magic: Lightning Bolt and", "Thunderstorm. Go back and", "study the elements.")
      end
      next()
      page(L, "Next, let us study insects.", "Now then...")
      next()
      local n = rand(1, 3)
      errand_take(set)
      v.job2nd_sage2 = 3 + n
      mes(L) mes("Bring me these next.") errand(INSECTS[n])
      next()
      page(L, "When you have them all, we go", "on. Off you go.")
      return close()
    end
    if q2 >= 4 and q2 <= 6 then
      local set = INSECTS[q2 - 3]
      if not errand_done(set) then
        page(L, "What, you forgot what I asked", "for? Fine, I'll say it again.")
        next()
        mes(L) errand(set)
        return close()
      end
      errand_take(set)
      v.job2nd_sage2 = 7
      page(L, "Good, you have them all. I hope", "you watched them too. That", "can't have been so hard.")
      next()
      page(L, "Insects come in all sorts of", "elements, so think again", "before you cast.")
      next()
      page(L, "Note this too: a Thief's", "Hiding and an Assassin's", "Cloaking don't fool them.")
      next()
      page(L, "Some insects move in swarms,", "a boss leading them like", "soldiers.")
      next()
      page(L, "Maya, the queen ant...", "Mistress, the queen bee...", "Golden Thief Bug, the", "thieves' boss...")
      next()
      page(L, "Face a boss like that alone,", "and be ready to die. Take", "friends when you go into", "their lairs.")
      next()
      page(L, "Enough talk: time to write", "your thesis. It's a formality,", "so let's get it done. First", "bring me these.")
      next()
      thesis_list(L)
      next()
      page(L, "Come back when you have them.", "Nearly there, keep it up.")
      return close()
    end
    if q2 == 7 then
      if not has_all(THESIS, 1) then
        page(L, "What, you're not ready? I'll", "say it again...")
        next()
        thesis_list(L)
        next()
        page(L, "You've come this far, this is", "nothing. Go on, get them.")
        return close()
      end
      take_all(THESIS, 1)
      page(L, "Good, you have them all.")
      next()
      page(L, "You may write it all yourself,", "but here is an example to", "follow.")
      next()
      page(L, "All right? What you have lived", "through already means plenty.", "Show me when you're done.")
      next()
      dots()
      for _, line in ipairs {
        "Monsters, by their race,", "have elements of all kinds.", "Know a monster's element",
        "and use your magic rightly.", "Most to be watched are", "the Holy and Shadow ones.",
      } do
        menu(line)
        mes(line)
      end
      next()
      dots()
      v.job2nd_sage = 11
      give_item(BOOK, 1)
      page(L, "Right, well done!", "Don't you dare lose this!")
      next()
      page(L, "The Dean reads it and decides", "whether you graduate. You did", "well to get this far.")
      return close()
    end
    if q == 8 then
      page(L, "So, you passed the written", "test. I am Lucius Celsus, at", "the forefront of biology.")
      next()
      page(L, "When someone gives their name,", "you give yours. That's", "manners. Your name?")
      next()
      menu("I am " .. player.name .. ".")
      page(L, "A good name. Pleased to meet", "you. Now, let me explain the", "field you're going into.")
      next()
      page(L, "As you know, your subject is", "monsters. Have you met and", "fought many of them?")
      next()
      if menu("Yes, I've tried hard.", "No, not really...") == 1 then
        page(L, "Tried hard? You mostly went", "to the easy places, I bet.", "Well, my lessons aren't easy.")
      else
        page(L, "I see... but my lessons aren't", "easy. You may have to go", "places you've never been.", "Go in with a fresh mind.")
      end
      next()
      page(L, "Let's begin. Remember: in my", "lessons you learn mostly", "out in the field.")
      next()
      local n = rand(1, 3)
      v.job2nd_sage2 = n
      mes(L) mes("Right, bring me these.") errand(SEA[n])
      next()
      page(L, "When you have them all, we go", "on. Off you go.")
      return close()
    end
    page(L, "Ugh, my head... far too much", "to prepare... What is it?", "I'm busy enough as it is!")
    next()
    page(L, "If you came to look, look!", "My head hurts as it is...", "Honestly... (mutter mutter)")
    close()
  end,
}

-- ------------------------------------------- the elemental magic course

local A = "[Aebecee George]"
local function arrows_page()
  mes(blue(50, CRYSTAL_ARROW)) mes(blue(50, STONE_ARROW)) mes(blue(50, ARROW_OF_WIND))
end
-- the elemental thesis (jAthena 438-555): ten lines, three choices each,
-- any will do; { line before, { { choice, lines written } x3 } }
local ELEMENTAL_THESIS = {
  { "Elemental magic has four", { { "Water, Earth, Fire, Wind", "elements: Water, Earth, Fire", "and Wind." },
    { "Earth, Water, Fire, Wind", "elements: Earth, Water, Fire", "and Wind." },
    { "Water, Wind, Earth, Fire", "elements: Water, Wind, Earth", "and Fire." } } },
  { "Each element has its traits:", { { "Wind beats Water", "Wind beats Water, and", "Earth beats Wind." },
    { "Water beats Fire", "Water beats Fire, and", "Wind beats Water." },
    { "Fire beats Earth", "Fire beats Earth, and", "Water beats Fire." } } },
  { nil, { { "But weaknesses exist too,", "But weaknesses exist too," },
    { "Elemental weapons the same,", "Elemental weapons the same," },
    { "Monsters' elements vary,", "Monsters' elements vary," } } },
  { nil, { { "so know your own powers.", "so know your own powers." },
    { "so change with the place.", "so change with the place." },
    { "Red Potions taste of berries.", "Red Potions taste of berries." } } },
  { "next", { { "Prontera's Amy is prettiest,", "Prontera's Amy is prettiest," },
    { "Red Potions come from Porings,", "Red Potions come from Porings," },
    { "Magic's depths pass our ken,", "Magic's depths pass our ken," } } },
  { nil, { { "Morroc's merchant girl too.", "Morroc's merchant girl too." },
    { "but why berries, who knows.", "but why berries, who knows." },
    { "and blind trust is dangerous.", "and blind trust is dangerous." } } },
  { nil, { { "Give me a Bunny Band", "Give me a Bunny Band" },
    { "And what of White Potions?", "And what of White Potions?" },
    { "Magic rightly used, and rest,", "Magic rightly used, and rest," } } },
  { nil, { { "and I'll be happy.", "and I'll be happy." },
    { "It's hard to imagine.", "It's hard to imagine." },
    { "keep a battle safe.", "keep a battle safe." } } },
  { nil, { { "Bunny Bands suit Acolytes,", "Bunny Bands suit Acolytes," },
    { "I love the thrill of it,", "I love the thrill of it," },
    { "Party with other jobs,", "Party with other jobs," } } },
  { nil, { { "but do they suit Knights?", "but do they suit Knights?" },
    { "not even at 1 HP.", "not even at 1 HP." },
    { "and go to battle so.", "and go to battle so." } } },
}

npc {
  map = IN, x = 244, y = 31, dir = 3, sprite = 120, name = "Aebecee George",
  replaces = r817("817:cities/yuno.txt:311"), src = SRC .. "165",
  on_click = function()
    local q, q2 = v.job2nd_sage, v.job2nd_sage2
    if player.class == Job_Sage then
      page(A, "Hyohoho, hello. What brings", "you? ...Oh, a Sage!", "How nice to see you~")
      next()
      page(A, "Killing time here, are you~?", "Well, better than those who", "drown in magic, hyohoho.")
      next()
      page(A, "What, you miss your old school", "now and then? Don't say that,", "come on, out you go, hyoho...")
      return close()
    end
    if not mage() then
      page(A, "Hm? Hm? What is it?")
      next()
      page(A, "Sightseeing? I don't think", "there's much to see here.")
      next()
      page(A, "It's unusual, you say?", "Oh my~ hyohohoho")
      return close()
    end
    if q == 11 then
      page(A, "What are you doing? Hurry up,", "off you go~!")
      next()
      page(A, "You'll forget something, you", "know? Go and graduate before", "it's too late!")
      return close()
    end
    if q2 == 8 then
      if count_item(STONE) < 30 then
        page(A, "What is it? Come to play", "tricks on me? Stop playing", "and off you go~!")
        next()
        page(A, "I said bring me 30", "^3051FD" .. item_name(STONE) .. "s^000000! Stones lying in", "the road are easy to pick up~")
        return close()
      end
      page(A, "Ah, you brought them!", "Wait a moment~")
      next()
      page(A, "Abracadabra!!") next()
      page(A, "Abra~cadabra!!") next()
      page(A, "Abraca~dabra!!") next()
      take_item(STONE, 30)
      v.job2nd_sage2 = 9
      page(A, "Look, three good stones out", "of plain old stones!")
      next()
      page(A, "Splendid. Happy, aren't you?", "Hyohoho!")
      next()
      give_item(CRYSTAL_BLUE, 1)
      give_item(GREEN_LIVE, 1)
      give_item(WIND_OF_VERDURE, 1)
      page(A, "There, elemental stones for", "you. And now, and now, hear", "me out to the end.")
      next()
      mes(A) mes("Make me these arrows:") arrows_page()
      next()
      page(A, "An Archer friend would make", "them for you. Off you go!", "Hyohoho~")
      return close()
    end
    if q2 == 9 then
      if count_item(CRYSTAL_ARROW) < 50 or count_item(STONE_ARROW) < 50 or count_item(ARROW_OF_WIND) < 50 then
        page(A, "What, what? Where are the", "elemental stones I gave you?", "You didn't sell them...")
        next()
        arrows_page()
        mes("Ask an Archer friend to make") mes("them~ hyohoho")
        return close()
      end
      page(A, "Ah, you brought them. Then", "let's start the lecture.")
      next()
      page(A, "I'll read it out, so listen", "and underline what matters.")
      next()
      page(A, "Water magic is strong against", "Fire. Like pouring water on", "a fire. Easy, isn't it?")
      next()
      page(A, "Wind magic is strong against", "Water. What happens when", "lightning hits a lake?", "Hyohoho!")
      next()
      page(A, "Earth magic is strong against", "Wind. To keep out the wind,", "you build a house of earth,", "don't you? That's it.")
      next()
      page(A, "Fire magic is strong against", "Earth. In a fire the trees", "burn, don't they? The same...")
      next()
      page(A, "Oh, is it that time already?", "We'll go on next lesson.")
      next()
      take_item(CRYSTAL_ARROW, 50)
      take_item(STONE_ARROW, 50)
      take_item(ARROW_OF_WIND, 50)
      v.job2nd_sage2 = 10
      page(A, "By next lesson, have one", blue(1, HOLY_WATER) .. " ready.", "If you have no priestly", "friends, make it yourself.")
      return close()
    end
    if q2 == 10 then
      if count_item(HOLY_WATER) < 1 then
        page(A, "What are you doing! Don't", "forget things~")
        next()
        page(A, blue(1, HOLY_WATER), "Ask an Acolyte or Priest", "friend. Hyohoho")
        return close()
      end
      page(A, "Good, good! You brought it!", "Now, on with the lecture.")
      next()
      page(A, "I'll read it out again, so", "listen and take notes.")
      next()
      page(A, "Water is weak to Wind magic!", "Lightning on you when frozen", "hurt, didn't it? Hyohoho!")
      next()
      page(A, "Wind is weak to Earth magic!", "One who flies about takes a", "bad fall, doesn't he? Hyo...")
      next()
      page(A, "Earth is weak to Fire magic!", "Set a tree growing from the", "earth alight, and it burns", "away, doesn't it?")
      next()
      page(A, "Fire is weak to Water magic!", "Water is what cools a hot", "thing, after all.")
      next()
      page(A, "Good, good, that ends the", "lectuuure! You understood", "it all, didn't you?")
      next()
      take_item(HOLY_WATER, 1)
      v.job2nd_sage2 = 11
      page(A, "Now only the thesis is left.", "My lectures really are the", "best. Hyohohohoho!")
      next()
      page(A, "Let's get ready to write it.", "Bring me:")
      next()
      thesis_list(A)
      next()
      page(A, "That's all. I'll be waiting.")
      return close()
    end
    if q2 == 11 then
      if not has_all(THESIS, 1) then
        page(A, "What, what? Don't forget~")
        next()
        thesis_list(A)
        next()
        page(A, "You can get ready on your own,", "can't you? Or with help, fine~", "I'll be waiting!")
        return close()
      end
      take_all(THESIS, 1)
      page(A, "Right. Let's write it, then.", "I taught you, so you'll be", "fine~ hyohoho!")
      next()
      dots()
      for i, part in ipairs(ELEMENTAL_THESIS) do
        if part[1] == "next" then next() elseif part[1] then mes(part[1]) end
        local labels = {}
        for k, ch in ipairs(part[2]) do labels[k] = ch[1] end
        local ch = part[2][menu(table.unpack(labels))]
        for k = 2, #ch do mes(ch[k]) end
      end
      next()
      dots()
      v.job2nd_sage = 11
      give_item(BOOK, 1)
      page(A, "Ho ho~ you have a feel for it~", "There, it's done. Hyohoho")
      next()
      page(A, "Show it to the Dean. Then you", "graduate too! Hyoho~")
      return close()
    end
    if q == 9 then
      v.job2nd_sage2 = 8
      page(A, "Hello, pleased to meet you.", "My first student in ages~")
      next()
      page(A, "Hyohoho, I'm the professor who", "teaches you. Happy, aren't", "you? Hyohoho!")
      next()
      page(A, "Before the lecture, would you", "do me a little favour~?", "No, no, it's nothing hard.")
      next()
      page(A, "Just bring me 30", "^3051FD" .. item_name(STONE) .. "s^000000. That's all.", "Not too hard, is it?")
      next()
      page(A, "If picking up stones is hard,", "ask a Thief friend. Well,", "I'll be waiting for 30~")
      return close()
    end
    page(A, "What, what?")
    next()
    page(A, "Here to look around? There's", "not much to see here, you", "know?")
    next()
    page(A, "Well, look around as you like~", "Hyohohoho")
    close()
  end,
}

-- ------------------------------------------------- the History Professor

local S = "[Saphien Layless]"
npc {
  map = IN, x = 62, y = 176, dir = 3, sprite = 109, name = "Saphien Layless",
  replaces = r817("817:cities/yuno.txt:296"), src = SRC .. "1649",
  on_click = function()
    if player.class == Job_Sage then
      page(S, "Who is it...? Ah, a student", "of mine. You graduated from", "the Academy, didn't you?")
      next()
      page(S, "Know the past, and you", "understand the present and", "can imagine the future. All", "truth lies there.")
      next()
      page(S, "You too seek the past. One day", "you will look back on 'now',", "miss it, and find how you", "have changed.")
      return close()
    end
    if not mage() then
      page(S, "Yes, I teach history.")
      next()
      page(S, "Today's world is shaped by", "countless memories and years.", "Knowing the past is a bridge", "from the present to the future.")
      next()
      page(S, "Look back on your own past", "too. You will surely see the", "road to your future.")
      return close()
    end
    if v.job2nd_sage == 11 then
      page(S, "What are you doing? Your", "thesis is done: show it to", "the Dean, before you lose it!")
      return close()
    end
    page(S, "I don't know why you came, but", "I want to think just now.", "Come back later...")
    close()
  end,
}

-- --------------------------------------------- the written test's questions
-- jAthena 1779-2222: three sets of twenty, five points each. Our English;
-- names as this game shows them. { question lines, choices, right one }
SETS = {
  { -- jAthena 1779
    { { "Which of these is not sold", "by Morroc's jewel sellers?" }, { "Topaz", "Garnet", "Diamond", "Sapphire" }, 2 },
    { { "Which town sells no", "Monster's Feed?" }, { "Prontera", "Morroc", "Aldebaran", "Alberta" }, 3 },
    { { "Which town is nearest to", "the Labyrinth Forest?" }, { "Prontera", "Morroc", "Geffen", "Payon" }, 1 },
    { { "Which one is not of the", "same race as the others?" }, { "Muka", "Drops", "Plankton", "Penomena" }, 4 },
    { { "Which one is not of the", "same element as the others?" }, { "Dokebi", "Isis", "Giearth", "Deviruchi" }, 3 },
    { { "Which one is not of the", "same size as the others?" }, { "Thief Bug Male", "Horn", "Metaller", "Argos" }, 4 },
    { { "Which monster does not drop", "the Yggdrasil Leaf?" }, { "Marduk", "Baphomet Jr.", "Angeling", "Wanderer" }, 1 },
    { { "Who has nothing to do with", "the Priest job change?" }, { "Thomas Cervantes", "Windsor Benedict", "Peter S. Albert", "Cecil Margarita" }, 2 },
    { { "Who is not in Morroc?" }, { "Abdul", "Aragham", "Antonio", "Cetsu" }, 3 },
    { { "What is the name of the", "Kafra with the charming", "blue hair?" }, { "Pavianne", "Blossom", "Leilah", "Curly Sue" }, 1 },
    { { "Which is not needed on the", "way to the skill Fire Wall?" }, { "Fire Bolt Lv 4", "Napalm Beat Lv 4", "Fire Ball Lv 5", "Sight Lv 1" }, 2 },
    { { "With Increase SP Recovery at", "Lv 6, how much SP does the", "skill alone restore in", "10 seconds?" }, { "14", "16", "18", "21" }, 3 },
    { { "How much INT does a Mage's", "job bonus give at Job", "Level 33?" }, { "7", "6", "5", "4" }, 4 },
    { { "The SP cost and length of", "the Archer's Improve", "Concentration at Lv 5?" }, { "45 / 140 s", "50 / 140 s", "45 / 150 s", "50 / 150 s" }, 1 },
    { { "Which is not needed on the", "way to the Blacksmith's", "Power Maximize?" }, { "Hilt Binding", "Skin Tempering", "Hammer Fall", "Weapon Perfection" }, 2 },
    -- jAthena's Ribbon gives SP +20 (its item db of 2004); ours, mod0631's, +10
    { { "The DEF and bonus of", "a Cute Ribbon?" }, { "0 / MaxSP +10", "0 / MaxSP +20", "1 / MaxSP +10", "1 / MaxSP +20" }, 3 },
    { { "Which job cannot wear", "a Saint's Robe?" }, { "Swordman", "Merchant", "Thief", "Acolyte" }, 3 },
    { { "Which ailment does a Green", "Potion not cure?" }, { "Silence", "Poison", "Blind", "Curse" }, 4 },
    { { "The lost ancient kingdom", "they say lies somewhere", "under Geffen?" }, { "Geffeyon", "Geffenia", "Geffendria", "Geffria" }, 2 },
    { { "The tree said to be the", "root of the world?" }, { "Yggdrasil", "Ygddrasil", "Mastela", "Dead Branch" }, 1 },
  },
  { -- jAthena 1929
    { { "Which of these does Geffen's", "magic shop not sell?" }, { "Mantle", "Wand", "Circlet", "Silver Robe" }, 1 },
    { { "Which town sells no Blade?" }, { "Prontera", "Izlude", "Aldebaran", "Payon" }, 3 },
    { { "Which town is nearest to", "Glast Heim?" }, { "Prontera", "Geffen", "Morroc", "Payon" }, 2 },
    { { "Which one is not of the", "same race as the others?" }, { "Aster", "Marc", "Mars", "Marin" }, 4 },
    { { "Which one is not of the", "same element as the others?" }, { "Desert Wolf Puppy", "Smokie", "Picky", "Choco" }, 2 },
    { { "Which one is not of the", "same size as the others?" }, { "Drake", "Wraith", "Evil Druid", "Khalitzburg" }, 1 },
    { { "Which monster does not drop", "Phracon?" }, { "Pupa", "Condor", "Savage Babe", "Desert Wolf Puppy" }, 2 },
    { { "Who has nothing to do with", "the Blacksmith job change?" }, { "Baisulitst", "Wickebine", "Barcardi", "Mitmayer" }, 3 },
    { { "Who is not in Aldebaran?" }, { "Chemirre", "Marcell", "Munster", "Isenberg" }, 2 },
    { { "The youngest of the Kafra", "staff?" }, { "Pavianne", "Blossom", "Leilah", "Curly Sue" }, 4 },
    -- jAthena: SP 40; ours, mod0631's and the client's own SP table, 35
    { { "Safety Wall at Lv 6: its SP", "cost and the blows it stops?" }, { "SP 40, 6 hits", "SP 35, 6 hits", "SP 35, 7 hits", "SP 40, 7 hits" }, 3 },
    { { "Napalm Beat at Lv 6 strikes", "with how many times MATK?" }, { "1.2 times", "1.3 times", "1.4 times", "1.5 times" }, 2 },
    { { "In the Mage test, which", "stone is the catalyst of", "Solution No. 4?" }, { "Blue Gemstone", "Red Gemstone", "Yellow Gemstone", "Diamond" }, 4 },
    { { "The Swordman's Bash at Lv 6:", "its damage and SP cost?" }, { "250% / 8", "280% / 8", "280% / 15", "310% / 15" }, 3 },
    { { "Which is not needed on the", "way to the Hunter's", "Claymore Trap?" }, { "Remove Trap", "Land Mine", "Ankle Snare", "Flasher" }, 1 },
    { { "The DEF and bonus of", "a Wedding Veil?" }, { "0 / MDEF +3", "0 / MDEF +5", "1 / MDEF +3", "1 / MDEF +5" }, 2 },
    { { "Which job cannot wear", "a Coat?" }, { "Swordman", "Merchant", "Thief", "Mage" }, 4 },
    { { "Which is not made into the", "blue dye?" }, { "Alchol", "Detrimindexta", "Karvodailnirol", "Blue Herb" }, 3 },
    { { "What did the god Odin use,", "they say, to make the world?" }, { "Ymir's Heart", "Ymir's Nail", "Ymir's Tooth", "Ymir's Relic" }, 2 },
    { { "The metal said to come before", "one who can change the", "world's fate, and to share it?" }, { "Emveretarcon", "Emperium", "Empera", "Phracon" }, 2 },
  },
  { -- jAthena 2079
    { { "Which does Prontera's gift", "merchant not sell?" }, { "China", "Red Frame", "Bouquet", "Glass Bead" }, 3 },
    { { "Which town sells no", "Stiletto?" }, { "Prontera", "Morroc", "Geffen", "Lutie" }, 1 },
    { { "Which town is nearest to", "Turtle Island?" }, { "Aldebaran", "Alberta", "Comodo", "Izlude" }, 2 },
    { { "Which one is not of the", "same race as the others?" }, { "Raggler", "Pest", "Frilldora", "Aster" }, 4 },
    { { "Which one is not of the", "same element as the others?" }, { "Mantis", "Metaller", "Rocker", "Horn" }, 2 },
    { { "Which one is not of the", "same size as the others?" }, { "Raydric", "Raydric Archer", "Wanderer", "Dark Frame" }, 1 },
    { { "Which monster does not drop", "Alchol?" }, { "Horong", "Plankton", "Poison Spore", "Toad" }, 3 },
    { { "Who has nothing to do with", "the Knight job change?" }, { "James Siracuse", "Thomas Cervantes", "Amy Beatrice", "Edmond Grost" }, 2 },
    { { "Who is not in Prontera?" }, { "Nami", "Aure Dupon", "Thomas", "Hollgrehenn" }, 2 },
    { { "The Kafra who wears glasses?" }, { "Pavianne", "Blossom", "Leilah", "Curly Sue" }, 3 },
    { { "The SP cost of Thunderstorm", "at Lv 7?" }, { "49", "59", "69", "74" }, 2 },
    { { "With half your SP left, how", "much does Energy Coat take", "off a blow, and how much SP", "does it cost?" },
      { "24% off, 1.5% SP", "24% off, 2% SP", "18% off, 1.5% SP", "18% off, 2% SP" }, 4 },
    { { "Which element has no bolt", "among the Mage's spells?" }, { "Water", "Earth", "Fire", "Wind" }, 2 },
    -- jAthena asks the chance and the damage (140%); ours doubles the blow, as mod0631
    { { "With the Thief's Double", "Attack at Lv 7, how often", "does it strike twice?" }, { "30%", "35%", "40%", "45%" }, 2 },
    -- jAthena's tree needs Heal (Sanctuary on the way); ours, mod0631's, does not: Lex Divina for it
    { { "Which is not needed on the", "way to the Priest's Magnus", "Exorcismus?" }, { "Divine Protection", "Lex Divina", "Ruwach", "Aqua Benedicta" }, 1 },
    { { "The DEF and bonus of", "a Bunny Band?" }, { "1 / LUK +2", "1 / LUK +5", "2 / LUK +2", "2 / LUK +5" }, 3 },
    { { "Which job cannot wear", "Padded Armor?" }, { "Swordman", "Merchant", "Thief", "Archer" }, 4 },
    { { "Which item restores all", "your HP and SP?" }, { "Royal Jelly", "Yggdrasil Seed", "Yggdrasilberry", "Mastela Fruit" }, 3 },
    { { "The name of the king of", "Rune-Midgard?" }, { "Triston III", "Tristan III", "Tristar III", "Trust III" }, 2 },
    { { "Which god do the Crusaders", "serve?" }, { "Odin", "Loki", "Thor", "Aragham Salehe" }, 1 },
  },
}
