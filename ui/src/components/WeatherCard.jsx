import { useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faLocationDot, faCrosshairs } from "@fortawesome/free-solid-svg-icons"; // Ajout de faCrosshairs
import { weatherThemes } from "../themes/weatherThemes";
import { Particles } from "./Particles";

function WeatherCard() {
  const [weather, setWeather] = useState(null);

  // Fonction pour demander la localisation au navigateur
const requestLocation = () => {
    if ("geolocation" in navigator) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          const lat = position.coords.latitude;
          const lon = position.coords.longitude;
          
          // Envoi au serveur Web (Gateway) qui fera le pont vers MQTT
          fetch("http://localhost:3001/location", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ lat, lon })
          })
          .then(() => console.log("✅ Position envoyée au serveur !"))
          .catch(err => console.error("❌ Erreur d'envoi :", err));
          
        },
        (error) => {
          console.error("Erreur de géolocalisation :", error.message);
          alert("Impossible de récupérer votre position.");
        }
      );
    } else {
      alert("La géolocalisation n'est pas supportée par votre navigateur.");
    }
  };

  useEffect(() => {
    fetch("http://localhost:3001/weather")
      .then((res) => res.json())
      .then((data) => setWeather(data))
      .catch((err) => console.error("Erreur météo:", err));
  }, []);

  if (!weather || weather.condition === "") return (
    <div className="rounded-2xl p-4 bg-stone-100 text-stone-400 text-sm text-center">
      Chargement météo...
    </div>
  );

  function getParticles(condition, isDay) {
    if (condition === "Brouillard") return "fog";
    if (condition === "Orageux") return "thunder";
    if (condition === "Pluie" || condition === "Averses") return "rain";
    if (condition === "Neige") return "snow";
    if (condition === "Nuageux") return "clouds";
    if (!isDay) return "stars";
    if (isDay && condition === "Ensoleillé") return "sun";
    return null;
  }

  const isDay = weather.day === 1; // cite: WeatherCard.jsx
  const theme = weatherThemes[weather.condition]?.[isDay ? "day" : "night"] // cite: WeatherCard.jsx
    ?? weatherThemes["Inconnu"][isDay ? "day" : "night"]; // cite: WeatherCard.jsx

  return (
    <div className={`relative overflow-hidden rounded-2xl p-4 bg-gradient-to-b ${theme.bg} flex flex-col items-center gap-6 text-center shadow-sm`}>
      <Particles type={getParticles(weather.condition, isDay)} />
      
      <FontAwesomeIcon icon={theme.icon} className={`text-5xl z-10 ${theme.iconClass}`} />
      <div className={`font-semibold text-3xl z-10 ${theme.text}`}>{weather.condition}</div>
      
      <div className={`flex items-center gap-2 text-xl z-10 ${theme.sub}`}>
        <FontAwesomeIcon icon={faLocationDot} />
        <span>{weather.city}</span>
        
        {/* Bouton pour déclencher la localisation */}
        <button 
          onClick={requestLocation}
          className="ml-2 hover:text-white transition-colors cursor-pointer"
          title="Me localiser"
        >
          <FontAwesomeIcon icon={faCrosshairs} size="sm" />
        </button>
      </div>
    </div>
  );
}

export default WeatherCard;