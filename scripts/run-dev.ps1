$root = Split-Path -Parent $PSScriptRoot

$tcp = New-Object System.Net.Sockets.TcpClient
try {
    $tcp.Connect("localhost", 1883)
    $tcp.Close()
    Write-Host "OK - Shiftr detecte" -ForegroundColor Green
} catch {
    Write-Host "ERREUR - Lancez Shiftr Desktop d'abord." -ForegroundColor Red
    exit 1
}

Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$root\services\weather-service'; cargo run -p weather-service" -WindowStyle Normal
Start-Sleep -Seconds 2
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$root\services\movie-service'; cargo run -p movie-service" -WindowStyle Normal
Start-Sleep -Seconds 2
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$root\services\recommendation-service'; cargo run -p recommendation-service" -WindowStyle Normal
Start-Sleep -Seconds 2
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$root\services\web-server'; cargo run -p web-server" -WindowStyle Normal
Start-Sleep -Seconds 3
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$root\ui'; npm run tauri dev" -WindowStyle Normal

Write-Host "Services demarres en mode dev !" -ForegroundColor Green