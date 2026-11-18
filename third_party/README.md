# third_party

Binaries fetched by `tools/fetch-third-party.sh` (pinned versions, sha256-checked) and
embedded into the Windows launcher. Not in git.

| what | version | licence | why |
|---|---|---|---|
| [DDrawCompat](https://github.com/narzoul/DDrawCompat) `ddraw.dll` | v0.7.2 | 0BSD | DirectDraw/Direct3D 7 fixes for Windows 10/11 (see docs/WINDOWS.md) |
| euRO patch server snapshot (`euro-patch/`) | Wayback captures 2004-02-21 (`patch2.txt`), 2004-07-18 (notices, `patch_allow.txt`) | euRO / Gravity, archived by web.archive.org | what `wpatch.euro-ro.net/patch/` served the CD's patcher; the launcher's local patch server serves it (docs/WINDOWS.md) |
| Arial 2.82 (`corefonts/arial32.exe`) | Microsoft core fonts for the Web, 2000 | Microsoft's EULA (the package may be passed on unchanged) | the font the client asks for (its table at `0x5fb410`: Arial for the English service); for Wine in the rig and the playable snapshot only, never shipped |
| [llvm-mingw](https://github.com/mstorsjo/llvm-mingw) (`llvm-mingw/`), msvcrt flavour | 20261020 | LLVM (Apache-2.0 with LLVM exception), mingw-w64 (permissive) | builds the Windows XP launcher against XP's own `msvcrt.dll` (`tools/xp/build.sh`, docs/WINDOWS-XP.md); a build tool, nothing of it is shipped beyond mingw-w64's CRT startup code |
| Rust 1.99.1 + rust-src (`rustup-xp/`) | 1.99.1 | MIT/Apache-2.0 | std rebuilt for `i686-win7-windows-gnu` (`-Zbuild-std`) in its own `RUSTUP_HOME`, so the workbench image is untouched |
| `xp-sysdlls/` | Windows XP SP3 | Microsoft's; never commit or publish | system DLLs copied from the XP test machine, only read for their export tables by `tools/xp/impcheck.py`. Not fetched: copy them from an XP `system32` |
