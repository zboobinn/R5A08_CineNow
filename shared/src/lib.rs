use serde::{Deserialize, Serialize};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub genres: Vec<String>,
    pub rating: f32,
    pub poster_path: Option<String>,
    pub overview: String,
    pub release_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieCatalog {
    pub movies: Vec<Movie>,
    pub total: usize,
}

/// Taille max d'un paquet MQTT : un catalogue dépasse parfois les 10 Ko par défaut de rumqttc
pub const MQTT_MAX_PACKET_SIZE: usize = 64 * 1024;

/// Demande publiée sur `mood/selected`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodRequest {
    pub request_id: u64,
    pub moods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedMovie {
    pub id: u32,
    pub score: f32,
}

/// Réponse publiée sur `recommendations/result`, reliée à sa demande par `request_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationResponse {
    pub request_id: u64,
    pub movies: Vec<RecommendedMovie>,
}