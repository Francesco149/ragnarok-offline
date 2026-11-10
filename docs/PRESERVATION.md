# Disc preservation record

The owner's original pressed CD, ripped 2026-11-09 on maximus. This file is the
record; a copy sits next to the image on cold as `README.md`.

**Archive location:** `cold:/gigavault/archive/discs/ragnarok-online-euro-2003-12/`
(`gigavault/archive` is the snapshotted dataset — this is an original, not a
re-fetchable download). Working copy on maximus: `~/data/ragnarok-cd/`
(mounted at `/data` in the workbench).

**Public copy:** the image, its logs, hashes and photos of the disc are on
the Internet Archive since 2026-11-09:
<https://archive.org/details/ragnarok-online-euro-cd-2003-12-ichipm01>. `RAGNAROK.iso` and
`RAGNAROK.bin` there have the hashes listed below.

## The disc

| | |
|---|---|
| Volume id | `RAGNAROK` (ISO 9660 + Joliet UCS-3, no Rock Ridge) |
| Mastering | Application id `NERO_BURNING_ROM` |
| Medium | **pressed CD-ROM** — drive reports no CD-R medium / no ATIP |
| Layout | 1 session, 1 track, MODE1, 312,254 sectors (69:23:29), "NO COPY" flag |
| Label | "RAGNAROK ONLINE / italia", **"Codice: ICHIPM01"**; © 2003, 2004 Clarence srl (DADA group), Burda Holding International GmbH, Gravity |
| Sleeve | Italian SIAE sticker: "11 73541234 / S 00007998 / OMAGGIO / CD ROM / RAGNAROK / ONLINE EUROPE / VOGEL BURDA C" (a free gift; Vogel Burda published CHIP Italia) |
| Ring codes | mastering code `CLARERAGNOL` + `WWW.IMSSPA.IT` (replicated by IMS S.p.A., Italy), mastering SID `IFPI LV03`, mould SID `IFPI 2536` |
| Installer | InstallShield Express 3.5 MSI, created 2003-12-19 11:31:06; `Setup.ini` `Name=INTL`; ProductCode `{96CD74F2-8DA3-4582-9EEF-53924E36EEB6}`, UpgradeCode `{DBB4A523-5166-4122-956D-F6A1EABBF9F2}` |
| ARP links | `www.ruro-ro.net` (template placeholders otherwise left in: "Your Comments", 1-555-555-4505) |
| Installs to | `Program Files\Gravity\RagnarokOnline\` (508 files), shortcuts to `Ragnarok.exe` |

Most likely the cover CD of CHIP (Italy), June 2004: euRO's Italian partner
Clarence wrote "Dal 20 maggio puoi trovare Ragnarok Online anche su CD-rom,
allegato alla rivista CHIP di giugno!"
([Wayback, 2004-05-28](https://web.archive.org/web/20040528162600/http://www.clarence.com:80/ragnarok/rodownload.html)).
Identification of what is *on* it lives in [CLIENT.md](CLIENT.md).

## Files in the archive

| file | what | sha256 |
|---|---|---|
| `RAGNAROK.iso` | 2048-byte user data, ddrescue | `b5d579a4edd9c366e52460565e30aa5b48984eb4d40c9b193fdc6c8c35bd19a6` |
| `RAGNAROK.bin` / `.cue` / `.toc` | raw 2352-byte sectors (sync+header+EDC/ECC), cdrdao `read-cd --read-raw`, LBA 991 re-read (step 4 below) | bin `e677024ed2cb9fbe8ba037ca38dcbcb301c09298e60e2ca51c1dd6982f642b52` |
| `RAGNAROK.cdrdao-lba991.bin` | LBA 991 as the first cdrdao read returned it; written back at byte 991 × 2352 it rebuilds that read (sha256 `8453bc10…fac63`) | |
| `redumper-pioneer-dvd120/` | second raw read (step 5): redumper log, cue, toc, full TOC, subchannel | |
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
Of `RAGNAROK.bin`: md5 `3a1eaa5cc68e8d2f37888b511450b14b`,
sha1 `78cb213a573e9a23ef46bbd9c6ecc12e4ba069f9`, crc32 `d7f57d77`.

## How it was read and verified

Drives: Optiarc DVD RW AD-5290S+ (rev 1.RN) for steps 1–4, Pioneer DVD-ROM
DVD-120 (rev 1.23, IDE) for step 5.

1. `ddrescue -b 2048 -r 5 -d /dev/sr0 RAGNAROK.iso map` — one pass, 0 read
   errors, 639,496,192 bytes = exactly 312,254 × 2048 (the TOC lead-out).
2. Independent second read: `dd if=/dev/sr0 bs=2048 count=312254 iflag=direct | sha256sum`
   → identical hash.
3. Raw read: `cdrdao read-cd --read-raw --driver generic-mmc-raw` →
   734,421,408 bytes = 312,254 × 2352; every sector has mode byte 1, and the
   2048-byte user data of all sectors hashes to the ISO's sha256 (third read).
4. Sector check of the raw read (sync, BCD header, mode, EDC, zero fill, ECC
   P and Q; checker after Neill Corlett's ECM): every sector passes except
   LBA 991, whose ECC Q parity had 3 wrong bytes (offsets 2267, 2318, 2319),
   outside the user data. Three fresh single-sector `READ CD` reads agree with
   each other and pass, and replace that sector in the BIN. The bad copy is
   kept as `RAGNAROK.cdrdao-lba991.bin`.
5. Second drive: redumper b768 `disc` (generic profile, read offset +691 from
   AccurateRip): 0 SCSI and 0 C2 errors, and its BIN is byte-identical to
   `RAGNAROK.bin`, which confirms step 4. Both drives return data sectors
   already descrambled, so the disc's write offset is unknown (the log's
   "-691" is only the negated read offset) and this is not a Redump-verified
   dump. That needs a drive that returns scrambled sectors (Plextor, or a
   MediaTek-based LG/ASUS).

Four reads on two drives agree: three of the user data (block layer and raw
SCSI) and two raw reads giving the same BIN byte for byte. The raw bin is kept
because it is what redump-style tools expect.

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
