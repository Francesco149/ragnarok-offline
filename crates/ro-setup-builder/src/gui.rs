//! The builder's window: plain Win32 controls (comctl32 6 through the
//! manifest), so it looks native from Windows XP to 11.
//!
//! One window: the disclaimer to accept, where the disc comes from (the
//! Internet Archive, or the user's own image or CD drive), what to make (disc
//! image, zip or folder) and where, a progress bar. The build runs on a
//! worker thread that posts WM_APP_UPDATE; Cancel sets a flag the worker
//! checks between reads.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoTaskMemFree};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Controls::Dialogs::*;
use windows_sys::Win32::UI::Controls::*;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{EnableWindow, SetFocus};
use windows_sys::Win32::UI::Shell::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

use crate::job::{self, Format, Source};
use crate::{DISCLAIMER, Ours, ia};

const TITLE: &str = "Ragnarok Offline Setup Builder";
const WM_APP_UPDATE: u32 = WM_APP + 1;

const ID_DISCLAIMER: i32 = 100;
const ID_ACCEPT: i32 = 101;
const ID_SRC_DL: i32 = 110;
const ID_SRC_LOCAL: i32 = 111;
const ID_SRC_PATH: i32 = 112;
const ID_SRC_BROWSE: i32 = 113;
const ID_FMT_ISO: i32 = 120;
const ID_FMT_ZIP: i32 = 121;
const ID_FMT_DIR: i32 = 122;
const ID_OUT_PATH: i32 = 123;
const ID_OUT_BROWSE: i32 = 124;
const ID_FMT_HINT: i32 = 125;
const ID_KEEP: i32 = 130;
const ID_PROGRESS: i32 = 140;
const ID_STATUS: i32 = 141;
const ID_GO: i32 = 150;
const ID_CLOSE: i32 = 151;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn from_wide(w: &[u16]) -> String {
    let end = w.iter().position(|&c| c == 0).unwrap_or(w.len());
    String::from_utf16_lossy(&w[..end])
}

// ------------------------------------------------------------ small helpers

pub fn local_app_data() -> Option<PathBuf> {
    // SHGetFolderPath: XP has no %LOCALAPPDATA% variable, and no KNOWNFOLDERID API
    let mut buf = [0u16; 260];
    let hr = unsafe { SHGetFolderPathW(std::ptr::null_mut(), CSIDL_LOCAL_APPDATA as i32, std::ptr::null_mut(), 0, buf.as_mut_ptr()) };
    (hr == 0).then(|| PathBuf::from(from_wide(&buf)))
}

fn desktop() -> Option<PathBuf> {
    let mut buf = [0u16; 260];
    let hr = unsafe { SHGetFolderPathW(std::ptr::null_mut(), CSIDL_DESKTOPDIRECTORY as i32, std::ptr::null_mut(), 0, buf.as_mut_ptr()) };
    (hr == 0).then(|| PathBuf::from(from_wide(&buf)))
}

/// Started from a console with arguments: print there, unless the output
/// already goes somewhere (redirected to a file or a pipe).
pub fn attach_console() {
    use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole, GetStdHandle, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};
    unsafe {
        let has = |h| {
            let h = GetStdHandle(h);
            !h.is_null() && h != windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE
        };
        if !has(STD_OUTPUT_HANDLE) && !has(STD_ERROR_HANDLE) {
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }
}

fn is_nt5() -> bool {
    let v = unsafe { windows_sys::Win32::System::SystemInformation::GetVersion() };
    (v & 0xff) < 6
}

/// Windows 8 and later open a disc image with a double-click.
fn can_mount_iso() -> bool {
    let v = unsafe { windows_sys::Win32::System::SystemInformation::GetVersion() };
    let (major, minor) = ((v & 0xff) as u32, ((v >> 8) & 0xff) as u32);
    major > 6 || (major == 6 && minor >= 2)
}

fn item(h: HWND, id: i32) -> HWND {
    unsafe { GetDlgItem(h, id) }
}

fn text(h: HWND, id: i32) -> String {
    let c = item(h, id);
    let n = unsafe { GetWindowTextLengthW(c) } as usize;
    let mut buf = vec![0u16; n + 1];
    unsafe { GetWindowTextW(c, buf.as_mut_ptr(), buf.len() as i32) };
    from_wide(&buf)
}

