/* xpcompat.c: the Windows Vista+ functions the i686 build imports, for Windows XP.
 *
 * Rust's std (target i686-win7-windows-gnu), tokio/mio and the launcher import 27
 * functions that XP SP3's system DLLs lack (tools/xp/impcheck.py lists them
 * against XP's own export tables). This file defines each one's import pointer
 * (`__imp__Name@N`), so the linker takes ours instead of the DLL's and the
 * import never reaches the exe's import table. It is linked first
 * (tools/xp/build.sh), before any import library.
 *
 * Every pointer starts at a resolver: on first call it takes the system's own
 * function if this Windows has one, else ours, and stores the choice. So the
 * same exe runs native code on Windows 7-11. The SRW lock, condition variable
 * and INIT_ONCE functions are chosen together (one decision for all): a lock of
 * ours must never meet a condition variable of the system's.
 *
 * Built on what XP has: keyed events (NtCreateKeyedEvent, since XP; what
 * Windows itself builds SRW locks on), TLS slots and a TLS callback for FLS,
 * GetQueuedCompletionStatus, RtlGenRandom (advapi32!SystemFunction036), the
 * Nt*InformationFile calls, psapi.
 *
 * Three existing functions are wrapped because callers ask them for Vista+
 * behaviour, which XP refuses with an error (the import check cannot see
 * these): WSASocketW (WSA_FLAG_NO_HANDLE_INHERIT is Windows 7 SP1+: XP
 * answers WSAEINVAL) and WSAIoctl(SIO_BASE_HANDLE) (Vista+; on a machine
 * without layered providers the socket is its own base handle), both for mio;
 * FindFirstFileExW (FindExInfoBasic, Windows 7) for std's read_dir.
 *
 * Ours. MIT, like the rest of the project.
 */
#define WIN32_LEAN_AND_MEAN
#define _WIN32_WINNT 0x0601
#include <winsock2.h>
#include <windows.h>
#include <winternl.h>

typedef LONG NTSTATUS_;
#define NT_OK(s) ((s) >= 0)

/* ---------------------------------------------------------------- lookups */

static FARPROC sym(const char *dll, const char *name)
{
    HMODULE m = GetModuleHandleA(dll);
    if (!m) m = LoadLibraryA(dll);
    return m ? GetProcAddress(m, name) : NULL;
}

static NTSTATUS_ (NTAPI *pNtCreateKeyedEvent)(HANDLE *, ACCESS_MASK, void *, ULONG);
static NTSTATUS_ (NTAPI *pNtWaitForKeyedEvent)(HANDLE, void *, BOOLEAN, LARGE_INTEGER *);
static NTSTATUS_ (NTAPI *pNtReleaseKeyedEvent)(HANDLE, void *, BOOLEAN, LARGE_INTEGER *);
static ULONG (NTAPI *pRtlNtStatusToDosError)(NTSTATUS_);

static NTSTATUS_ fail_status(NTSTATUS_ s)
{
    if (!pRtlNtStatusToDosError)
        pRtlNtStatusToDosError = (void *)sym("ntdll.dll", "RtlNtStatusToDosError");
    SetLastError(pRtlNtStatusToDosError ? pRtlNtStatusToDosError(s) : ERROR_GEN_FAILURE);
    return s;
}

/* ------------------------------------------------- keyed event (one per process) */

static HANDLE volatile g_keyed;

static HANDLE keyed(void)
{
    HANDLE h = g_keyed;
    if (h) return h;
    pNtCreateKeyedEvent = (void *)sym("ntdll.dll", "NtCreateKeyedEvent");
    pNtWaitForKeyedEvent = (void *)sym("ntdll.dll", "NtWaitForKeyedEvent");
    pNtReleaseKeyedEvent = (void *)sym("ntdll.dll", "NtReleaseKeyedEvent");
    HANDLE n = NULL;
    if (!pNtCreateKeyedEvent || !NT_OK(pNtCreateKeyedEvent(&n, 0x1f0003 /* all access */, NULL, 0)))
        RaiseException(0xE0585043 /* 'XPC' */, EXCEPTION_NONCONTINUABLE, 0, NULL);
    if (InterlockedCompareExchangePointer((void *volatile *)&g_keyed, n, NULL) != NULL)
        CloseHandle(n);
    return g_keyed;
}

/* A keyed-event release blocks until a waiter on that key arrives, so a
 * waker that counted N registered waiters can release exactly N times. */
/* keyed() first, as its own statement: it is what fills in the pointers, and
 * in `p(keyed(), ...)` C may load `p` before calling keyed() (it did: the
 * first contended lock of a run called address 0). */
static void kwait(void *key)
{
    HANDLE h = keyed();
    pNtWaitForKeyedEvent(h, key, FALSE, NULL);
}

static void kwake(void *key)
{
    HANDLE h = keyed();
    pNtReleaseKeyedEvent(h, key, FALSE, NULL);
}

