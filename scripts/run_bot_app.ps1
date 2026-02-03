$ErrorActionPreference = 'Stop'

# Stop existing listener on 8080 (if any).
$listener = Get-NetTCPConnection -LocalPort 8080 -State Listen -ErrorAction SilentlyContinue | Select-Object -First 1
if ($listener) {
    try { Stop-Process -Id $listener.OwningProcess -ErrorAction SilentlyContinue } catch {}
}

# Load .env into process environment (does not echo values).
if (Test-Path ".env") {
    Get-Content .env | ForEach-Object {
        $line = $_.Trim()
        if ($line -eq "" -or $line.StartsWith("#")) { return }
        $parts = $line -split "=",2
        if ($parts.Count -lt 2) { return }
        $name = $parts[0].Trim()
        $val = $parts[1].Trim()
        if ($val.StartsWith('"') -and $val.EndsWith('"')) { $val = $val.Substring(1, $val.Length-2) }
        if ($val.StartsWith("'") -and $val.EndsWith("'")) { $val = $val.Substring(1, $val.Length-2) }
        Set-Item -Path "env:$name" -Value $val
    }
}

$stdout = Join-Path (Get-Location) "bot_app.log"
$stderr = Join-Path (Get-Location) "bot_app.err.log"
Remove-Item -Force -ErrorAction SilentlyContinue $stdout, $stderr

# Build first so we can run the binary directly and capture its logs.
& "C:\\Users\\judge\\.cargo\\bin\\cargo.exe" build -p bot_app | Out-Null

$exe = Join-Path (Get-Location) "target\\debug\\bot_app.exe"
$p = Start-Process -FilePath $exe -WorkingDirectory (Get-Location) -RedirectStandardOutput $stdout -RedirectStandardError $stderr -PassThru

Start-Sleep -Seconds 2
$listen = Get-NetTCPConnection -LocalPort 8080 -State Listen -ErrorAction SilentlyContinue
"PID=$($p.Id) LISTEN=$([bool]$listen)"
