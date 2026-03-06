use axum::{extract::State, routing::get, Json, Router};
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct WeatherData {
    temp: f32,
    period: String,
    condition: String,
    city: String,
}

type SharedState = Arc<Mutex<WeatherData>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("🚀 Web Server démarré...");

    let broker_host = env::var("MQTT_BROKER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let state: SharedState = Arc::new(Mutex::new(WeatherData::default()));

    let broker_port: u16 = env::var("MQTT_BROKER_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .unwrap_or(1883);

    let mut mqttoptions = MqttOptions::new("web-server", broker_host, broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    // On s'abonne à weather/current
    // Note: on a besoin du client pour subscribe, on le recrée proprement
    let mut mqttoptions2 = MqttOptions::new("web-server", env::var("MQTT_BROKER_HOST").unwrap_or_else(|_| "localhost".to_string()), 1883);
    mqttoptions2.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions2, 10);
    client.subscribe("weather/current", QoS::AtLeastOnce).await?;

    let state_mqtt = state.clone();
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(rumqttc::Event::Incoming(Packet::Publish(msg))) => {
                    let payload = String::from_utf8_lossy(&msg.payload);
                    if let Ok(data) = serde_json::from_str::<WeatherData>(&payload) {
                        *state_mqtt.lock().await = data;
                        println!("🌤️  Météo mise à jour");
                    }
                }
                Err(e) => eprintln!("Erreur MQTT: {:?}", e),
                _ => {}
            }
        }
    });

    let cors = CorsLayer::new()
        .allow_methods([axum::http::Method::GET])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/weather", get(get_weather))
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("🌐 API dispo sur http://localhost:3001");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_weather(State(state): State<SharedState>) -> Json<WeatherData> {
    Json(state.lock().await.clone())
}