/* -------------------------------------------------------------- SRW lock
 * One 32-bit word: bit 0 = held exclusive, bits 1-15 = readers holding it,
 * bits 16-31 = threads registered to wait. Whoever frees the lock takes the
 * waiter count to 0 and wakes that many; they all retry (no hand-off, no
 * fairness: enough for a single-player server). Readers enter while no
 * writer holds it.
 */
#define SRW_X 1u
#define SRW_R 2u
#define SRW_RMASK 0xfffeu
#define SRW_W 0x10000u

static LONG *word(void *p) { return (LONG *)p; }

static void srw_wake_all(void *lock, ULONG n)
{
    while (n--) kwake(lock);
}

static void WINAPI my_AcquireSRWLockExclusive(SRWLOCK *l)
{
    for (;;) {
        ULONG v = (ULONG)*word(l);
        if (!(v & (SRW_X | SRW_RMASK))) {
            if ((ULONG)InterlockedCompareExchange(word(l), v | SRW_X, v) == v) return;
            continue;
        }
        if ((ULONG)InterlockedCompareExchange(word(l), v + SRW_W, v) == v) kwait(l);
    }
}

static void WINAPI my_AcquireSRWLockShared(SRWLOCK *l)
{
    for (;;) {
        ULONG v = (ULONG)*word(l);
        if (!(v & SRW_X)) {
            if ((ULONG)InterlockedCompareExchange(word(l), v + SRW_R, v) == v) return;
            continue;
        }
        if ((ULONG)InterlockedCompareExchange(word(l), v + SRW_W, v) == v) kwait(l);
    }
}

static BOOLEAN WINAPI my_TryAcquireSRWLockExclusive(SRWLOCK *l)
{
    ULONG v = (ULONG)*word(l);
    return !(v & (SRW_X | SRW_RMASK)) &&
           (ULONG)InterlockedCompareExchange(word(l), v | SRW_X, v) == v;
}

static BOOLEAN WINAPI my_TryAcquireSRWLockShared(SRWLOCK *l)
{
    ULONG v = (ULONG)*word(l);
    return !(v & SRW_X) && (ULONG)InterlockedCompareExchange(word(l), v + SRW_R, v) == v;
}

static void WINAPI my_ReleaseSRWLockExclusive(SRWLOCK *l)
{
    for (;;) {
        ULONG v = (ULONG)*word(l);
        if ((ULONG)InterlockedCompareExchange(word(l), 0 | (v & SRW_RMASK), v) == v) {
            srw_wake_all(l, v >> 16);
            return;
        }
    }
}

static void WINAPI my_ReleaseSRWLockShared(SRWLOCK *l)
{
    for (;;) {
        ULONG v = (ULONG)*word(l);
        ULONG n = v - SRW_R;
        ULONG wake = 0;
        if (!(n & SRW_RMASK)) { wake = n >> 16; n &= 0xffffu; }
        if ((ULONG)InterlockedCompareExchange(word(l), n, v) == v) {
            srw_wake_all(l, wake);
            return;
        }
    }
}

/* ---------------------------------------------------- condition variable
 * The word counts registered sleepers. A sleeper registers before it
 * releases the lock, so a wake after that cannot be lost. On a timeout the
 * sleeper takes itself off the count; if a waker already took it (count
 * went down for it), the waker is blocked in its release until someone
 * waits on the key, so the sleeper waits once more to take that release.
 */
static void WINAPI my_WakeConditionVariable(CONDITION_VARIABLE *c)
{
    for (;;) {
        LONG v = *word(c);
        if (v <= 0) return;
        if (InterlockedCompareExchange(word(c), v - 1, v) == v) { kwake(c); return; }
    }
}

static void WINAPI my_WakeAllConditionVariable(CONDITION_VARIABLE *c)
{
    LONG n = InterlockedExchange(word(c), 0);
    while (n-- > 0) kwake(c);
}

static BOOL WINAPI my_SleepConditionVariableSRW(CONDITION_VARIABLE *c, SRWLOCK *l, DWORD ms, ULONG flags)
{
    BOOL shared = (flags & CONDITION_VARIABLE_LOCKMODE_SHARED) != 0;
    InterlockedIncrement(word(c));
    if (shared) my_ReleaseSRWLockShared(l); else my_ReleaseSRWLockExclusive(l);
    LARGE_INTEGER t, *pt = NULL;
    if (ms != INFINITE) { t.QuadPart = -(LONGLONG)ms * 10000; pt = &t; }
    keyed();
    NTSTATUS_ s = pNtWaitForKeyedEvent(g_keyed, c, FALSE, pt);
    BOOL ok = TRUE;
    if (s == STATUS_TIMEOUT) {
        for (;;) {
            LONG v = *word(c);
            if (v <= 0) { kwait(c); break; }            /* a waker took us: take its release */
            if (InterlockedCompareExchange(word(c), v - 1, v) == v) { ok = FALSE; break; }
        }
    }
    if (shared) my_AcquireSRWLockShared(l); else my_AcquireSRWLockExclusive(l);
    if (!ok) SetLastError(ERROR_TIMEOUT);
    return ok;
}

