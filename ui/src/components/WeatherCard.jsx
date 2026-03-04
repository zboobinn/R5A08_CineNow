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

  function getParticles(condition, isDay) {
  if (condition === "Pluie" || condition === "Averses" || condition === "Orageux") return "rain";
  if (condition === "Neige") return "snow";
  if (!isDay) return "stars";
  return null;
}


  const isDay = weather.day === 1;
  const theme = weatherThemes[weather.condition]?.[isDay ? "day" : "night"]
    ?? weatherThemes["Inconnu"][isDay ? "day" : "night"];

  return (
    <div className={`relative overflow-hidden rounded-2xl p-4 bg-linear-to-b ${theme.bg} flex flex-col items-center gap-6 text-center`}>
      <Particles type={getParticles(weather.condition, isDay)} />
      <FontAwesomeIcon icon={theme.icon} className={`text-3xl ${theme.iconClass}`} />
      <div className={`font-semibold text-3xl ${theme.text}`}>{weather.condition}</div>
      <div className={`flex items-center gap-2 text-xl ${theme.sub}`}>
        <FontAwesomeIcon icon={faLocationDot} />
        <span>{weather.city}</span>
      </div>
    </div>
  );
}

export default WeatherCard;