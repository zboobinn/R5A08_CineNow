import { useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faLocationDot, faCrosshairs } from "@fortawesome/free-solid-svg-icons";
import { weatherThemes } from "../themes/weatherThemes";
import { Particles } from "./Particles";


function getParticles(condition, isDay) {
  if (condition === "Pluie" || condition === "Averses") return "rain";
  if (condition === "Orageux") return "rain";
  if (condition === "Neige") return "snow";
  if (!isDay) return "stars";
  return null;
}

function WeatherCard() {
  const [weather, setWeather] = useState(null);

  const fetchWeather = () => {
    fetch("http://localhost:3001/weather")
      .then((res) => res.json())
      .then((data) => setWeather(data))
      .catch((err) => console.error("Erreur météo:", err));
  };

  useEffect(() => {
    fetchWeather();
    const interval = setInterval(fetchWeather, 30000); // Mise à jour toutes les 30s
    return () => clearInterval(interval);
  }, []);

  const requestLocation = () => {
    if ("geolocation" in navigator) {
      console.log("⏳ Recherche de la position en cours...");
      
      navigator.geolocation.getCurrentPosition(
        (position) => {
          const lat = position.coords.latitude;
          const lon = position.coords.longitude;
          
          console.log(`📍 Coordonnées trouvées : Lat ${lat}, Lon ${lon}`);

          fetch("http://localhost:3001/location", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ lat, lon }),
          })
          .then((res) => {
            if (res.ok) {
              console.log("✅ Position envoyée ! Actualisation dans 1.5s...");
              
              setTimeout(() => {
                fetchWeather();
              }, 1500);
            }
          })
          .catch((err) => console.error("❌ Erreur envoi position :", err));
        },
        (error) => {
          console.error("❌ Erreur géolocalisation :", error.message);
          alert("Impossible de récupérer votre position.");
        }
      );
    } else {
      alert("Géolocalisation non supportée.");
    }
  };

  if (!weather || weather.condition === "") return (
    <div className="rounded-2xl p-5 bg-stone-100 text-stone-400 text-sm text-center">
      Chargement météo...
    </div>
  );

  const isDay = weather.period === "Matin" || weather.period === "Après-midi";
  const theme = weatherThemes[weather.condition]?.[isDay ? "day" : "night"]
    ?? weatherThemes["Nuageux"][isDay ? "day" : "night"];

  return (
    <div className={`relative overflow-hidden rounded-2xl bg-linear-to-b ${theme.bg} flex flex-col items-center text-center shadow-md`}>
      
      <Particles type={getParticles(weather.condition, isDay)} />

      <div className="pt-6 pb-2 z-10">
        <FontAwesomeIcon icon={theme.icon} className={`text-6xl ${theme.iconClass}`} />
      </div>

      <div className={`text-5xl font-bold z-10 ${theme.text}`}>
        {Math.round(weather.temp)}°C
      </div>

      <div className={`text-lg font-semibold mt-1 z-10 ${theme.text}`}>
        {weather.condition}
      </div>

      <div className={`text-sm mt-1 z-10 ${theme.sub}`}>
        {weather.period}
      </div>

      <div className="w-full border-t border-white/10 mt-4" />

      <div className={`flex items-center justify-center gap-2 py-3 px-4 z-10 ${theme.sub} text-sm w-full`}>
        <FontAwesomeIcon icon={faLocationDot} />
        <span>{weather.city}</span>
        <button
          onClick={requestLocation}
          className={`ml-auto hover:opacity-100 opacity-60 transition-opacity cursor-pointer ${theme.sub}`}
          title="Utiliser ma position"
        >
          <FontAwesomeIcon icon={faCrosshairs} />
        </button>
      </div>

    </div>
  );
}

export default WeatherCard;