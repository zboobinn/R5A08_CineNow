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

/// Retourne les genres recommandés avec leur multiplicateur pour une humeur + météo donnés
fn get_genre_weights(mood: &str, condition: &str, period: &str) -> Vec<(&'static str, f32)> {
    let is_night = period == "Nuit" || period == "Soir";
    let is_rain = matches!(condition, "Pluie" | "Averses" | "Orageux");
    let is_sun = condition == "Ensoleillé";
    let is_cloudy = matches!(condition, "Nuageux" | "Brouillard");
    
    
    match mood {
        "joy" => {
            if is_sun    { vec![("comedie", 1.5), ("romance", 1.5), ("animation", 1.4)] }
            else if is_rain { vec![("comedie", 1.2), ("romance", 1.2), ("animation", 1.3)] }
            else         { vec![("comedie", 1.3), ("animation", 1.2), ("familial", 1.2)] }
        }
        "sad" => {
            if is_rain   { vec![("drame", 1.5), ("romance", 1.4)] }
            else if is_sun { vec![("drame", 1.2), ("histoire", 1.2)] }
            else         { vec![("drame", 1.3), ("romance", 1.2)] }
        }
        "angry" => {
            if is_rain   { vec![("action", 1.5), ("thriller", 1.5), ("guerre", 1.4)] }
            else if is_sun { vec![("action", 1.4), ("thriller", 1.3)] }
            else         { vec![("action", 1.4), ("thriller", 1.3), ("guerre", 1.2)] }
        }
        "calm" => {
            if is_cloudy { vec![("documentaire", 1.4), ("histoire", 1.3), ("animation", 1.2)] }
            else if is_sun { vec![("animation", 1.4), ("familial", 1.3), ("documentaire", 1.2)] }
            else         { vec![("documentaire", 1.3), ("animation", 1.3)] }
        }
        "tired" => {
            if is_rain   { vec![("familial", 1.5), ("animation", 1.5), ("comedie", 1.3)] }
            else         { vec![("animation", 1.4), ("comedie", 1.3), ("familial", 1.4)] }
        }
        "fear" => {
            if is_sun    { vec![("comedie", 1.5), ("animation", 1.4), ("familial", 1.3)] }
            else if is_night { vec![("horreur", 1.0), ("thriller", 1.1)] }
            else         { vec![("thriller", 1.2), ("comedie", 1.3)] }
        }
        _ =>             { vec![("comedie", 1.0), ("action", 1.0), ("drame", 1.0)] }
    }
}

/// Calcule le score d'un film : note TMDb × multiplicateur de genre
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

    for mood in moods {
        let weights = get_genre_weights(mood, &weather.condition, &weather.period);
        for (genre, multiplier) in &weights {
            if let Some(films) = movies_by_genre.get(*genre) {
                for film in films {
                    let score = compute_score(film, genre, *multiplier);
                    // On garde le meilleur score si le film apparaît dans plusieurs genres
                    let entry = scores.entry(film.id).or_insert(0.0);
                    if score > *entry {
                        *entry = score;
                    }
                }
            }
        }
    }

    // Trier par score décroissant, garder le top 10
    let mut result: Vec<RecommendedMovie> = scores
        .into_iter()
        .map(|(id, score)| RecommendedMovie { id, score })
        .collect();

    result.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    result.truncate(10);
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

    // Spawn eventloop EN PREMIER
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

    // Subscribes après le spawn
    client.subscribe(TOPIC_WEATHER, QoS::AtLeastOnce).await?;
    client.subscribe(TOPIC_MOOD, QoS::AtLeastOnce).await?;
    for topic in MOVIE_TOPICS {
        client.subscribe(*topic, QoS::AtLeastOnce).await?;
    }

    println!("Abonné aux topics, en attente...");

    // Garder le main en vie
    tokio::signal::ctrl_c().await?;
    Ok(())
}