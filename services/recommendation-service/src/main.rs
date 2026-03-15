use dotenvy::dotenv;
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, sync::Arc, time::Duration};
use tokio::sync::Mutex;

// Topics entrants
const TOPIC_WEATHER: &str = "weather/current";
const TOPIC_MOOD: &str = "mood/selected";
const MOVIE_TOPICS: &[&str] = &[
    "movies/comedie", "movies/romance", "movies/animation",
    "movies/drame", "movies/action", "movies/thriller",
    "movies/guerre", "movies/horreur", "movies/familial",
    "movies/aventure", "movies/fantastique", "movies/science-fiction",
    "movies/documentaire", "movies/histoire",
];

// Topic sortant
const TOPIC_RESULT: &str = "recommendations/result";

#[derive(Debug, Clone, Deserialize, Default)]
struct WeatherData {
    condition: String,
    period: String,
}

#[derive(Debug, Deserialize)]
struct MoodPayload {
    moods: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RecommendedMovie {
    id: u32,
    score: f32,
}

#[derive(Clone)]
struct State {
    weather: Arc<Mutex<WeatherData>>,
    movies: Arc<Mutex<HashMap<String, Vec<shared::Movie>>>>,
}

/// Retourne les genres recommandés avec leur multiplicateur pour une humeur + météo + période donnés
fn get_genre_weights(mood: &str, condition: &str, period: &str) -> Vec<(&'static str, f32)> {
    let is_night = period == "Nuit" || period == "Soir";
    let is_rain = matches!(condition, "Pluie" | "Averses");
    let is_storm = condition == "Orageux";
    let is_sun = condition == "Ensoleillé";
    let is_cloudy = matches!(condition, "Nuageux" | "Brouillard");
    let is_snow = condition == "Neige";

    match mood {
        "joy" => {
            if is_sun        { vec![("comedie", 1.5), ("romance", 1.4), ("animation", 1.3), ("familial", 1.1)] }
            else if is_rain  { vec![("comedie", 1.3), ("romance", 1.3), ("animation", 1.2)] }
            else if is_storm { vec![("comedie", 1.2), ("animation", 1.2), ("familial", 1.1)] }
            else if is_snow  { vec![("animation", 1.4), ("familial", 1.4), ("comedie", 1.3)] }
            else             { vec![("comedie", 1.3), ("animation", 1.2), ("familial", 1.2)] }
        }
        "sad" => {
            if is_rain       { vec![("drame", 1.5), ("romance", 1.4), ("histoire", 1.2)] }
            else if is_storm { vec![("drame", 1.5), ("thriller", 1.2)] }
            else if is_sun   { vec![("drame", 1.2), ("histoire", 1.3), ("romance", 1.1)] }
            else if is_snow  { vec![("drame", 1.3), ("romance", 1.4), ("familial", 1.1)] }
            else             { vec![("drame", 1.4), ("romance", 1.2), ("histoire", 1.1)] }
        }
        "angry" => {
            if is_storm      { vec![("action", 1.5), ("thriller", 1.5), ("guerre", 1.4)] }
            else if is_rain  { vec![("action", 1.4), ("thriller", 1.4), ("guerre", 1.3)] }
            else if is_sun   { vec![("action", 1.4), ("aventure", 1.3), ("thriller", 1.2)] }
            else if is_snow  { vec![("action", 1.3), ("thriller", 1.3), ("guerre", 1.2)] }
            else             { vec![("action", 1.4), ("thriller", 1.3), ("guerre", 1.2)] }
        }
        "calm" => {
            if is_sun        { vec![("documentaire", 1.4), ("animation", 1.3), ("familial", 1.3)] }
            else if is_cloudy { vec![("documentaire", 1.5), ("histoire", 1.4), ("romance", 1.2)] }
            else if is_rain  { vec![("documentaire", 1.4), ("histoire", 1.3), ("drame", 1.2)] }
            else if is_snow  { vec![("documentaire", 1.4), ("animation", 1.4), ("familial", 1.3)] }
            else             { vec![("documentaire", 1.3), ("histoire", 1.3), ("animation", 1.2)] }
        }
        "tired" => {
            if is_rain       { vec![("familial", 1.5), ("animation", 1.5), ("fantastique", 1.2)] }
            else if is_snow  { vec![("familial", 1.5), ("animation", 1.5), ("comedie", 1.3)] }
            else if is_storm { vec![("animation", 1.4), ("familial", 1.4), ("comedie", 1.2)] }
            else if is_sun   { vec![("animation", 1.3), ("comedie", 1.3), ("romance", 1.2)] }
            else             { vec![("animation", 1.4), ("familial", 1.4), ("comedie", 1.2)] }
        }
        "fear" => {
            if is_sun              { vec![("comedie", 1.5), ("familial", 1.4), ("romance", 1.3)] }
            else if is_night && is_storm { vec![("horreur", 1.3), ("thriller", 1.4)] }
            else if is_night       { vec![("thriller", 1.3), ("horreur", 1.2), ("comedie", 1.1)] }
            else if is_rain        { vec![("thriller", 1.2), ("comedie", 1.3), ("familial", 1.2)] }
            else if is_snow        { vec![("familial", 1.4), ("animation", 1.3), ("comedie", 1.2)] }
            else                   { vec![("thriller", 1.2), ("comedie", 1.3), ("familial", 1.2)] }
        }
        "adventure" => {
            if is_sun        { vec![("aventure", 1.5), ("fantastique", 1.5), ("science-fiction", 1.4), ("action", 1.3)] }
            else if is_rain  { vec![("aventure", 1.4), ("fantastique", 1.3), ("science-fiction", 1.3)] }
            else if is_storm { vec![("action", 1.5), ("aventure", 1.4), ("guerre", 1.3)] }
            else if is_snow  { vec![("aventure", 1.5), ("fantastique", 1.4), ("action", 1.3)] }
            else             { vec![("aventure", 1.4), ("fantastique", 1.4), ("science-fiction", 1.3)] }
        }
        "reflection" => {
            if is_rain       { vec![("documentaire", 1.5), ("drame", 1.5), ("science-fiction", 1.3)] }
            else if is_cloudy { vec![("documentaire", 1.5), ("histoire", 1.4), ("science-fiction", 1.3)] }
            else if is_sun   { vec![("histoire", 1.3), ("documentaire", 1.3), ("drame", 1.2)] }
            else if is_snow  { vec![("documentaire", 1.4), ("science-fiction", 1.4), ("histoire", 1.3)] }
            else if is_storm { vec![("drame", 1.4), ("thriller", 1.3), ("science-fiction", 1.3)] }
            else             { vec![("documentaire", 1.4), ("histoire", 1.3), ("drame", 1.3)] }
        }
        _ => { vec![("comedie", 1.0), ("action", 1.0), ("drame", 1.0)] }
    }
}

/// Calcule le score d'un film : note TMDb × multiplicateur de genre
#[allow(dead_code)]
fn compute_score(movie: &shared::Movie, genre: &str, weight: f32) -> f32 {
    if movie.genres.iter().any(|g| g.to_lowercase().contains(&genre.to_lowercase())) {
        movie.rating * weight
    } else {
        movie.rating
    }
}

async fn compute_recommendations(
    moods: &[String],
    weather: &WeatherData,
    movies_by_genre: &HashMap<String, Vec<shared::Movie>>,
) -> Vec<RecommendedMovie> {
    let mut scores: HashMap<u32, f32> = HashMap::new();

    // Score de base pénalisé pour tous les films
    for films in movies_by_genre.values() {
        for film in films {
            scores.entry(film.id).or_insert(film.rating * 0.7);
        }
    }

    // Appliquer les multiplicateurs selon humeur + météo
    for mood in moods {
        let weights = get_genre_weights(mood, &weather.condition, &weather.period);
        for (genre, multiplier) in &weights {
            if let Some(films) = movies_by_genre.get(*genre) {
                for film in films {
                    let score = film.rating * multiplier;
                    let entry = scores.entry(film.id).or_insert(0.0);
                    if score > *entry {
                        *entry = score;
                    }
                }
            }
        }
    }

    // Mapper chaque film à son genre principal
    let mut all_movies_map: HashMap<u32, String> = HashMap::new();
    for (genre, films) in movies_by_genre {
        for film in films {
            all_movies_map.entry(film.id).or_insert(genre.clone());
        }
    }

    // Trier par score, diversifier (max 4 films par genre), garder top 20
    let mut genre_count: HashMap<String, usize> = HashMap::new();
    let mut sorted: Vec<(u32, f32)> = scores.into_iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut result: Vec<RecommendedMovie> = Vec::new();
    for (id, score) in sorted {
        if result.len() >= 20 { break; }
        let genre = all_movies_map.get(&id).cloned().unwrap_or_default();
        let count = genre_count.entry(genre).or_insert(0);
        if *count < 4 {
            *count += 1;
            result.push(RecommendedMovie { id, score });
        }
    }

    result
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Recommendation Service démarré...");

    let broker_host = env::var("MQTT_BROKER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let broker_port: u16 = env::var("MQTT_BROKER_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .unwrap_or(1883);

    let mut mqttoptions = MqttOptions::new("recommendation-service", broker_host, broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 20);

    let state = State {
        weather: Arc::new(Mutex::new(WeatherData::default())),
        movies: Arc::new(Mutex::new(HashMap::new())),
    };

    let state_clone = state.clone();
    let client_clone = client.clone();

    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(rumqttc::Event::Incoming(Packet::Publish(msg))) => {
                    let payload = String::from_utf8_lossy(&msg.payload);

                    if msg.topic == TOPIC_WEATHER {
                        if let Ok(data) = serde_json::from_str::<WeatherData>(&payload) {
                            *state_clone.weather.lock().await = data;
                        }
                    } else if msg.topic.starts_with("movies/") {
                        if let Ok(catalog) = serde_json::from_str::<shared::MovieCatalog>(&payload) {
                            let genre = msg.topic.replace("movies/", "");
                            state_clone.movies.lock().await.insert(genre, catalog.movies);
                        }
                    } else if msg.topic == TOPIC_MOOD {
                        if let Ok(mood_data) = serde_json::from_str::<MoodPayload>(&payload) {
                            println!("Humeur reçue : {:?}", mood_data.moods);

                            let weather = state_clone.weather.lock().await.clone();
                            let movies = state_clone.movies.lock().await.clone();

                            let recommendations = compute_recommendations(
                                &mood_data.moods,
                                &weather,
                                &movies,
                            ).await;

                            println!("{} films recommandés", recommendations.len());

                            if let Ok(json) = serde_json::to_string(&recommendations) {
                                let _ = client_clone
                                    .publish(TOPIC_RESULT, QoS::AtLeastOnce, false, json)
                                    .await;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Erreur MQTT: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                _ => {}
            }
        }
    });

    client.subscribe(TOPIC_WEATHER, QoS::AtLeastOnce).await?;
    client.subscribe(TOPIC_MOOD, QoS::AtLeastOnce).await?;
    for topic in MOVIE_TOPICS {
        client.subscribe(*topic, QoS::AtLeastOnce).await?;
    }

    println!("Abonné aux topics, en attente...");
    tokio::signal::ctrl_c().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_movie(id: u32, genre: &str, rating: f32) -> shared::Movie {
        shared::Movie {
            id,
            title: format!("Film {}", id),
            genres: vec![genre.to_string()],
            rating,
            poster_path: None,
            overview: String::new(),
            release_date: None,
        }
    }

    // --- get_genre_weights ---

    #[test]
    fn test_joy_soleil() {
        let weights = get_genre_weights("joy", "Ensoleillé", "Matin");
        assert!(weights.iter().any(|(g, w)| *g == "comedie" && *w == 1.5));
        assert!(weights.iter().any(|(g, w)| *g == "romance" && *w == 1.4));
    }

    #[test]
    fn test_joy_pluie() {
        let weights = get_genre_weights("joy", "Pluie", "Après-midi");
        assert!(weights.iter().any(|(g, _)| *g == "comedie"));
        assert!(weights.iter().all(|(_, w)| *w <= 1.3));
    }

    #[test]
    fn test_sad_pluie() {
        let weights = get_genre_weights("sad", "Pluie", "Soir");
        assert!(weights.iter().any(|(g, w)| *g == "drame" && *w == 1.5));
    }

    #[test]
    fn test_angry_orageux() {
        let weights = get_genre_weights("angry", "Orageux", "Nuit");
        assert!(weights.iter().any(|(g, w)| *g == "action" && *w == 1.5));
        assert!(weights.iter().any(|(g, w)| *g == "thriller" && *w == 1.5));
    }

    #[test]
    fn test_fear_nuit() {
        let weights = get_genre_weights("fear", "Nuageux", "Nuit");
        assert!(weights.iter().any(|(g, _)| *g == "horreur" || *g == "thriller"));
    }

    #[test]
    fn test_fear_soleil() {
        let weights = get_genre_weights("fear", "Ensoleillé", "Matin");
        assert!(weights.iter().any(|(g, w)| *g == "comedie" && *w == 1.5));
    }

    #[test]
    fn test_adventure_soleil() {
        let weights = get_genre_weights("adventure", "Ensoleillé", "Matin");
        assert!(weights.iter().any(|(g, w)| *g == "aventure" && *w == 1.5));
        assert!(weights.iter().any(|(g, w)| *g == "fantastique" && *w == 1.5));
    }

    #[test]
    fn test_reflection_pluie() {
        let weights = get_genre_weights("reflection", "Pluie", "Soir");
        assert!(weights.iter().any(|(g, w)| *g == "documentaire" && *w == 1.5));
        assert!(weights.iter().any(|(g, w)| *g == "drame" && *w == 1.5));
    }

    #[test]
    fn test_neige_tired() {
        let weights = get_genre_weights("tired", "Neige", "Soir");
        assert!(weights.iter().any(|(g, _)| *g == "familial"));
        assert!(weights.iter().any(|(g, _)| *g == "animation"));
    }

    #[test]
    fn test_humeur_inconnue() {
        let weights = get_genre_weights("unknown", "Pluie", "Soir");
        assert!(!weights.is_empty());
        assert!(weights.iter().all(|(_, w)| *w == 1.0));
    }

    // --- compute_score ---

    #[test]
    fn test_compute_score_genre_matching() {
        let movie = make_movie(1, "comedie", 7.5);
        let score = compute_score(&movie, "comedie", 1.5);
        assert_eq!(score, 7.5 * 1.5);
    }

    #[test]
    fn test_compute_score_genre_non_matching() {
        let movie = make_movie(2, "Action", 8.0);
        let score = compute_score(&movie, "comedie", 1.5);
        assert_eq!(score, 8.0);
    }

    #[test]
    fn test_compute_score_rating_zero() {
        let movie = make_movie(3, "Drame", 0.0);
        let score = compute_score(&movie, "drame", 1.5);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_compute_score_multiplicateur_neutre() {
        let movie = make_movie(4, "Horreur", 6.0);
        let score = compute_score(&movie, "horreur", 1.0);
        assert_eq!(score, 6.0);
    }
}