fn set_text(h: HWND, id: i32, s: &str) {
    let w = wide(s);
    unsafe { SetWindowTextW(item(h, id), w.as_ptr()) };
}

fn checked(h: HWND, id: i32) -> bool {
    unsafe { SendMessageW(item(h, id), BM_GETCHECK, 0, 0) == BST_CHECKED as isize }
}

fn set_checked(h: HWND, id: i32, on: bool) {
    unsafe { SendMessageW(item(h, id), BM_SETCHECK, if on { BST_CHECKED } else { BST_UNCHECKED } as usize, 0) };
}

fn enable(h: HWND, id: i32, on: bool) {
    unsafe { EnableWindow(item(h, id), on as i32) };
}

fn message(h: HWND, text: &str, style: MESSAGEBOX_STYLE) -> i32 {
    let (t, c) = (wide(text), wide(TITLE));
    unsafe { MessageBoxW(h, t.as_ptr(), c.as_ptr(), style) }
}

fn shell_open(what: &str, args: Option<&str>) {
    let (v, w) = (wide("open"), wide(what));
    let a = args.map(wide);
    unsafe {
        ShellExecuteW(std::ptr::null_mut(), v.as_ptr(), w.as_ptr(), a.as_ref().map_or(std::ptr::null(), |a| a.as_ptr()), std::ptr::null(), SW_SHOWNORMAL);
    }
}

/// A folder we may write into: the builder's own (it is usually in
/// Downloads), else the desktop.
fn default_out_dir() -> PathBuf {
    let here = std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf));
    if let Some(d) = here {
        let probe = d.join(".ro-setup-builder-write-test");
        if std::fs::write(&probe, b"").is_ok() {
            let _ = std::fs::remove_file(&probe);
            return d;
        }
    }
    desktop().unwrap_or_else(|| PathBuf::from("C:\\"))
}

/// A copy of the disc the user already has, so nothing needs downloading:
/// RAGNAROK.iso next to the builder, or the CD in a drive.
fn find_own_disc() -> Option<PathBuf> {
    if let Some(d) = std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)) {
        let iso = d.join(ia::FILE);
        if iso.metadata().map(|m| m.len() == ia::ISO_SIZE).unwrap_or(false) {
            return Some(iso);
        }
    }
    use windows_sys::Win32::Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives};
    use windows_sys::Win32::System::Diagnostics::Debug::{SEM_FAILCRITICALERRORS, SetErrorMode};
    // an empty CD drive must not pop up "no disk" boxes
    let old = unsafe { SetErrorMode(SEM_FAILCRITICALERRORS) };
    let mask = unsafe { GetLogicalDrives() };
    let mut found = None;
    for i in 0..26u32 {
        if mask & (1 << i) == 0 {
            continue;
        }
        let root = format!("{}:\\", (b'A' + i as u8) as char);
        let w = wide(&root);
        if unsafe { GetDriveTypeW(w.as_ptr()) } != 5 {
            continue; // DRIVE_CDROM
        }
        let msi = Path::new(&root).join(crate::aio::MSI_NAME);
        if msi.metadata().map(|m| m.len() == 1_323_520).unwrap_or(false) {
            found = Some(PathBuf::from(root));
            break;
        }
    }
    unsafe { SetErrorMode(old) };
    found
}

// ------------------------------------------------------------ worker state

struct Shared {
    hwnd: AtomicIsize,
    cancel: AtomicBool,
    status: Mutex<String>,
    done: AtomicU64,
    total: AtomicU64,
    /// set while an update message is queued, so the worker does not flood the queue
    pending: AtomicBool,
    result: Mutex<Option<Result<job::Done, String>>>,
}

impl Shared {
    fn poke(&self) {
        if !self.pending.swap(true, Ordering::AcqRel) {
            unsafe { PostMessageW(self.hwnd.load(Ordering::Relaxed) as HWND, WM_APP_UPDATE, 0, 0) };
        }
    }
}

impl job::Report for Shared {
    fn status(&self, text: &str) {
        *self.status.lock().unwrap() = text.to_string();
        self.done.store(0, Ordering::Relaxed);
        self.total.store(0, Ordering::Relaxed);
        self.poke();
    }
    fn progress(&self, done: u64, total: u64) {
        self.done.store(done, Ordering::Relaxed);
        self.total.store(total, Ordering::Relaxed);
        self.poke();
    }
    fn cancelled(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
    }
}