/* -------------------------------------------------------------- INIT_ONCE
 * Low two bits: 0 = new, 1 = running, 2 = done; the context sits above them
 * (INIT_ONCE_CTX_RESERVED_BITS = 2), as on Windows.
 */
static BOOL WINAPI my_InitOnceBeginInitialize(INIT_ONCE *o, DWORD flags, BOOL *pending, void **ctx)
{
    LONG_PTR *p = (LONG_PTR *)o;
    for (;;) {
        LONG_PTR v = *p;
        if ((v & 3) == 2) {
            *pending = FALSE;
            if (ctx) *ctx = (void *)(v & ~(LONG_PTR)3);
            return TRUE;
        }
        if (flags & INIT_ONCE_CHECK_ONLY) { SetLastError(ERROR_GEN_FAILURE); return FALSE; }
        if (v == 0 && InterlockedCompareExchangePointer((void *volatile *)p, (void *)1, NULL) == NULL) {
            *pending = TRUE;
            return TRUE;
        }
        if (flags & INIT_ONCE_ASYNC) { *pending = TRUE; return TRUE; }
        Sleep(0);   /* another thread is initializing: rare and short */
    }
}

static BOOL WINAPI my_InitOnceComplete(INIT_ONCE *o, DWORD flags, void *ctx)
{
    LONG_PTR *p = (LONG_PTR *)o;
    if (flags & INIT_ONCE_INIT_FAILED) { InterlockedExchangePointer((void *volatile *)p, NULL); return TRUE; }
    InterlockedExchangePointer((void *volatile *)p, (void *)(((LONG_PTR)ctx & ~(LONG_PTR)3) | 2));
    return TRUE;
}

/* ------------------------------------------------------------------- FLS
 * Fiber-local storage on a thread that never becomes a fiber is thread-local
 * storage whose callback runs when the thread ends. A TLS callback (below)
 * runs the callbacks at thread exit.
 */
#define FLS_SLOTS 64
static struct { DWORD tls; PFLS_CALLBACK_FUNCTION cb; LONG used; } g_fls[FLS_SLOTS];

static DWORD WINAPI my_FlsAlloc(PFLS_CALLBACK_FUNCTION cb)
{
    for (DWORD i = 0; i < FLS_SLOTS; i++) {
        if (InterlockedCompareExchange(&g_fls[i].used, 2, 0) == 0) {
            DWORD t = TlsAlloc();
            if (t == TLS_OUT_OF_INDEXES) { g_fls[i].used = 0; return FLS_OUT_OF_INDEXES; }
            g_fls[i].tls = t; g_fls[i].cb = cb;
            InterlockedExchange(&g_fls[i].used, 1);
            return i;
        }
    }
    SetLastError(ERROR_NO_MORE_ITEMS);
    return FLS_OUT_OF_INDEXES;
}

static BOOL WINAPI my_FlsFree(DWORD i)
{
    if (i >= FLS_SLOTS || g_fls[i].used != 1) { SetLastError(ERROR_INVALID_PARAMETER); return FALSE; }
    TlsFree(g_fls[i].tls);
    InterlockedExchange(&g_fls[i].used, 0);
    return TRUE;
}

static BOOL WINAPI my_FlsSetValue(DWORD i, void *v)
{
    if (i >= FLS_SLOTS || g_fls[i].used != 1) { SetLastError(ERROR_INVALID_PARAMETER); return FALSE; }
    return TlsSetValue(g_fls[i].tls, v);
}

static void *WINAPI my_FlsGetValue(DWORD i)
{
    if (i >= FLS_SLOTS || g_fls[i].used != 1) { SetLastError(ERROR_INVALID_PARAMETER); return NULL; }
    return TlsGetValue(g_fls[i].tls);
}

static int g_fls_ours;  /* set once our FLS is chosen: the TLS callback does nothing otherwise */

static void NTAPI fls_thread_exit(void *mod, DWORD reason, void *res)
{
    (void)mod; (void)res;
    if (!g_fls_ours || (reason != DLL_THREAD_DETACH && reason != DLL_PROCESS_DETACH)) return;
    for (int round = 0; round < 4; round++) {        /* a callback may set other slots */
        int any = 0;
        for (DWORD i = 0; i < FLS_SLOTS; i++) {
            if (g_fls[i].used != 1) continue;
            void *v = TlsGetValue(g_fls[i].tls);
            if (!v) continue;
            TlsSetValue(g_fls[i].tls, NULL);
            if (g_fls[i].cb) { g_fls[i].cb(v); any = 1; }
        }
        if (!any) break;
    }
}

/* mingw's CRT walks .CRT$XLA..XLZ as the image's TLS callbacks. */
__attribute__((section(".CRT$XLY"), used)) PIMAGE_TLS_CALLBACK xpcompat_tls_cb = fls_thread_exit;

static BOOL WINAPI my_IsThreadAFiber(void)
{
    return FALSE;   /* this program never converts a thread to a fiber */
}

/* -------------------------------------------------------------- odds and ends */

