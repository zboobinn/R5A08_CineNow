## 1. Vision du Projet

### 1.1 Objectif Général

Développer une application répartie de recommandation de films personnalisée, basée sur l'humeur de l'utilisateur et les conditions météorologiques actuelles.

### 1.2 Valeur Ajoutée

- **Personnalisation contextuelle** : Recommandations adaptées à l'état émotionnel et au contexte météorologique
- **Expérience utilisateur fluide** : Interface moderne et responsive
- **Architecture modulaire** : Services indépendants et réutilisables
- **Apprentissage utilisateur** : Amélioration des recommandations via l'historique

---

## 2. Fonctionnalités

### 2.1 Fonctionnalités Essentielles

### F1 - Récupération des données météorologiques

- **Description** : Récupération automatique des conditions météo actuelles
- **Données récupérées** :
    - Température (°C)
    - Condition météorologique (pluie, soleil, nuageux, neige)
    - Heure de la journée (matin, après-midi, soir, nuit)
- **Source** : API Open-Meteo (gratuite, sans clé API)
- **Fréquence** : À la demande + cache de 15 minutes

### F2 - Sélection de l'humeur utilisateur

- **Description** : Interface permettant à l'utilisateur de choisir son état émotionnel
- **Humeurs disponibles** :
    - Joie / Bonne humeur
    - Tristesse / Mélancolie
    - Colère / Frustration, Vengeance
    - Peur / Anxiété
    - Fatigue / Besoin de détente
    - Aventure / Énergie, Action
    - Réflexion / Introspection, Psychologie
- **Interface** : Boutons/icônes cliquables

### F3 - Récupération des données films

- **Description** : Collecte d'informations détaillées sur les films
- **Données récupérées** :
    - Titre original et traduit
    - Genres
    - Synopsis
    - Note IMDb/TMDb
    - Affiche (poster)
    - Acteurs principaux (3-5)
    - Réalisateur
    - Année de sortie
    - Durée
- **Source** : API TMDb (clé API gratuite requise)
- **Volume initial** : Top 100 films populaires par genre

### F4 - Génération de recommandations (météo + humeur)

- **Description** : Algorithme de recommandation basé sur humeur + météo
- **Logique de mapping** :

## Tableau amélioré avec valeurs numériques

| Humeur | Météo | Genres Recommandés | Poids | Multiplicateur | Explication |
| --- | --- | --- | --- | --- | --- |
| **Joie** | ☀️ Soleil | Comédie, Romance, Animation | ★★★ | **1.5** | Match parfait : humeur + contexte |
| **Joie** | 🌧️ Pluie | Comédie romantique, Feel-good | ★★ | **1.2** | Bon match mais moins évident |
| **Joie** | ☁️ Nuageux | Comédie, Animation | ★★ | **1.2** | Match correct |
|  |  |  |  |  |  |
| **Tristesse** | 🌧️ Pluie | Drame, Romance dramatique | ★★★ | **1.5** | Atmosphère cohérente |
| **Tristesse** | ☀️ Soleil | Drame léger, Biopic inspirant | ★★ | **1.2** | Contraste potentiellement aidant |
| **Tristesse** | ☁️ Nuageux | Drame | ★★½ | **1.3** | Bon match atmosphérique |
|  |  |  |  |  |  |
| **Colère** | 🌩️ Orage | Action, Thriller intense, Guerre | ★★★ | **1.5** | Exutoire maximal |
| **Colère** | ☀️ Soleil | Action, Sport | ★★ | **1.2** | Action pour évacuer |
| **Colère** | Tous | Action, Thriller | ★★½ | **1.3** | Toujours pertinent |
|  |  |  |  |  |  |
| **Peur** | 🌧️ Pluie | Thriller psychologique | ★★ | **1.2** | Si veut intensifier |
| **Peur** | ☀️ Soleil | Comédie, Feel-good | ★★★ | **1.5** | Pour rassurer (inverse) |
| **Peur** | 🌙 Nuit | Horreur | ★ | **1.0** | Déconseillé mais disponible |
|  |  |  |  |  |  |
| **Fatigue** | Tous | Animation, Comédie légère | ★★★ | **1.4** | Facile à suivre |
| **Fatigue** | 🌧️ Pluie | Film familial, Comédies courtes | ★★★ | **1.5** | Parfait pour rester au chaud |
|  |  |  |  |  |  |
| **Aventure** | ☀️ Soleil | Aventure, Fantastique, SF | ★★★ | **1.5** | Envie d'évasion maximale |
| **Aventure** | 🌧️ Pluie | Aventure exotique | ★★½ | **1.3** | Évasion compensatoire |
|  |  |  |  |  |  |
| **Réflexion** | ☁️ Nuageux | Drame, SF philosophique | ★★★ | **1.4** | Ambiance propice |
| **Réflexion** | 🌧️ Pluie | Drame intense, Documentaire | ★★★ | **1.5** | Concentration maximale |
| **Réflexion** | ☀️ Soleil | Biopic, Drame léger | ★★ | **1.2** | Moins intense |