struct App {
    shared: Option<Arc<Shared>>,
    worker: Option<std::thread::JoinHandle<()>>,
    font: HFONT,
    closing: bool,
    started: Option<std::time::Instant>,
    speed_mark: Option<(std::time::Instant, u64)>,
    speed: f64,
}

thread_local! {
    static APP: std::cell::RefCell<App> = const { std::cell::RefCell::new(App {
        shared: None, worker: None, font: std::ptr::null_mut(), closing: false, started: None, speed_mark: None, speed: 0.0,
    }) };
}

fn running() -> bool {
    APP.with(|a| a.borrow().worker.is_some())
}

// ------------------------------------------------------------ layout

struct Layout {
    hwnd: HWND,
    font: HFONT,
    dpi: i32,
}

impl Layout {
    fn px(&self, v: i32) -> i32 {
        v * self.dpi / 96
    }

    fn add(&self, class: &str, text: &str, style: u32, ex: u32, x: i32, y: i32, w: i32, h: i32, id: i32) -> HWND {
        let (c, t) = (wide(class), wide(text));
        unsafe {
            let ctl = CreateWindowExW(
                ex,
                c.as_ptr(),
                t.as_ptr(),
                WS_CHILD | WS_VISIBLE | style,
                self.px(x),
                self.px(y),
                self.px(w),
                self.px(h),
                self.hwnd,
                id as isize as HMENU,
                GetModuleHandleW(std::ptr::null()),
                std::ptr::null(),
            );
            SendMessageW(ctl, WM_SETFONT, self.font as usize, 1);
            ctl
        }
    }
}

const W: i32 = 560; // client width at 96 dpi
const H: i32 = 518;

