//! Lua scripting: NPCs, warps, shops, monster spawns, @commands, game rules.
//!
//! Loading: `prelude.lua` (embedded) then every `*.lua` under the script
//! directory in path order. Files *declare* things (`npc{}`, `warp{}`,
//! `shop{}`, `spawn{}`, `command()`, `on()`); declarations are collected in
//! [`Defs`] and instantiated by the game.
//!
//! Running: every call into Lua that may touch the world goes through
//! [`ScriptEngine::run`], which installs one scoped native function `_rt(op,
//! ...)` borrowing the live [`Host`] for the duration of the call. The
//! prelude wraps `_rt` in the friendly API (`mes`, `give_item`, ...).
//! NPC dialogs are coroutines: `next()`, `menu()`, `input_*()` yield until the
//! client answers.
//!
//! Reloading builds a brand-new Lua state; if anything fails to load the old
//! state stays active and the error is reported (safe to iterate live).

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::{Context, Result, anyhow};
use mlua::{Function, Lua, LuaSerdeExt, Table, Thread, ThreadStatus, Value as LuaValue, Variadic};
use serde_json::Value;

const PRELUDE: &str = include_str!("prelude.lua");

/// mlua errors are not Send/Sync (no `send` feature); carry them as text.
trait Ah<T> {
    fn ah(self) -> Result<T>;
}
impl<T> Ah<T> for mlua::Result<T> {
    fn ah(self) -> Result<T> {
        self.map_err(|e| anyhow!("{e}"))
    }
}

/// Who a script call acts for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ctx {
    /// Player unit id (0 = none, e.g. server events).
    pub player: u32,
    /// NPC unit id (0 = none, e.g. @commands and item scripts).
    pub npc: u32,
    pub kind: CtxKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtxKind {
    Dialog,
    Touch,
    Command,
    Item,
    Event,
    Console,
}

/// The game side of `_rt(op, ...)`.
pub trait Host {
    fn call(&mut self, ctx: &Ctx, op: &str, args: Vec<Value>) -> std::result::Result<Value, String>;
}

