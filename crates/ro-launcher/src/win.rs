//! Win32 side of the launcher: message boxes, a progress window, elevation,
//! Wine detection and the client's registry settings.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicIsize, Ordering};

use anyhow::{Result, bail};
use windows_sys::Win32::Foundation::{CloseHandle, ERROR_SUCCESS, HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{COLOR_BTNFACE, CreateFontIndirectW, DEFAULT_GUI_FONT, GetStockObject, HFONT, UpdateWindow};
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
use windows_sys::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_QUERY_VALUE, KEY_SET_VALUE, KEY_WOW64_32KEY, REG_BINARY, REG_DWORD,
    REG_OPTION_NON_VOLATILE, REG_SZ, RegCloseKey, RegCreateKeyExW, RegOpenKeyExW, RegSetValueExW,
};
use windows_sys::Win32::System::Threading::{GetExitCodeProcess, INFINITE, WaitForSingleObject};
use windows_sys::Win32::UI::Controls::{ICC_PROGRESS_CLASS, INITCOMMONCONTROLSEX, InitCommonControlsEx, PBM_SETMARQUEE, PBM_SETPOS, PBM_SETRANGE32, PBS_MARQUEE};
use windows_sys::Win32::UI::HiDpi::GetDpiForSystem;
use windows_sys::Win32::UI::Shell::{SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW};
use windows_sys::Win32::UI::WindowsAndMessaging::*;

use crate::RegValue;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

const TITLE: &str = "Ragnarok Offline";

fn message_box(text: &str, style: MESSAGEBOX_STYLE) -> i32 {
    let (t, c) = (wide(text), wide(TITLE));
    unsafe { MessageBoxW(std::ptr::null_mut(), t.as_ptr(), c.as_ptr(), style | MB_SETFOREGROUND) }
}

pub fn error(text: &str) {
    message_box(text, MB_OK | MB_ICONERROR);
}

pub fn ask(text: &str) -> bool {
    message_box(text, MB_YESNO | MB_ICONWARNING) == IDYES
}

/// Is this Wine (which has its own working DirectDraw and no UAC)?
pub fn is_wine() -> bool {
    let ntdll = wide("ntdll.dll");
    unsafe {
        let h = GetModuleHandleW(ntdll.as_ptr());
        !h.is_null() && GetProcAddress(h, c"wine_get_version".as_ptr().cast()).is_some()
    }
}

/// Let the client take the foreground. We got that right from Explorer when
/// the user double-clicked us, but have no window of our own to pass it on
/// with, and a fullscreen DirectDraw game that cannot become the foreground
/// window just minimizes itself.
pub fn allow_foreground(pid: u32) {
    unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::AllowSetForegroundWindow(pid);
    }
}

