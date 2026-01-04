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

Avant de commencer, installez les logiciels suivants :

#### 1. **Rust** (version 1.75 ou supérieure)
- **Windows** : https://rustup.rs/
  - Télécharger et exécuter ``rustup-init.exe``
  - Redémarrer PowerShell après l'installation
- **Linux/Mac** : 
``````bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
``````
- Vérifier l'installation :
``````bash
  rustc --version
  cargo --version
``````

#### 2. **Git**
- **Windows** : https://git-scm.com/download/win
- **Linux (Debian/Ubuntu)** :
``````bash
  sudo apt install git
``````
- **Mac** :
``````bash
  brew install git
``````
- Vérifier l'installation :
``````bash
  git --version
``````

#### 3. **Mosquitto** (broker MQTT)
- **Windows** : https://mosquitto.org/download/
  - Télécharger l'installateur 64-bit
  - Ajouter ``C:\Program Files\mosquitto`` au PATH système
  - Redémarrer PowerShell
- **Linux (Debian/Ubuntu)** :
``````bash
  sudo apt update
  sudo apt install mosquitto mosquitto-clients
``````
- **Mac** :
``````bash
  brew install mosquitto
  brew services start mosquitto
``````
- Vérifier l'installation :
``````bash
  mosquitto -h
``````

#### 4. **Clé API TMDb** (gratuite)
- Créer un compte : https://www.themoviedb.org/signup
- Demander une clé API :
  1. Se connecter sur TMDb
  2. Aller dans **Settings** → **API**
  3. Cliquer sur **"Request an API Key"**
  4. Choisir **"Developer"**
  5. Remplir le formulaire (Type: Education/Student)
  6. Copier votre **API Key (v3 auth)**
  

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

### Lancement

//todo

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