## 🔢 Échelle des Poids

| Symbole | Multiplicateur | Signification | Usage |
| --- | --- | --- | --- |
| ★★★ | **1.4 - 1.5** | **Match parfait** | Combinaison idéale humeur + météo |
| ★★½ | **1.3** | **Très bon match** | Une des deux dimensions très pertinente |
| ★★ | **1.2** | **Bon match** | Recommandation solide mais pas évidente |
| ★½ | **1.1** | **Match acceptable** | Peut fonctionner |
| ★ | **1.0** | **Neutre/Déconseillé** | Pas de bonus |
- **Nombre de recommandations** : 5-10 films
- **Tri** : Par score de pertinence (décroissant)
- **Optionnel :** Ajout des réalisateurs/acteurs favoris dans le système de tri

### F5 - Génération de recommandations (historique personnel)

- **Description** : Algorithme de recommandation basé sur son propre historiques de films
- **Logique de mapping** :

| Humeur | Météo | Genres Recommandés | Poids |
| --- | --- | --- | --- |
| Joie | Soleil | Comédie, Romance, Animation | ★★★ |
| Joie | Pluie | Comédie romantique, Feel-good | ★★ |
| Tristesse | Pluie | Drame, Romance dramatique | ★★★ |
| Tristesse | Soleil | Drame léger, Biopic inspirant | ★★ |
| Colère | Tous | Action, Thriller, Guerre | ★★★ |
| Peur | Tous | Horreur, Thriller psychologique | ★★ |
| Fatigue | Tous | Animation, Comédie légère | ★★★ |
| Aventure | Soleil | Aventure, Fantastique, SF | ★★★ |
| Réflexion | Tous | Drame, Documentaire, SF philosophique | ★★ |

### F5 - Affichage des recommandations

- **Description** : Interface présentant les films recommandés
- **Informations affichées par film** :
    - Affiche (grande taille)
    - Titre
    - Genres (badges)
    - Note ⭐ (sur 10)
    - Durée
    - Année
    - Synopsis (résumé 2-3 lignes)
    - Acteurs principaux
- **Interactions** :
    - Clic sur un film → Détails complets
    - Bouton "J'ai vu" → Ajout à l'historique
    - Bouton "Pas intéressé" → Exclusion temporaire

### F6 - Affichage des conditions météo

- **Description** : Visualisation en temps réel de la météo
- **Éléments UI** :
    - Icône météo animée
    - Température
    - Description textuelle ("Pluie légère", "Ensoleillé", etc.)
    - Localisation (ville)
- **Position** : En-tête de l'application

### 2.2 Fonctionnalités Secondaires (Extensions)

### E1 - Historique utilisateur

- **Description** : Mémorisation des films consultés et notés
- **Données enregistrées** :
    - Films vus (avec date)
    - Films notés par l'utilisateur (1-5 étoiles)
    - Films exclus
- **Utilisation** :
    - Ne pas recommander les films déjà vus
    - Améliorer les suggestions futures

### E2 - Profil utilisateur

- **Description** : Personnalisation des préférences
- **Paramètres** :
    - Genres favoris (liste)
    - Acteurs favoris (liste)
    - Réalisateurs favoris (liste)
    - Décennies préférées (ex: 1990-2000)
    - Exclusions (genres à éviter)
- **Impact** : Bonus de score pour les films correspondants

### E3 - Justification des recommandations

- **Description** : Explication du choix de chaque film
- **Exemples** :
    - "Correspond à ton humeur *colère* et au temps *orageux*"
    - "Action intense recommandée pour évacuer la frustration"
    - "Inclut ton acteur favori *Leonardo DiCaprio*"
- **Affichage** : Badge/tooltip sous chaque film

### E4 - Système de notation

- **Description** : Permettre à l'utilisateur de noter les films vus
- **Échelle** : 1 à 5 étoiles
- **Utilisation** :
    - Apprentissage : affiner les goûts de l'utilisateur
    - Statistiques : films les mieux notés

### E5 - Recherche manuelle

- **Description** : Recherche de films par titre/acteur/genre
- **Interface** : Barre de recherche
- **Résultats** : Liste avec mêmes infos que recommandations

### E6 - Mode "Découverte"

- **Description** : Recommander des films moins connus
- **Critères** :
    - Note > 7/10
    - Nombre de votes < 50 000
    - Années variées