fn build_ui(hwnd: HWND) {
    let dpi = unsafe {
        let dc = GetDC(hwnd);
        let d = GetDeviceCaps(dc, LOGPIXELSY as i32);
        ReleaseDC(hwnd, dc);
        d.max(96)
    };
    // the system's message font (Tahoma on XP, Segoe UI later)
    let font = unsafe {
        let mut ncm: NONCLIENTMETRICSW = std::mem::zeroed();
        // XP's struct ends before iPaddedBorderWidth (Vista)
        ncm.cbSize = (size_of::<NONCLIENTMETRICSW>() - if is_nt5() { 4 } else { 0 }) as u32;
        if SystemParametersInfoW(SPI_GETNONCLIENTMETRICS, ncm.cbSize, (&mut ncm as *mut NONCLIENTMETRICSW).cast(), 0) != 0 {
            CreateFontIndirectW(&ncm.lfMessageFont)
        } else {
            GetStockObject(DEFAULT_GUI_FONT) as HFONT
        }
    };
    APP.with(|a| a.borrow_mut().font = font);
    let l = Layout { hwnd, font, dpi };
    let m = 12; // margin
    let iw = W - 2 * m; // inner width
    let btn = (BS_PUSHBUTTON as u32) | WS_TABSTOP;
    let edit_style = (ES_AUTOHSCROLL as u32) | WS_TABSTOP | WS_BORDER;

    l.add(
        "STATIC",
        "Makes the Ragnarok Offline all-in-one installer from the preserved December 2003 euRO CD: \
         the original setup, with the single-player server added. It runs on Windows XP to 11.",
        0,
        0,
        m,
        m,
        iw,
        30,
        -1,
    );
    // fits a 800x600 desktop with its taskbar (XP machines often run at that)
    let mut y = m + 36;
    l.add("STATIC", "Please read:", 0, 0, m, y, iw, 16, -1);
    y += 18;
    let disclaimer = format!("{DISCLAIMER}\r\n\r\nThe disc on the Internet Archive: {}", ia::details_url());
    l.add(
        "EDIT",
        &disclaimer,
        (ES_MULTILINE | ES_READONLY | ES_AUTOVSCROLL) as u32 | WS_VSCROLL | WS_TABSTOP,
        WS_EX_CLIENTEDGE,
        m,
        y,
        iw,
        88,
        ID_DISCLAIMER,
    );
    y += 96;
    l.add(
        "BUTTON",
        "I understand that the disc is Gravity's copyrighted software, preserved as lost media, and that what this makes is for my own use only.",
        (BS_AUTOCHECKBOX | BS_MULTILINE) as u32 | WS_TABSTOP,
        0,
        m,
        y,
        iw,
        30,
        ID_ACCEPT,
    );
    y += 34;

    // where the disc comes from
    l.add("BUTTON", "The disc", BS_GROUPBOX as u32, 0, m, y, iw, 92, -1);
    let gx = m + 12;
    y += 18;
    l.add(
        "BUTTON",
        &format!("Download it from the Internet Archive ({} MB, once)", (ia::ISO_SIZE + (1 << 19)) >> 20),
        BS_AUTORADIOBUTTON as u32 | WS_GROUP | WS_TABSTOP,
        0,
        gx,
        y,
        iw - 24,
        18,
        ID_SRC_DL,
    );
    y += 22;
    l.add("BUTTON", "Use my own copy: a disc image (.iso) or the CD drive (e.g. D:\\)", BS_AUTORADIOBUTTON as u32, 0, gx, y, iw - 24, 18, ID_SRC_LOCAL);
    y += 22;
    l.add("EDIT", "", edit_style, WS_EX_CLIENTEDGE, gx + 18, y, iw - 24 - 18 - 92, 22, ID_SRC_PATH);
    l.add("BUTTON", "Browse...", btn, 0, m + iw - 12 - 84, y - 1, 84, 24, ID_SRC_BROWSE);
    y += 38;

    // what to make
    l.add("BUTTON", "Make", BS_GROUPBOX as u32, 0, m, y, iw, 108, -1);
    y += 18;
    l.add("BUTTON", "A disc image (.iso)", BS_AUTORADIOBUTTON as u32 | WS_GROUP | WS_TABSTOP, 0, gx, y, 150, 18, ID_FMT_ISO);
    l.add("BUTTON", "A zip file", BS_AUTORADIOBUTTON as u32, 0, gx + 160, y, 110, 18, ID_FMT_ZIP);
    l.add("BUTTON", "A folder", BS_AUTORADIOBUTTON as u32, 0, gx + 280, y, 110, 18, ID_FMT_DIR);
    y += 22;
    l.add("STATIC", "", 0, 0, gx, y, iw - 24, 28, ID_FMT_HINT);
    y += 30;
    l.add("STATIC", "Save to:", 0, 0, gx, y + 3, 50, 18, -1);
    l.add("EDIT", "", edit_style, WS_EX_CLIENTEDGE, gx + 52, y, iw - 24 - 52 - 92, 22, ID_OUT_PATH);
    l.add("BUTTON", "Browse...", btn, 0, m + iw - 12 - 84, y - 1, 84, 24, ID_OUT_BROWSE);
    y += 46;

    l.add(
        "BUTTON",
        "Keep the downloaded disc image for next time (in your user folder's local application data)",
        BS_AUTOCHECKBOX as u32 | WS_TABSTOP,
        0,
        m,
        y,
        iw,
        18,
        ID_KEEP,
    );
    y += 24;
    let pb = l.add("msctls_progress32", "", 0, 0, m, y, iw, 16, ID_PROGRESS);
    unsafe { SendMessageW(pb, PBM_SETRANGE32, 0, 1000) };
    y += 22;
    l.add("STATIC", "", 12 /* SS_LEFTNOWORDWRAP */ | 128 /* SS_NOPREFIX */, 0, m, y, iw, 18, ID_STATUS);
    y += 22;
    debug_assert!(y + 26 + m <= H, "layout taller than the window");
    l.add("BUTTON", "Make it", (BS_DEFPUSHBUTTON as u32) | WS_TABSTOP, 0, m + iw - 2 * 96 - 8, y, 96, 26, ID_GO);
    l.add("BUTTON", "Close", btn, 0, m + iw - 96, y, 96, 26, ID_CLOSE);

    // defaults
    match find_own_disc() {
        Some(p) => {
            set_checked(hwnd, ID_SRC_LOCAL, true);
            set_text(hwnd, ID_SRC_PATH, &p.to_string_lossy());
        }
        None => set_checked(hwnd, ID_SRC_DL, true),
    }
    // XP cannot open a disc image without extra software: a folder installs directly
    let fmt = if can_mount_iso() { ID_FMT_ISO } else { ID_FMT_DIR };
    set_checked(hwnd, fmt, true);
    set_checked(hwnd, ID_KEEP, true);
    set_text(hwnd, ID_OUT_PATH, &default_out_dir().join(format_of(hwnd).default_name()).to_string_lossy());
    format_changed(hwnd);
    refresh_enabled(hwnd);
}

