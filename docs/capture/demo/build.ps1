<#
.SYNOPSIS
    Builds demo/demo.db, the populated profile most documentation captures start from.

    1. Starts Simulacra once on an empty folder so it creates and seeds its database.
    2. seed.py adds months of posts and points the AI and embedding engines at stub-ai.mjs.
    3. populate.ps1 (if present) drives the app to create what is easiest made through the UI.
    4. history.py (if present) back-dates copies of journal, mood, focus and review records.

.NOTES
    Needs `cargo build` and Python 3 with the standard library. Starts stub-ai.mjs itself.
#>
param([string]$Exe, [switch]$SkipPopulate)
$ErrorActionPreference = 'Stop'
Import-Module (Join-Path $PSScriptRoot '../scripts/Simulacra.psm1') -Force -WarningAction SilentlyContinue

$profileDir = Join-Path ([System.IO.Path]::GetTempPath()) "simulacra-demo-build-$(Get-Date -Format yyyyMMddHHmmss)"
$db = Join-Path $profileDir 'simulacra.db'
$startArgs = @{ ProfileDir = $profileDir }
if ($Exe) { $startArgs.Exe = $Exe }

function Wait-MainWindow {
    $deadline = (Get-Date).AddSeconds(60)
    while (-not (Get-SimulacraWindow)) {
        if ((Get-Date) -gt $deadline) { throw 'Simulacra did not open a window' }
        Start-Sleep -Milliseconds 300
    }
    Start-Sleep -Seconds 9
}

function Stop-App($proc) {
    # Ctrl+Q is not bound; close the window so SQLite finishes its writes, then make sure.
    $h = Get-SimulacraWindow
    if ($h) { [void][SimWin]::SetForegroundWindow($h); [System.Windows.Forms.SendKeys]::SendWait('%{F4}') }
    Start-Sleep -Seconds 3
    if (-not $proc.HasExited) { Stop-Process -Id $proc.Id -Force }
    Start-Sleep -Milliseconds 500
}

Get-Process simulacra -ErrorAction SilentlyContinue | Stop-Process -Force
Write-Host "1/4 first start (seed) in $profileDir" -ForegroundColor Cyan
$proc = Start-Simulacra @startArgs
Wait-MainWindow
Stop-App $proc

Write-Host '2/4 demo posts and engines' -ForegroundColor Cyan
python (Join-Path $PSScriptRoot 'seed.py') $db
if ($LASTEXITCODE) { throw 'seed.py failed' }

$populate = Join-Path $PSScriptRoot 'populate.ps1'
if (-not $SkipPopulate -and (Test-Path $populate)) {
    Write-Host '3/4 content made through the UI' -ForegroundColor Cyan
    $stub = $null
    if (-not (Get-NetTCPConnection -LocalPort 8765 -State Listen -ErrorAction SilentlyContinue)) {
        $stub = Start-Process node -ArgumentList (Join-Path $PSScriptRoot '../stub-ai.mjs') -PassThru -WindowStyle Hidden
    }
    $proc = Start-Simulacra @startArgs
    Wait-MainWindow
    . $populate
    Stop-App $proc
    if ($stub) { Stop-Process -Id $stub.Id -Force }
}

$history = Join-Path $PSScriptRoot 'history.py'
if (Test-Path $history) {
    Write-Host '4/4 back-dated history' -ForegroundColor Cyan
    python $history $db
    if ($LASTEXITCODE) { throw 'history.py failed' }
}

# Fold the write-ahead log into the file before copying it.
python -c "import sqlite3,sys; c=sqlite3.connect(sys.argv[1]); c.execute('PRAGMA wal_checkpoint(TRUNCATE)'); c.close()" $db
Copy-Item $db (Join-Path $PSScriptRoot 'demo.db') -Force
$media = Join-Path $profileDir 'media'
if (Test-Path $media) {
    Remove-Item (Join-Path $PSScriptRoot 'media') -Recurse -Force -ErrorAction SilentlyContinue
    Copy-Item $media (Join-Path $PSScriptRoot 'media') -Recurse
}
Write-Host "demo.db written from $profileDir" -ForegroundColor Green
