//! The all-in-one installer's changes to the CD's "Ragnarok Online.msi" and
//! Setup.ini: the same rows tools/aio/make_aio.py adds with msitools (that
//! script's docstring explains each one; keep the two in step).
//!
//! - + RagnarokOffline.exe, ddraw.dll (DDrawCompat), DDrawCompat.ini and its
//!   licence, in an embedded cabinet (#offline.cab, Media 2, sequences 509+)
//! - + deferred, elevated custom actions running `RagnarokOffline.exe
//!   --prepare --quiet` after InstallFiles and `--unprepare --quiet` before
//!   RemoveFiles
//! - DDrawCompat's component only from Vista on (VersionNT >= 600)
//! - the "Ragnarok Online" shortcuts start RagnarokOffline.exe, with the CD's
//!   ragnarok.ico
//! - a new PackageCode (summary info + Setup.ini), derived from our files'
//!   hashes; ProductCode and UpgradeCode stay the CD's.
//!
//! The msi crate writes every table it touched sorted by its key columns
//! (string columns by string id), which is what Windows Installer requires
//! (make_aio.py has to re-sort msibuild's output by hand: error 2211).

use std::io::{Cursor, Write};

use anyhow::{Context, Result, bail};
use msi::{Expr, Insert, Package, Select, Update, Value};
use sha2::{Digest, Sha256};

/// "Ragnarok Online.msi" from the preserved disc (docs/PRESERVATION.md)
pub const ORIGINAL_MSI_SHA256: &str = "8b5207805d40b636101a2ff696f8f2eeaedf0df8c5dac35f7ffd6cc88673d768";
pub const MSI_NAME: &str = "Ragnarok Online.msi";
// Component GUIDs must never change between builds (component rules); the
// same as make_aio.py's.
const COMPONENT_LAUNCHER: &str = "{6E1F4C2A-3B7D-4E8A-9C1F-2D5B8A7E0F11}";
const COMPONENT_COMPAT: &str = "{6E1F4C2A-3B7D-4E8A-9C1F-2D5B8A7E0F12}";
const PACKAGE_NAMESPACE: uuid::Uuid = uuid::uuid!("6e1f4c2a-3b7d-4e8a-9c1f-2d5b8a7e0f00");
// 18 (exe from an installed File) + 64 (ignore exit code: the launcher retries
// on first start) + 1024 (deferred) + 2048 (no impersonation: elevated)
const CA_TYPE: i32 = 18 + 64 + 1024 + 2048;

/// Our files, as they go into the package.
pub struct Payload<'a> {
    pub launcher: &'a [u8],
    pub launcher_version: &'a str,
    pub ddraw: &'a [u8],
    pub ddraw_version: &'a str,
    pub ddraw_ini: &'a [u8],
    pub ddraw_license: &'a [u8],
}

pub struct Patched {
    pub msi: Vec<u8>,
    pub setup_ini: Vec<u8>,
    pub package_code: String,
}

