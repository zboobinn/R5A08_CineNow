use axum::{
    extract::State,
    http::Method,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use shared::{Movie, MovieCatalog};
use std::{collections::HashMap, env, sync::Arc, time::Duration};
use tokio::sync::Mutex;
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

// Ce que le recommendation-service publie
#[derive(Debug, Deserialize)]
struct RecommendedMovie {
    id: u32,
}

#[derive(Clone)]
struct AppState {
    weather: Arc<Mutex<WeatherData>>,
    movies: Arc<Mutex<HashMap<String, Vec<Movie>>>>,
    recommendations: Arc<Mutex<Vec<Movie>>>,
    mqtt_client: AsyncClient,
}

const MOVIE_TOPICS: &[&str] = &[
    "movies/comedie", "movies/romance", "movies/animation",
    "movies/drame", "movies/action", "movies/thriller",
    "movies/guerre", "movies/horreur", "movies/familial",
    "movies/aventure", "movies/fantastique", "movies/science-fiction",
    "movies/documentaire", "movies/histoire",
];

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

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 20);

    let weather_state = Arc::new(Mutex::new(WeatherData::default()));
    let movies_state: Arc<Mutex<HashMap<String, Vec<Movie>>>> = Arc::new(Mutex::new(HashMap::new()));
    let recommendations_state: Arc<Mutex<Vec<Movie>>> = Arc::new(Mutex::new(Vec::new()));

    let weather_clone = weather_state.clone();
    let movies_clone = movies_state.clone();
    let recommendations_clone = recommendations_state.clone();

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
                        if let Ok(reco) = serde_json::from_str::<Vec<RecommendedMovie>>(&payload) {
                            // Reconstituer les films complets depuis la HashMap movies
                            let movies = movies_clone.lock().await;
                            let all_movies: HashMap<u32, &Movie> = movies
                                .values()
                                .flatten()
                                .map(|m| (m.id, m))
                                .collect();

                            // Respecter l'ordre des scores
                            let result: Vec<Movie> = reco
                                .iter()
                                .filter_map(|r| all_movies.get(&r.id).map(|m| (*m).clone()))
                                .collect();

                            println!("Recommandations reçues : {} films", result.len());
                            *recommendations_clone.lock().await = result;
                        }
                    }
                }
                Err(e) => eprintln!("Erreur MQTT: {:?}", e),
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
        recommendations: recommendations_state,
        mqtt_client: client,
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/weather", get(get_weather))
        .route("/movies", get(get_movies))
        .route("/recommendations", get(get_recommendations))
        .route("/location", post(post_location))
        .route("/mood", post(post_mood))
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

async fn get_recommendations(State(state): State<AppState>) -> Json<Vec<Movie>> {
    Json(state.recommendations.lock().await.clone())
}

async fn post_mood(
    State(state): State<AppState>,
    Json(payload): Json<MoodPayload>,
) -> Json<serde_json::Value> {
    println!("Humeur reçue : {:?}", payload.moods);
    if let Ok(mqtt_msg) = serde_json::to_string(&payload) {
        let _ = state.mqtt_client.publish("mood/selected", QoS::AtLeastOnce, false, mqtt_msg).await;
    }
    Json(serde_json::json!({ "status": "success" }))
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