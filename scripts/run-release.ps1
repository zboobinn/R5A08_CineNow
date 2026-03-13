$root = Split-Path -Parent $PSScriptRoot
$bin = "$root\target\release"

$tcp = New-Object System.Net.Sockets.TcpClient
try {
    $tcp.Connect("localhost", 1883)
    $tcp.Close()
    Write-Host "OK - Shiftr detecte" -ForegroundColor Green
} catch {
    Write-Host "ERREUR - Lancez Shiftr Desktop d'abord." -ForegroundColor Red
    exit 1
}

Write-Host "Build des services Rust..." -ForegroundColor Cyan
Set-Location $root
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERREUR - cargo build --release a echoue." -ForegroundColor Red
    exit 1
}

Write-Host "Build de l'interface Tauri..." -ForegroundColor Cyan
Set-Location "$root\ui"
npm run tauri build
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERREUR - tauri build a echoue." -ForegroundColor Red
    exit 1
}

Set-Location $root
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$root\services\weather-service'; & '$bin\weather-service.exe'" -WindowStyle Normal
Start-Sleep -Seconds 2
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$root\services\movie-service'; & '$bin\movie-service.exe'" -WindowStyle Normal
Start-Sleep -Seconds 2
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$root\services\recommendation-service'; & '$bin\recommendation-service.exe'" -WindowStyle Normal
Start-Sleep -Seconds 2
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$root\services\web-server'; & '$bin\web-server.exe'" -WindowStyle Normal
Start-Sleep -Seconds 3

Write-Host "Services demarres ! Ouvrez le .exe Tauri dans :" -ForegroundColor Green
Write-Host "  $bin" -ForegroundColor Cyan
Invoke-Item $bin