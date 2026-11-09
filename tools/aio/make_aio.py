#!/usr/bin/env python3
"""Build the all-in-one installer: the original 2003 euRO CD installer with
ragnarok-offline added to it.

    python3 -I tools/aio/make_aio.py CD_DIR OUT_DIR LAUNCHER_EXE DDRAW_DLL DDRAWCOMPAT_INI DDRAWCOMPAT_LICENSE

Runs in the workbench (needs msitools: msiinfo, msibuild, gcab). OUT_DIR gets a
copy of the CD layout (setup.exe, Setup.ini, Data1.cab, ... - Gravity's files,
so OUT_DIR is for your own use and never goes into git) in which only
"Ragnarok Online.msi" and Setup.ini differ:

- same dialogs, same 508 files, same install folder: the original UI is left
  exactly as it is (only table rows are added, nothing is replaced)
- + RagnarokOffline.exe, ddraw.dll (DDrawCompat), DDrawCompat.ini and its
  licence, in an embedded cabinet (#offline.cab, Media 2, sequences 509+)
- + deferred, elevated custom actions: `RagnarokOffline.exe --prepare --quiet`
  after the files are in place (merges the patches into data.grf the way the
  euRO patcher did, writes the client's first-run settings) and
  `--unprepare --quiet` before they are removed on uninstall
- the "Ragnarok Online" Start menu / desktop shortcuts start RagnarokOffline.exe
  instead of the 2003 patcher (whose servers are gone: it hangs, and its
  domain could be re-registered by anyone), with the CD's ragnarok.ico
- a new PackageCode (summary info + Setup.ini); ProductCode and UpgradeCode
  stay the CD's: it is the same product, and installing over an existing
  original install goes through Windows Installer's normal maintenance path.
"""

import hashlib
import os
import shutil
import subprocess
import sys
import tempfile
import uuid

# "Ragnarok Online.msi" from the preserved disc (docs/PRESERVATION.md)
ORIGINAL_MSI_SHA256 = "8b5207805d40b636101a2ff696f8f2eeaedf0df8c5dac35f7ffd6cc88673d768"
MSI = "Ragnarok Online.msi"
# Component GUIDs must never change between builds (component rules).
COMPONENT_LAUNCHER = "{6E1F4C2A-3B7D-4E8A-9C1F-2D5B8A7E0F11}"
COMPONENT_COMPAT = "{6E1F4C2A-3B7D-4E8A-9C1F-2D5B8A7E0F12}"
PACKAGE_NAMESPACE = uuid.UUID("6e1f4c2a-3b7d-4e8a-9c1f-2d5b8a7e0f00")
# 18 (exe from an installed File) + 64 (ignore exit code: the launcher retries
# on first start) + 1024 (deferred) + 2048 (no impersonation: elevated)
CA_TYPE = 18 + 64 + 1024 + 2048


def run(*cmd, **kw):
    return subprocess.run(cmd, check=True, **kw)


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def export(msi, table, d):
    out = run("msiinfo", "export", msi, table, capture_output=True).stdout
    with open(os.path.join(d, table + ".idt"), "wb") as f:
        f.write(out)
    return out.decode("cp1252").splitlines()


def write_idt(d, table, lines):
    with open(os.path.join(d, table + ".idt"), "wb") as f:
        f.write(("\r\n".join(lines) + "\r\n").encode("cp1252"))


STREAM_ALPHABET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz._"


def decode_stream_name(name):
    """MSI packs table/stream names into the 0x3800-0x4840 code point range."""
    out = ""
    for ch in name:
        c = ord(ch)
        if 0x3800 <= c < 0x4800:
            c -= 0x3800
            out += STREAM_ALPHABET[c & 0x3F] + STREAM_ALPHABET[c >> 6]
        elif 0x4800 <= c < 0x4840:
            out += STREAM_ALPHABET[c - 0x4800]
        elif c == 0x4840:
            out += "!"
        else:
            out += ch
    return out


