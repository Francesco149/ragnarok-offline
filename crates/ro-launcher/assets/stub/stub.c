/* Ragnarok.exe - the stub RagnarokOffline.exe puts where the CD's patcher was.
 *
 * Players start the game the way the CD taught them: Ragnarok.exe in the game
 * folder. That patcher alone reaches the client but starts no server, so the
 * client says it cannot connect. When RagnarokOffline.exe prepares the folder
 * it keeps the patcher as Ragnarok.exe.original (run from there, patched in
 * memory) and writes this stub as Ragnarok.exe, with the patcher's icon copied
 * onto it. The stub starts RagnarokOffline.exe from its own folder, passing
 * its arguments on, lets it take the foreground (the game needs it) and
 * returns its exit code. --unprepare puts the patcher back.
 *
 * Built by build.rs with i686 mingw, without a C runtime (runs on Windows XP
 * up and under Wine); the launcher embeds the result. The marker string below
 * is how the launcher recognises the stub (crates/ro-launcher/src/main.rs).
 */
#include <windows.h>

__attribute__((used)) const char ragnarok_offline_stub_marker[] = "ragnarok-offline Ragnarok.exe stub 1";

/* the compiler may emit calls to these even without a C runtime */
void *memset(void *d, int c, size_t n) {
    unsigned char *p = d;
    while (n--) *p++ = (unsigned char)c;
    return d;
}
void *memcpy(void *d, const void *s, size_t n) {
    unsigned char *p = d;
    const unsigned char *q = s;
    while (n--) *p++ = *q++;
    return d;
}

static const WCHAR LAUNCHER[] = L"RagnarokOffline.exe";
static const WCHAR TITLE[] = L"Ragnarok Offline";

static int wlen(const WCHAR *s) {
    int n = 0;
    while (s[n]) n++;
    return n;
}

static void fail(const WCHAR *what, const WCHAR *path) {
    static WCHAR msg[1400];
    int n = 0;
    for (const WCHAR *s = what; *s && n < 300; s++) msg[n++] = *s;
    for (const WCHAR *s = path; *s && n < 1360; s++) msg[n++] = *s;
    msg[n] = 0;
    MessageBoxW(NULL, msg, TITLE, MB_OK | MB_ICONERROR);
    ExitProcess(1);
}

void WinMainCRTStartup(void) {
    static WCHAR path[1024];
    DWORD n = GetModuleFileNameW(NULL, path, 1024 - 32);
    if (n == 0 || n >= 1024 - 32) ExitProcess(1);
    DWORD dirlen = n;
    while (dirlen > 0 && path[dirlen - 1] != L'\\' && path[dirlen - 1] != L'/') dirlen--;
    static WCHAR dir[1024];
    memcpy(dir, path, dirlen * sizeof(WCHAR));
    dir[dirlen] = 0;
    memcpy(path + dirlen, LAUNCHER, sizeof LAUNCHER);

    /* our arguments: the command line after argv[0] */
    const WCHAR *args = GetCommandLineW();
    if (*args == L'"') {
        args++;
        while (*args && *args != L'"') args++;
        if (*args) args++;
    } else {
        while (*args && *args != L' ' && *args != L'\t') args++;
    }
    int plen = wlen(path), alen = wlen(args);
    WCHAR *cmd = HeapAlloc(GetProcessHeap(), 0, (plen + alen + 3) * sizeof(WCHAR));
    if (!cmd) ExitProcess(1);
    cmd[0] = L'"';
    memcpy(cmd + 1, path, plen * sizeof(WCHAR));
    cmd[plen + 1] = L'"';
    memcpy(cmd + plen + 2, args, (alen + 1) * sizeof(WCHAR));

    STARTUPINFOW si;
    memset(&si, 0, sizeof si);
    si.cb = sizeof si;
    PROCESS_INFORMATION pi;
    if (!CreateProcessW(path, cmd, NULL, NULL, FALSE, 0, NULL, dir, &si, &pi))
        fail(L"Ragnarok Offline is missing or would not start. This Ragnarok.exe only starts it; "
             L"it should be next to it:\r\n\r\n", path);
    /* Explorer gave us the foreground; the game needs it */
    AllowSetForegroundWindow(pi.dwProcessId);
    CloseHandle(pi.hThread);
    WaitForSingleObject(pi.hProcess, INFINITE);
    DWORD code = 1;
    GetExitCodeProcess(pi.hProcess, &code);
    ExitProcess(code);
}