/// Start `exe` suspended, replace bytes in its memory, resume it, and wait
/// until it and everything it started have exited.
///
/// Each patch is (address, the bytes expected there, the replacement); the
/// replacement is zero-padded to the expected length. If any expected bytes
/// do not match, the process is killed before it ran a single instruction
/// and nothing is changed. The file on disk is never touched.
pub fn run_patched(exe: &Path, dir: &Path, patches: &[(usize, &[u8], Vec<u8>)]) -> Result<()> {
    use windows_sys::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
    use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, JobObjectBasicAccountingInformation,
        QueryInformationJobObject,
    };
    use windows_sys::Win32::System::Threading::{
        CREATE_SUSPENDED, CreateProcessW, PROCESS_INFORMATION, ResumeThread, STARTUPINFOW, TerminateProcess,
    };
    let app = wide(&exe.to_string_lossy());
    let mut cmdline = wide(&format!("\"{}\"", exe.display()));
    let cwd = wide(&dir.to_string_lossy());
    unsafe {
        let job = CreateJobObjectW(std::ptr::null(), std::ptr::null());
        if job.is_null() {
            bail!("CreateJobObject failed");
        }
        let mut si: STARTUPINFOW = std::mem::zeroed();
        si.cb = size_of::<STARTUPINFOW>() as u32;
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
        let ok = CreateProcessW(app.as_ptr(), cmdline.as_mut_ptr(), std::ptr::null(), std::ptr::null(), 0, CREATE_SUSPENDED, std::ptr::null(), cwd.as_ptr(), &si, &mut pi);
        if ok == 0 {
            CloseHandle(job);
            bail!("starting {}: {}", exe.display(), std::io::Error::last_os_error());
        }
        let fail = |msg: String| -> Result<()> {
            TerminateProcess(pi.hProcess, 1);
            CloseHandle(pi.hThread);
            CloseHandle(pi.hProcess);
            CloseHandle(job);
            bail!(msg)
        };
        if AssignProcessToJobObject(job, pi.hProcess) == 0 {
            return fail(format!("AssignProcessToJobObject: {}", std::io::Error::last_os_error()));
        }
        for (addr, expected, replacement) in patches {
            let mut got = vec![0u8; expected.len()];
            let mut n = 0usize;
            if ReadProcessMemory(pi.hProcess, *addr as *const _, got.as_mut_ptr().cast(), got.len(), &mut n) == 0 || got != *expected {
                return fail(format!("{} is not the expected build (bytes at {addr:#x} differ)", exe.display()));
            }
            let mut bytes = replacement.clone();
            bytes.resize(expected.len().max(replacement.len() + 1), 0);
            if WriteProcessMemory(pi.hProcess, *addr as *const _, bytes.as_ptr().cast(), bytes.len(), &mut n) == 0 {
                return fail(format!("WriteProcessMemory at {addr:#x}: {}", std::io::Error::last_os_error()));
            }
        }
        allow_foreground(pi.dwProcessId);
        ResumeThread(pi.hThread);
        CloseHandle(pi.hThread);
        CloseHandle(pi.hProcess);
        loop {
            let mut info: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION = std::mem::zeroed();
            let ok = QueryInformationJobObject(
                job,
                JobObjectBasicAccountingInformation,
                (&mut info as *mut JOBOBJECT_BASIC_ACCOUNTING_INFORMATION).cast(),
                size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>() as u32,
                std::ptr::null_mut(),
            );
            if ok == 0 || info.ActiveProcesses == 0 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        CloseHandle(job);
    }
    Ok(())
}

pub fn local_appdata() -> PathBuf {
    std::env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir)
}

/// Run ourselves elevated (UAC prompt) with `args`; returns the exit code.
pub fn run_elevated(args: &str) -> Result<u32> {
    let exe = wide(&std::env::current_exe()?.to_string_lossy());
    let (verb, params) = (wide("runas"), wide(args));
    unsafe {
        let mut sei: SHELLEXECUTEINFOW = std::mem::zeroed();
        sei.cbSize = size_of::<SHELLEXECUTEINFOW>() as u32;
        sei.fMask = SEE_MASK_NOCLOSEPROCESS;
        sei.lpVerb = verb.as_ptr();
        sei.lpFile = exe.as_ptr();
        sei.lpParameters = params.as_ptr();
        sei.nShow = SW_SHOWNORMAL;
        if ShellExecuteExW(&mut sei) == 0 || sei.hProcess.is_null() {
            bail!("administrator rights are needed once to prepare this folder, and the request was declined");
        }
        WaitForSingleObject(sei.hProcess, INFINITE);
        let mut code = 1u32;
        GetExitCodeProcess(sei.hProcess, &mut code);
        CloseHandle(sei.hProcess);
        Ok(code)
    }
}

// ---- progress window -------------------------------------------------------

const WM_PROGRESS: u32 = WM_APP + 1;
const WM_DONE: u32 = WM_APP + 2;
static BAR: AtomicIsize = AtomicIsize::new(0);

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_PROGRESS => {
                SendMessageW(BAR.load(Ordering::Relaxed) as HWND, PBM_SETPOS, wp, 0);
                0
            }
            WM_DONE => {
                DestroyWindow(hwnd);
                0
            }
            WM_CLOSE => 0, // not cancellable: a half-written data.grf is worse than waiting
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, msg, wp, lp),
        }
    }
}

