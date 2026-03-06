use rumqttc::{MqttOptions, AsyncClient, QoS};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv; // Import pour le chargement du .env
use std::env;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current: CurrentWeather,
}

#[derive(Debug, Deserialize, Serialize)]
struct CurrentWeather {
    is_day: f32,
    weather_code: i32,
}

#[derive(Serialize)]
struct WeatherUpdate {
    day: f32,
    condition: String,
    city: String,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Chargement des variables d'environnement
    dotenv().ok();
    println!("🚀 Weather Service démarré...");

    // Lecture de la config du .env avec valeurs de secours
    let lat = env::var("DEFAULT_LATITUDE").unwrap_or_else(|_| "44.9155".to_string());
    let lon = env::var("DEFAULT_LONGITUDE").unwrap_or_else(|_| "4.9147".to_string());
    let city = env::var("DEFAULT_CITY").unwrap_or_else(|_| "Valence".to_string());

    // 1. Configuration MQTT [cite: 72, 73]
    let broker_host = env::var("MQTT_BROKER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let mut mqttoptions = MqttOptions::new("weather-service", broker_host, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    tokio::spawn(async move {
        loop {
            if let Err(e) = eventloop.poll().await {
                eprintln!("Erreur MQTT: {:?}", e);
            }
        }
    });

    // 2. Boucle de collecte
    loop {
        println!("📍 Utilisation de la localisation par défaut : {}", city);

        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=weather_code,is_day",
            lat, lon
        );

        if let Ok(response) = reqwest::get(&url).await {
            if let Ok(data) = response.json::<WeatherResponse>().await {
                let update = WeatherUpdate {
                    day: data.current.is_day,
                    condition: translate_weather_code(data.current.weather_code),
                    city: city.clone(),
                };

                let payload = serde_json::to_string(&update)?;

                // Publication vers le service de recommandation [cite: 79, 81]
                client.publish("weather/current", QoS::AtLeastOnce, false, payload).await?;
                println!("📡 Données envoyées pour {} : Jour={}, Condition={}", city, update.day, update.condition);
            }
        }

        tokio::time::sleep(Duration::from_secs(600)).await;
    }
}