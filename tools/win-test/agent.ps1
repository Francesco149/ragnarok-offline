# ro-agent: runs inside the interactive desktop session of a real Windows box
# (started by a scheduled task, see win.sh) and executes PowerShell snippets
# dropped into its queue directory over ssh. ssh sessions live in session 0
# and cannot see or drive the desktop; this can.
#
#   agent.ps1 -Name user|admin      (one medium-IL and one elevated instance:
#                                    UIPI stops a medium process from driving
#                                    elevated windows such as msiexec's UI)
#
# Helpers available to snippets: Shot, Click, RClick, DClick, MoveTo, Key, Send,
# Run, Launch, Windows, Focus, Restore, GameClick. The agent exits after $IdleHours without commands.
param([string]$Name = 'user', [double]$IdleHours = 4)
$ErrorActionPreference = 'Continue'
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$Q = Join-Path $Root "q-$Name"
New-Item -ItemType Directory -Force $Q | Out-Null
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @"
using System; using System.Runtime.InteropServices; using System.Text;
public static class U {
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, int dx, int dy, uint d, UIntPtr e);
  [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint f, UIntPtr e);
  [DllImport("user32.dll")] public static extern uint MapVirtualKey(uint code, uint type);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int c);
  [DllImport("user32.dll")] public static extern bool BringWindowToTop(IntPtr h);
  public delegate bool EnumProc(IntPtr h, IntPtr l);
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc p, IntPtr l);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint flags);
  [DllImport("user32.dll")] public static extern bool PostMessageW(IntPtr h, uint m, IntPtr w, IntPtr l);
}
"@
[U]::SetProcessDPIAware() | Out-Null

