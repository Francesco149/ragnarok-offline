//! Save files: `saves/accounts/<id>.json`, `saves/chars/<id>.json`,
//! `saves/ids.json`. Writes are atomic (temp file + rename); the previous
//! version is kept as `<file>.bak`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::model::{Account, Character};

pub const FIRST_ACCOUNT_ID: u32 = 2_000_000; // Athena START_ACCOUNT_NUM
pub const FIRST_CHAR_ID: u32 = 150_000; // Athena START_CHAR_NUM

#[derive(Serialize, Deserialize)]
struct Ids {
    next_account: u32,
    next_char: u32,
}

pub struct Store {
    dir: PathBuf,
    pub accounts: BTreeMap<u32, Account>,
    pub chars: BTreeMap<u32, Character>,
    /// Server-wide script variables (Athena `$name`), `saves/world.json`.
    pub globals: BTreeMap<String, serde_json::Value>,
    next_account: u32,
    next_char: u32,
}

fn write_atomic(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, data)?;
    if path.exists() {
        let _ = std::fs::copy(path, path.with_extension("json.bak"));
    }
    std::fs::rename(&tmp, path).with_context(|| format!("saving {}", path.display()))
}

fn load_dir<T: for<'de> Deserialize<'de>>(dir: &Path) -> Result<Vec<T>> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir) else { return Ok(out) };
    for e in rd {
        let p = e?.path();
        if p.extension().and_then(|s| s.to_str()) == Some("json") {
            let s = std::fs::read_to_string(&p)?;
            out.push(serde_json::from_str(&s).with_context(|| format!("parsing {}", p.display()))?);
        }
    }
    Ok(out)
}

impl Store {
    pub fn open(dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(dir.join("accounts"))?;
        std::fs::create_dir_all(dir.join("chars"))?;
        let mut s = Store {
            dir: dir.to_path_buf(),
            accounts: BTreeMap::new(),
            chars: BTreeMap::new(),
            globals: BTreeMap::new(),
            next_account: FIRST_ACCOUNT_ID,
            next_char: FIRST_CHAR_ID,
        };
        for a in load_dir::<Account>(&dir.join("accounts"))? {
            s.next_account = s.next_account.max(a.id + 1);
            s.accounts.insert(a.id, a);
        }
        for c in load_dir::<Character>(&dir.join("chars"))? {
            s.next_char = s.next_char.max(c.id + 1);
            s.chars.insert(c.id, c);
        }
        if let Ok(b) = std::fs::read(dir.join("world.json")) {
            s.globals = serde_json::from_slice(&b).with_context(|| format!("{}", dir.join("world.json").display()))?;
        }
        if let Ok(b) = std::fs::read(dir.join("ids.json")) {
            if let Ok(ids) = serde_json::from_slice::<Ids>(&b) {
                s.next_account = s.next_account.max(ids.next_account);
                s.next_char = s.next_char.max(ids.next_char);
            }
        }
        Ok(s)
    }

    pub fn save_globals(&self) -> Result<()> {
        write_atomic(&self.dir.join("world.json"), &serde_json::to_vec_pretty(&self.globals)?)
    }

    fn save_ids(&self) -> Result<()> {
        let ids = Ids { next_account: self.next_account, next_char: self.next_char };
        write_atomic(&self.dir.join("ids.json"), &serde_json::to_vec_pretty(&ids)?)
    }

    pub fn account_by_userid(&self, userid: &str) -> Option<&Account> {
        self.accounts.values().find(|a| a.userid.eq_ignore_ascii_case(userid))
    }

    pub fn create_account(&mut self, userid: &str, password: &str, sex: u8, gm_level: u8) -> Result<u32> {
        let id = self.next_account;
        self.next_account += 1;
        let a = Account { id, userid: userid.into(), password: password.into(), sex, gm_level, ..Default::default() };
        self.accounts.insert(id, a);
        self.save_account(id)?;
        self.save_ids()?;
        Ok(id)
    }

    pub fn save_account(&self, id: u32) -> Result<()> {
        if let Some(a) = self.accounts.get(&id) {
            write_atomic(&self.dir.join(format!("accounts/{id}.json")), &serde_json::to_vec_pretty(a)?)?;
        }
        Ok(())
    }

    pub fn name_taken(&self, name: &str) -> bool {
        self.chars.values().any(|c| c.name.eq_ignore_ascii_case(name))
    }

    pub fn create_char(&mut self, mut c: Character) -> Result<u32> {
        c.id = self.next_char;
        self.next_char += 1;
        let id = c.id;
        if let Some(a) = self.accounts.get_mut(&c.account_id) {
            a.slots.insert(c.slot, id);
        }
        let aid = c.account_id;
        self.chars.insert(id, c);
        self.save_char(id)?;
        self.save_account(aid)?;
        self.save_ids()?;
        Ok(id)
    }

    pub fn save_char(&self, id: u32) -> Result<()> {
        if let Some(c) = self.chars.get(&id) {
            write_atomic(&self.dir.join(format!("chars/{id}.json")), &serde_json::to_vec_pretty(c)?)?;
        }
        Ok(())
    }

    /// Deleting keeps the file as `<id>.json.deleted` so nothing is ever lost.
    pub fn delete_char(&mut self, id: u32) -> Result<()> {
        if let Some(c) = self.chars.remove(&id) {
            if let Some(a) = self.accounts.get_mut(&c.account_id) {
                a.slots.retain(|_, cid| *cid != id);
            }
            let p = self.dir.join(format!("chars/{id}.json"));
            let _ = std::fs::rename(&p, p.with_extension("json.deleted"));
            self.save_account(c.account_id)?;
        }
        Ok(())
    }

    pub fn chars_of(&self, account_id: u32) -> Vec<&Character> {
        let Some(a) = self.accounts.get(&account_id) else { return vec![] };
        a.slots.values().filter_map(|cid| self.chars.get(cid)).collect()
    }
}
