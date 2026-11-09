# Modern Windows

How the 2003 CD and client behave on Windows 10/11, and the two ways this
project makes them "just work" while staying as close to the XP-era
experience as possible:

1. **`RagnarokOffline.exe`**: drop it into an installed game folder and
   double-click it.
2. **The all-in-one installer**: the original CD installer with our files
   added, built from your own disc.

Everything below was tested on 2026-11-09 on Windows 11 Pro 25H2 (build
26200), NVIDIA RTX 5060, 1920×1080 at 100% scaling, unless it says otherwise.
The test rig is `tools/win-test` (DEVELOPMENT.md).

## Playing

### Drop-in: `RagnarokOffline.exe`

Copy `RagnarokOffline.exe` (from the Windows zip, or `dist/RagnarokOffline-<ver>.exe`)
into the folder that has `Ragexe.exe` and `data.grf`, then double-click it.
It is one 11 MB file with the server, its Lua scripts and game database,
DDrawCompat, and a snapshot of euRO's patch server inside.

First run, once:

- **Prepares the folder.** It merges the CD's Turtle Island/Juno patches and
  our `clientinfo.xml` into `data.grf`, as the euRO patcher did; the CD's file
  is kept as `data.grf.original`. It also writes DDrawCompat's `ddraw.dll`
  and `DDrawCompat.ini` next to `Ragexe.exe`. It never overwrites a
  `ddraw.dll` that isn't ours, and never overwrites the `.ini`. A small
  progress window shows while it works; on an SSD this takes about a second.
- **Asks for administrator rights only if the folder is read-only** (for
  example under `C:\Program Files (x86)`). It relaunches itself through UAC
  with `--prepare`; the game itself never runs elevated.
- **Writes the client's settings if its `Setup.exe` never ran:** 800×600×16
  fullscreen (emulated, see below), sound on. These are the same values the
  client's own `Setup.exe` writes, so the original `Setup.exe` (still in the
  folder, and still working) can change them later.

Every run:

- **Unpacks the server's `scripts/` and `data/db/`** into a data folder:
  `<game>\offline\` if the game folder is writable, otherwise
  `%LOCALAPPDATA%\Ragnarok Offline\`. Hand-written scripts you have edited
  are never overwritten; the generated ones (`scripts\import\`, translated
  from the era's emulators) and `data\db\` are replaced when a new launcher
  brings different ones, so override imported NPCs from `scripts\npc\`. The folder also holds `ragnarok-offline.toml` (rates, start
  items...), `saves\` and `logs\`.
- **Starts the server in-process, then starts the game the way the CD did:
  through its patcher, `Ragnarok.exe`.** The patcher shows euRO's notice,
  checks the patch list, reports "Patch process completed" and offers
  **Start Game**, which runs `Ragexe.exe 1rag1`. Its requests now go to a
  local stand-in for euRO's patch servers, which serves an archived snapshot
  (see "The patcher" below). When the patcher and the game have both exited,
  the launcher saves and stops the server. If port 6900 is already taken (a
  server you started by hand), it only starts the game.

Other switches: `--direct` skips the patcher and starts `Ragexe.exe 1rag1`
itself; `--settings` runs the client's `Setup.exe`; `--prepare` /
`--unprepare` prepare the folder or put the CD's `data.grf` back, and
`--quiet` suppresses dialogs (exit code 0 means success).

Under Wine the same exe works, patcher included, but skips DDrawCompat,
since Wine's own DirectDraw is fine. The Linux build (`RagnarokOffline` in
the tarball) runs `wine Ragexe.exe 1rag1` directly, without the patcher.

### All-in-one installer (from your own CD)

```sh
tools/wb exec tools/aio/build.sh            # CD files from /data/iso (= ~/data/ragnarok-cd/iso)
# -> ~/data/ragnarok-cd/aio/RagnarokOffline-CD/      the CD layout, install from setup.exe
# -> ~/data/ragnarok-cd/aio/RagnarokOffline-CD.iso   the same as a disc image (volume RAGNAROK)
```

The output contains Gravity's installer and data: it is for your own use.
Never commit or publish it. Only "Ragnarok Online.msi" and `Setup.ini`
differ from the disc:

- **Same dialogs, artwork, files and default folder.** Rows are only added
  to the MSI, never replaced, so the original wizard runs exactly as it did.
- **Added files.** `RagnarokOffline.exe`, `ddraw.dll`, `DDrawCompat.ini` and
  `DDrawCompat-LICENSE.txt` are installed from an embedded cabinet
  (`#offline.cab`, Media 2).