fn format_of(h: HWND) -> Format {
    if checked(h, ID_FMT_ZIP) {
        Format::Zip
    } else if checked(h, ID_FMT_DIR) {
        Format::Folder
    } else {
        Format::Iso
    }
}

fn hint(f: Format) -> String {
    match f {
        Format::Iso if can_mount_iso() => "Double-click the image to open it as a drive, then run setup.exe. It also fits a 700 MB CD-R.".into(),
        Format::Iso => "Burn it to a 700 MB CD-R, or open it with a virtual drive program, then run setup.exe.".into(),
        Format::Zip => "Extract it anywhere, then run setup.exe in the RagnarokOffline-CD folder.".into(),
        Format::Folder => "Run setup.exe in it. Works on every Windows, XP included.".into(),
    }
}

/// Keep the folder the user picked, swap the name for the new format's.
fn format_changed(h: HWND) {
    let f = format_of(h);
    set_text(h, ID_FMT_HINT, &hint(f));
    let cur = PathBuf::from(text(h, ID_OUT_PATH));
    let dir = cur.parent().map(Path::to_path_buf).unwrap_or_else(default_out_dir);
    let name = cur.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let defaults = [Format::Iso, Format::Zip, Format::Folder].map(|f| f.default_name());
    if name.is_empty() || defaults.iter().any(|d| d.eq_ignore_ascii_case(&name)) {
        set_text(h, ID_OUT_PATH, &dir.join(f.default_name()).to_string_lossy());
    } else {
        let stem = Path::new(&name).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or(name.clone());
        let new = match f {
            Format::Iso => format!("{stem}.iso"),
            Format::Zip => format!("{stem}.zip"),
            Format::Folder => stem,
        };
        set_text(h, ID_OUT_PATH, &dir.join(new).to_string_lossy());
    }
}

fn refresh_enabled(h: HWND) {
    let busy = running();
    let local = checked(h, ID_SRC_LOCAL);
    for id in [ID_ACCEPT, ID_SRC_DL, ID_SRC_LOCAL, ID_FMT_ISO, ID_FMT_ZIP, ID_FMT_DIR, ID_OUT_PATH, ID_OUT_BROWSE] {
        enable(h, id, !busy);
    }
    enable(h, ID_SRC_PATH, !busy && local);
    enable(h, ID_SRC_BROWSE, !busy && local);
    enable(h, ID_KEEP, !busy && !local);
    enable(h, ID_GO, !busy && checked(h, ID_ACCEPT));
    set_text(h, ID_CLOSE, if busy { "Cancel" } else { "Close" });
}

// ------------------------------------------------------------ dialogs

fn browse_file(h: HWND, save: bool, filter: &str, ext: &str, current: &str) -> Option<String> {
    let mut buf = vec![0u16; 1024];
    let cur: Vec<u16> = current.encode_utf16().take(1000).collect();
    buf[..cur.len()].copy_from_slice(&cur);
    let filter: Vec<u16> = filter.encode_utf16().map(|c| if c == '|' as u16 { 0 } else { c }).chain([0, 0]).collect();
    let ext = wide(ext);
    unsafe {
        let mut ofn: OPENFILENAMEW = std::mem::zeroed();
        ofn.lStructSize = size_of::<OPENFILENAMEW>() as u32;
        ofn.hwndOwner = h;
        ofn.lpstrFilter = filter.as_ptr();
        ofn.lpstrFile = buf.as_mut_ptr();
        ofn.nMaxFile = buf.len() as u32;
        ofn.lpstrDefExt = ext.as_ptr();
        ofn.Flags = OFN_EXPLORER | OFN_HIDEREADONLY | OFN_NOCHANGEDIR | if save { OFN_OVERWRITEPROMPT | OFN_PATHMUSTEXIST } else { OFN_FILEMUSTEXIST };
        let ok = if save { GetSaveFileNameW(&mut ofn) } else { GetOpenFileNameW(&mut ofn) };
        (ok != 0).then(|| from_wide(&buf))
    }
}