static BOOL WINAPI my_SetThreadStackGuarantee(ULONG *size)
{
    if (size) *size = 0;   /* XP keeps no reserve for stack-overflow handlers */
    return TRUE;
}

static int WINAPI my_CompareStringOrdinal(const WCHAR *a, int na, const WCHAR *b, int nb, BOOL icase)
{
    static WCHAR (NTAPI *up)(WCHAR);
    if (!up) up = (void *)sym("ntdll.dll", "RtlUpcaseUnicodeChar");
    if (na < 0) na = lstrlenW(a);
    if (nb < 0) nb = lstrlenW(b);
    int n = na < nb ? na : nb;
    for (int i = 0; i < n; i++) {
        WCHAR x = a[i], y = b[i];
        if (icase) { x = up(x); y = up(y); }
        if (x != y) return x < y ? CSTR_LESS_THAN : CSTR_GREATER_THAN;
    }
    return na == nb ? CSTR_EQUAL : na < nb ? CSTR_LESS_THAN : CSTR_GREATER_THAN;
}

static HANDLE WINAPI my_CreateWaitableTimerExW(SECURITY_ATTRIBUTES *sa, const WCHAR *name, DWORD flags, DWORD access)
{
    (void)access;
    if (flags & ~(DWORD)CREATE_WAITABLE_TIMER_MANUAL_RESET) {
        /* CREATE_WAITABLE_TIMER_HIGH_RESOLUTION (Windows 10): std falls back to Sleep */
        SetLastError(ERROR_INVALID_PARAMETER);
        return NULL;
    }
    return CreateWaitableTimerW(sa, (flags & CREATE_WAITABLE_TIMER_MANUAL_RESET) != 0, name);
}

static UINT WINAPI my_GetDpiForSystem(void)
{
    static int (WINAPI *caps)(HDC, int);   /* gdi32: not every exe links it */
    if (!caps) caps = (void *)sym("gdi32.dll", "GetDeviceCaps");
    HDC dc = GetDC(NULL);
    int d = dc && caps ? caps(dc, LOGPIXELSX) : 96;
    if (dc) ReleaseDC(NULL, dc);
    return d > 0 ? (UINT)d : 96;
}

static BOOL WINAPI my_K32EnumProcessModules(HANDLE p, HMODULE *m, DWORD cb, DWORD *need)
{
    static BOOL (WINAPI *f)(HANDLE, HMODULE *, DWORD, DWORD *);
    if (!f) f = (void *)sym("psapi.dll", "EnumProcessModules");
    if (!f) { SetLastError(ERROR_PROC_NOT_FOUND); return FALSE; }
    return f(p, m, cb, need);
}

static LONG WINAPI my_BCryptGenRandom(void *alg, UCHAR *buf, ULONG len, ULONG flags)
{
    static BOOLEAN (WINAPI *rtl)(void *, ULONG);
    (void)alg; (void)flags;
    if (!rtl) rtl = (void *)sym("advapi32.dll", "SystemFunction036");
    if (!rtl) return (LONG)0xC0000001;           /* STATUS_UNSUCCESSFUL */
    return rtl(buf, len) ? 0 : (LONG)0xC0000001;
}

/* ------------------------------------------------------------ files
 * The FILE_INFO_BY_HANDLE_CLASS values std uses, mapped to the NT classes XP
 * has (their structures are the same layout).
 */
static NTSTATUS_ (NTAPI *pNtQueryInformationFile)(HANDLE, IO_STATUS_BLOCK *, void *, ULONG, ULONG);
static NTSTATUS_ (NTAPI *pNtSetInformationFile)(HANDLE, IO_STATUS_BLOCK *, void *, ULONG, ULONG);
static NTSTATUS_ (NTAPI *pNtQueryDirectoryFile)(HANDLE, HANDLE, void *, void *, IO_STATUS_BLOCK *, void *, ULONG,
                                                ULONG, BOOLEAN, UNICODE_STRING *, BOOLEAN);

static void nt_files(void)
{
    if (pNtQueryDirectoryFile) return;
    pNtQueryInformationFile = (void *)sym("ntdll.dll", "NtQueryInformationFile");
    pNtSetInformationFile = (void *)sym("ntdll.dll", "NtSetInformationFile");
    pNtQueryDirectoryFile = (void *)sym("ntdll.dll", "NtQueryDirectoryFile");
}

