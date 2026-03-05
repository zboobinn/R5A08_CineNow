# 🎬 Ciné-Now

**Application de recommandation de films basée sur l'humeur et la météo**

Projet de Programmation Répartie - Rust  
Info3-TP1K 

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
  - Télécharger et exécuter `rustup-init.exe`
  - Redémarrer PowerShell après l'installation
- **Linux/Mac** : 
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Vérifier l'installation :
  ```bash
  rustc --version
  cargo --version
  ```

#### 2. **Git**
- **Windows** : https://git-scm.com/download/win
- **Linux (Debian/Ubuntu)** :
  ```bash
  sudo apt install git
  ```
- **Mac** :
  ```bash
  brew install git
  ```
- Vérifier l'installation :
  ```bash
  git --version
  ```

#### 3. **Shiftr Desktop** (broker MQTT avec interface visuelle)
- **Windows** : https://www.shiftr.io/desktop#downloads
  - Télécharger et installer `shiftr-desktop-setup.exe`
  - Lancer l'application
  - Interface web : http://localhost:1884
- **Linux** :
  ```bash
  wget https://www.shiftr.io/desktop/download/latest/linux/deb
  sudo dpkg -i shiftr-desktop_*.deb
  shiftr-desktop
  ```
- Vérifier l'installation :
  - Ouvrir http://localhost:1884
  - L'interface Shiftr doit s'afficher

#### 4. **Clé API TMDb** (gratuite)
- Créer un compte : https://www.themoviedb.org/signup
- Demander une clé API :
  1. Se connecter sur TMDb
  2. Aller dans **Settings** → **API**
  3. Cliquer sur **"Request an API Key"**
  4. Choisir **"Developer"**
  5. Remplir le formulaire (Type: Education/Student)
  6. Copier votre **API Key (v3 auth)**

#### 5. **VS Code** (recommandé)
- Télécharger : https://code.visualstudio.com/
- Extensions recommandées :
  - `rust-analyzer` (Rust Language Server)
  - `Even Better TOML`
  - `GitLens`
  - `Error Lens`

### Configuration

#### Windows

1. Cloner le dépôt :
```powershell
git clone https://gitlab.iut-valence.fr/canalsal/r5.a_08-cinenow.git
cd r5.a_08-cinenow
```

2. Configurer l'environnement :
```powershell
Copy-Item .env.example .env
code .env  # Ajouter votre clé TMDb
```

3. Compiler le projet :
```powershell
cargo build
```

#### Linux

1. Cloner le dépôt :
```bash
git clone https://gitlab.iut-valence.fr/canalsal/r5.a_08-cinenow.git
cd r5.a_08-cinenow
```

2. Configurer l'environnement :
```bash
cp .env.example .env
nano .env  # Ajouter votre clé TMDb
```

3. Compiler le projet :
```bash
cargo build
```

### Lancement

#### Mode développement

Ouvrir un terminal par service, dans cet ordre :

1. Lancer **Shiftr Desktop**

2. **Weather Service** :
```bash
cargo run -p weather-service
```

3. **Movie Service** :
```bash
cargo run -p movie-service
```

4. **Recommendation Service** :
```bash
cargo run -p recommendation-service
```

5. **Web Server** :
```bash
cargo run -p web-server
```

6. **Interface UI** :
```bash
cd ui
npm run tauri dev
```

#### Lancement via l'exécutable

1. Lancer **Shiftr Desktop**
2. Lancer les services dans l'ordre ci-dessus avec `cargo run`
3. Lancer `ui/src-tauri/target/release/bundle/nsis/CineNow_0.1.0_x64-setup.exe` pour installer l'application
4. Ouvrir **CineNow** depuis le bureau ou le menu démarrer

## 📚 Documentation

- [Cahier des charges](docs/cahier_des_charges.md)
- [Architecture détaillée](docs/architecture.md) (à venir)

## 🧪 Tests

#### Windows
```powershell
# Tests unitaires
cargo test

# Tests d'intégration (à venir)
cargo test --test integration
```

#### Linux
```bash
# Tests unitaires
cargo test

# Tests d'intégration (à venir)
cargo test --test integration
```