fn browse_folder(h: HWND, title: &str) -> Option<String> {
    let t = wide(title);
    unsafe {
        let mut bi: BROWSEINFOW = std::mem::zeroed();
        bi.hwndOwner = h;
        bi.lpszTitle = t.as_ptr();
        bi.ulFlags = BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE;
        let pidl = SHBrowseForFolderW(&bi);
        if pidl.is_null() {
            return None;
        }
        let mut buf = [0u16; 260];
        let ok = SHGetPathFromIDListW(pidl, buf.as_mut_ptr());
        CoTaskMemFree(pidl as *const _);
        (ok != 0).then(|| from_wide(&buf))
    }
}

// ------------------------------------------------------------ running

fn start(h: HWND) {
    let format = format_of(h);
    let out = PathBuf::from(text(h, ID_OUT_PATH).trim());
    if out.as_os_str().is_empty() || out.parent().is_none_or(|p| p.as_os_str().is_empty()) {
        message(h, "Choose where to save it (a full path).", MB_OK | MB_ICONWARNING);
        return;
    }
    let source = if checked(h, ID_SRC_LOCAL) {
        let p = PathBuf::from(text(h, ID_SRC_PATH).trim());
        if !p.exists() {
            message(h, "Choose your copy of the disc: a .iso file, or the CD drive (for example D:\\).", MB_OK | MB_ICONWARNING);
            return;
        }
        Source::Local(p)
    } else {
        Source::Download { cache: crate::default_cache(), keep: checked(h, ID_KEEP) }
    };
    let opts = job::Options { source, format, out };

    // disk space: the output, plus the download when there is none yet
    let (need_out, need_dl) = job::space_needed(&opts);
    let mut warn = Vec::new();
    if let Some(free) = job::free_space(&opts.out) {
        if free < need_out {
            warn.push(format!("{} has {} MB free; the result needs about {} MB.", opts.out.parent().unwrap().display(), free >> 20, need_out >> 20));
        }
    }
    if let Source::Download { cache, .. } = &opts.source {
        if let Some(free) = job::free_space(cache) {
            let same = job::free_space(&opts.out) == Some(free);
            let need = need_dl + if same { need_out } else { 0 };
            if need_dl > 0 && free < need {
                warn.push(format!("{} has {} MB free; the download needs about {} MB.", cache.display(), free >> 20, need_dl >> 20));
            }
        }
    }
    if !warn.is_empty() && message(h, &format!("{}\n\nTry anyway?", warn.join("\n")), MB_YESNO | MB_ICONWARNING) != IDYES {
        return;
    }

    let ours = match Ours::load(None, None) {
        Ok(o) => o,
        Err(e) => {
            message(h, &format!("{e:#}"), MB_OK | MB_ICONERROR);
            return;
        }
    };
    let shared = Arc::new(Shared {
        hwnd: AtomicIsize::new(h as isize),
        cancel: AtomicBool::new(false),
        status: Mutex::new("Starting".into()),
        done: AtomicU64::new(0),
        total: AtomicU64::new(0),
        pending: AtomicBool::new(false),
        result: Mutex::new(None),
    });
    let s2 = shared.clone();
    let worker = std::thread::spawn(move || {
        let r = job::run(&opts, &ours.payload(), crate::cab_time(), &*s2).map_err(|e| {
            if e.is::<job::Cancelled>() { "cancelled".to_string() } else { format!("{e:#}") }
        });
        *s2.result.lock().unwrap() = Some(r);
        s2.pending.store(false, Ordering::Release);
        s2.poke();
    });
    APP.with(|a| {
        let mut a = a.borrow_mut();
        a.shared = Some(shared);
        a.worker = Some(worker);
        a.started = Some(std::time::Instant::now());
        a.speed_mark = None;
        a.speed = 0.0;
    });
    refresh_enabled(h);
}

