use axum::{
    extract::State,
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use axum::extract::Path;
use dotenvy::dotenv;
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use shared::{
    Movie, MovieCatalog, MoodRequest, RecommendationResponse, RecommendedMovie, MQTT_MAX_PACKET_SIZE,
};
use std::{
    collections::HashMap,
    env,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::{oneshot, Mutex};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct WeatherData {
    temp: f32,
    period: String,
    condition: String,
    city: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LocationData {
    lat: f32,
    lon: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct MoodPayload {
    moods: Vec<String>,
}

#[derive(Clone)]
struct AppState {
    weather: Arc<Mutex<WeatherData>>,
    movies: Arc<Mutex<HashMap<String, Vec<Movie>>>>,
    pending: PendingRequests,
    next_request_id: Arc<AtomicU64>,
    mqtt_client: AsyncClient,
}

// Demandes en attente de leur réponse sur recommendations/result
type PendingRequests = Arc<Mutex<HashMap<u64, oneshot::Sender<Vec<Movie>>>>>;

type ApiError = (StatusCode, Json<serde_json::Value>);

fn api_error(status: StatusCode, message: &str) -> ApiError {
    (status, Json(serde_json::json!({ "error": message })))
}

#[derive(Debug, Serialize)]
struct MovieDetails {
    id: u32,
    title: String,
    genres: Vec<String>,
    rating: f32,
    poster_path: Option<String>,
    overview: String,
    release_date: Option<String>,
    duration: Option<u32>,        // en minutes
    director: Option<String>,
    actors: Vec<String>,          // 5 premiers
}

const MOVIE_TOPICS: &[&str] = &[
    "movies/comedie", "movies/romance", "movies/animation",
    "movies/drame", "movies/action", "movies/thriller",
    "movies/guerre", "movies/horreur", "movies/familial",
    "movies/aventure", "movies/fantastique", "movies/science-fiction",
    "movies/documentaire", "movies/histoire",
];

const RECOMMENDATION_TIMEOUT: Duration = Duration::from_secs(5);
const TMDB_TIMEOUT: Duration = Duration::from_secs(10);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Web Server démarré...");

    let broker_host = env::var("MQTT_BROKER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let broker_port: u16 = env::var("MQTT_BROKER_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .unwrap_or(1883);

    let mut mqttoptions = MqttOptions::new("web-server", broker_host, broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_max_packet_size(MQTT_MAX_PACKET_SIZE, MQTT_MAX_PACKET_SIZE);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 20);

    let weather_state = Arc::new(Mutex::new(WeatherData::default()));
    let movies_state: Arc<Mutex<HashMap<String, Vec<Movie>>>> = Arc::new(Mutex::new(HashMap::new()));
    let pending: PendingRequests = Arc::new(Mutex::new(HashMap::new()));

    let weather_clone = weather_state.clone();
    let movies_clone = movies_state.clone();
    let pending_clone = pending.clone();

    // Spawn l'eventloop EN PREMIER
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(rumqttc::Event::Incoming(Packet::Publish(msg))) => {
                    let payload = String::from_utf8_lossy(&msg.payload);

                    if msg.topic == "weather/current" {
                        if let Ok(data) = serde_json::from_str::<WeatherData>(&payload) {
                            *weather_clone.lock().await = data;
                            println!("Météo mise à jour");
                        }
                    } else if msg.topic.starts_with("movies/") {
                        if let Ok(catalog) = serde_json::from_str::<MovieCatalog>(&payload) {
                            let genre = msg.topic.replace("movies/", "");
                            movies_clone.lock().await.insert(genre.clone(), catalog.movies);
                            println!("Films mis à jour : {}", genre);
                        }
                    } else if msg.topic == "recommendations/result" {
                        if let Ok(response) = serde_json::from_str::<RecommendationResponse>(&payload) {
                            let result = resolve_movies(&response.movies, &*movies_clone.lock().await);
                            println!("Recommandations reçues : {} films", result.len());

                            // Une réponse sans demande en attente (délai dépassé) est ignorée
                            if let Some(sender) = pending_clone.lock().await.remove(&response.request_id) {
                                let _ = sender.send(result);
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

    // ENSUITE les subscribes
    client.subscribe("weather/current", QoS::AtLeastOnce).await?;
    client.subscribe("recommendations/result", QoS::AtLeastOnce).await?;
    for topic in MOVIE_TOPICS {
        client.subscribe(*topic, QoS::AtLeastOnce).await?;
    }

    let app_state = AppState {
        weather: weather_state,
        movies: movies_state,
        pending,
        next_request_id: Arc::new(AtomicU64::new(1)),
        mqtt_client: client,
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/weather", get(get_weather))
        .route("/movies", get(get_movies))
        .route("/recommendations", post(post_recommendations))
        .route("/location", post(post_location))
        .route("/movie/:id", get(get_movie_details))
        .with_state(app_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("API dispo sur http://localhost:3001");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_weather(State(state): State<AppState>) -> Json<WeatherData> {
    Json(state.weather.lock().await.clone())
}

async fn get_movies(State(state): State<AppState>) -> Json<HashMap<String, Vec<Movie>>> {
    Json(state.movies.lock().await.clone())
}

async fn get_movie_details(
    Path(id): Path<u32>,
) -> Result<Json<MovieDetails>, ApiError> {
    let tmdb_key = std::env::var("TMDB_API_KEY").unwrap_or_default();

    // Fetch détails + crédits en parallèle
    let details_url = format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}&language=fr-FR",
        id, tmdb_key
    );
    let credits_url = format!(
        "https://api.themoviedb.org/3/movie/{}/credits?api_key={}&language=fr-FR",
        id, tmdb_key
    );

    let client = reqwest::Client::builder()
        .timeout(TMDB_TIMEOUT)
        .build()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Client HTTP indisponible"))?;
    let (details_res, credits_res) = tokio::join!(
        client.get(&details_url).send(),
        client.get(&credits_url).send(),
    );

    let details: serde_json::Value = match details_res {
        Ok(r) if r.status() == StatusCode::NOT_FOUND => {
            return Err(api_error(StatusCode::NOT_FOUND, "Film introuvable"));
        }
        Ok(r) if r.status().is_success() => r
            .json()
            .await
            .map_err(|_| api_error(StatusCode::BAD_GATEWAY, "Réponse TMDb illisible"))?,
        _ => return Err(api_error(StatusCode::BAD_GATEWAY, "Erreur TMDb")),
    };

    let credits: serde_json::Value = match credits_res {
        Ok(r) => r.json().await.unwrap_or_default(),
        Err(_) => serde_json::json!({}),
    };

    // Extraire réalisateur
    let director = credits["crew"]
        .as_array()
        .and_then(|crew| crew.iter().find(|p| p["job"] == "Director"))
        .and_then(|d| d["name"].as_str())
        .map(|s| s.to_string());

    // Extraire acteurs (5 premiers)
    let actors: Vec<String> = credits["cast"]
        .as_array()
        .map(|cast| {
            cast.iter()
                .take(5)
                .filter_map(|a| a["name"].as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    // Extraire genres
    let genres: Vec<String> = details["genres"]
        .as_array()
        .map(|g| {
            g.iter()
                .filter_map(|genre| genre["name"].as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let movie = MovieDetails {
        id,
        title: details["title"].as_str().unwrap_or("").to_string(),
        genres,
        rating: details["vote_average"].as_f64().unwrap_or(0.0) as f32,
        poster_path: details["poster_path"]
            .as_str()
            .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p)),
        overview: details["overview"].as_str().unwrap_or("").to_string(),
        release_date: details["release_date"].as_str().map(|s| s.to_string()),
        duration: details["runtime"].as_u64().map(|d| d as u32),
        director,
        actors,
    };

    Ok(Json(movie))
}

/// Publie la demande sur MQTT et attend la réponse portant le même request_id
async fn post_recommendations(
    State(state): State<AppState>,
    Json(payload): Json<MoodPayload>,
) -> Result<Json<Vec<Movie>>, ApiError> {
    println!("Humeur reçue : {:?}", payload.moods);

    let request_id = state.next_request_id.fetch_add(1, Ordering::Relaxed);
    let request = MoodRequest { request_id, moods: payload.moods };
    let mqtt_msg = serde_json::to_string(&request)
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Demande invalide"))?;

    let (sender, receiver) = oneshot::channel();
    state.pending.lock().await.insert(request_id, sender);

    let published = state.mqtt_client.publish("mood/selected", QoS::AtLeastOnce, false, mqtt_msg).await;
    if published.is_err() {
        state.pending.lock().await.remove(&request_id);
        return Err(api_error(StatusCode::BAD_GATEWAY, "Broker MQTT indisponible"));
    }

    match tokio::time::timeout(RECOMMENDATION_TIMEOUT, receiver).await {
        Ok(Ok(movies)) => Ok(Json(movies)),
        _ => {
            state.pending.lock().await.remove(&request_id);
            Err(api_error(StatusCode::GATEWAY_TIMEOUT, "Le service de recommandation ne répond pas"))
        }
    }
}

/// Reconstitue les films complets en respectant l'ordre des scores
fn resolve_movies(reco: &[RecommendedMovie], movies_by_genre: &HashMap<String, Vec<Movie>>) -> Vec<Movie> {
    let all_movies: HashMap<u32, &Movie> = movies_by_genre
        .values()
        .flatten()
        .map(|m| (m.id, m))
        .collect();

    reco.iter()
        .filter_map(|r| all_movies.get(&r.id).map(|m| (*m).clone()))
        .collect()
}

async fn post_location(
    State(state): State<AppState>,
    Json(payload): Json<LocationData>,
) -> Json<serde_json::Value> {
    println!("Position reçue (Lat: {}, Lon: {})", payload.lat, payload.lon);
    if let Ok(mqtt_msg) = serde_json::to_string(&payload) {
        let _ = state.mqtt_client.publish("weather/location", QoS::AtLeastOnce, false, mqtt_msg).await;
    }
    Json(serde_json::json!({ "status": "success" }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_movie(id: u32) -> Movie {
        Movie {
            id,
            title: format!("Film {}", id),
            genres: vec!["Action".to_string()],
            rating: 7.0,
            poster_path: None,
            overview: String::new(),
            release_date: None,
        }
    }

    #[test]
    fn test_resolve_movies_respecte_ordre_des_scores() {
        let catalog = HashMap::from([("action".to_string(), vec![make_movie(1), make_movie(2)])]);
        let reco = vec![
            RecommendedMovie { id: 2, score: 9.0 },
            RecommendedMovie { id: 1, score: 8.0 },
        ];

        let ids: Vec<u32> = resolve_movies(&reco, &catalog).iter().map(|m| m.id).collect();
        assert_eq!(ids, vec![2, 1]);
    }

    #[test]
    fn test_resolve_movies_ignore_id_inconnu() {
        let catalog = HashMap::from([("action".to_string(), vec![make_movie(1)])]);
        let reco = vec![RecommendedMovie { id: 99, score: 9.0 }];

        assert!(resolve_movies(&reco, &catalog).is_empty());
    }
}