pub fn sha256_hex(data: &[u8]) -> String {
    hex(&Sha256::digest(data))
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// `cab_time`: the date the cabinet gives our files (Windows Installer
/// installs them with it).
pub fn patch(msi_bytes: &[u8], setup_ini: &[u8], icon: &[u8], p: &Payload, cab_time: time::PrimitiveDateTime) -> Result<Patched> {
    if sha256_hex(msi_bytes) != ORIGINAL_MSI_SHA256 {
        bail!("{MSI_NAME} is not the 2003-12-19 euRO installer this tool patches (sha256 mismatch)");
    }
    // (File key, short|long name, data, version, component)
    let files: [(&str, &str, &[u8], &str, &str); 4] = [
        ("RagnarokOffline.exe", "RAGOFF.EXE|RagnarokOffline.exe", p.launcher, p.launcher_version, "OfflineLauncher"),
        ("ddraw.dll", "ddraw.dll", p.ddraw, p.ddraw_version, "OfflineCompat"),
        ("DDrawCompat.ini", "DDRAWCMP.INI|DDrawCompat.ini", p.ddraw_ini, "", "OfflineCompat"),
        ("DDrawCompat_LICENSE.txt", "DDCLIC.TXT|DDrawCompat-LICENSE.txt", p.ddraw_license, "", "OfflineCompat"),
    ];
    let joined: String = files.iter().map(|f| sha256_hex(f.2)).collect();
    let package_uuid = uuid::Uuid::new_v5(&PACKAGE_NAMESPACE, joined.as_bytes());
    let package_code = format!("{{{}}}", package_uuid.hyphenated()).to_uppercase();

    // our cabinet: entry names = File keys, in Sequence order, MSZIP like gcab -z
    let mut cb = cab::CabinetBuilder::new();
    {
        let folder = cb.add_folder(cab::CompressionType::MsZip);
        for f in &files {
            folder.add_file(f.0).set_datetime(cab_time);
        }
    }
    let mut cw = cb.build(Cursor::new(Vec::new()))?;
    let mut i = 0;
    while let Some(mut w) = cw.next_file()? {
        w.write_all(files[i].2)?;
        i += 1;
    }
    let cab_bytes = cw.finish()?.into_inner();

    let mut pkg = Package::open(Cursor::new(msi_bytes.to_vec())).context("opening the MSI")?;

    let last_seq = pkg
        .select_rows(Select::table("File"))?
        .map(|r| r["Sequence"].as_int().unwrap_or(0))
        .max()
        .unwrap_or(0);
    if last_seq != 508 {
        bail!("unexpected File table: last sequence {last_seq}, not 508");
    }

    let mut rows = Vec::new();
    for (n, (key, name, data, ver, comp)) in files.iter().enumerate() {
        rows.push(vec![
            Value::from(*key),
            Value::from(*comp),
            Value::from(*name),
            Value::Int(data.len() as i32),
            if ver.is_empty() { Value::Null } else { Value::from(*ver) },
            if ver.is_empty() { Value::Null } else { Value::from("1033") },
            Value::Int(16384), // msidbFileAttributesCompressed
            Value::Int(last_seq + 1 + n as i32),
        ]);
    }
    pkg.insert_rows(Insert::into("File").rows(rows)).context("File rows")?;

    pkg.insert_rows(
        Insert::into("Component")
            .row(vec![
                Value::from("OfflineLauncher"),
                Value::from(COMPONENT_LAUNCHER),
                Value::from("INSTALLDIR"),
                Value::Int(0),
                Value::Null,
                Value::from("RagnarokOffline.exe"),
            ])
            // DDrawCompat only from Vista on (it does not support XP, and XP's
            // own DirectDraw is the one the client was made for: docs/WINDOWS-XP.md)
            .row(vec![
                Value::from("OfflineCompat"),
                Value::from(COMPONENT_COMPAT),
                Value::from("INSTALLDIR"),
                Value::Int(0),
                Value::from("VersionNT >= 600"),
                Value::from("DDrawCompat.ini"),
            ]),
    )
    .context("Component rows")?;

    pkg.insert_rows(
        Insert::into("FeatureComponents")
            .row(vec![Value::from("NewFeature1"), Value::from("OfflineLauncher")])
            .row(vec![Value::from("NewFeature1"), Value::from("OfflineCompat")]),
    )
    .context("FeatureComponents rows")?;

    pkg.insert_rows(Insert::into("Media").row(vec![
        Value::Int(2),
        Value::Int(last_seq + files.len() as i32),
        Value::Null,
        Value::from("#offline.cab"),
        Value::Null,
        Value::Null,
    ]))
    .context("Media row")?;

    pkg.insert_rows(
        Insert::into("CustomAction")
            .row(vec![Value::from("OfflinePrepare"), Value::Int(CA_TYPE), Value::from("RagnarokOffline.exe"), Value::from("--prepare --quiet")])
            .row(vec![Value::from("OfflineUnprepare"), Value::Int(CA_TYPE), Value::from("RagnarokOffline.exe"), Value::from("--unprepare --quiet")]),
    )
    .context("CustomAction rows")?;

    pkg.insert_rows(
        Insert::into("InstallExecuteSequence")
            // before RemoveFiles (1750)
            .row(vec![Value::from("OfflineUnprepare"), Value::from("REMOVE~=\"ALL\""), Value::Int(1590)])
            // after InstallFiles, before InstallFinalize
            .row(vec![Value::from("OfflinePrepare"), Value::from("NOT REMOVE~=\"ALL\""), Value::Int(3110)]),
    )
    .context("InstallExecuteSequence rows")?;

    let n_shortcuts = pkg
        .select_rows(Select::table("Shortcut").with(Expr::col("Target").eq(Expr::string("[INSTALLDIR]Ragnarok.exe"))))?
        .count();
    if n_shortcuts == 0 {
        bail!("no shortcut to [INSTALLDIR]Ragnarok.exe in the MSI");
    }
    pkg.update_rows(
        Update::table("Shortcut")
            .set("Target", Value::from("[INSTALLDIR]RagnarokOffline.exe"))
            .set("Description", Value::from("Ragnarok Online (offline)"))
            .set("Icon_", Value::from("ragnarok.ico"))
            .set("IconIndex", Value::Int(0))
            .with(Expr::col("Target").eq(Expr::string("[INSTALLDIR]Ragnarok.exe"))),
    )
    .context("Shortcut rows")?;

    pkg.insert_rows(Insert::into("Icon").row(vec![Value::from("ragnarok.ico"), Value::Binary])).context("Icon row")?;
    pkg.write_stream("Icon.ragnarok.ico")?.write_all(icon)?;
    pkg.write_stream("offline.cab")?.write_all(&cab_bytes)?;

    pkg.summary_info_mut().set_uuid(package_uuid);
    pkg.flush()?;
    let msi = pkg.into_inner()?.into_inner();

    // Setup.ini: setup.exe checks the package it launches against this
    let text: String = setup_ini.iter().map(|&b| b as char).collect(); // cp1252, ASCII in practice
    if !text.split("\r\n").any(|l| l.starts_with("PackageCode=")) {
        bail!("Setup.ini has no PackageCode line");
    }
    let text = text
        .split("\r\n")
        .map(|l| if l.starts_with("PackageCode=") { format!("PackageCode={package_code}") } else { l.to_string() })
        .collect::<Vec<_>>()
        .join("\r\n");
    let setup_ini = text.chars().map(|c| c as u8).collect();

    Ok(Patched { msi, setup_ini, package_code })
}
