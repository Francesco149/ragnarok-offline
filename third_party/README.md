# third_party

Binaries fetched by `tools/fetch-third-party.sh` (pinned versions, sha256-checked) and
embedded into the Windows launcher. Not in git.

| what | version | licence | why |
|---|---|---|---|
| [DDrawCompat](https://github.com/narzoul/DDrawCompat) `ddraw.dll` | v0.7.2 | 0BSD | DirectDraw/Direct3D 7 fixes for Windows 10/11 (see docs/WINDOWS.md) |
| euRO patch server snapshot (`euro-patch/`) | Wayback captures 2004-02-21 (`patch2.txt`), 2004-07-18 (notices, `patch_allow.txt`) | euRO / Gravity, archived by web.archive.org | what `wpatch.euro-ro.net/patch/` served the CD's patcher; the launcher's local patch server serves it (docs/WINDOWS.md) |
