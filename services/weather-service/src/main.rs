use rumqttc::{MqttOptions, AsyncClient, QoS};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;
use tokio::sync::watch;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current: CurrentWeather,
}

#[derive(Debug, Deserialize, Serialize)]
struct CurrentWeather {
    time: String, 
    weather_code: i32,
    temperature_2m: f32,
}

#[derive(Serialize)]
struct WeatherUpdate {
    temp: f32,
    period: String, 
    condition: String,
    city: String,
}

// Décoder la position reçue
#[derive(Deserialize)]
struct LocationUpdate {
    lat: f32,
    lon: f32,
}

fn translate_weather_code(code: i32) -> String {
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

fn get_time_of_day(time_str: &str) -> String {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("🚀 Weather Service démarré...");

    let default_lat = env::var("DEFAULT_LATITUDE").unwrap_or_else(|_| "44.9155".to_string());
    let default_lon = env::var("DEFAULT_LONGITUDE").unwrap_or_else(|_| "4.9147".to_string());
    let default_city = env::var("DEFAULT_CITY").unwrap_or_else(|_| "Valence".to_string());

    let broker_host = env::var("MQTT_BROKER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let broker_port: u16 = env::var("MQTT_BROKER_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .unwrap_or(1883);

    let mut mqttoptions = MqttOptions::new("weather-service", broker_host, broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client.subscribe("weather/location", QoS::AtLeastOnce).await?;

    // canal pour envoyer les new coordonnées à la boucle principale
    let (tx, mut rx) = watch::channel((default_lat, default_lon, default_city));
    let client_clone = client.clone();

    // écoute les messages MQTT en permanence
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(p))) => {
                    // Si on reçoit un message sur le topic location...
                    if p.topic == "weather/location" {
                        if let Ok(loc) = serde_json::from_slice::<LocationUpdate>(&p.payload) {
                            println!("📍 Nouvelle localisation reçue : lat={}, lon={}", loc.lat, loc.lon);
                            // On envoie les nouvelles données dans le canal, la ville devient "Position GPS"
                            let _ = tx.send((loc.lat.to_string(), loc.lon.to_string(), "Position GPS".to_string()));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Erreur MQTT: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                _ => {} // On ignore les autres types d'événements MQTT (Ping, PubAck...)
            }
        }
    });

    // Interroge l'API Météo
    loop {
        // On récupère les valeurs actuelles du canal (les valeurs par défaut, ou celles reçues par MQTT)
        let (current_lat, current_lon, current_city) = rx.borrow().clone();

        println!("Recherche météo pour : {} ({}, {})", current_city, current_lat, current_lon);

        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=weather_code,is_day,temperature_2m",
            current_lat, current_lon
        );

        if let Ok(response) = reqwest::get(&url).await {
            if let Ok(data) = response.json::<WeatherResponse>().await {
                let update = WeatherUpdate {
                    temp: data.current.temperature_2m,
                    period: get_time_of_day(&data.current.time),
                    condition: translate_weather_code(data.current.weather_code),
                    city: current_city.clone(),
                };

                if let Ok(payload) = serde_json::to_string(&update) {
                    let _ = client_clone.publish("weather/current", QoS::AtLeastOnce, true, payload).await;
                    println!("📡 Météo envoyée : {}°C, {}", update.temp, update.condition);
                }
            }
        }

        // On attend 15 minutes OU une maj immédiate
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(900)) => {}
            _ = rx.changed() => {
                println!("⚡ Position mise à jour ! Actualisation météo immédiate...");
            }
        }
    }
}