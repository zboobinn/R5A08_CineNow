import { useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faSun, faCloud, faCloudRain, faSnowflake,
  faCloudShowersHeavy, faBolt, faQuestion,
  faMoon, faLocationDot
} from "@fortawesome/free-solid-svg-icons";

const weatherIcons = {
  "Ensoleillé": faSun,
  "Nuageux": faCloud,
  "Pluie": faCloudRain,
  "Neige": faSnowflake,
  "Averses": faCloudShowersHeavy,
  "Orageux": faBolt,
  "Inconnu": faQuestion,
};

function WeatherCard() {
  const [weather, setWeather] = useState(null);

  useEffect(() => {
    fetch("http://localhost:3001/weather")
      .then((res) => res.json())
      .then((data) => setWeather(data))
      .catch((err) => console.error("Erreur météo:", err));
  }, []);

  if (!weather) return (
    <div className="bg-stone-100 rounded-2xl p-4 text-stone-400 text-sm">
      Chargement météo...
    </div>
  );

  const icon = weatherIcons[weather.condition] ?? faQuestion;
  const isDay = weather.day === 1;

  return (
    <div className="bg-stone-100 rounded-2xl p-4 flex flex-col gap-3 items-center ">
      <FontAwesomeIcon icon={icon} className="text-3xl text-stone-600" />
      <div className="font-semibold text-stone-800">{weather.condition}</div>
      <div className="flex items-center gap-1 text-stone-500 text-sm">
        <FontAwesomeIcon icon={faLocationDot} />
        <span>{weather.city}</span>
      </div>
      <div className="flex items-center gap-1 text-stone-400 text-xs">
        <FontAwesomeIcon icon={isDay ? faSun : faMoon}/>
        <span>{isDay ? "Journée" : "Nuit"}</span>
      </div>
    </div>
  );
}

export default WeatherCard;