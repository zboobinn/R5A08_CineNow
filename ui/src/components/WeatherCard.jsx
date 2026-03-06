import { useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faLocationDot } from "@fortawesome/free-solid-svg-icons";
import { weatherThemes } from "../themes/weatherThemes";
import { Particles } from "./Particles";

function WeatherCard() {
  const [weather, setWeather] = useState(null);

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

  // 👈 MISE À JOUR ICI : Ajout de toutes les particules
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

  const isDay = weather.day === 1;
  const theme = weatherThemes[weather.condition]?.[isDay ? "day" : "night"]
    ?? weatherThemes["Inconnu"][isDay ? "day" : "night"];

  return (
    <div className={`relative overflow-hidden rounded-2xl p-4 bg-gradient-to-b ${theme.bg} flex flex-col items-center gap-6 text-center shadow-sm`}>
      <Particles type={getParticles(weather.condition, isDay)} />
      
      {/* 👈 MISE À JOUR ICI : Ajout de z-10 pour passer au-dessus du brouillard */}
      <FontAwesomeIcon icon={theme.icon} className={`text-5xl z-10 ${theme.iconClass}`} />
      <div className={`font-semibold text-3xl z-10 ${theme.text}`}>{weather.condition}</div>
      <div className={`flex items-center gap-2 text-xl z-10 ${theme.sub}`}>
        <FontAwesomeIcon icon={faLocationDot} />
        <span>{weather.city}</span>
      </div>
    </div>
  );
}

export default WeatherCard;