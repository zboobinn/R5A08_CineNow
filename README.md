# 🎬 Ciné-Now

**Application de recommandation de films basée sur l'humeur et la météo**

Projet de Programmation Répartie - Rust  
Info3-TP1K - IUT Lyon 1

## 👥 Équipe

- Albin CANALS
- Léo NOAILLES
- Abel CHAREYRON
- Antoine LANDON

## 📖 Description

Ciné-Now est une application distribuée qui recommande des films personnalisés en fonction de :
- L'humeur de l'utilisateur (joie, tristesse, colère, etc.)
- Les conditions météorologiques actuelles


## 🏗️ Architecture

- **5 services Rust** communiquant via **MQTT**
- **Interface utilisateur** : Tauri (à venir)
- **APIs externes** :
    - Open-Meteo (météo)
    - TMDb (films)

## 🚀 Installation

### Prérequis

- Rust 1.75+
- Mosquitto (broker MQTT)
- Clé API TMDb

### Configuration

#### Windows

1. Cloner le dépôt :
``````powershell
git clone https://gitlab.iut-valence.fr/canalsal/r5.a_08-cinenow.git
cd r5.a_08-cinenow
``````

2. Configurer l'environnement :
``````powershell
Copy-Item .env.example .env
code .env  # Ajouter votre clé TMDb
``````

3. Compiler le projet :
``````powershell
cargo build
``````

#### Linux

1. Cloner le dépôt :
``````bash
git clone https://gitlab.iut-valence.fr/canalsal/r5.a_08-cinenow.git
cd r5.a_08-cinenow
``````

2. Configurer l'environnement :
``````bash
cp .env.example .env
nano .env  # Ajouter votre clé TMDb
``````

3. Compiler le projet :
``````bash
cargo build
``````

### Lancement (à venir)

#### Windows
``````powershell
# Démarrer le broker MQTT (Terminal 1)
mosquitto -c mosquitto.conf

# Lancer les services (scripts à venir)
.\scripts\run-all.ps1
``````

#### Linux
``````bash
# Démarrer le broker MQTT (Terminal 1)
mosquitto -c mosquitto.conf

# Lancer les services (scripts à venir)
./scripts/run-all.sh
``````

## 📚 Documentation

- [Cahier des charges](docs/cahier_des_charges.md)
- [Architecture détaillée](docs/architecture.md) (à venir)

## 🧪 Tests

#### Windows
``````powershell
# Tests unitaires
cargo test

# Tests d'intégration (à venir)
cargo test --test integration
``````

#### Linux
``````bash
# Tests unitaires
cargo test

# Tests d'intégration (à venir)
cargo test --test integration
``````