fn message_font() -> HFONT {
    unsafe {
        let mut m: NONCLIENTMETRICSW = std::mem::zeroed();
        m.cbSize = size_of::<NONCLIENTMETRICSW>() as u32;
        if SystemParametersInfoW(SPI_GETNONCLIENTMETRICS, m.cbSize, (&mut m as *mut NONCLIENTMETRICSW).cast(), 0) != 0 {
            CreateFontIndirectW(&m.lfMessageFont)
        } else {
            GetStockObject(DEFAULT_GUI_FONT) as HFONT
        }
    }
}

/// Run `work` on a worker thread while showing a small progress window.
/// `work` reports progress as (done, total).
pub fn progress<T: Send>(text: &str, work: impl FnOnce(&(dyn Fn(usize, usize) + Sync)) -> T + Send) -> T {
    window_while(text, false, work)
}

/// Show a small "starting" window while `work` runs (the server starting).
///
/// Besides feedback, this makes us the foreground process for a moment:
/// Windows only lets a process take the foreground if, among other rules, it
/// was started by the foreground process or was allowed to by it. Explorer
/// gives us that right on a double-click but we have no window to use it
/// with; a fullscreen DirectDraw client that cannot become the foreground
/// window never gets exclusive mode (with DDrawCompat it stays hidden, then
/// minimizes). So: window up, work, window down, then spawn the client and
/// AllowSetForegroundWindow it. The window must be gone (and must not be
/// topmost) by the time the client starts, or the client deactivates and
/// minimizes itself.
pub fn starting<T: Send>(text: &str, work: impl FnOnce() -> T + Send) -> T {
    window_while(text, true, |_| work())
}

fn window_while<T: Send>(text: &str, marquee: bool, work: impl FnOnce(&(dyn Fn(usize, usize) + Sync)) -> T + Send) -> T {
    unsafe {
        let icc = INITCOMMONCONTROLSEX { dwSize: size_of::<INITCOMMONCONTROLSEX>() as u32, dwICC: ICC_PROGRESS_CLASS };
        InitCommonControlsEx(&icc);
        let inst = GetModuleHandleW(std::ptr::null());
        let class = wide("RagnarokOfflineProgress");
        let mut wc: WNDCLASSEXW = std::mem::zeroed();
        wc.cbSize = size_of::<WNDCLASSEXW>() as u32;
        wc.lpfnWndProc = Some(wndproc);
        wc.hInstance = inst;
        wc.lpszClassName = class.as_ptr();
        wc.hbrBackground = (COLOR_BTNFACE + 1) as _; // same grey as the STATIC label
        wc.hCursor = LoadCursorW(std::ptr::null_mut(), IDC_WAIT);
        RegisterClassExW(&wc);
        let s = |v: i32| v * GetDpiForSystem() as i32 / 96;
        let (w, h) = (s(440), s(130));
        let x = (GetSystemMetrics(SM_CXSCREEN) - w) / 2;
        let y = (GetSystemMetrics(SM_CYSCREEN) - h) / 2;
        let title = wide(TITLE);
        let hwnd = CreateWindowExW(
            WS_EX_DLGMODALFRAME,
            class.as_ptr(),
            title.as_ptr(),
            WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
            x,
            y,
            w,
            h,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            inst,
            std::ptr::null(),
        );
        let font = message_font();
        let label = wide(text);
        let st = wide("STATIC");
        let lbl = CreateWindowExW(0, st.as_ptr(), label.as_ptr(), WS_CHILD | WS_VISIBLE, s(16), s(14), w - s(40), s(36), hwnd, std::ptr::null_mut(), inst, std::ptr::null());
        SendMessageW(lbl, WM_SETFONT, font as WPARAM, 1);
        let pc = wide("msctls_progress32");
        let style = WS_CHILD | WS_VISIBLE | if marquee { PBS_MARQUEE } else { 0 };
        let bar = CreateWindowExW(0, pc.as_ptr(), std::ptr::null(), style, s(16), s(56), w - s(40), s(20), hwnd, std::ptr::null_mut(), inst, std::ptr::null());
        SendMessageW(bar, PBM_SETRANGE32, 0, 1000);
        if marquee {
            SendMessageW(bar, PBM_SETMARQUEE, 1, 30);
        }
        BAR.store(bar as isize, Ordering::Relaxed);
        UpdateWindow(hwnd);
        SetForegroundWindow(hwnd);

        let hwnd_i = hwnd as isize;
        std::thread::scope(|sc| {
            let worker = sc.spawn(move || {
                let report = move |done: usize, total: usize| {
                    let pos = (done * 1000).checked_div(total).unwrap_or(0);
                    PostMessageW(hwnd_i as HWND, WM_PROGRESS, pos, 0);
                };
                let r = work(&report);
                PostMessageW(hwnd_i as HWND, WM_DONE, 0, 0);
                r
            });
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            worker.join().expect("worker thread panicked")
        })
    }
}

