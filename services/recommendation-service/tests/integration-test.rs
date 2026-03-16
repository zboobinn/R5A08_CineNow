use std::collections::HashMap;
use recommendation_service::{WeatherData, compute_recommendations};

fn make_movie(id: u32, genre_label: &str, rating: f32) -> shared::Movie {
    shared::Movie {
        id,
        title: format!("Film {}", id),
        genres: vec![genre_label.to_string()],
        rating,
        poster_path: None,
        overview: String::new(),
        release_date: None,
    }
}

fn make_catalog() -> HashMap<String, Vec<shared::Movie>> {
    let mut catalog = HashMap::new();
    catalog.insert("comedie".to_string(), vec![
        make_movie(1, "Comédie", 8.0),
        make_movie(2, "Comédie", 7.5),
    ]);
    catalog.insert("action".to_string(), vec![
        make_movie(3, "Action", 8.5),
        make_movie(4, "Action", 7.0),
    ]);
    catalog.insert("drame".to_string(), vec![
        make_movie(5, "Drame", 9.0),
        make_movie(6, "Drame", 6.5),
    ]);
    catalog.insert("romance".to_string(), vec![
        make_movie(7, "Romance", 7.8),
    ]);
    catalog.insert("thriller".to_string(), vec![
        make_movie(8, "Thriller", 8.2),
    ]);
    catalog
}

/// Scénario 1 : Utilisateur joyeux + soleil → comédie/romance en tête
#[test]
fn test_joy_soleil_comedie_en_tete() {
    let catalog = make_catalog();
    let weather = WeatherData {
        condition: "Ensoleillé".to_string(),
        period: "Matin".to_string(),
    };
    let moods = vec!["joy".to_string()];

    let result = compute_recommendations(&moods, &weather, &catalog);

    assert!(!result.is_empty());
    let top3_ids: Vec<u32> = result.iter().take(3).map(|r| r.id).collect();
    assert!(top3_ids.iter().any(|id| [1, 2, 7].contains(id)),
        "Comédie/Romance devrait dominer avec joy+soleil, top3: {:?}", top3_ids);
}

/// Scénario 2 : Utilisateur en colère + orage → action/thriller en tête
#[test]
fn test_angry_orageux_action_en_tete() {
    let catalog = make_catalog();
    let weather = WeatherData {
        condition: "Orageux".to_string(),
        period: "Soir".to_string(),
    };
    let moods = vec!["angry".to_string()];

    let result = compute_recommendations(&moods, &weather, &catalog);

    assert!(!result.is_empty());
    let top3_ids: Vec<u32> = result.iter().take(3).map(|r| r.id).collect();
    assert!(top3_ids.iter().any(|id| [3, 4, 8].contains(id)),
        "Action/Thriller devrait dominer avec angry+orage, top3: {:?}", top3_ids);
}

/// Scénario 3 : Multi-humeurs joy + sad → films comédie ET drame présents
#[test]
fn test_multi_humeurs_diversite() {
    let catalog = make_catalog();
    let weather = WeatherData {
        condition: "Nuageux".to_string(),
        period: "Après-midi".to_string(),
    };
    let moods = vec!["joy".to_string(), "sad".to_string()];

    let result = compute_recommendations(&moods, &weather, &catalog);

    assert!(!result.is_empty());
    let ids: Vec<u32> = result.iter().map(|r| r.id).collect();
    let has_comedie = ids.iter().any(|id| [1, 2].contains(id));
    let has_drame = ids.iter().any(|id| [5, 6].contains(id));
    assert!(has_comedie && has_drame,
        "Multi-humeurs devrait inclure comédie et drame, ids: {:?}", ids);
}

/// Scénario 4 : Max 4 films par genre respecté
#[test]
fn test_diversite_max_4_par_genre() {
    let mut catalog = HashMap::new();
    catalog.insert("action".to_string(), (1u32..=6)
        .map(|i| make_movie(i, "Action", 8.0))
        .collect());
    catalog.insert("comedie".to_string(), vec![make_movie(10, "Comédie", 7.0)]);

    let weather = WeatherData {
        condition: "Orageux".to_string(),
        period: "Nuit".to_string(),
    };
    let moods = vec!["angry".to_string()];

    let result = compute_recommendations(&moods, &weather, &catalog);

    let action_count = result.iter().filter(|r| r.id >= 1 && r.id <= 6).count();
    assert!(action_count <= 4,
        "Max 4 films par genre, mais on a {} films d'action", action_count);
}

/// Scénario 5 : Catalogue vide → résultat vide sans panique
#[test]
fn test_catalogue_vide() {
    let catalog = HashMap::new();
    let weather = WeatherData {
        condition: "Ensoleillé".to_string(),
        period: "Matin".to_string(),
    };
    let moods = vec!["joy".to_string()];

    let result = compute_recommendations(&moods, &weather, &catalog);
    assert!(result.is_empty());
}