def sort_tables(msi, schemas):
    """Windows Installer expects every table stream sorted by primary key
    (string columns by string id); it refuses to load one that is not (error
    2211, "could not create table"). msibuild appends new rows at the end, and
    new strings may reuse low ids, so re-sort the tables we touched. Tables
    are stored column by column; reordering rows keeps the stream size, so
    olefile can rewrite it in place.

    schemas: {table: [column types from the .idt]}, primary-key column count
    from the .idt's third line."""
    import olefile

    with olefile.OleFileIO(msi, write_mode=True) as o:
        entries = {decode_stream_name(e[0]): e for e in o.listdir()}
        long_refs = o.get_size(entries["!_StringPool"]) // 4 > 0xFFFF
        for table, (types, nkeys) in schemas.items():
            widths = [4 if t[0] in "iI" and t[1:] == "4" else 2 if t[0] in "iI" else (3 if long_refs else 2) for t in types]
            e = entries["!" + table]
            data = o.openstream(e).read()
            n = len(data) // sum(widths)
            assert n * sum(widths) == len(data), table
            cols, off = [], 0
            for w in widths:
                cols.append([int.from_bytes(data[off + i * w: off + (i + 1) * w], "little") for i in range(n)])
                off += w * n
            order = sorted(range(n), key=lambda r: tuple(cols[c][r] for c in range(nkeys)))
            if order != list(range(n)):
                o.write_stream(e, b"".join(b"".join(cols[c][r].to_bytes(w, "little") for r in order) for c, w in enumerate(widths)))


