use axum::{
    extract::State,
    http::Method,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct WeatherData {
    temp: f32,
    period: String,
    condition: String,
    city: String,
}

// 👈 1. Nouvelle structure pour décoder le JSON reçu de React
#[derive(Debug, Deserialize, Serialize)]
struct LocationData {
    lat: f32,
    lon: f32,
}

// 👈 2. Nouvel état partagé (Contient tes données Météo ET le client MQTT)
#[derive(Clone)]
struct AppState {
    weather: Arc<Mutex<WeatherData>>,
    mqtt_client: AsyncClient,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("🚀 Web Server démarré...");

    let broker_host = env::var("MQTT_BROKER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let broker_port: u16 = env::var("MQTT_BROKER_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .unwrap_or(1883);

    // J'ai nettoyé tes variables mqttoptions (il n'y en a plus qu'une seule)
    let mut mqttoptions = MqttOptions::new("web-server", broker_host, broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("weather/current", QoS::AtLeastOnce).await?;

    let weather_state = Arc::new(Mutex::new(WeatherData::default()));
    let weather_state_clone = weather_state.clone();

    // Boucle d'écoute MQTT
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(rumqttc::Event::Incoming(Packet::Publish(msg))) => {
                    let payload = String::from_utf8_lossy(&msg.payload);
                    if let Ok(data) = serde_json::from_str::<WeatherData>(&payload) {
                        *weather_state_clone.lock().await = data;
                        println!("🌤️  Météo mise à jour dans le serveur Web");
                    }
                }
                Err(e) => eprintln!("Erreur MQTT: {:?}", e),
                _ => {}
            }
        }
    });

    // 👈 3. Création de l'état global de l'application
    let app_state = AppState {
        weather: weather_state,
        mqtt_client: client, // On donne le client MQTT pour l'utiliser dans nos routes
    };

    // 👈 4. On ajoute Method::POST au CORS pour que React ait le droit de nous envoyer des données
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/weather", get(get_weather))
        .route("/location", post(post_location)) // 👈 5. Ajout de la nouvelle route
        .with_state(app_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("🌐 API dispo sur http://localhost:3001");
    axum::serve(listener, app).await?;

    Ok(())
}

// Handler GET
async fn get_weather(State(state): State<AppState>) -> Json<WeatherData> {
    Json(state.weather.lock().await.clone())
}

// 👈 6. Nouveau Handler POST
async fn post_location(
    State(state): State<AppState>,
    Json(payload): Json<LocationData>, // Extrait automatiquement le { lat, lon } de React
) -> Json<serde_json::Value> {
    println!("📍 API Web : Coordonnées reçues (Lat: {}, Lon: {}). Transfert vers MQTT...", payload.lat, payload.lon);
    
    // On transforme les coordonnées en texte et on les publie sur le broker MQTT
    if let Ok(mqtt_msg) = serde_json::to_string(&payload) {
        let _ = state.mqtt_client.publish("weather/location", QoS::AtLeastOnce, false, mqtt_msg).await;
    }

    // On répond à React que tout s'est bien passé (Status 200 OK par défaut)
    Json(serde_json::json!({ "status": "success", "message": "Position transmise au réseau MQTT" }))
}