// ---- the client's settings (what its Setup.exe writes) ---------------------

const KEY: &str = r"SOFTWARE\Gravity Soft\Ragnarok";
/// Where Windows redirects a non-elevated, unmanifested 32-bit program's
/// HKLM\SOFTWARE writes (UAC registry virtualization) - i.e. where Ragexe.exe
/// and Setup.exe really keep their settings on Vista and later.
const VIRTUAL_KEY: &str = r"Software\Classes\VirtualStore\MACHINE\SOFTWARE\WOW6432Node\Gravity Soft\Ragnarok";

fn key_exists(root: HKEY, path: &str, flags: u32) -> bool {
    let p = wide(path);
    let mut k: HKEY = std::ptr::null_mut();
    unsafe {
        if RegOpenKeyExW(root, p.as_ptr(), 0, KEY_QUERY_VALUE | flags, &mut k) == ERROR_SUCCESS {
            RegCloseKey(k);
            true
        } else {
            false
        }
    }
}

/// Has the client been configured (by its Setup.exe, by us, by a .reg)?
pub fn client_settings_exist() -> bool {
    key_exists(HKEY_LOCAL_MACHINE, KEY, KEY_WOW64_32KEY) || key_exists(HKEY_CURRENT_USER, VIRTUAL_KEY, 0)
}

/// Write the client's settings where it will read them: HKLM (32-bit view) if
/// we may, else the per-user VirtualStore copy. Returns which one.
pub fn write_client_settings(values: &[(&str, RegValue)]) -> Result<&'static str> {
    for (root, path, flags, name) in [
        (HKEY_LOCAL_MACHINE, KEY, KEY_WOW64_32KEY, "HKLM"),
        (HKEY_CURRENT_USER, VIRTUAL_KEY, 0, "VirtualStore"),
    ] {
        let p = wide(path);
        let mut k: HKEY = std::ptr::null_mut();
        let r = unsafe {
            RegCreateKeyExW(root, p.as_ptr(), 0, std::ptr::null(), REG_OPTION_NON_VOLATILE, KEY_SET_VALUE | flags, std::ptr::null(), &mut k, std::ptr::null_mut())
        };
        if r != ERROR_SUCCESS {
            continue;
        }
        for (n, v) in values {
            let n = wide(n);
            let (ty, data): (u32, Vec<u8>) = match v {
                RegValue::Dword(d) => (REG_DWORD, d.to_le_bytes().to_vec()),
                RegValue::Str(s) => (REG_SZ, wide(s).iter().flat_map(|c| c.to_le_bytes()).collect()),
                RegValue::Bin(b) => (REG_BINARY, b.to_vec()),
            };
            unsafe {
                RegSetValueExW(k, n.as_ptr(), 0, ty, data.as_ptr(), data.len() as u32);
            }
        }
        unsafe { RegCloseKey(k) };
        return Ok(name);
    }
    bail!("could not write the client settings to the registry")
}
