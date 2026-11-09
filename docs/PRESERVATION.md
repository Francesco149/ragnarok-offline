# Disc preservation record

The owner's original pressed CD, ripped 2026-11-09 on maximus. This file is the
record; a copy sits next to the image on cold as `README.md`.

**Archive location:** `cold:/gigavault/archive/discs/ragnarok-online-euro-2003-12/`
(`gigavault/archive` is the snapshotted dataset — this is an original, not a
re-fetchable download). Working copy on maximus: `~/data/ragnarok-cd/`
(mounted at `/data` in the workbench).

## The disc

| | |
|---|---|
| Volume id | `RAGNAROK` (ISO 9660 + Joliet UCS-3, no Rock Ridge) |
| Mastering | Application id `NERO_BURNING_ROM` |
| Medium | **pressed CD-ROM** — drive reports no CD-R medium / no ATIP |
| Layout | 1 session, 1 track, MODE1, 312,254 sectors (69:23:29), "NO COPY" flag |
| Installer | InstallShield Express 3.5 MSI, created 2003-12-19 11:31:06; `Setup.ini` `Name=INTL`; ProductCode `{96CD74F2-8DA3-4582-9EEF-53924E36EEB6}`, UpgradeCode `{DBB4A523-5166-4122-956D-F6A1EABBF9F2}` |
| ARP links | `www.ruro-ro.net` (template placeholders otherwise left in: "Your Comments", 1-555-555-4505) |
| Installs to | `Program Files\Gravity\RagnarokOnline\` (508 files), shortcuts to `Ragnarok.exe` |

Identification of what is *on* it lives in [CLIENT.md](CLIENT.md).

## Files in the archive

| file | what | sha256 |
|---|---|---|
| `RAGNAROK.iso` | 2048-byte user data, ddrescue | `b5d579a4edd9c366e52460565e30aa5b48984eb4d40c9b193fdc6c8c35bd19a6` |
| `RAGNAROK.bin` / `.cue` / `.toc` | raw 2352-byte sectors (sync+header+EDC/ECC), cdrdao `read-cd --read-raw` | bin `8453bc103ad357eeb78a99b71f12efd5b46c6bae644e2be16ff65b2cf91fac63` |
| `RAGNAROK.ddrescue.map`, `ddrescue.log` | rescue map: one `+` region, 0 bad sectors | |
| `cdrdao.log`, `cdrdao-disk-info.txt` | raw read log, medium info | |
| `iso-files.sha256` | hashes of the 9 files on the disc | |
| `installed-files.sha256` | hashes of the 508 files the MSI installs | |
| `msi-tables/` | every MSI table exported as `.idt` (`msiinfo export`) | |
| `CLIENT.md` | identification of the client (copy of docs/CLIENT.md) | |
| `packet_lengths.tsv` | the 386 packet lengths extracted from Ragexe.exe | |
| `SHA256SUMS` | hashes of everything above except the two .md files | |

Other hashes of `RAGNAROK.iso`: md5 `7baf518ac29a797b528885705156014f`,
sha1 `3d3b23207b2bb74df0a92d5b1ab3d2670f0f890a`, crc32 `22564766`.

## How it was read and verified

Drive: Optiarc DVD RW AD-5290S+ (rev 1.RN).

1. `ddrescue -b 2048 -r 5 -d /dev/sr0 RAGNAROK.iso map` — one pass, 0 read
   errors, 639,496,192 bytes = exactly 312,254 × 2048 (the TOC lead-out).
2. Independent second read: `dd if=/dev/sr0 bs=2048 count=312254 iflag=direct | sha256sum`
   → identical hash.
3. Raw read: `cdrdao read-cd --read-raw --driver generic-mmc-raw` →
   734,421,408 bytes = 312,254 × 2352; every sector has mode byte 1, and the
   2048-byte user data of all sectors hashes to the ISO's sha256 (third read).

Three reads, two drives paths (block layer and raw SCSI), one hash. The raw
bin is kept as well because it is what redump-style tools expect.

## Unpacking (reproducible, nothing here needs the disc)

```sh
tools/wb exec bash -c 'cd /data && mkdir iso && cd iso && 7z x ../RAGNAROK.iso'
tools/wb exec bash -c 'cd /data/client && cp "../iso/Ragnarok Online.msi" ../iso/Data1.cab . && msiextract -C out "Ragnarok Online.msi"'
```

`installed-files.sha256` checks the result.

## Restore

```sh
rsync -a cold:/gigavault/archive/discs/ragnarok-online-euro-2003-12/ ~/data/ragnarok-cd/
cd ~/data/ragnarok-cd && sha256sum -c SHA256SUMS
```

To burn a copy: `cdrdao write RAGNAROK.toc` (raw) or any burner with the
`.cue`/`.iso`.