#[derive(Debug, Clone)]
pub struct NpcDef {
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub dir: u8,
    pub sprite: u16,
    pub name: String,
    pub on_click: Option<Function>,
    pub on_touch: Option<Function>,
    /// Touch area half-size (xs, ys): a cell triggers when |dx|<=xs && |dy|<=ys.
    pub touch: Option<(u16, u16)>,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct WarpDef {
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub span: (u16, u16),
    pub to: (String, u16, u16),
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct ShopDef {
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub dir: u8,
    pub sprite: u16,
    pub name: String,
    /// (item id, price; None = item db price)
    pub items: Vec<(u32, Option<u32>)>,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct SpawnDef {
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub xs: u16,
    pub ys: u16,
    pub mob: u32,
    pub count: u32,
    /// Respawn delay in ms (Athena delay1; 0 = default).
    pub delay: u32,
    pub delay2: u32,
    pub source: String,
}

#[derive(Default)]
pub struct Defs {
    pub npcs: Vec<NpcDef>,
    pub warps: Vec<WarpDef>,
    pub shops: Vec<ShopDef>,
    pub spawns: Vec<SpawnDef>,
    pub commands: HashMap<String, Function>,
    pub events: HashMap<String, Vec<Function>>,
    pub files: Vec<PathBuf>,
}

/// What a suspended dialog is waiting for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wait {
    Next,
    Menu,
    Close,
    InputNumber,
    InputString,
}

pub enum Outcome {
    /// The coroutine yielded; resume it with the client's answer.
    Suspended(Wait),
    /// The script finished.
    Done(Value),
}

pub struct ScriptEngine {
    lua: Lua,
    pub defs: Defs,
    dir: PathBuf,
    item_fns: RefCell<HashMap<u32, Function>>,
}

fn source_of(lua: &Lua) -> String {
    // level 2 = the script line that called npc{}/warp{}/...
    lua.inspect_stack(2)
        .map(|d| {
            let src = d.source();
            let name = src.short_src.as_deref().unwrap_or("?").to_string();
            format!("{}:{}", name, d.curr_line())
        })
        .unwrap_or_default()
}

fn get<T: mlua::FromLua>(t: &Table, k: &str) -> mlua::Result<T> {
    t.get::<T>(k).map_err(|e| mlua::Error::external(format!("field `{k}`: {e}")))
}

fn opt<T: mlua::FromLua>(t: &Table, k: &str) -> mlua::Result<Option<T>> {
    t.get::<Option<T>>(k).map_err(|e| mlua::Error::external(format!("field `{k}`: {e}")))
}

fn collect_lua_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect_lua_files(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("lua") {
            out.push(p);
        }
    }
}

impl ScriptEngine {
    /// Load prelude + all scripts under `dir`.
    pub fn load(dir: &Path) -> Result<Self> {
        let lua = Lua::new();
        let defs = Rc::new(RefCell::new(Defs::default()));
        Self::install_decls(&lua, defs.clone()).ah()?;
        lua.load(PRELUDE).set_name("@prelude.lua").exec().map_err(|e| anyhow!("prelude: {e}"))?;
        let mut files = Vec::new();
        // rules first, so content files can rely on them
        collect_lua_files(&dir.join("rules"), &mut files);
        let mut rest = Vec::new();
        collect_lua_files(dir, &mut rest);
        files.extend(rest.into_iter().filter(|p| !p.starts_with(dir.join("rules"))));
        for f in &files {
            let code = std::fs::read_to_string(f).with_context(|| format!("reading {}", f.display()))?;
            let rel = f.strip_prefix(dir).unwrap_or(f).display().to_string();
            lua.load(&code)
                .set_name(format!("@{rel}"))
                .exec()
                .map_err(|e| anyhow!("{rel}: {e}"))?;
        }
        let mut defs = std::mem::take(&mut *defs.borrow_mut());
        defs.files = files;
        Ok(ScriptEngine { lua, defs, dir: dir.to_path_buf(), item_fns: RefCell::new(HashMap::new()) })
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn install_decls(lua: &Lua, defs: Rc<RefCell<Defs>>) -> mlua::Result<()> {
        let g = lua.globals();
        let d = defs.clone();
        g.set(
            "npc",
            lua.create_function(move |lua, t: Table| {
                let touch: Option<Table> = opt(&t, "touch")?;
                let touch = match touch {
                    Some(tt) => Some((tt.get::<u16>(1)?, tt.get::<u16>(2)?)),
                    None => None,
                };
                d.borrow_mut().npcs.push(NpcDef {
                    map: get(&t, "map")?,
                    x: get(&t, "x")?,
                    y: get(&t, "y")?,
                    dir: opt(&t, "dir")?.unwrap_or(0),
                    sprite: opt(&t, "sprite")?.unwrap_or(111),
                    name: opt(&t, "name")?.unwrap_or_default(),
                    on_click: opt(&t, "on_click")?,
                    on_touch: opt(&t, "on_touch")?,
                    touch,
                    source: source_of(lua),
                });
                Ok(())
            })?,
        )?;
        let d = defs.clone();
        g.set(
            "warp",
            lua.create_function(move |lua, t: Table| {
                let span: Option<Table> = opt(&t, "span")?;
                let span = match span {
                    Some(s) => (s.get::<u16>(1)?, s.get::<u16>(2)?),
                    None => (1, 1),
                };
                let to: Table = get(&t, "to")?;
                d.borrow_mut().warps.push(WarpDef {
                    map: get(&t, "map")?,
                    x: get(&t, "x")?,
                    y: get(&t, "y")?,
                    span,
                    to: (to.get(1)?, to.get(2)?, to.get(3)?),
                    source: source_of(lua),
                });
                Ok(())
            })?,
        )?;
        let d = defs.clone();
        g.set(
            "shop",
            lua.create_function(move |lua, t: Table| {
                let mut items = Vec::new();
                let list: Table = get(&t, "items")?;
                for e in list.sequence_values::<LuaValue>() {
                    match e? {
                        LuaValue::Integer(id) => items.push((id as u32, None)),
                        LuaValue::Table(p) => items.push((p.get::<u32>(1)?, p.get::<Option<u32>>(2)?)),
                        other => return Err(mlua::Error::external(format!("bad shop item {other:?}"))),
                    }
                }
                d.borrow_mut().shops.push(ShopDef {
                    map: get(&t, "map")?,
                    x: get(&t, "x")?,
                    y: get(&t, "y")?,
                    dir: opt(&t, "dir")?.unwrap_or(0),
                    sprite: opt(&t, "sprite")?.unwrap_or(83),
                    name: opt(&t, "name")?.unwrap_or_else(|| "Shop".into()),
                    items,
                    source: source_of(lua),
                });
                Ok(())
            })?,
        )?;
        let d = defs.clone();
        g.set(
            "spawn",
            lua.create_function(move |lua, t: Table| {
                d.borrow_mut().spawns.push(SpawnDef {
                    map: get(&t, "map")?,
                    x: opt(&t, "x")?.unwrap_or(0),
                    y: opt(&t, "y")?.unwrap_or(0),
                    xs: opt(&t, "xs")?.unwrap_or(0),
                    ys: opt(&t, "ys")?.unwrap_or(0),
                    mob: get(&t, "mob")?,
                    count: opt(&t, "count")?.unwrap_or(1),
                    delay: opt(&t, "delay")?.unwrap_or(0),
                    delay2: opt(&t, "delay2")?.unwrap_or(0),
                    source: source_of(lua),
                });
                Ok(())
            })?,
        )?;
        let d = defs.clone();
        g.set(
            "command",
            lua.create_function(move |_, (name, f): (String, Function)| {
                d.borrow_mut().commands.insert(name.to_ascii_lowercase(), f);
                Ok(())
            })?,
        )?;
        let d = defs;
        g.set(
            "on",
            lua.create_function(move |_, (event, f): (String, Function)| {
                d.borrow_mut().events.entry(event).or_default().push(f);
                Ok(())
            })?,
        )?;
        Ok(())
    }

    /// Run `body` with `_rt` bound to `host` for `ctx`.
    fn with_host<R>(&self, host: &mut dyn Host, ctx: Ctx, body: impl FnOnce(&Lua) -> mlua::Result<R>) -> mlua::Result<R> {
        let host = RefCell::new(host);
        self.lua.scope(|scope| {
            let rt = scope.create_function(|lua, (op, args): (String, Variadic<LuaValue>)| {
                let mut jargs = Vec::with_capacity(args.len());
                for a in args.iter() {
                    jargs.push(match a {
                        LuaValue::Function(_) | LuaValue::Thread(_) | LuaValue::UserData(_) => Value::Null,
                        v => lua.from_value::<Value>(v.clone()).unwrap_or(Value::Null),
                    });
                }
                let r = host.borrow_mut().call(&ctx, &op, jargs).map_err(mlua::Error::external)?;
                lua.to_value(&r)
            })?;
            self.lua.globals().set("_rt", rt)?;
            body(&self.lua)
        })
    }

    fn outcome(&self, thread: &Thread, ret: mlua::MultiValue) -> Result<Outcome> {
        match thread.status() {
            ThreadStatus::Resumable => {
                let tag = ret.into_iter().next().and_then(|v| v.as_string().map(|s| s.to_string_lossy()));
                let wait = match tag.as_deref() {
                    Some("next") => Wait::Next,
                    Some("menu") => Wait::Menu,
                    Some("close") => Wait::Close,
                    Some("input_number") => Wait::InputNumber,
                    Some("input_string") => Wait::InputString,
                    other => return Err(anyhow!("script yielded unknown value {other:?}")),
                };
                Ok(Outcome::Suspended(wait))
            }
            _ => {
                let v = ret.into_iter().next().unwrap_or(LuaValue::Nil);
                Ok(Outcome::Done(self.lua.from_value(v).unwrap_or(Value::Null)))
            }
        }
    }

    /// Start `f` as a coroutine (NPC click/touch).
    pub fn start(&self, host: &mut dyn Host, ctx: Ctx, f: &Function) -> Result<(Thread, Outcome)> {
        let thread = self.lua.create_thread(f.clone()).ah()?;
        let ret = self
            .with_host(host, ctx, |_| thread.resume::<mlua::MultiValue>(()))
            .map_err(|e| anyhow!("{e}"))?;
        let o = self.outcome(&thread, ret)?;
        Ok((thread, o))
    }

    /// Resume a suspended dialog with the client's answer.
    pub fn resume(&self, host: &mut dyn Host, ctx: Ctx, thread: &Thread, answer: Value) -> Result<Outcome> {
        let ret = self
            .with_host(host, ctx, |lua| thread.resume::<mlua::MultiValue>(lua.to_value(&answer)?))
            .map_err(|e| anyhow!("{e}"))?;
        self.outcome(thread, ret)
    }

    /// Call a function to completion (commands, events, item scripts). Yields are errors.
    pub fn call(&self, host: &mut dyn Host, ctx: Ctx, f: &Function, args: Vec<Value>) -> Result<Value> {
        let r = self
            .with_host(host, ctx, |lua| {
                let mut lv = Vec::new();
                for a in &args {
                    lv.push(lua.to_value(a)?);
                }
                f.call::<LuaValue>(mlua::MultiValue::from_vec(lv))
            })
            .map_err(|e| anyhow!("{e}"))?;
        Ok(self.lua.from_value(r).unwrap_or(Value::Null))
    }

    /// Run an item's use script (compiled once per item).
    pub fn run_item(&self, host: &mut dyn Host, ctx: Ctx, item_id: u32, code: &str) -> Result<()> {
        let f = {
            let mut cache = self.item_fns.borrow_mut();
            match cache.get(&item_id) {
                Some(f) => f.clone(),
                None => {
                    let f = self
                        .lua
                        .load(code)
                        .set_name(format!("=item {item_id}"))
                        .into_function()
                        .map_err(|e| anyhow!("item {item_id}: {e}"))?;
                    cache.insert(item_id, f.clone());
                    f
                }
            }
        };
        self.call(host, ctx, &f, vec![]).map(|_| ())
    }

    /// Evaluate arbitrary Lua (admin console). Returns the printed result.
    pub fn eval(&self, host: &mut dyn Host, ctx: Ctx, code: &str) -> Result<String> {
        let f = match self.lua.load(format!("return {code}")).set_name("=console").into_function() {
            Ok(f) => f,
            Err(_) => self.lua.load(code).set_name("=console").into_function().map_err(|e| anyhow!("{e}"))?,
        };
        let v = self.call(host, ctx, &f, vec![])?;
        Ok(match v {
            Value::Null => "nil".into(),
            Value::String(s) => s,
            other => other.to_string(),
        })
    }

    /// Call `rules.<name>(arg)` (pure functions: formulas). None if undefined.
    pub fn rule(&self, name: &str, arg: &Value) -> Result<Option<Value>> {
        let rules: Option<Table> = self.lua.globals().get("rules").ah()?;
        let Some(rules) = rules else { return Ok(None) };
        let f: Option<Function> = rules.get(name).ah()?;
        let Some(f) = f else { return Ok(None) };
        let v: LuaValue = f.call(self.lua.to_value(arg).ah()?).map_err(|e| anyhow!("rules.{name}: {e}"))?;
        Ok(Some(self.lua.from_value(v).ah()?))
    }
}
