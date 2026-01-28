use rumqttc::{MqttOptions, AsyncClient, QoS};
use std::time::Duration;
use serde::{Deserialize, Serialize};

// Structure pour interpréter la réponse de l'API Open-Meteo
#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current: CurrentWeather,
}

#[derive(Debug, Deserialize, Serialize)]
struct CurrentWeather {
    is_day: f32,
    weather_code: i32,
}

// Structure simplifiée que nous allons envoyer en MQTT aux autres services
#[derive(Serialize)]
struct WeatherUpdate {
    day: f32,
    condition: String,
}

#[derive(Debug, Deserialize)]
struct IpLocation {
    lat: f32,
    lon: f32,
    city: String,
}

// Récupérer la position actuelle
async fn get_current_location() -> Result<IpLocation, Box<dyn std::error::Error>> {
    let url = "http://ip-api.com/json/"; // API gratuite pour test
    let resp = reqwest::get(url).await?.json::<IpLocation>().await?;
    Ok(resp)
}

// convertir les chiffre API en texte
fn translate_weather_code(code: i32) -> String {
    match code {
        0 => "Ensoleillé".to_string(),
        1..=3 => "Nuageux".to_string(),
        51..=67 => "Pluie".to_string(),
        71..=77 => "Neige".to_string(),
        80..=82 => "Averses".to_string(),
        95..=99 => "Orageux".to_string(),
        _ => "Inconnu".to_string(),
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Weather Service démarré...");

    // 1. Configuration MQTT [cite: 72, 73]
    let mut mqttoptions = MqttOptions::new("weather-service", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Boucle d'événements MQTT en tâche de fond [cite: 73]
    tokio::spawn(async move {
        loop {
            if let Err(e) = eventloop.poll().await {
                eprintln!("Erreur MQTT: {:?}", e);
            }
        }
    });

    // 2. Boucle de collecte
    loop {
        println!("📍 Tentative de localisation...");

        // On récupère la position actuelle via l'IP
        match get_current_location().await {
            Ok(loc) => {
                println!("📍 Position détectée : {} (lat: {}, lon: {})", loc.city, loc.lat, loc.lon);

                // Construction de l'URL avec les coordonnées dynamiques
                let url = format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=weather_code,is_day",
                    loc.lat, loc.lon
                );

                if let Ok(response) = reqwest::get(&url).await {
                    if let Ok(data) = response.json::<WeatherResponse>().await {
                        let update = WeatherUpdate {
                            day: data.current.is_day,
                            condition: translate_weather_code(data.current.weather_code),
                        };

                        let payload = serde_json::to_string(&update)?;

                        // Publication vers le service de recommandation [cite: 79, 81]
                        client.publish("weather/current", QoS::AtLeastOnce, false, payload).await?;
                        println!("📡 Données envoyées pour {} : Jour={}, Condition={}", loc.city, update.day, update.condition);
                    }
                }
            },
            Err(e) => eprintln!("❌ Erreur de localisation : {}", e),
        }

        // Attente de 10 minutes [cite: 71]
        tokio::time::sleep(Duration::from_secs(600)).await;
    }
}