function Shot([string]$File, [int]$X = 0, [int]$Y = 0, [int]$W = 0, [int]$H = 0) {
  $b = [System.Windows.Forms.SystemInformation]::VirtualScreen
  if ($W -eq 0) { $X = $b.X; $Y = $b.Y; $W = $b.Width; $H = $b.Height }
  $bmp = New-Object System.Drawing.Bitmap $W, $H
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($X, $Y, 0, 0, $bmp.Size)
  $g.Dispose()
  New-Item -ItemType Directory -Force (Split-Path -Parent $File) | Out-Null
  $bmp.Save($File, [System.Drawing.Imaging.ImageFormat]::Png); $bmp.Dispose()
  $File
}
# a real (absolute) mouse move through the input stream, so hooks that remap
# the cursor (e.g. DDrawCompat's emulated fullscreen) see it like a user's mouse
function MoveTo([int]$X, [int]$Y) {
  $b = [System.Windows.Forms.SystemInformation]::VirtualScreen
  $nx = [int](($X - $b.X) * 65535 / ($b.Width - 1)); $ny = [int](($Y - $b.Y) * 65535 / ($b.Height - 1))
  [U]::mouse_event(0x8001 -bor 0x4000, $nx, $ny, 0, [UIntPtr]::Zero)  # MOVE | ABSOLUTE | VIRTUALDESK
}
function Click([int]$X, [int]$Y, [int]$Hold = 150) {
  MoveTo $X $Y; Start-Sleep -Milliseconds 120
  [U]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds $Hold
  [U]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero)
}
function RClick([int]$X, [int]$Y) {
  MoveTo $X $Y; Start-Sleep -Milliseconds 120
  [U]::mouse_event(8, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 150
  [U]::mouse_event(16, 0, 0, 0, [UIntPtr]::Zero)
}
function DClick([int]$X, [int]$Y) { Click $X $Y 60; Start-Sleep -Milliseconds 60; Click $X $Y 60 }
# Key 'RETURN' 'TAB' 'ALT+N' ... : virtual-key names from System.Windows.Forms.Keys
function Key {
  foreach ($k in $args) {
    $parts = "$k".ToUpper().Split('+')
    $vks = foreach ($p in $parts) {
      switch ($p) { 'ALT' { 0x12 } 'CTRL' { 0x11 } 'SHIFT' { 0x10 } 'ENTER' { 0x0D } 'ESC' { 0x1B }
        default { [int][System.Windows.Forms.Keys]::Parse([System.Windows.Forms.Keys], $p, $true) } }
    }
    foreach ($v in $vks) { [U]::keybd_event([byte]$v, [byte][U]::MapVirtualKey($v, 0), 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 40 }
    Start-Sleep -Milliseconds 80
    [array]::Reverse($vks)
    foreach ($v in $vks) { [U]::keybd_event([byte]$v, [byte][U]::MapVirtualKey($v, 0), 2, [UIntPtr]::Zero); Start-Sleep -Milliseconds 40 }
    Start-Sleep -Milliseconds 120
  }
}
function Send([string]$Text) {
  foreach ($c in $Text.ToCharArray()) {
    $s = "$c"; if ('+^%~(){}[]'.Contains($s)) { $s = "{$s}" }
    [System.Windows.Forms.SendKeys]::SendWait($s); Start-Sleep -Milliseconds 70
  }
}
function Run([string]$Exe, [string]$ArgList = '', [string]$Dir = '') {
  if (-not $Dir) { $Dir = Split-Path -Parent $Exe }
  $p = if ($ArgList) { Start-Process -FilePath $Exe -ArgumentList $ArgList -WorkingDirectory $Dir -PassThru }
       else { Start-Process -FilePath $Exe -WorkingDirectory $Dir -PassThru }
  "pid $($p.Id)"
}
# visible top-level windows: hwnd pid process [x,y wxh] class "title"
function Windows([string]$Match = '') {
  $list = New-Object System.Collections.ArrayList
  $cb = [U+EnumProc] { param($h, $l)
    if ([U]::IsWindowVisible($h)) {
      $t = New-Object System.Text.StringBuilder 256; [U]::GetWindowText($h, $t, 256) | Out-Null
      $c = New-Object System.Text.StringBuilder 256; [U]::GetClassName($h, $c, 256) | Out-Null
      $r = New-Object U+RECT; [U]::GetWindowRect($h, [ref]$r) | Out-Null
      $procId = [uint32]0; [U]::GetWindowThreadProcessId($h, [ref]$procId) | Out-Null
      $pn = (Get-Process -Id $procId -ErrorAction SilentlyContinue).ProcessName
      if ($t.Length -gt 0 -and "$t $c $pn" -match $Match) {
        [void]$list.Add(('{0} {1} {2} [{3},{4} {5}x{6}] {7} "{8}"' -f $h, $procId, $pn, $r.L, $r.T, ($r.R - $r.L), ($r.B - $r.T), $c, $t))
      }
    }
    $true }
  [U]::EnumWindows($cb, [IntPtr]::Zero) | Out-Null
  $list
}
function Focus([string]$Title) {
  $w = Windows $Title | Select-Object -First 1
  if (-not $w) { return "no window matching $Title" }
  $h = [IntPtr][long]($w.Split(' ')[0])
  [U]::ShowWindow($h, 9) | Out-Null
  # SetForegroundWindow works for the process that received the last input
  # event: tap Shift (Alt would put the target window into menu mode)
  [U]::keybd_event(0x10, 0, 0, [UIntPtr]::Zero); [U]::keybd_event(0x10, 0, 2, [UIntPtr]::Zero)
  [U]::SetForegroundWindow($h) | Out-Null; [U]::BringWindowToTop($h) | Out-Null
  $w
}
# Click inside a DDrawCompat-emulated fullscreen game at game coordinates.
# DDrawCompat clips the real cursor to the emulated resolution and rescales
# relative mouse motion in a low-level hook (MouseSensitivity), so absolute
# moves do not land; step with relative moves until the cursor is there.
function GameClick([int]$X, [int]$Y, [int]$Hold = 150) {
  for ($i = 0; $i -lt 25; $i++) {
    $p = [System.Windows.Forms.Cursor]::Position
    $dx = $X - $p.X; $dy = $Y - $p.Y
    if ([math]::Abs($dx) -le 1 -and [math]::Abs($dy) -le 1) { break }
    [U]::mouse_event(1, [int]($dx / 2), [int]($dy / 2), 0, [UIntPtr]::Zero)
    if ([math]::Abs($dx) -le 3 -and [math]::Abs($dy) -le 3) { [U]::mouse_event(1, [math]::Sign($dx), [math]::Sign($dy), 0, [UIntPtr]::Zero) }
    Start-Sleep -Milliseconds 40
  }
  Start-Sleep -Milliseconds 80
  [U]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds $Hold
  [U]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero)
  "$([System.Windows.Forms.Cursor]::Position)"
}

# un-minimize a window the way the taskbar does (fullscreen games minimize
# themselves when they lose focus and ignore a plain ShowWindow)
function Restore([string]$Title) {
  $w = Windows $Title | Select-Object -First 1
  if (-not $w) { return "no window matching $Title" }
  $h = [IntPtr][long]($w.Split(' ')[0])
  # become the foreground process first (own form), then hand it over, like a taskbar click
  $f = New-Object System.Windows.Forms.Form
  $f.Width = 220; $f.Height = 60; $f.StartPosition = 'Manual'; $f.Left = 0; $f.Top = 0
  $f.Show(); [System.Windows.Forms.Application]::DoEvents()
  [U]::keybd_event(0x10, 0, 0, [UIntPtr]::Zero); [U]::keybd_event(0x10, 0, 2, [UIntPtr]::Zero)
  [U]::SetForegroundWindow($f.Handle) | Out-Null; [System.Windows.Forms.Application]::DoEvents()
  [U]::PostMessageW($h, 0x112, [IntPtr]0xF120, [IntPtr]::Zero) | Out-Null  # WM_SYSCOMMAND SC_RESTORE
  [U]::SetForegroundWindow($h) | Out-Null
  Start-Sleep -Milliseconds 300; $f.Close(); $f.Dispose()
  Start-Sleep -Milliseconds 700
  Windows $Title | Select-Object -First 1
}
# start a program the way a double-click in Explorer does: from the process
# that owns the foreground window, so the program may take the foreground
# (a hidden agent process could not pass that right on)
function Launch([string]$Exe, [string]$ArgList = '') {
  $f = New-Object System.Windows.Forms.Form
  $f.Text = 'ro-agent launch'; $f.Width = 220; $f.Height = 60; $f.StartPosition = 'Manual'; $f.Left = 0; $f.Top = 0
  $f.Show(); [System.Windows.Forms.Application]::DoEvents()
  [U]::keybd_event(0x10, 0, 0, [UIntPtr]::Zero); [U]::keybd_event(0x10, 0, 2, [UIntPtr]::Zero)
  [U]::SetForegroundWindow($f.Handle) | Out-Null; [System.Windows.Forms.Application]::DoEvents()
  $fg = ([U]::GetForegroundWindow() -eq $f.Handle)
  # CreateProcess, not ShellExecute: ShellExecute may broadcast DDE to every
  # top-level window and wait - including our own form, which is not pumping
  $si = New-Object System.Diagnostics.ProcessStartInfo $Exe, $ArgList
  $si.UseShellExecute = $false; $si.WorkingDirectory = Split-Path -Parent $Exe
  $r = "pid $([System.Diagnostics.Process]::Start($si).Id)"
  Start-Sleep -Milliseconds 800; $f.Close(); $f.Dispose()
  "$r (agent was foreground: $fg)"
}

"agent $Name pid $PID started $(Get-Date -Format s)" | Set-Content (Join-Path $Root "agent-$Name.log")
$last = Get-Date
while (((Get-Date) - $last).TotalHours -lt $IdleHours) {
  foreach ($f in @(Get-ChildItem $Q -Filter *.cmd | Sort-Object Name)) {
    $last = Get-Date
    $code = [IO.File]::ReadAllText($f.FullName)
    Remove-Item $f.FullName
    if ($code.Trim() -eq 'quit') { exit 0 }
    try { $out = & ([scriptblock]::Create($code)) 2>&1 | Out-String -Width 400 }
    catch { $out = "ERROR: $($_ | Out-String)" }
    $o = $f.FullName -replace '\.cmd$', ''
    [IO.File]::WriteAllText("$o.tmp", "$out")
    Move-Item "$o.tmp" "$o.out" -Force
  }
  Start-Sleep -Milliseconds 100
}
