# Windows XP

The all-in-one installer and `RagnarokOffline.exe` run on Windows XP: install
from the disc image, double-click the desktop shortcut, and the CD's 2003
patcher shows euRO's notice, says "Patch process completed" and starts the
game against the server inside the launcher, as it did when euRO was online.

Tested 2026-11-18 on real hardware (`timemachine`: Windows XP Professional
SP3, 32-bit, i7-4790K, GTX 750 Ti on NVIDIA's last XP driver), from the
all-in-one installer, with nothing else installed for it:

1. `msiexec` install of the patched CD package; its deferred custom action
   prepares `data.grf` (16,326 files in about 2 s). No `ddraw.dll` is
   installed (below).
2. The launcher unpacks its data, starts the server (Lua rules, 1,261 NPCs,
   the login/char/map servers, the local patch server on HTTP and FTP),
   starts `Ragnarok.exe`: notice, patch list, **Start Game**.
3. The client: login (a new account), character creation, the novice
   training ground; windowed and native fullscreen 800×600×16.
4. Three first runs in a row from an empty data folder, then uninstall.

The setup builder (`RagnarokOfflineSetupBuilder.exe`, docs/WINDOWS.md) runs
there too, tested the same day: its window on XP's Luna theme at 800×600, a
download resumed over HTTPS (rustls: XP's own TLS stops at 1.0), a setup
folder built from it, installed and started as above, and the whole flow
through the window with `setup.exe` starting the 2003 wizard. On XP the
builder offers a folder by default: XP cannot open a disc image by itself.

## How

The same Rust code, built for 32-bit Windows with two additions
(`tools/xp/`, run `tools/wb exec tools/xp/build.sh`):

- **Target `i686-win7-windows-gnu`**, std rebuilt from source
  (`-Zbuild-std`), linked by llvm-mingw's msvcrt flavour so the C runtime is
  XP's own `msvcrt.dll` (Arch's MinGW links the Universal CRT, which XP
  lacks). The PE header says 5.1: XP refuses an image marked 6.0 as "not a
  valid Win32 application".
- **`tools/xp/xpcompat.c`**, our compatibility layer, linked before every
  import library. std, tokio/mio and the launcher import 27 functions
  XP's DLLs do not export (SRW locks, condition variables, INIT_ONCE, FLS,
  `GetQueuedCompletionStatusEx`, `BCryptGenRandom`...). The layer defines
  each import itself; on first call it takes the system's function if this
  Windows has one, else its own, built on what XP has (keyed events, TLS and
  a TLS callback, `RtlGenRandom`, the `Nt*InformationFile` calls). So the
  same exe uses the native functions on Vista to 11. Three functions XP has
  but answers with an error when asked for later behaviour are wrapped too:
  `FindFirstFileExW` (std's `read_dir` asks for Windows 7's
  `FindExInfoBasic`), `WSASocketW` (`WSA_FLAG_NO_HANDLE_INHERIT`) and
  `WSAIoctl(SIO_BASE_HANDLE)` (mio).
- **`tools/xp/impcheck.py`** checks every import of both exes against the
  export tables of XP SP3's own DLLs (copied from the test machine into
  `third_party/xp-sysdlls/`, git-ignored: they are Microsoft's). The build
  prints the result; it was 0 missing for both.

tokio's I/O on XP works through mio's AFD polling (`\Device\Afd`), which XP's
AFD driver accepts. `NtCancelIoFileEx` is XP's `NtCancelIoFile`, which only
cancels requests the calling thread made: not seen to matter so far.

On XP the layer also records crashes: `%TEMP%\RagnarokOffline-crash.txt`
gets the exception and the exe's return addresses on the stack (resolve them
with `llvm-nm`, image base 0x400000). The launcher logs panics to
`launcher.log`.

The all-in-one installer now carries this 32-bit launcher for every Windows,
and installs DDrawCompat only from Vista on (`VersionNT >= 600`): it does not
support XP, and XP's own DirectDraw is the one the client was made for. The
launcher skips it on XP too.

## Known issues

- **Black squares on the ground** on the GTX 750 Ti (NVIDIA 368.81, XP's
  last driver) with "Use Lightmap" on and Texture Detail at its top step:
  one on every ground tile, windowed and fullscreen, with either 3D device
  ("Direct3D HAL" or "T&L HAL"). The driver's profile is stock (no
  `nvdrsdb0/1.bin`, and its database has no Ragnarok profile, only one for
  Ragnarok Online 2). One step down (`TEXTUREMODE` 1) is clean, lightmaps on
  and still sharp; 0 is clean and softer. So the launcher writes 1 on XP.
  Probably the newer GPU's texture sampling against a DirectX 7 lightmap
  atlas; a period card would show whether 2003 hardware drew it right.
- **First-run settings.** The launcher writes windowed 800×600 on XP. For
  fullscreen, run `Setup.exe` (the "Setup" shortcut): its mode list depends
  on the card (800×600×16 was index 3 here, 1 under Wine and with DDrawCompat),
  so the launcher does not guess it.
- **Saves stay in the game folder after uninstall.** Program Files is
  writable on XP, so the launcher keeps `offline\` (saves, settings, logs)
  next to the game, and the uninstaller leaves it, as user data should be.
- Not verified yet: playing through the 32-bit launcher on Windows 10/11
  (it loads and prepares the folder on Windows 11; docs/WINDOWS.md),
  sound, a period graphics card, XP SP2 or earlier.

## Driving the test machine

The XP machine's xpdash/netexec tooling is in `../xpdash` and
`../retro-hardware` (`projects/xp-remote-probe`). For this test it was
cabled to maximus's second port (`enp3s0`, NetworkManager connection
`cutestation-tmp`, 10.77.0.1/24) with a dnsmasq on that port only handing out
10.77.0.100-150; `nxc smb … --exec-method smbexec` with `iexec` runs GUI
programs on its console, `nircmd savescreenshotfull` takes screenshots, and
WSH `SendKeys` types into the client (nircmd's mouse clicks reach dialogs but
not always the game).

The box has no route to the internet. To test the setup builder's download
there, a small HTTP CONNECT proxy on maximus at 10.77.0.1:3128, allowing
only `*.archive.org:443`, and `set HTTPS_PROXY=http://10.77.0.1:3128` in the
batch file that runs the builder: no forwarding or NAT on the host. Seed
the cache with all but the last half megabyte of our image (over SMB) and
only that half megabyte comes from the Archive.
