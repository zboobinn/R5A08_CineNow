import { useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faLocationDot, faCrosshairs, faSpinner } from "@fortawesome/free-solid-svg-icons";
import { getWeather, sendLocation } from "../api";
import { usePolledResource } from "../hooks/usePolledResource";
import { weatherThemes } from "../themes/weatherThemes";
import { Particles } from "./Particles";

const isWeatherEmpty = (weather) => weather.condition === "";

function getParticles(condition, isDay) {
  if (condition === "Pluie" || condition === "Averses") return "rain";
  if (condition === "Orageux") return "rain";
  if (condition === "Neige") return "snow";
  if (!isDay) return "stars";
  return null;
}

function WeatherCard() {
  const { data: weather, status, retry, refresh } = usePolledResource(getWeather, isWeatherEmpty);
  const [isLocating, setIsLocating] = useState(false);
  const [locationError, setLocationError] = useState(null);

  const requestLocation = () => {
    if (!("geolocation" in navigator)) {
      setLocationError("Géolocalisation non supportée.");
      return;
    }

    setIsLocating(true);
    setLocationError(null);

    navigator.geolocation.getCurrentPosition(
      async (position) => {
        try {
          await sendLocation(position.coords.latitude, position.coords.longitude);
          // Le temps que le weather-service interroge Open-Meteo et publie sur MQTT
          setTimeout(() => {
            refresh();
            setIsLocating(false);
          }, 1500);
        } catch {
          setLocationError("Envoi de la position impossible.");
          setIsLocating(false);
        }
      },
      () => {
        setLocationError("Position indisponible.");
        setIsLocating(false);
      },
      { timeout: 10000 }
    );
  };

  if (status === "error") return (
    <div role="alert" className="flex items-center justify-between gap-3 rounded-2xl bg-stone-100 p-4 text-sm text-stone-700">
      <span>Météo indisponible</span>
      <button type="button" onClick={retry} className="font-semibold text-orange-800 underline underline-offset-2 hover:text-orange-950">
        Réessayer
      </button>
    </div>
  );

  if (status !== "ready") return (
    <div role="status" className="h-28 animate-pulse rounded-2xl bg-stone-200 lg:h-60">
      <span className="sr-only">Chargement de la météo</span>
    </div>
  );

  const isDay = weather.period === "Matin" || weather.period === "Après-midi";
  const theme = weatherThemes[weather.condition]?.[isDay ? "day" : "night"]
    ?? weatherThemes["Nuageux"][isDay ? "day" : "night"];

  return (
    <section
      aria-label="Météo actuelle"
      className={`relative overflow-hidden rounded-2xl bg-linear-to-b ${theme.bg} shadow-md`}
    >
      <div aria-hidden="true">
        <Particles type={getParticles(weather.condition, isDay)} />
      </div>

      <div className="relative z-10 flex items-center gap-4 px-4 pt-4 pb-3 lg:flex-col lg:gap-1 lg:px-5 lg:pt-6 lg:text-center">
        <FontAwesomeIcon icon={theme.icon} className={`text-4xl lg:mb-1 lg:text-5xl ${theme.iconClass}`} />
        <div className={`font-display text-4xl leading-none font-extrabold lg:text-5xl ${theme.text}`}>
          {Math.round(weather.temp)}°C
        </div>
        <div className="min-w-0">
          <div className={`font-semibold ${theme.text}`}>{weather.condition}</div>
          <div className={`text-sm ${theme.sub}`}>{weather.period}</div>
        </div>
      </div>

      <div className={`relative z-10 flex items-center gap-2 border-t border-white/20 px-4 py-1.5 text-sm ${theme.sub}`}>
        <FontAwesomeIcon icon={faLocationDot} />
        <span className="min-w-0 truncate" title={weather.city}>{locationError ?? weather.city}</span>
        <a
          href="https://open-meteo.com/"
          target="_blank"
          rel="noreferrer"
          className="ml-auto shrink-0 text-xs underline underline-offset-2"
        >
          Open-Meteo
        </a>
        <button
          type="button"
          onClick={requestLocation}
          disabled={isLocating}
          aria-label="Utiliser ma position"
          title="Utiliser ma position"
          className="-mr-2 flex h-10 w-10 shrink-0 items-center justify-center rounded-full transition-colors hover:bg-white/15"
        >
          <FontAwesomeIcon icon={isLocating ? faSpinner : faCrosshairs} spin={isLocating} />
        </button>
      </div>
    </section>
  );
}

export default WeatherCard;
