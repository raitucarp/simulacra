# Puts one file on the clipboard (as Explorer's Copy does) for a paste in Simulacra, then puts the
# previous clipboard back once the flag file appears (or after a minute). Runs as a hidden
# Windows PowerShell -STA process: the clipboard needs a single-threaded apartment.
param([Parameter(Mandatory)][string]$File, [Parameter(Mandatory)][string]$Flag)
Add-Type -AssemblyName System.Windows.Forms
$saved = $null
try {
    $old = [System.Windows.Forms.Clipboard]::GetDataObject()
    if ($old) {
        $saved = New-Object System.Windows.Forms.DataObject
        foreach ($format in $old.GetFormats($false)) {
            try { $data = $old.GetData($format, $false); if ($null -ne $data) { $saved.SetData($format, $data) } } catch { }
        }
    }
} catch { $saved = $null }
$list = New-Object System.Collections.Specialized.StringCollection
[void]$list.Add($File)
[System.Windows.Forms.Clipboard]::SetFileDropList($list)
New-Item -ItemType File -Force -Path "$Flag.ready" | Out-Null
$until = (Get-Date).AddMinutes(1)
while (-not (Test-Path $Flag) -and (Get-Date) -lt $until) { Start-Sleep -Milliseconds 100 }
if ($saved -and $saved.GetFormats($false).Count) { [System.Windows.Forms.Clipboard]::SetDataObject($saved, $true) }
else { [System.Windows.Forms.Clipboard]::Clear() }
