#!/usr/bin/env bash
# Build the 32-bit, Windows XP-compatible RagnarokOffline.exe, ragnarok-offline.exe
# and RagnarokOfflineSetupBuilder.exe (which embeds the first).
# Run in the workbench: tools/wb exec tools/xp/build.sh   (docs/WINDOWS-XP.md)
#
#   target   i686-win7-windows-gnu, std rebuilt from source (-Zbuild-std)
#   linker   llvm-mingw (msvcrt flavour: XP's own msvcrt.dll, no UCRT)
#   shim     tools/xp/xpcompat.c, linked first: the Vista+ functions XP lacks
#   check    tools/xp/impcheck.py against XP SP3's system DLLs, if present
#
# Needs (git-ignored, fetched by tools/fetch-third-party.sh):
#   third_party/rustup-xp      Rust 1.99.1 with rust-src
#   third_party/llvm-mingw/    llvm-mingw 20261020 msvcrt
# Output: target/xp/i686-win7-windows-gnu/release/{RagnarokOffline,ragnarok-offline,RagnarokOfflineSetupBuilder}.exe
set -euo pipefail
cd "$(dirname "$0")/../.."
ROOT=$PWD
LM=$ROOT/third_party/llvm-mingw/llvm-mingw-20261020-msvcrt-ubuntu-22.04-x86_64
SHIM=$ROOT/third_party/xp-shim-libs
OUT=$ROOT/target/xp
export RUSTUP_HOME=$ROOT/third_party/rustup-xp RUSTUP_TOOLCHAIN=1.99.1 RUSTC_BOOTSTRAP=1
export PATH=$LM/bin:$PATH
[ -x "$LM/bin/i686-w64-mingw32-clang" ] && [ -d "$RUSTUP_HOME/toolchains" ] ||
  { echo "missing toolchain: run tools/wb exec tools/fetch-third-party.sh" >&2; exit 1; }

# llvm-mingw has compiler-rt and libunwind where Rust's gnu targets ask for
# libgcc and libgcc_eh: alias them.
mkdir -p "$SHIM" "$OUT"
ln -sf "$LM/lib/clang/23/lib/windows/libclang_rt.builtins-i386.a" "$SHIM/libgcc.a"
ln -sf "$LM/i686-w64-mingw32/lib/libunwind.a" "$SHIM/libgcc_eh.a"

i686-w64-mingw32-clang -O2 -Wall -Wno-unused-function -c tools/xp/xpcompat.c -o "$OUT/xpcompat.o"
# cargo does not see a pre-linked object change: relink the two exes when it does
h=$(sha256sum "$OUT/xpcompat.o" | cut -c1-16)
if [ "$(cat "$OUT/xpcompat.hash" 2>/dev/null)" != "$h" ]; then
  rm -f "$OUT"/i686-win7-windows-gnu/release/deps/{RagnarokOffline,ragnarok_offline,RagnarokOfflineSetupBuilder}-*.exe
  rm -rf "$OUT"/i686-win7-windows-gnu/release/.fingerprint/{ro-launcher,ro-server,ro-setup-builder}-*
  echo "$h" > "$OUT/xpcompat.hash"
fi

T=i686-win7-windows-gnu
# The win7 target stamps the image "needs Windows 6.0"; XP's loader refuses
# anything above 5.1 ("not a valid Win32 application").
PE51=$(printf -- '-C link-arg=-Wl,--%s,%s ' major-os-version 5 minor-os-version 1 \
  major-subsystem-version 5 minor-subsystem-version 1)
export CARGO_TARGET_I686_WIN7_WINDOWS_GNU_LINKER=i686-w64-mingw32-clang
export CARGO_TARGET_I686_WIN7_WINDOWS_GNU_RUSTFLAGS="-L $SHIM -Z pre-link-arg=$OUT/xpcompat.o --cfg ro_xp $PE51 ${XP_EXTRA_RUSTFLAGS:-}"
export CC_i686_win7_windows_gnu=i686-w64-mingw32-clang AR_i686_win7_windows_gnu=llvm-ar
cargo build --release -j2 -Zbuild-std=std,panic_abort --target $T --target-dir "$OUT" \
  -p ro-launcher -p ro-server "$@"
# the setup builder carries the launcher just built (crates/ro-setup-builder/build.rs)
RO_SETUP_LAUNCHER=$OUT/$T/release/RagnarokOffline.exe \
  cargo build --release -j2 -Zbuild-std=std,panic_abort --target $T --target-dir "$OUT" -p ro-setup-builder

xp=${XP_SYSDLLS:-$ROOT/third_party/xp-sysdlls}
if [ -d "$xp" ]; then
  for e in RagnarokOffline ragnarok-offline RagnarokOfflineSetupBuilder; do
    echo "== imports of $e.exe against Windows XP SP3"
    python3 -I tools/xp/impcheck.py "$OUT/$T/release/$e.exe" "$xp" "$LM/bin/llvm-readobj" | grep -v " 0 missing"
  done
else
  echo "(no $xp: import check skipped; copy kernel32.dll etc. from an XP system32 there)"
fi
