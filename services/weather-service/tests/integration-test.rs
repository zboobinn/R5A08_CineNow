use weather_service::{translate_weather_code, get_time_of_day};

/// Scénario 1 : Codes météo limites — vérifier les bornes des plages
#[test]
fn test_bornes_codes_meteo() {
    // Borne basse de chaque plage
    assert_eq!(translate_weather_code(0), "Ensoleillé");
    assert_eq!(translate_weather_code(1), "Nuageux");
    assert_eq!(translate_weather_code(42), "Brouillard");
    assert_eq!(translate_weather_code(51), "Pluie");
    assert_eq!(translate_weather_code(71), "Neige");
    assert_eq!(translate_weather_code(80), "Averses");
    assert_eq!(translate_weather_code(95), "Orageux");

    // Borne haute de chaque plage
    assert_eq!(translate_weather_code(41), "Nuageux");
    assert_eq!(translate_weather_code(48), "Brouillard");
    assert_eq!(translate_weather_code(67), "Pluie");
    assert_eq!(translate_weather_code(77), "Neige");
    assert_eq!(translate_weather_code(86), "Averses");
    assert_eq!(translate_weather_code(99), "Orageux");
}

/// Scénario 2 : Codes météo hors plage → Inconnu
#[test]
fn test_codes_inconnus() {
    assert_eq!(translate_weather_code(-1), "Inconnu");
    assert_eq!(translate_weather_code(100), "Inconnu");
    assert_eq!(translate_weather_code(999), "Inconnu");
}

/// Scénario 3 : Périodes de la journée — vérifier les transitions d'heures
#[test]
fn test_transitions_periodes() {
    // Transitions exactes
    assert_eq!(get_time_of_day("2024-01-01T05:59"), "Nuit");
    assert_eq!(get_time_of_day("2024-01-01T06:00"), "Matin");
    assert_eq!(get_time_of_day("2024-01-01T11:59"), "Matin");
    assert_eq!(get_time_of_day("2024-01-01T12:00"), "Après-midi");
    assert_eq!(get_time_of_day("2024-01-01T17:59"), "Après-midi");
    assert_eq!(get_time_of_day("2024-01-01T18:00"), "Soir");
    assert_eq!(get_time_of_day("2024-01-01T21:59"), "Soir");
    assert_eq!(get_time_of_day("2024-01-01T22:00"), "Nuit");
}

/// Scénario 4 : Format de timestamp invalide → valeur par défaut sans panique
#[test]
fn test_timestamp_invalide() {
    assert_eq!(get_time_of_day(""), "Après-midi");
    assert_eq!(get_time_of_day("format-invalide"), "Après-midi");
    assert_eq!(get_time_of_day("T"), "Après-midi");
}