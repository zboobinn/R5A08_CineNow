use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;

fn main() {
    println!("🧪 Test MQTT avec Shiftr Desktop");
    
    // Connexion localhost:1883
    let mut mqttoptions = MqttOptions::new("test-client", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    
    let (client, mut connection) = Client::new(mqttoptions, 10);
    
    // Thread de connexion
    std::thread::spawn(move || {
        for notification in connection.iter() {
            match notification {
                Ok(notif) => println!("✅ Notification: {:?}", notif),
                Err(e) => eprintln!("❌ Erreur: {}", e),
            }
        }
    });
    
    // Attendre connexion
    std::thread::sleep(Duration::from_secs(1));
    
    // Publier
    match client.publish("test/rust", QoS::AtLeastOnce, false, "Hello from Rust!") {
        Ok(_) => println!("✅ Message publié sur test/rust"),
        Err(e) => eprintln!("❌ Erreur publish: {}", e),
    }
    
    println!("📊 Vérifier sur : http://localhost:3000");
    
    // Garder actif 3 secondes
    std::thread::sleep(Duration::from_secs(3));
}