fn on_update(h: HWND) {
    let Some(shared) = APP.with(|a| a.borrow().shared.clone()) else { return };
    shared.pending.store(false, Ordering::Release);
    let status = shared.status.lock().unwrap().clone();
    let (done, total) = (shared.done.load(Ordering::Relaxed), shared.total.load(Ordering::Relaxed));
    let pos = if total > 0 { (done * 1000 / total) as usize } else { 0 };
    unsafe { SendMessageW(item(h, ID_PROGRESS), PBM_SETPOS, pos, 0) };
    let line = if total > 0 {
        // a smoothed rate, for downloads
        let now = std::time::Instant::now();
        let speed = APP.with(|a| {
            let mut a = a.borrow_mut();
            match a.speed_mark {
                Some((t, d)) if now.duration_since(t).as_secs_f64() >= 1.0 && done >= d => {
                    let inst = (done - d) as f64 / now.duration_since(t).as_secs_f64();
                    a.speed = if a.speed == 0.0 { inst } else { a.speed * 0.7 + inst * 0.3 };
                    a.speed_mark = Some((now, done));
                }
                Some((_, d)) if done < d => a.speed_mark = Some((now, done)),
                None => a.speed_mark = Some((now, done)),
                _ => {}
            }
            a.speed
        });
        let rate = if status.starts_with("Downloading") || status.starts_with("Resuming") {
            if speed > 0.0 {
                let left = (total - done) as f64 / speed;
                format!(", {:.1} MB/s, about {} min left", speed / 1048576.0, (left / 60.0).ceil() as u64)
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        format!("{status}: {} of {} MB{rate}", done >> 20, total >> 20)
    } else {
        status
    };
    set_text(h, ID_STATUS, &line);

    let result = shared.result.lock().unwrap().take();
    let Some(result) = result else { return };
    if let Some(w) = APP.with(|a| a.borrow_mut().worker.take()) {
        let _ = w.join();
    }
    APP.with(|a| a.borrow_mut().shared = None);
    refresh_enabled(h);
    if APP.with(|a| a.borrow().closing) {
        unsafe { DestroyWindow(h) };
        return;
    }
    match result {
        Ok(d) => {
            unsafe { SendMessageW(item(h, ID_PROGRESS), PBM_SETPOS, 1000, 0) };
            set_text(h, ID_STATUS, &format!("Done: {}", d.out.display()));
            let kept = d.cached.as_ref().map(|c| format!("\n\nThe original disc image stays in {} for next time; delete it whenever you like.", c.display())).unwrap_or_default();
            let f = format_of(h);
            let out = d.out.to_string_lossy().into_owned();
            if f == Format::Folder {
                let q = format!("Done: {out}\n\nStart the installer (setup.exe) now?{kept}");
                if message(h, &q, MB_YESNO | MB_ICONINFORMATION) == IDYES {
                    shell_open(&d.out.join("setup.exe").to_string_lossy(), None);
                } else {
                    shell_open(&out, None);
                }
            } else {
                let q = format!("Done: {out}\n\n{}\n\nShow it in its folder?{kept}", hint(f));
                if message(h, &q, MB_YESNO | MB_ICONINFORMATION) == IDYES {
                    shell_open("explorer.exe", Some(&format!("/select,\"{out}\"")));
                }
            }
        }
        Err(e) if e == "cancelled" => {
            unsafe { SendMessageW(item(h, ID_PROGRESS), PBM_SETPOS, 0, 0) };
            set_text(h, ID_STATUS, "Cancelled. A partial download is kept and resumes next time.");
        }
        Err(e) => {
            set_text(h, ID_STATUS, "Failed");
            message(h, &e, MB_OK | MB_ICONERROR);
        }
    }
}

fn cancel(h: HWND) -> bool {
    if message(h, "Stop now? A partial download is kept and resumes next time.", MB_YESNO | MB_ICONQUESTION) != IDYES {
        return false;
    }
    if let Some(s) = APP.with(|a| a.borrow().shared.clone()) {
        s.cancel.store(true, Ordering::Relaxed);
        set_text(h, ID_STATUS, "Stopping...");
    }
    true
}

unsafe extern "system" fn wndproc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            build_ui(h);
            0
        }
        WM_COMMAND => {
            let id = (wp & 0xffff) as i32;
            let code = ((wp >> 16) & 0xffff) as u32;
            match id {
                ID_ACCEPT | ID_SRC_DL | ID_SRC_LOCAL => refresh_enabled(h),
                ID_FMT_ISO | ID_FMT_ZIP | ID_FMT_DIR if code == BN_CLICKED => format_changed(h),
                ID_SRC_BROWSE => {
                    if let Some(p) = browse_file(h, false, "Disc images (*.iso)|*.iso|All files|*.*", "iso", "") {
                        set_text(h, ID_SRC_PATH, &p);
                    }
                }
                ID_OUT_BROWSE => {
                    let f = format_of(h);
                    let cur = text(h, ID_OUT_PATH);
                    let picked = match f {
                        Format::Iso => browse_file(h, true, "Disc images (*.iso)|*.iso", "iso", &cur),
                        Format::Zip => browse_file(h, true, "Zip files (*.zip)|*.zip", "zip", &cur),
                        Format::Folder => browse_folder(h, "Where to put the RagnarokOffline-CD folder").map(|d| Path::new(&d).join(f.default_name()).to_string_lossy().into_owned()),
                    };
                    if let Some(p) = picked {
                        set_text(h, ID_OUT_PATH, &p);
                    }
                }
                ID_GO => start(h),
                1 /* IDOK: Enter */ => {
                    if !running() && checked(h, ID_ACCEPT) {
                        start(h)
                    }
                }
                ID_CLOSE | 2 /* IDCANCEL: Esc */ => {
                    if running() {
                        cancel(h);
                    } else {
                        unsafe { DestroyWindow(h) };
                    }
                }
                _ => {}
            }
            0
        }
        WM_APP_UPDATE => {
            on_update(h);
            0
        }
        WM_CLOSE => {
            if running() {
                if cancel(h) {
                    APP.with(|a| a.borrow_mut().closing = true);
                }
            } else {
                unsafe { DestroyWindow(h) };
            }
            0
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            0
        }
        _ => unsafe { DefWindowProcW(h, msg, wp, lp) },
    }
}

