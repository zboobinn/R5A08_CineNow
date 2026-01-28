use anyhow::Result;
use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;
use reqwest;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎬 Movie Service");
    println!("================");
    
    // 1. Charger .env
    dotenv::dotenv().ok();
    
    // 2. Récupérer clé TMDb
    let tmdb_key = std::env::var("TMDB_API_KEY")
        .expect("TMDB_API_KEY manquante dans .env");
    
    println!("✅ Clé TMDb chargée");
    
    // 3. Connexion MQTT
    let mqtt_client = create_mqtt_client();
    println!("✅ Connecté à MQTT");
    
    // 4. Récupérer films (on va créer cette fonction)
    println!("📡 Récupération des films...");
    let movies = fetch_popular_movies(&tmdb_key).await?;
    println!("✅ {} films récupérés", movies.len());

    println!("✅ {} films récupérés", movies.len());
    println!("");
    println!("📽️  Aperçu des films :");
    for (i, movie) in movies.iter().take(5).enumerate() {
        println!("  {}. {} (Note: {}/10)", i + 1, movie.title, movie.rating);
    }
    println!("  ... et {} autres films", movies.len().saturating_sub(5));
    
    // 5. Publier sur MQTT
    publish_movies(&mqtt_client, &movies)?;
    println!("✅ Films publiés sur MQTT");
    
    println!("");
    println!("📊 Vérifier : http://localhost:3000");
    
    // Garder actif
    tokio::time::sleep(Duration::from_secs(10)).await;
    
    Ok(())
}

// Fonction de connexion MQTT
fn create_mqtt_client() -> Client {
    let mut opts = MqttOptions::new("movie-service", "localhost", 1883);
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

async fn fetch_popular_movies(api_key: &str) -> Result<Vec<shared::Movie>> {
    let url = format!(
        "https://api.themoviedb.org/3/movie/popular?api_key={}&language=fr-FR&page=1",
        api_key
    );
    
    // Appel API
    let response = reqwest::get(&url).await?;
    let json: Value = response.json().await?;
    
    // Parser les résultats
    let results = json["results"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Pas de résultats"))?;
    
    // Convertir en Movie
    let movies: Vec<shared::Movie> = results
        .iter()
        .filter_map(|item| {
            Some(shared::Movie {
                id: item["id"].as_u64()? as u32,
                title: item["title"].as_str()?.to_string(),
                genres: vec![], // On va améliorer ça après
                rating: item["vote_average"].as_f64()? as f32,
                poster_path: item["poster_path"]
                    .as_str()
                    .map(|p| format!("https://image.tmdb.org/t/p/w500{}", p)),
                overview: item["overview"].as_str()?.to_string(),
            })
        })
        .collect();
    
    Ok(movies)
}

fn publish_movies(client: &Client, movies: &[shared::Movie]) -> Result<()> {
    let catalog = shared::MovieCatalog {
        movies: movies.to_vec(),
        total: movies.len(),
    };
    
    let json = serde_json::to_string(&catalog)?;
    
    client.publish(
        "movies/catalog",
        QoS::AtLeastOnce,
        true, // retained = true
        json.as_bytes()
    )?;
    
    Ok(())
}