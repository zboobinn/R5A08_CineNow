use rumqttc::{MqttOptions, AsyncClient, QoS};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;

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

// Condition météo
fn translate_weather_code(code: i32) -> String {
    match code {
        0 => "Ensoleillé".to_string(),
        1..=41 => "Nuageux".to_string(), // cite: main.rs
        42..=48 => "Brouillard".to_string(), // cite: main.rs
        51..=67 => "Pluie".to_string(), // cite: main.rs
        71..=77 => "Neige".to_string(), // cite: main.rs
        80..=86 => "Averses".to_string(), // cite: main.rs
        95..=99 => "Orageux".to_string(), // cite: main.rs
        _ => "Inconnu".to_string(), // cite: main.rs
    }
}

// Période de la journée (basée sur l'heure)
fn get_time_of_day(time_str: &str) -> String {
    // Extrait l'heure de la chaîne "YYYY-MM-DDTHH:MM"
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

    let lat = env::var("DEFAULT_LATITUDE").unwrap_or_else(|_| "44.9155".to_string()); // cite: main.rs
    let lon = env::var("DEFAULT_LONGITUDE").unwrap_or_else(|_| "4.9147".to_string()); // cite: main.rs
    let city = env::var("DEFAULT_CITY").unwrap_or_else(|_| "Valence".to_string()); // cite: main.rs

    let broker_host = env::var("MQTT_BROKER_HOST").unwrap_or_else(|_| "localhost".to_string()); // cite: main.rs
    let mut mqttoptions = MqttOptions::new("weather-service", broker_host, 1883); // cite: main.rs
    mqttoptions.set_keep_alive(Duration::from_secs(5)); // cite: main.rs
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10); // cite: main.rs

    tokio::spawn(async move {
        loop {
            if let Err(e) = eventloop.poll().await {
                eprintln!("Erreur MQTT: {:?}", e);
            }
        }
    });

    loop {
        println!("📍 Localisation : {}", city);

        // 👈 Ajout de temperature_2m dans l'URL
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=weather_code,is_day,temperature_2m",
            lat, lon
        );

        if let Ok(response) = reqwest::get(&url).await {
            if let Ok(data) = response.json::<WeatherResponse>().await {
                let update = WeatherUpdate {
                    temp: data.current.temperature_2m,
                    period: get_time_of_day(&data.current.time),
                    condition: translate_weather_code(data.current.weather_code), // cite: main.rs
                    city: city.clone(), // cite: main.rs
                };

                let payload = serde_json::to_string(&update)?;

                //retain: true permet de "cacher" la dernière valeur pour que le front l'ait à la demande
                client.publish("weather/current", QoS::AtLeastOnce, true, payload).await?;
                println!("📡 Météo envoyée : {}°C, {}, {} à {}", update.temp, update.condition, update.period, update.city);
            }
        }

        //Délai de 15minutes
        tokio::time::sleep(Duration::from_secs(900)).await;
    }
}