use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeoResponse {
    pub city: Option<String>,
    pub locality: Option<String>,
}

pub fn translate_weather_code(code: i32) -> String {
    match code {
        0 => "Ensoleillé".to_string(),
        1..=41 => "Nuageux".to_string(),
        42..=48 => "Brouillard".to_string(),
        51..=67 => "Pluie".to_string(),
        71..=77 => "Neige".to_string(),
        80..=86 => "Averses".to_string(),
        95..=99 => "Orageux".to_string(),
        _ => "Inconnu".to_string(),
    }
}

pub fn get_time_of_day(time_str: &str) -> String {
    let hour = time_str.split('T').nth(1)
        .and_then(|t| t.split(':').next())
        .and_then(|h| h.parse::<u32>().ok())
        .unwrap_or(12);

    match hour {
        6..=11 => "Matin".to_string(),
        12..=17 => "Après-midi".to_string(),
        18..=21 => "Soir".to_string(),
        _ => "Nuit".to_string(),
    }
}

pub async fn get_city_name(lat: f32, lon: f32) -> String {
    let url = format!(
        "https://api.bigdatacloud.net/data/reverse-geocode-client?latitude={}&longitude={}&localityLanguage=fr",
        lat, lon
    );

    if let Ok(response) = reqwest::get(&url).await {
        if let Ok(data) = response.json::<GeoResponse>().await {
            // On essaie d'abord "city", sinon "locality" (pour les petits villages)
            if let Some(city) = data.city.filter(|c| !c.is_empty()) {
                return city;
            }
            if let Some(locality) = data.locality.filter(|l| !l.is_empty()) {
                return locality;
            }
        }
    }
    // Si l'API échoue, on garde une valeur par défaut
    "Position locale".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- translate_weather_code ---

    #[test]
    fn test_weather_code_ensoleille() {
        assert_eq!(translate_weather_code(0), "Ensoleillé");
    }

    #[test]
    fn test_weather_code_nuageux() {
        assert_eq!(translate_weather_code(1), "Nuageux");
        assert_eq!(translate_weather_code(41), "Nuageux");
    }

    #[test]
    fn test_weather_code_brouillard() {
        assert_eq!(translate_weather_code(45), "Brouillard");
    }

    #[test]
    fn test_weather_code_pluie() {
        assert_eq!(translate_weather_code(51), "Pluie");
        assert_eq!(translate_weather_code(67), "Pluie");
    }

    #[test]
    fn test_weather_code_neige() {
        assert_eq!(translate_weather_code(71), "Neige");
        assert_eq!(translate_weather_code(77), "Neige");
    }

    #[test]
    fn test_weather_code_averses() {
        assert_eq!(translate_weather_code(80), "Averses");
        assert_eq!(translate_weather_code(86), "Averses");
    }

    #[test]
    fn test_weather_code_orageux() {
        assert_eq!(translate_weather_code(95), "Orageux");
        assert_eq!(translate_weather_code(99), "Orageux");
    }

    #[test]
    fn test_weather_code_inconnu() {
        assert_eq!(translate_weather_code(200), "Inconnu");
    }

    // --- get_time_of_day ---

    #[test]
    fn test_period_matin() {
        assert_eq!(get_time_of_day("2024-01-01T06:00"), "Matin");
        assert_eq!(get_time_of_day("2024-01-01T11:00"), "Matin");
    }

    #[test]
    fn test_period_apres_midi() {
        assert_eq!(get_time_of_day("2024-01-01T12:00"), "Après-midi");
        assert_eq!(get_time_of_day("2024-01-01T17:00"), "Après-midi");
    }

    #[test]
    fn test_period_soir() {
        assert_eq!(get_time_of_day("2024-01-01T18:00"), "Soir");
        assert_eq!(get_time_of_day("2024-01-01T21:00"), "Soir");
    }

    #[test]
    fn test_period_nuit() {
        assert_eq!(get_time_of_day("2024-01-01T22:00"), "Nuit");
        assert_eq!(get_time_of_day("2024-01-01T05:00"), "Nuit");
    }

    #[test]
    fn test_period_format_invalide() {
        // Doit retourner une valeur par défaut sans paniquer
        assert_eq!(get_time_of_day("format-invalide"), "Après-midi");
    }
}