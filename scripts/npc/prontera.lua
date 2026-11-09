-- Hand-written NPCs. Each npc{} is a sprite on a map; on_click runs as a
-- coroutine: mes() adds text, next()/menu()/close() wait for the player.
-- Sprite ids are the client's NPC job ids (data/sprite/npc, jobname table).
-- Tier: reconstruction - wording is ours, services follow the era's
-- Kafra (save, storage) and the Athena "healer" convention.

npc {
  map = "prontera", x = 152, y = 187, dir = 4, sprite = 117, name = "Guide",
  on_click = function()
    mes("[Guide]")
    mes("Welcome to ragnarok-offline - a single-player")
    mes("server for the 2003 euRO client.")
    next()
    local c = menu("Where am I?", "What can I do here?", "Nothing, thanks.")
    if c == 1 then
      mes("[Guide]")
      mes("This is Prontera, capital of the Rune-Midgarts")
      mes("kingdom. You stand at " .. player.x .. ", " .. player.y .. ".")
    elseif c == 2 then
      mes("[Guide]")
      mes("Talk to the Kafra to save your position, hunt")
      mes("outside the gates, and type @help in chat for")
      mes("the admin commands of this offline server.")
    else
      mes("[Guide]")
      mes("Safe travels, " .. player.name .. ".")
    end
    close()
  end,
}

npc {
  map = "prontera", x = 146, y = 89, dir = 6, sprite = 112, name = "Kafra",
  on_click = function()
    cutin("kafra_01", 2)
    mes("[Kafra]")
    mes("Welcome to the Kafra Corporation. How may I help you?")
    next()
    local c = menu("Save", "Heal (free in offline mode)", "Cancel")
    if c == 1 then
      save_point("prontera", 116, 73)
      mes("[Kafra]")
      mes("Your respawn point has been saved here in Prontera.")
    elseif c == 2 then
      heal(99999, 99999)
      mes("[Kafra]")
      mes("All better!")
    else
      mes("[Kafra]")
      mes("We, Kafra, are always at your service.")
    end
    close()
    cutin()
  end,
}

-- Remembers how often you talked to it (persistent per-character variable).
npc {
  map = "prontera", x = 160, y = 187, dir = 4, sprite = 97, name = "Counter Boy",
  on_click = function()
    local n = (var("counter_boy") or 0) + 1
    var("counter_boy", n)
    mes("[Counter Boy]")
    mes("You have talked to me " .. n .. (n == 1 and " time." or " times."))
    if n == 10 then
      mes("That deserves a prize!")
      give_item("Red_Potion", 5)
    end
    close()
  end,
}