- **Prepared at install time.** A deferred, elevated custom action runs
  `RagnarokOffline.exe --prepare --quiet` once the files are in place, so the
  first launch needs no UAC prompt. On uninstall `--unprepare` runs before
  the files are removed, and the folder disappears completely. Saves in
  `%LOCALAPPDATA%\Ragnarok Offline` are kept, as user data should be.
- **Shortcuts.** The desktop and Start-menu "Ragnarok Online" shortcuts start
  `RagnarokOffline.exe`, with the CD's `ragnarok.ico`. On the CD they pointed
  straight at the 2003 patcher; the launcher still shows that patcher, but
  with its requests answered locally. The "Setup" shortcut is unchanged.
- **Same product.** The PackageCode is new; ProductCode and UpgradeCode stay
  the CD's.

Unattended installs follow the CD's own rules: the wizard's "Typical" sets
`INSTALLLEVEL` to 200, which includes the two skin sets, 508 files in all. A
silent `msiexec /i "Ragnarok Online.msi" /qn` stays at the default level of
100 (96 files, no skins), so pass `INSTALLLEVEL=200` to match "Typical".

Tested on Windows 11, from the ISO mounted as a drive:

1. Install through `setup.exe` with the original dialogs (about 15 s).
2. The desktop shortcut, with no UAC prompt.
3. The original patcher with euRO's notice, then **Start Game**.
4. The game in fullscreen: login, a new character, the map.
5. Uninstall with nothing left in Program Files.

In VirtualStore the patcher leaves only its three small text files, as any
2003 program would. `tools/aio/make_aio.py` documents each
table change. Note: msitools appends rows unsorted, which Windows refuses
(error 2211), so the script re-sorts the touched tables by primary key.

## The original installer on Windows 11

| | 2003 / XP | Windows 11 | |
|---|---|---|---|
| `setup.exe` (InstallShield Express 3.5 bootstrapper, 2002) | runs | runs; 32-bit with a comctl32 v6 manifest. `Setup.ini` lists only Win95–2000, but `SuppressWrongOS=Y`, so there is no warning | tested |
| Elevation | none (XP users were admins) | the name "setup.exe" triggers UAC Installer Detection. A default Win11 shows a yellow "unknown publisher" prompt; the test box elevates silently (`ConsentPromptBehaviorAdmin=0`), so the prompt itself was not seen | tested / inferred |
| Wizard UI | MSI dialogs from `msiexec`: same bitmaps, text and layout | identical content and layout at 100% DPI. The window chrome follows the Win11 theme: title bar, rounded buttons, a smooth progress bar instead of XP's Luna (or Classic) blocks. Tahoma 8 is drawn with ClearType | tested |
| Default folder | `C:\Program Files\Gravity\RagnarokOnline` | `C:\Program Files (x86)\...` (32-bit package on 64-bit Windows) | tested |
| Files | 508 | all 508 byte-identical to `installed-files.sha256`, in about 12 s from a mounted ISO | tested |
| Shortcuts | `Ragnarok.exe` (the patcher) | the patcher waits forever on "Attempt to connect patch server": `wpatch`/`fpatch.euro-ro.net` are NXDOMAIN and `euro-ro.net` has no A record. The domain is still registered (since 2003-12-10, at Cloudflare, expiring 2026-12-10). If its owner ever points those names somewhere, that patcher would download and merge whatever it is served. The launcher redirects it locally instead (below) | tested |
| Game folder writes | Users could write to Program Files | read-only for Users. The unmanifested client's writes (Chat\, ScreenShot\, its HKLM settings) are silently redirected to `%LOCALAPPDATA%\VirtualStore` and `HKCU\Software\Classes\VirtualStore` | tested |
| Custom actions | – | none that run code (one property setter, one async "check for updates" that is allowed to fail) | MSI tables |
| Uninstall | | clean | tested |