def main():
    cd, out, launcher, ddraw, ini, lic = sys.argv[1:7]
    src_msi = os.path.join(cd, MSI)
    if sha256(src_msi) != ORIGINAL_MSI_SHA256:
        sys.exit(f"{src_msi} is not the 2003-12-19 euRO installer this tool patches (sha256 mismatch)")

    # 1. the CD layout (Data1.cab hard-linked when possible: 634 MB)
    os.makedirs(out, exist_ok=True)
    for name in os.listdir(cd):
        s, t = os.path.join(cd, name), os.path.join(out, name)
        if os.path.isfile(s):
            if os.path.exists(t):
                os.remove(t)
            try:
                os.link(s, t) if name == "Data1.cab" else shutil.copy2(s, t)
            except OSError:
                shutil.copy2(s, t)
            os.chmod(t, 0o644)
    msi = os.path.join(out, MSI)

    files = [  # (File key, short|long name, path, version, component)
        ("RagnarokOffline.exe", "RAGOFF.EXE|RagnarokOffline.exe", launcher, "0.1.0.0", "OfflineLauncher"),
        ("ddraw.dll", "ddraw.dll", ddraw, "0.7.2.0", "OfflineCompat"),
        ("DDrawCompat.ini", "DDRAWCMP.INI|DDrawCompat.ini", ini, "", "OfflineCompat"),
        ("DDrawCompat_LICENSE.txt", "DDCLIC.TXT|DDrawCompat-LICENSE.txt", lic, "", "OfflineCompat"),
    ]
    package_code = "{%s}" % str(uuid.uuid5(PACKAGE_NAMESPACE, "".join(sha256(f[2]) for f in files))).upper()

    with tempfile.TemporaryDirectory() as tmp:
        # 2. our cabinet; entry names = File keys, in Sequence order
        stage = os.path.join(tmp, "stage")
        os.makedirs(stage)
        for key, _, path, _, _ in files:
            shutil.copy(path, os.path.join(stage, key))
        cab = os.path.join(tmp, "offline.cab")
        run("gcab", "-c", "-z", "-n", cab, *[os.path.join(stage, f[0]) for f in files])

        # 3. table rows (appended; the original rows stay as they are)
        idt = os.path.join(tmp, "idt")
        os.makedirs(idt)
        file_t = export(msi, "File", idt)
        last_seq = max(int(l.split("\t")[7]) for l in file_t[3:] if l.strip())
        assert last_seq == 508, last_seq
        for i, (key, name, path, ver, comp) in enumerate(files):
            lang = "1033" if ver else ""
            file_t.append("\t".join([key, comp, name, str(os.path.getsize(path)), ver, lang, "16384", str(last_seq + 1 + i)]))
        write_idt(idt, "File", file_t)

        comp_t = export(msi, "Component", idt)
        comp_t.append("\t".join(["OfflineLauncher", COMPONENT_LAUNCHER, "INSTALLDIR", "0", "", "RagnarokOffline.exe"]))
        comp_t.append("\t".join(["OfflineCompat", COMPONENT_COMPAT, "INSTALLDIR", "0", "", "DDrawCompat.ini"]))
        write_idt(idt, "Component", comp_t)

        fc_t = export(msi, "FeatureComponents", idt)
        fc_t += ["NewFeature1\tOfflineLauncher", "NewFeature1\tOfflineCompat"]
        write_idt(idt, "FeatureComponents", fc_t)

        media_t = export(msi, "Media", idt)
        media_t.append("\t".join(["2", str(last_seq + len(files)), "", "#offline.cab", "", ""]))
        write_idt(idt, "Media", media_t)

        ca_t = export(msi, "CustomAction", idt)
        ca_t.append(f"OfflinePrepare\t{CA_TYPE}\tRagnarokOffline.exe\t--prepare --quiet")
        ca_t.append(f"OfflineUnprepare\t{CA_TYPE}\tRagnarokOffline.exe\t--unprepare --quiet")
        write_idt(idt, "CustomAction", ca_t)

        seq_t = export(msi, "InstallExecuteSequence", idt)
        seq_t.append('OfflineUnprepare\tREMOVE~="ALL"\t1590')  # before RemoveFiles (1750)
        seq_t.append('OfflinePrepare\tNOT REMOVE~="ALL"\t3110')  # after InstallFiles, before InstallFinalize
        write_idt(idt, "InstallExecuteSequence", seq_t)

        sc_t = export(msi, "Shortcut", idt)
        for i, l in enumerate(sc_t):
            c = l.split("\t")
            if len(c) == 12 and c[4] == "[INSTALLDIR]Ragnarok.exe":
                c[4] = "[INSTALLDIR]RagnarokOffline.exe"
                c[6] = "Ragnarok Online (offline)"
                c[8], c[9] = "ragnarok.ico", "0"
                sc_t[i] = "\t".join(c)
        write_idt(idt, "Shortcut", sc_t)

        icon_dir = os.path.join(idt, "Icon")
        os.makedirs(icon_dir)
        shutil.copy(os.path.join(cd, "ragnarok.ico"), os.path.join(icon_dir, "ragnarok.ico"))
        write_idt(idt, "Icon", ["Name\tData", "s72\tv0", "Icon\tName", "ragnarok.ico\tragnarok.ico"])

        si_t = export(msi, "_SummaryInformation", idt)
        si_t = [("9\t" + package_code) if l.startswith("9\t") else l for l in si_t]
        write_idt(idt, "_SummaryInformation", si_t)

        tables = ["File", "Component", "FeatureComponents", "Media", "CustomAction",
                  "InstallExecuteSequence", "Shortcut", "Icon", "_SummaryInformation"]
        schemas = {}
        for t in tables[:-1]:
            with open(os.path.join(idt, t + ".idt"), "rb") as f:
                head = f.read().decode("cp1252").splitlines()
            schemas[t] = (head[1].split("\t"), len(head[2].split("\t")) - 1)
        for t in tables:  # one at a time, from the idt dir (stream files are resolved as <Table>/<name>)
            r = subprocess.run(["msibuild", msi, "-i", t + ".idt"], cwd=idt, capture_output=True, text=True)
            if r.returncode:
                sys.exit(f"msibuild: importing {t} failed: {r.stdout}{r.stderr}")
        run("msibuild", msi, "-a", "offline.cab", cab)
        sort_tables(msi, schemas)

    for name in os.listdir(out):  # libgsf temp files left by an interrupted msibuild
        if name.startswith(".gsf-save-"):
            os.remove(os.path.join(out, name))

    # 4. Setup.ini: setup.exe checks the package it launches against this
    ini_path = os.path.join(out, "Setup.ini")
    with open(ini_path, "rb") as f:
        text = f.read().decode("cp1252")
    text = "\r\n".join(("PackageCode=" + package_code) if l.startswith("PackageCode=") else l for l in text.split("\r\n"))
    with open(ini_path, "wb") as f:
        f.write(text.encode("cp1252"))

    # 5. check what we wrote
    got = run("msiinfo", "suminfo", msi, capture_output=True).stdout.decode("cp1252")
    assert package_code in got, got
    print(f"{out}: PackageCode {package_code}")
    for t in ("Media", "CustomAction", "Shortcut"):
        print(run("msiinfo", "export", msi, t, capture_output=True).stdout.decode("cp1252").strip().splitlines()[-2:])


if __name__ == "__main__":
    main()
