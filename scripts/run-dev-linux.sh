#!/bin/bash
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Vérifier que Shiftr écoute sur le port 1883
if ! nc -z localhost 1883 2>/dev/null; then
    echo "ERREUR - Lancez Shiftr Desktop d'abord."
    exit 1
fi
echo "OK - Shiftr detecte"

# Lancer chaque service dans un terminal séparé
gnome-terminal -- bash -c "cd '$ROOT/services/weather-service' && cargo run -p weather-service; exec bash" 2>/dev/null || \
xterm -e "cd '$ROOT/services/weather-service' && cargo run -p weather-service; bash" &
sleep 2

gnome-terminal -- bash -c "cd '$ROOT/services/movie-service' && cargo run -p movie-service; exec bash" 2>/dev/null || \
xterm -e "cd '$ROOT/services/movie-service' && cargo run -p movie-service; bash" &
sleep 2

gnome-terminal -- bash -c "cd '$ROOT/services/recommendation-service' && cargo run -p recommendation-service; exec bash" 2>/dev/null || \
xterm -e "cd '$ROOT/services/recommendation-service' && cargo run -p recommendation-service; bash" &
sleep 2

gnome-terminal -- bash -c "cd '$ROOT/services/web-server' && cargo run -p web-server; exec bash" 2>/dev/null || \
xterm -e "cd '$ROOT/services/web-server' && cargo run -p web-server; bash" &
sleep 3

gnome-terminal -- bash -c "cd '$ROOT/ui' && npm run dev; exec bash" 2>/dev/null || \
xterm -e "cd '$ROOT/ui' && npm run dev; bash" &
sleep 3

echo "Services demarres en mode dev !"