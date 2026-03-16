use dotenvy::dotenv;
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use serde::Deserialize;
use std::{collections::HashMap, env, sync::Arc, time::Duration};
use tokio::sync::Mutex;

use recommendation_service::{WeatherData, compute_recommendations};

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

#[derive(Debug, Deserialize)]
struct MoodPayload {
    moods: Vec<String>,
}

#[derive(Clone)]
struct State {
    weather: Arc<Mutex<WeatherData>>,
    movies: Arc<Mutex<HashMap<String, Vec<shared::Movie>>>>,
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

    // Spawn l'eventloop EN PREMIER
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
                            );

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

    // ENSUITE les subscribes
    client.subscribe(TOPIC_WEATHER, QoS::AtLeastOnce).await?;
    client.subscribe(TOPIC_MOOD, QoS::AtLeastOnce).await?;
    for topic in MOVIE_TOPICS {
        client.subscribe(*topic, QoS::AtLeastOnce).await?;
    }

    println!("Abonné aux topics, en attente...");
    tokio::signal::ctrl_c().await?;
    Ok(())
}