#!/bin/bash
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="$ROOT/target/release"

# Vérifier que Shiftr écoute sur le port 1883
if ! nc -z localhost 1883 2>/dev/null; then
    echo "ERREUR - Lancez Shiftr Desktop d'abord."
    exit 1
fi
echo "OK - Shiftr detecte"

# Build des services Rust
echo "Build des services Rust..."
cd "$ROOT"
cargo build --release
if [ $? -ne 0 ]; then
    echo "ERREUR - cargo build --release a echoue."
    exit 1
fi

cd "$ROOT"

# Lancer chaque service depuis son dossier
gnome-terminal -- bash -c "cd '$ROOT/services/weather-service' && '$BIN/weather-service'; exec bash" 2>/dev/null || \
xterm -e "cd '$ROOT/services/weather-service' && '$BIN/weather-service'; bash" &
sleep 2

gnome-terminal -- bash -c "cd '$ROOT/services/movie-service' && '$BIN/movie-service'; exec bash" 2>/dev/null || \
xterm -e "cd '$ROOT/services/movie-service' && '$BIN/movie-service'; bash" &
sleep 2

gnome-terminal -- bash -c "cd '$ROOT/services/recommendation-service' && '$BIN/recommendation-service'; exec bash" 2>/dev/null || \
xterm -e "cd '$ROOT/services/recommendation-service' && '$BIN/recommendation-service'; bash" &
sleep 2

gnome-terminal -- bash -c "cd '$ROOT/services/web-server' && '$BIN/web-server'; exec bash" 2>/dev/null || \
xterm -e "cd '$ROOT/services/web-server' && '$BIN/web-server'; bash" &
sleep 3

echo "Services demarres !"