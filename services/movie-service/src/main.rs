use anyhow::Result;
use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;
use serde_json::Value;

const GENRES: &[(&str, u32, &str)] = &[
    ("comedie", 35, "Comédie"),
    ("romance", 10749, "Romance"),
    ("animation", 16, "Animation"),
    ("drame", 18, "Drame"),
    ("action", 28, "Action"),
    ("thriller", 53, "Thriller"),
    ("guerre", 10752, "Guerre"),
    ("horreur", 27, "Horreur"),
    ("familial", 10751, "Familial"),
    ("aventure", 12, "Aventure"),
    ("fantastique", 14, "Fantastique"),
    ("science-fiction", 878, "Science-Fiction"),
    ("documentaire", 99, "Documentaire"),
    ("histoire", 36, "Histoire"),
];

const PAGES: u32 = 5;
const MAX_MOVIES: usize = 30;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Movie Service démarré...");
    dotenvy::dotenv().ok();

    let tmdb_key = std::env::var("TMDB_API_KEY")
        .expect("TMDB_API_KEY manquante dans .env");

    let broker_host = std::env::var("MQTT_BROKER_HOST")
        .unwrap_or_else(|_| "localhost".to_string());
    let broker_port: u16 = std::env::var("MQTT_BROKER_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .unwrap_or(1883);

    let mqtt_client = create_mqtt_client(&broker_host, broker_port);
    println!("Connecté à MQTT");

    loop {
        println!("Récupération des films par genre...");

        for (topic_name, genre_id, genre_label) in GENRES {
            match fetch_movies_by_genre(&tmdb_key, *genre_id, genre_label).await {
                Ok(movies) => {
                    let count = movies.len();
                    let catalog = shared::MovieCatalog {
                        total: count,
                        movies,
                    };
                    println!(" {} : {} films", genre_label, count);
                    if let Ok(json) = serde_json::to_string(&catalog) {
                        let topic = format!("movies/{}", topic_name);
                        let _ = mqtt_client.publish(&topic, QoS::AtLeastOnce, true, json.as_bytes());
                    }
                }
                Err(e) => eprintln!(" Erreur {} : {}", genre_label, e),
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }

        println!("Tous les genres publiés sur MQTT !");
        println!("Prochain fetch dans 24h...");
        tokio::time::sleep(Duration::from_secs(86400)).await;
    }
}

fn create_mqtt_client(host: &str, port: u16) -> Client {
    let mut opts = MqttOptions::new("movie-service", host, port);
    opts.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(opts, 10);

    std::thread::spawn(move || {
        for notification in connection.iter() {
            if let Err(e) = notification {
                eprintln!("MQTT Error: {}", e);
            }
        }
    });

    std::thread::sleep(Duration::from_millis(500));
    client
}

async fn fetch_movies_by_genre(api_key: &str, genre_id: u32, genre_label: &str) -> Result<Vec<shared::Movie>> {
    let mut all_movies: Vec<shared::Movie> = Vec::new();

    for page in 1..=PAGES {
        let url = format!(
            "https://api.themoviedb.org/3/discover/movie?api_key={}&language=fr-FR&with_genres={}&sort_by=popularity.desc&page={}",
            api_key, genre_id, page
        );

        let response = reqwest::get(&url).await?;
        let json: Value = response.json().await?;

        let results = match json["results"].as_array() {
            Some(r) => r,
            None => break,
        };

        let movies: Vec<shared::Movie> = results
            .iter()
            .filter_map(|item| {
                let overview = item["overview"]
                    .as_str()
                    .unwrap_or("")
                    .chars()
                    .take(100)
                    .collect::<String>();

                Some(shared::Movie {
                    id: item["id"].as_u64()? as u32,
                    title: item["title"].as_str()?.to_string(),
                    genres: vec![genre_label.to_string()],
                    rating: item["vote_average"].as_f64()? as f32,
                    poster_path: item["poster_path"]
                        .as_str()
                        .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p)),
                    overview,
                    release_date: item["release_date"].as_str().map(|d: &str| d.to_string()),
                })
            })
            .collect();

        all_movies.extend(movies);
        tokio::time::sleep(Duration::from_millis(250)).await;
    }

    all_movies.dedup_by_key(|m| m.id);
    all_movies.truncate(MAX_MOVIES);
    Ok(all_movies)
}