**Restoring the XP look of the installer.** The content is already identical.
The differences are OS chrome and ClearType text, which Windows applies to
every themed app. Since Windows 8 there is no per-application switch back
(the "disable visual themes" compatibility flag is a no-op), and changing the
system theme for one installer is not acceptable. The only faithful
reference would be the installer running on real XP. VMware Workstation is
on the test box, so an XP VM would provide one, given XP install media.

## The patcher and the local patch server

**What the patcher does.** `Ragnarok.exe` is dated 2003-12-12 and is plain
Win32 and WinINet. Its behaviour was observed under Wine against logging
servers, then checked in its code:

1. **HTTP:** `http://wpatch.euro-ro.net/patch/` + `patch_allow.txt`. That
   file says "allow", "deny" (then it shows `notice_close.txt`) or "xmas"
   (then it shows `notice_xmas.txt` and needs `2003-10-31gdata_xmas.gpf`).
   Then `notice_patch.txt`, shown in the "RO Notice" box, and `patch2.txt`,
   the patch list. Each response needs a Content-Length, and each file is
   saved into the game folder.
2. **FTP:** port 21 on `fpatch.euro-ro.net`, anonymous, passive mode:
   `CWD /patch`, `TYPE A`, `PASV`, `LIST`. It does this even when there is
   nothing to download. Then it fetches each patch above the level in
   `patch.inf`.
3. **The patch list.** Each line is `<number> <file>`; lines starting with
   `//` are skipped, and a bare number counts as applied. When the patcher
   raises `patch.inf` it also rewrites `data.grf`'s file table, which happens
   even if nothing was merged, and it then needs exclusive write access to
   `data.grf`.
4. **Start Game** runs `ragexe.exe 1rag1`, and the patcher exits.

The CD's `patch.inf` is 303. In the earliest archived list, entry 303 is
`2003-12-19gdata_g_b.gpf`, dated the day the CD was mastered.

**Redirect without changing the exe.** The two host names are 64-byte fields
in the patcher's `.data` section, at `0x41d210` and `0x41d250`. The image
base is fixed at 0x400000 and there are no relocations. The launcher:

- starts `Ragnarok.exe` suspended, inside a job object;
- checks that both fields still say `wpatch.euro-ro.net` / `fpatch.euro-ro.net`;
- writes `127.0.0.1:6950` and `127.0.0.1` over them, then resumes it;
- waits until the job is empty, which covers the game the patcher started.

The exe on disk is never modified. The launcher uses this path only for the
known patcher (by sha256), and falls back to starting `Ragexe.exe` directly
if anything doesn't match or port 21/6950 can't be bound. The FTP port can't
be moved: `InternetConnectA` is called with the constant 21, which Windows
lets a normal user bind.