static BOOL WINAPI my_GetFileInformationByHandleEx(HANDLE h, FILE_INFO_BY_HANDLE_CLASS cls, void *buf, DWORD size)
{
    IO_STATUS_BLOCK io;
    NTSTATUS_ s;
    nt_files();
    switch (cls) {
    case FileBasicInfo:          s = pNtQueryInformationFile(h, &io, buf, size, 4);  break;
    case FileStandardInfo:       s = pNtQueryInformationFile(h, &io, buf, size, 5);  break;
    case FileNameInfo:           s = pNtQueryInformationFile(h, &io, buf, size, 9);  break;
    case FileStreamInfo:         s = pNtQueryInformationFile(h, &io, buf, size, 22); break;
    case FileCompressionInfo:    s = pNtQueryInformationFile(h, &io, buf, size, 28); break;
    case FileAttributeTagInfo:   s = pNtQueryInformationFile(h, &io, buf, size, 35); break;
    case FileIdBothDirectoryInfo:
    case FileIdBothDirectoryRestartInfo:
        s = pNtQueryDirectoryFile(h, NULL, NULL, NULL, &io, buf, size, 37 /* FileIdBothDirectoryInformation */,
                                  FALSE, NULL, cls == FileIdBothDirectoryRestartInfo);
        if (s == (NTSTATUS_)0x80000006 /* STATUS_NO_MORE_FILES */) { SetLastError(ERROR_NO_MORE_FILES); return FALSE; }
        break;
    default:
        SetLastError(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if (!NT_OK(s)) { fail_status(s); return FALSE; }
    return TRUE;
}

static BOOL WINAPI my_SetFileInformationByHandle(HANDLE h, FILE_INFO_BY_HANDLE_CLASS cls, void *buf, DWORD size)
{
    IO_STATUS_BLOCK io;
    ULONG nt;
    nt_files();
    switch (cls) {
    case FileBasicInfo:       nt = 4;  break;
    case FileRenameInfo:      nt = 10; break;
    case FileDispositionInfo: nt = 13; break;
    case FileAllocationInfo:  nt = 19; break;
    case FileEndOfFileInfo:   nt = 20; break;
    default:   /* FileDispositionInfoEx, FileRenameInfoEx...: std falls back */
        SetLastError(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    NTSTATUS_ s = pNtSetInformationFile(h, &io, buf, size, nt);
    if (!NT_OK(s)) { fail_status(s); return FALSE; }
    return TRUE;
}

/* \Device\HarddiskVolume1\x -> \\?\C:\x, by asking every drive letter for its
 * device name. Only VOLUME_NAME_DOS (what std asks for). */
static DWORD WINAPI my_GetFinalPathNameByHandleW(HANDLE h, WCHAR *out, DWORD cch, DWORD flags)
{
    static NTSTATUS_ (NTAPI *qo)(HANDLE, int, void *, ULONG, ULONG *);
    if (!qo) qo = (void *)sym("ntdll.dll", "NtQueryObject");
    if (flags & 0xf /* VOLUME_NAME_GUID/NT/NONE */) { SetLastError(ERROR_NOT_SUPPORTED); return 0; }
    union { UNICODE_STRING u; BYTE b[4096]; } name;
    ULONG got;
    NTSTATUS_ s = qo(h, 1 /* ObjectNameInformation */, &name, sizeof name - 2, &got);
    if (!NT_OK(s)) { fail_status(s); return 0; }
    WCHAR *p = name.u.Buffer;
    DWORD n = name.u.Length / 2;
    WCHAR dev[MAX_PATH], drv[3] = L"A:";
    for (WCHAR d = L'A'; d <= L'Z'; d++) {
        drv[0] = d;
        if (!QueryDosDeviceW(drv, dev, MAX_PATH)) continue;
        DWORD dl = lstrlenW(dev);
        if (dl == 0 || dl > n || (n > dl && p[dl] != L'\\')) continue;
        if (CompareStringOrdinal(p, dl, dev, dl, TRUE) != CSTR_EQUAL) continue;
        DWORD need = 4 + 2 + (n - dl);   /* \\?\ + C: + rest */
        if (cch <= need) return need + 1;
        lstrcpyW(out, L"\\\\?\\");
        out[4] = d; out[5] = L':';
        CopyMemory(out + 6, p + dl, (n - dl) * 2);
        out[need] = 0;
        return need;
    }
    SetLastError(ERROR_PATH_NOT_FOUND);
    return 0;
}

/* ---------------------------------------------------------- I/O completion */

static BOOL WINAPI my_GetQueuedCompletionStatusEx(HANDLE port, OVERLAPPED_ENTRY *e, ULONG max, ULONG *removed,
                                                  DWORD ms, BOOL alertable)
{
    (void)alertable;
    ULONG n = 0;
    while (n < max) {
        DWORD bytes = 0;
        ULONG_PTR key = 0;
        OVERLAPPED *ov = NULL;
        BOOL ok = GetQueuedCompletionStatus(port, &bytes, &key, &ov, n ? 0 : ms);
        if (!ok && !ov) break;               /* timed out (or the port failed) */
        e[n].lpCompletionKey = key;
        e[n].lpOverlapped = ov;
        e[n].Internal = ov ? ov->Internal : 0;
        e[n].dwNumberOfBytesTransferred = bytes;
        n++;
    }
    *removed = n;
    if (n == 0) {
        if (GetLastError() != ERROR_ABANDONED_WAIT_0) SetLastError(WAIT_TIMEOUT);
        return FALSE;
    }
    return TRUE;
}

static BOOL WINAPI my_SetFileCompletionNotificationModes(HANDLE h, UCHAR flags)
{
    /* mio asks only for FILE_SKIP_SET_EVENT_ON_HANDLE, an optimisation. Not
     * skipping is always correct; FILE_SKIP_COMPLETION_PORT_ON_SUCCESS would
     * not be, so refuse that. */
    (void)h;
    if (flags & 1 /* FILE_SKIP_COMPLETION_PORT_ON_SUCCESS */) { SetLastError(ERROR_NOT_SUPPORTED); return FALSE; }
    return TRUE;
}

/* XP's NtCancelIoFile cancels the calling thread's requests on the handle. */
static NTSTATUS_ NTAPI my_NtCancelIoFileEx(HANDLE h, IO_STATUS_BLOCK *req, IO_STATUS_BLOCK *io)
{
    static NTSTATUS_ (NTAPI *f)(HANDLE, IO_STATUS_BLOCK *);
    (void)req;
    if (!f) f = (void *)sym("ntdll.dll", "NtCancelIoFile");
    return f(h, io);
}

/* ------------------------------------------------------------ winsock wraps */

static SOCKET (WINAPI *real_WSASocketW)(int, int, int, LPWSAPROTOCOL_INFOW, GROUP, DWORD);
static int (WINAPI *real_WSAIoctl)(SOCKET, DWORD, void *, DWORD, void *, DWORD, DWORD *, LPWSAOVERLAPPED,
                                   LPWSAOVERLAPPED_COMPLETION_ROUTINE);

static SOCKET WINAPI my_WSASocketW(int af, int type, int proto, LPWSAPROTOCOL_INFOW info, GROUP g, DWORD flags)
{
    if (!real_WSASocketW) real_WSASocketW = (void *)sym("ws2_32.dll", "WSASocketW");
    SOCKET s = real_WSASocketW(af, type, proto, info, g, flags);
    if (s == INVALID_SOCKET && (flags & 0x80) && WSAGetLastError() == WSAEINVAL) {
        s = real_WSASocketW(af, type, proto, info, g, flags & ~0x80u);  /* WSA_FLAG_NO_HANDLE_INHERIT */
        if (s != INVALID_SOCKET) SetHandleInformation((HANDLE)s, HANDLE_FLAG_INHERIT, 0);
    }
    return s;
}

static int WINAPI my_WSAIoctl(SOCKET s, DWORD code, void *in, DWORD nin, void *out, DWORD nout, DWORD *ret,
                              LPWSAOVERLAPPED ov, LPWSAOVERLAPPED_COMPLETION_ROUTINE cr)
{
    if (!real_WSAIoctl) real_WSAIoctl = (void *)sym("ws2_32.dll", "WSAIoctl");
    int r = real_WSAIoctl(s, code, in, nin, out, nout, ret, ov, cr);
    if (r != 0 && code == 0x48000022 /* SIO_BASE_HANDLE */ && !ov && out && nout >= sizeof(SOCKET)) {
        int err = WSAGetLastError();
        if (err == WSAEINVAL || err == WSAEOPNOTSUPP) {
            *(SOCKET *)out = s;
            if (ret) *ret = sizeof(SOCKET);
            return 0;
        }
    }
    return r;
}

/* FindExInfoBasic and FIND_FIRST_EX_LARGE_FETCH are Windows 7: XP answers
 * ERROR_INVALID_PARAMETER (std's read_dir asks for the first). Basic only
 * leaves out the 8.3 name, so Standard is a superset. */
static HANDLE (WINAPI *real_FindFirstFileExW)(const WCHAR *, FINDEX_INFO_LEVELS, void *, FINDEX_SEARCH_OPS, void *, DWORD);

static HANDLE WINAPI my_FindFirstFileExW(const WCHAR *path, FINDEX_INFO_LEVELS lvl, void *data, FINDEX_SEARCH_OPS op,
                                         void *filter, DWORD flags)
{
    if (!real_FindFirstFileExW) real_FindFirstFileExW = (void *)sym("kernel32.dll", "FindFirstFileExW");
    HANDLE h = real_FindFirstFileExW(path, lvl, data, op, filter, flags);
    if (h == INVALID_HANDLE_VALUE && GetLastError() == ERROR_INVALID_PARAMETER &&
        (lvl == FindExInfoBasic || (flags & 6 /* LARGE_FETCH | ON_DISK_ENTRIES_ONLY */)))
        h = real_FindFirstFileExW(path, FindExInfoStandard, data, op, filter, flags & ~6u);
    return h;
}

/* --------------------------------------------------------- crash record
 * XP shows only "has encountered a problem". Write what the crash was to
 * %TEMP%\RagnarokOffline-crash.txt: code, address, and every word on the
 * faulting stack that points into this exe (return addresses, mostly; read
 * them with nm). A Rust abort is `int 29h`, which XP turns into an access
 * violation, so it lands here too. Only installed on XP (NT 5).
 */
static LONG WINAPI crash_record(EXCEPTION_POINTERS *ep)
{
    char path[MAX_PATH + 32];
    DWORD n = GetTempPathA(MAX_PATH, path);
    if (!n || n > MAX_PATH) return EXCEPTION_CONTINUE_SEARCH;
    lstrcatA(path, "RagnarokOffline-crash.txt");
    HANDLE f = CreateFileA(path, FILE_APPEND_DATA, FILE_SHARE_READ, NULL, OPEN_ALWAYS, 0, NULL);
    if (f == INVALID_HANDLE_VALUE) return EXCEPTION_CONTINUE_SEARCH;
    HMODULE exe = GetModuleHandleA(NULL);
    IMAGE_NT_HEADERS *nt = (IMAGE_NT_HEADERS *)((BYTE *)exe + ((IMAGE_DOS_HEADER *)exe)->e_lfanew);
    ULONG_PTR lo = (ULONG_PTR)exe, hi = lo + nt->OptionalHeader.SizeOfImage;
    char line[160];
    DWORD w;
    EXCEPTION_RECORD *er = ep->ExceptionRecord;
    wsprintfA(line, "exception %08lx at %08lx (exe+%lx) thread %lu, info %08lx %08lx\r\nstack:",
              er->ExceptionCode, (ULONG_PTR)er->ExceptionAddress, (ULONG_PTR)er->ExceptionAddress - lo,
              GetCurrentThreadId(), er->NumberParameters > 0 ? er->ExceptionInformation[0] : 0,
              er->NumberParameters > 1 ? er->ExceptionInformation[1] : 0);
    WriteFile(f, line, lstrlenA(line), &w, NULL);
    ULONG_PTR *sp = (ULONG_PTR *)ep->ContextRecord->Esp;
    NT_TIB *tib = (NT_TIB *)NtCurrentTeb();
    int shown = 0;
    for (; (void *)sp < tib->StackBase && shown < 64; sp++) {
        if (*sp >= lo && *sp < hi) {
            wsprintfA(line, " %lx", *sp - lo);
            WriteFile(f, line, lstrlenA(line), &w, NULL);
            shown++;
        }
    }
    WriteFile(f, "\r\n", 2, &w, NULL);
    CloseHandle(f);
    return EXCEPTION_CONTINUE_SEARCH;
}

__attribute__((constructor)) static void crash_record_install(void)
{
    if ((GetVersion() & 0xff) < 6) SetUnhandledExceptionFilter(crash_record);
}

/* ------------------------------------------------------------- resolvers
 * IMP(dll, Name, N, ret, params, args): defines `__imp__Name@N`, first
 * pointing at a resolver that picks the system's function or ours.
 * Wrappers (WSASocketW, WSAIoctl) always take ours: they call the real one.
 */
static LONG g_sync = -1;   /* 1 = the system's SRW/CV/INIT_ONCE, 0 = ours */

static int native_sync(void)
{
    if (g_sync < 0)
        InterlockedCompareExchange(&g_sync, sym("kernel32.dll", "SleepConditionVariableSRW") != NULL, -1);
    return g_sync;
}

#define STR_(x) #x
#define STR(x) STR_(x)
/* Three spellings of each pointer: std and the C libraries import the stdcall
 * name (`__imp__Name@N`), some import libraries the cdecl one (`__imp__Name`),
 * windows-sys's raw-dylib (mio) the verbatim one (`__imp_Name`). The resolver
 * sets all three. Code that calls without dllimport
 * wants the import thunk (`_Name@N`, `_Name`): one jmp through the pointer,
 * defined here too, or the linker pulls the import library's member, whose
 * pointer then clashes with ours. Names are quoted for the assembler, which
 * would read `@4` as a symbol variant. */
#define IMP(group, dll, name, n, ret, params, args)                                         \
    static ret WINAPI rs_##name params;                                                       \
    void *imp_##name __asm__("__imp__" #name "@" #n) = (void *)rs_##name;                     \
    void *impu_##name __asm__("__imp__" #name) = (void *)rs_##name;                           \
    void *impv_##name __asm__("__imp_" #name) = (void *)rs_##name;                            \
    __asm__(".text\n"                                                                         \
            ".globl \"_" #name "@" #n "\"\n\"_" #name "@" #n "\":\n"                          \
            "\tjmp *\"__imp__" #name "@" #n "\"\n"                                            \
            ".globl _" #name "\n_" #name ":\n"                                                \
            "\tjmp *__imp__" #name "\n");                                                     \
    static ret WINAPI rs_##name params                                                        \
    {                                                                                         \
        void *xp_fn = NULL;                                                                   \
        if (group == 0 || (group == 1 && native_sync()))                                      \
            xp_fn = (void *)sym(dll, #name);                                                  \
        if (!xp_fn) xp_fn = (void *)my_##name;                                                \
        if (xp_fn == (void *)my_FlsAlloc) g_fls_ours = 1;                                     \
        imp_##name = xp_fn;                                                                   \
        impu_##name = xp_fn;                                                                  \
        impv_##name = xp_fn;                                                                  \
        return ((ret(WINAPI *) params)xp_fn) args;                                            \
    }

/* group 0: system's if present; 1: the sync group; 2: always ours */
IMP(1, "kernel32.dll", AcquireSRWLockExclusive, 4, void, (SRWLOCK *a), (a))
IMP(1, "kernel32.dll", AcquireSRWLockShared, 4, void, (SRWLOCK *a), (a))
IMP(1, "kernel32.dll", ReleaseSRWLockExclusive, 4, void, (SRWLOCK *a), (a))
IMP(1, "kernel32.dll", ReleaseSRWLockShared, 4, void, (SRWLOCK *a), (a))
IMP(1, "kernel32.dll", TryAcquireSRWLockExclusive, 4, BOOLEAN, (SRWLOCK *a), (a))
IMP(1, "kernel32.dll", TryAcquireSRWLockShared, 4, BOOLEAN, (SRWLOCK *a), (a))
IMP(1, "kernel32.dll", SleepConditionVariableSRW, 16, BOOL, (CONDITION_VARIABLE *a, SRWLOCK *b, DWORD c, ULONG d), (a, b, c, d))
IMP(1, "kernel32.dll", WakeConditionVariable, 4, void, (CONDITION_VARIABLE *a), (a))
IMP(1, "kernel32.dll", WakeAllConditionVariable, 4, void, (CONDITION_VARIABLE *a), (a))
IMP(1, "kernel32.dll", InitOnceBeginInitialize, 16, BOOL, (INIT_ONCE *a, DWORD b, BOOL *c, void **d), (a, b, c, d))
IMP(1, "kernel32.dll", InitOnceComplete, 12, BOOL, (INIT_ONCE *a, DWORD b, void *c), (a, b, c))
IMP(1, "kernel32.dll", FlsAlloc, 4, DWORD, (PFLS_CALLBACK_FUNCTION a), (a))
IMP(1, "kernel32.dll", FlsFree, 4, BOOL, (DWORD a), (a))
IMP(1, "kernel32.dll", FlsSetValue, 8, BOOL, (DWORD a, void *b), (a, b))
IMP(1, "kernel32.dll", FlsGetValue, 4, void *, (DWORD a), (a))
IMP(0, "kernel32.dll", IsThreadAFiber, 0, BOOL, (void), ())
IMP(0, "kernel32.dll", SetThreadStackGuarantee, 4, BOOL, (ULONG *a), (a))
IMP(0, "kernel32.dll", CompareStringOrdinal, 20, int, (const WCHAR *a, int b, const WCHAR *c, int d, BOOL e), (a, b, c, d, e))
IMP(0, "kernel32.dll", CreateWaitableTimerExW, 16, HANDLE, (SECURITY_ATTRIBUTES *a, const WCHAR *b, DWORD c, DWORD d), (a, b, c, d))
IMP(0, "kernel32.dll", GetFileInformationByHandleEx, 16, BOOL, (HANDLE a, FILE_INFO_BY_HANDLE_CLASS b, void *c, DWORD d), (a, b, c, d))
IMP(0, "kernel32.dll", SetFileInformationByHandle, 16, BOOL, (HANDLE a, FILE_INFO_BY_HANDLE_CLASS b, void *c, DWORD d), (a, b, c, d))
IMP(0, "kernel32.dll", GetFinalPathNameByHandleW, 16, DWORD, (HANDLE a, WCHAR *b, DWORD c, DWORD d), (a, b, c, d))
IMP(0, "kernel32.dll", GetQueuedCompletionStatusEx, 24, BOOL, (HANDLE a, OVERLAPPED_ENTRY *b, ULONG c, ULONG *d, DWORD e, BOOL f), (a, b, c, d, e, f))
IMP(0, "kernel32.dll", SetFileCompletionNotificationModes, 8, BOOL, (HANDLE a, UCHAR b), (a, b))
IMP(0, "kernel32.dll", K32EnumProcessModules, 16, BOOL, (HANDLE a, HMODULE *b, DWORD c, DWORD *d), (a, b, c, d))
IMP(0, "ntdll.dll", NtCancelIoFileEx, 12, NTSTATUS_, (HANDLE a, IO_STATUS_BLOCK *b, IO_STATUS_BLOCK *c), (a, b, c))
IMP(0, "user32.dll", GetDpiForSystem, 0, UINT, (void), ())
IMP(0, "bcrypt.dll", BCryptGenRandom, 16, LONG, (void *a, UCHAR *b, ULONG c, ULONG d), (a, b, c, d))
IMP(2, "kernel32.dll", FindFirstFileExW, 24, HANDLE, (const WCHAR *a, FINDEX_INFO_LEVELS b, void *c, FINDEX_SEARCH_OPS d, void *e, DWORD f), (a, b, c, d, e, f))
IMP(2, "ws2_32.dll", WSASocketW, 24, SOCKET, (int a, int b, int c, LPWSAPROTOCOL_INFOW d, GROUP e, DWORD f), (a, b, c, d, e, f))
IMP(2, "ws2_32.dll", WSAIoctl, 36, int, (SOCKET a, DWORD b, void *c, DWORD d, void *e, DWORD f, DWORD *g, LPWSAOVERLAPPED h, LPWSAOVERLAPPED_COMPLETION_ROUTINE i), (a, b, c, d, e, f, g, h, i))