pub fn run() {
    unsafe {
        CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32);
        let icc = INITCOMMONCONTROLSEX { dwSize: size_of::<INITCOMMONCONTROLSEX>() as u32, dwICC: ICC_PROGRESS_CLASS | ICC_STANDARD_CLASSES };
        InitCommonControlsEx(&icc);
        let inst = GetModuleHandleW(std::ptr::null());
        let class = wide("RagnarokOfflineSetupBuilder");
        let wc = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(wndproc),
            hInstance: inst,
            hIcon: LoadIconW(inst, 1 as _),
            hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_BTNFACE + 1) as usize as HBRUSH,
            lpszClassName: class.as_ptr(),
            ..std::mem::zeroed()
        };
        RegisterClassExW(&wc);
        let dpi = {
            let dc = GetDC(std::ptr::null_mut());
            let d = GetDeviceCaps(dc, LOGPIXELSY as i32);
            ReleaseDC(std::ptr::null_mut(), dc);
            d.max(96)
        };
        let style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;
        let mut r = RECT { left: 0, top: 0, right: W * dpi / 96, bottom: H * dpi / 96 };
        AdjustWindowRectEx(&mut r, style, 0, 0);
        let (w, h) = (r.right - r.left, r.bottom - r.top);
        // centred in the work area (the screen minus the taskbar)
        let mut wa = RECT { left: 0, top: 0, right: GetSystemMetrics(SM_CXSCREEN), bottom: GetSystemMetrics(SM_CYSCREEN) };
        SystemParametersInfoW(SPI_GETWORKAREA, 0, (&mut wa as *mut RECT).cast(), 0);
        let x = wa.left + (wa.right - wa.left - w).max(0) / 2;
        let y = wa.top + (wa.bottom - wa.top - h).max(0) / 2;
        let title = wide(TITLE);
        let hwnd = CreateWindowExW(0, class.as_ptr(), title.as_ptr(), style, x, y, w, h, std::ptr::null_mut(), std::ptr::null_mut(), inst, std::ptr::null());
        if hwnd.is_null() {
            return;
        }
        ShowWindow(hwnd, SW_SHOWNORMAL);
        UpdateWindow(hwnd);
        SetFocus(item(hwnd, ID_ACCEPT));
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            if IsDialogMessageW(hwnd, &msg) != 0 {
                continue;
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        let font = APP.with(|a| a.borrow().font);
        if !font.is_null() {
            DeleteObject(font as _);
        }
    }
}
