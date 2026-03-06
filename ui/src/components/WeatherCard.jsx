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

const requestLocation = () => {
  if ("geolocation" in navigator) {
    navigator.geolocation.getCurrentPosition(
      (position) => {
        fetch("http://localhost:3001/location", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            lat: position.coords.latitude,
            lon: position.coords.longitude,
          }),
        }).catch((err) => console.error("Erreur envoi position :", err));
      },
      (error) => {
        console.error("Erreur géolocalisation :", error.message);
        alert("Impossible de récupérer votre position.");
      }
    );
  } else {
    alert("Géolocalisation non supportée.");
  }
};

function WeatherCard() {
  const [weather, setWeather] = useState(null);

  useEffect(() => {
    const fetchWeather = () => {
      fetch("http://localhost:3001/weather")
        .then((res) => res.json())
        .then((data) => setWeather(data))
        .catch((err) => console.error("Erreur météo:", err));
    };
    fetchWeather();
    const interval = setInterval(fetchWeather, 30000);
    return () => clearInterval(interval);
  }, []);

  if (!weather || weather.condition === "") return (
    <div className="rounded-2xl p-5 bg-stone-100 text-stone-400 text-sm text-center">
      Chargement météo...
    </div>
  );

  const isDay = weather.period === "Matin" || weather.period === "Après-midi";
  const theme = weatherThemes[weather.condition]?.[isDay ? "day" : "night"]
    ?? weatherThemes["Nuageux"][isDay ? "day" : "night"];

  return (
    <div className={`relative overflow-hidden rounded-2xl bg-gradient-to-b ${theme.bg} flex flex-col items-center text-center shadow-md`}>
      
      {/* Particules en arrière-plan */}
      <Particles type={getParticles(weather.condition, isDay)} />

      {/* Icône météo */}
      <div className="pt-6 pb-2 z-10">
        <FontAwesomeIcon icon={theme.icon} className={`text-6xl ${theme.iconClass}`} />
      </div>

      {/* Température */}
      <div className={`text-5xl font-bold z-10 ${theme.text}`}>
        {Math.round(weather.temp)}°C
      </div>

      {/* Condition */}
      <div className={`text-lg font-semibold mt-1 z-10 ${theme.text}`}>
        {weather.condition}
      </div>

      {/* Période */}
      <div className={`text-sm mt-1 z-10 ${theme.sub}`}>
        {weather.period}
      </div>

      {/* Séparateur */}
      <div className="w-full border-t border-white/10 mt-4" />

      {/* Ville + bouton localisation */}
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