**The snapshot** (`tools/fetch-third-party.sh` → `third_party/euro-patch/`,
embedded in the launcher, unpacked to `<data>\patchsrv\`, where you can edit
it). These are the Wayback Machine's captures of
`http://wpatch.euro-ro.net/patch/`, the closest ones that exist. Each file's
SHA-1 was checked against the archive's digest.

| file | capture | |
|---|---|---|
| `patch2.txt` | 2004-02-21 (earliest; two weeks into open beta) | served at the CD's level, below |
| `notice_patch.txt` | 2004-07-18 (earliest) | as archived: July 2004 news in four languages (Juno Kafra, "The_Paper") |
| `notice_close.txt` | 2004-07-18 | as archived: weekly maintenance, Wednesdays 07:00–10:00 CET |
| `patch_allow.txt` | 2004-07-18 | `allow` |
| `/` (`index.html`) | 2004-02-21 | euRO's front page; the client's `registrationweb` now points at it |

**Served at the CD's level.** The list is served as archived, except that
every numbered line above 303 is commented out with the list's own `//`,
plus a one-line note at the top. Their patch files (`.gpf`/`.rgz` over FTP)
were never archived. Serving them as listed would also make the patcher
raise `patch.inf` and rewrite `data.grf`. Under Program Files that rewrite
means a 490 MB VirtualStore copy, and a clash with the server reading the
same file. At the CD's level the patcher finds nothing to do and changes
nothing.

**`clientinfo.xml`** is now the CD's own file (servicetype `germany`, the
empty `hideaccountlist`, `desc` None) with only the address, port, display
name and `registrationweb` changed. As a result the client goes straight to
LogOn, as with the untouched CD. The client only opens `registrationweb`
from a UI button that euRO's login window doesn't have, but if it ever does,
the URL is local.

**Not archived, so not served:**

- the patch files;
- anything from the December 2003 closed beta;
- the website's images and other pages, which the Wayback Machine has only
  partly.

## The client on Windows 11

- **Native DirectX 7 works, windowed.** `Ragexe.exe` reaches the login
  screen on the RTX 5060 with no wrapper.
- **The client's `Setup.exe` offers only 16-bit fullscreen modes at least 480
  lines tall.** That is the DirectX 7 SDK `d3denum` sample's mode filter (code
  near `0x4033e1`); its `DEVICECNT`/`MODECNT` values are indices into the
  device and mode lists. Windows 8+ only emulates 8/16-bit modes, and on the
  test box their enumeration was unreliable: present on the first run, absent
  later, with `EnumDisplaySettings` listing none. Then the resolution list is
  empty. With DDrawCompat it is always filled.
- **Native fullscreen would switch the real display mode** (emulated 16 bpp),
  with the usual side effects: rearranged desktop icons and slow Alt-Tab. It
  was deliberately not tried on the test box.
- **Text.** The client draws all text with GDI: `CreateFontA` with the face
  and charset picked by service type. For euRO ("germany") that is Arial with
  `ANSI_CHARSET`, at `DEFAULT_QUALITY`, so the system's font smoothing
  applies. With ClearType (the Win11 default) the antialiased edges blend
  with the colour-keyed (magenta) text surface, and names and stats come out
  magenta and bold. XP's default "Standard" smoothing leaves Arial aliased at
  these sizes (its gasp table), so `FontAntialiasing = off` gives both the
  correct and the period look.
- **`¡æ` and `¡Ø` in the tips of the day.** `tipOfTheDay.txt` contains
  CP949/EUC-KR symbols (`→` A1E6, `※` A1D8) left over from Gravity's Korean
  text. Since the client renders with `ANSI_CHARSET`, they show as Latin-1
  glyphs on any Windows, including a Korean-locale XP. That is the 2003 look,
  so it is kept as is.
- **Registry.** Settings live in `HKLM\SOFTWARE\WOW6432Node\Gravity Soft\Ragnarok`.
  A non-elevated client or `Setup.exe` actually reads and writes the
  per-user VirtualStore copy (`HKCU\Software\Classes\VirtualStore\MACHINE\SOFTWARE\WOW6432Node\Gravity Soft\Ragnarok`).
  `Ragexe.exe` adds `ID`, `NUMACCOUNT`, `CHECKSAVE`, `BITWISE` and
  `NUMSERVER` there.

## Compatibility layer: DDrawCompat

Chosen: **[DDrawCompat](https://github.com/narzoul/DDrawCompat) v0.7.2** (0BSD,
fetched pinned and hash-checked by `tools/fetch-third-party.sh`). It is the
original `ddraw.dll`/Direct3D 7 runtime, with fixes hooked in, so the
client's own rendering path is kept. The alternatives:

| | |
|---|---|
| dgVoodoo2 | reimplements DirectX on D3D11/12, very compatible, but freeware without a clear redistribution licence in its repository |
| DxWrapper | zlib; converts DirectDraw/D3D7 to D3D9 (a reimplementation, 10 MB) |
| cnc-ddraw | MIT, but 2D DirectDraw only; this client needs Direct3D 7 |

Our `DDrawCompat.ini` (`crates/ro-launcher/assets/DDrawCompat.ini`):

| setting | why |
|---|---|
| `FontAntialiasing = off` | the XP look, and no magenta text fringes (see above) |
| `SupportedResolutions = 640x480, 800x600, 1024x768, 1152x864, 1280x960, 1280x1024, 1600x1200` | the 4:3/5:4 modes a 2003 monitor offered, in `Setup.exe`'s 16-bit list. This also fixes `MODECNT` = 1 for 800×600. Add `native` for the monitor's own modes |
| `RenderColorDepth = appd8` | 16-bit modes are rendered at 8 bits per channel, then ordered-dithered to 5:6:5, like a 2003 GPU's 16-bit output. DDrawCompat dithers after scaling, so the pattern is on the display's pixel grid, not the game's: an approximation. `32` gives smooth colour |
| defaults kept | `DisplayResolution = desktop` (fullscreen is a borderless window at desktop resolution, so the display mode never changes); `DisplayAspectRatio = app` (4:3 with black bars); `DisplayFilter = bilinear(0)`; `CpuAffinity = 1`; overlay on Shift+F11 |

Tested with DDrawCompat: `Setup.exe` lists the modes and saves them. The
client works in emulated fullscreen 800×600 scaled to 1440×1080 with black
bars, and windowed. A full session ran in both: login, character creation,
the map. Restoring a minimized fullscreen client brings back the scaled
picture (keyboard afterwards: see "Not verified yet"). Text is aliased.

DDrawCompat in fullscreen clips the cursor to the emulated resolution and
rescales relative mouse motion, so absolute synthetic input (the test rig)
needs feedback. `GameClick` in `tools/win-test` handles that. A real mouse is
unaffected.

### Things the launcher has to get right

- **Foreground.** A fullscreen DirectDraw client that can't become the
  foreground window never gets exclusive mode. With DDrawCompat it stays a
  hidden 800×600 window and then minimizes. Windows only grants the
  foreground to processes started by the foreground process. The launcher
  therefore shows its "Starting..." window while the server starts, closes
  it, then spawns the client and calls `AllowSetForegroundWindow`. The window
  must be gone, and must not be topmost, by then, or the client deactivates.
- **Waiting for the right process.** The launcher waits for the patcher's
  job object to empty, not for one process: the game the patcher starts is
  in the job too.
- **No virtualization for ourselves.** The launcher's manifest
  (`asInvoker`) turns UAC virtualization off for it, so it sees the real
  permissions of Program Files. The client keeps virtualization, which is why
  its settings may live in the VirtualStore copy.

## Not verified yet

- Keyboard input after a real Alt-Tab out of and back into fullscreen: the
  rig's synthetic minimize/restore left typing dead, and that needs checking
  by hand with a real keyboard. If it fails, try DDrawCompat's
  `AltTabFix = keepvidmem`.
- The UAC prompt on a default-configured Windows (`setup.exe` and the
  launcher's one-time `--prepare` from a read-only folder).
- High-DPI displays: DDrawCompat makes the client per-monitor DPI aware, so
  windowed 800×600 is small on a 4K screen; fullscreen scales.
- Windows 10, AMD/Intel GPUs, and sound output (sound init succeeds, but
  nobody listened on the test box).
- A side-by-side with real Windows XP.
- Port 21 or 6950 already in use on someone's machine: the launcher then
  skips the patcher, which is tested under Wine but not on Windows.
