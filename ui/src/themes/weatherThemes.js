import {
  faSun, faCloud, faCloudRain, faSnowflake,
  faCloudShowersHeavy, faBolt, faQuestion, faMoon, faSmog
} from "@fortawesome/free-solid-svg-icons";

export const weatherThemes = {
  "Ensoleillé": {
    day: {
      bg: "from-blue-400 to-sky-300",
      icon: faSun,
      iconClass: "text-yellow-300 sun-spin",
      text: "text-white",
      sub: "text-white/70",
      particles: "sun",
    },
    night: {
      bg: "from-indigo-950 to-blue-900",
      icon: faMoon,
      iconClass: "text-yellow-200",
      text: "text-white",
      sub: "text-white/60",
      particles: "stars",
    }
  },
  "Nuageux": {
    day: {
      bg: "from-gray-300 to-gray-400",
      icon: faCloud,
      iconClass: "text-white",
      text: "text-white",
      sub: "text-white/70",
      particles: "clouds",
    },
    night: {
      bg: "from-gray-700 to-gray-900",
      icon: faCloud,
      iconClass: "text-gray-300",
      text: "text-white",
      sub: "text-white/60",
      particles: "clouds", 
    }
  },
  "Brouillard": {
    day: {
      bg: "from-stone-400 to-stone-500",
      icon: faSmog,
      iconClass: "text-stone-200",
      text: "text-white",
      sub: "text-white/70",
      particles: "fog",
    },
    night: {
      bg: "from-stone-700 to-stone-800",
      icon: faSmog,
      iconClass: "text-stone-400",
      text: "text-stone-200",
      sub: "text-white/60",
      particles: "fog",
    }
  },
  "Pluie": {
    day: {
      bg: "from-slate-500 to-slate-700",
      icon: faCloudRain,
      iconClass: "text-blue-200",
      text: "text-white",
      sub: "text-white/70",
      particles: "rain",
    },
    night: {
      bg: "from-slate-800 to-slate-900",
      icon: faCloudRain,
      iconClass: "text-blue-300",
      text: "text-white",
      sub: "text-white/60",
      particles: "rain",
    }
  },
  "Neige": {
    day: {
      bg: "from-blue-100 to-blue-200",
      icon: faSnowflake,
      iconClass: "text-blue-400",
      text: "text-blue-900",
      sub: "text-blue-700/70",
      particles: "snow",
    },
    night: {
      bg: "from-blue-900 to-indigo-950",
      icon: faSnowflake,
      iconClass: "text-blue-200",
      text: "text-white",
      sub: "text-white/60",
      particles: "snow",
    }
  },
  "Averses": {
    day: {
      bg: "from-slate-400 to-blue-500",
      icon: faCloudShowersHeavy,
      iconClass: "text-white",
      text: "text-white",
      sub: "text-white/70",
      particles: "rain",
    },
    night: {
      bg: "from-slate-700 to-blue-900",
      icon: faCloudShowersHeavy,
      iconClass: "text-blue-200",
      text: "text-white",
      sub: "text-white/60",
      particles: "rain",
    }
  },
  "Orageux": {
    day: {
      bg: "from-gray-600 to-gray-800",
      icon: faBolt,
      iconClass: "text-yellow-300",
      text: "text-white",
      sub: "text-white/70",
      particles: "thunder", // Modifié "rain" en "thunder" pour avoir les éclairs
    },
    night: {
      bg: "from-gray-800 to-gray-950",
      icon: faBolt,
      iconClass: "text-yellow-300",
      text: "text-white",
      sub: "text-white/60",
      particles: "thunder", // Idem
    }                               
  },
  "Inconnu": {
    day: {
      bg: "from-stone-300 to-stone-400",
      icon: faQuestion,
      iconClass: "text-white",
      text: "text-white",
      sub: "text-white/70",
      particles: "none",
    },
    night: {
      bg: "from-stone-700 to-stone-900",
      icon: faQuestion,
      iconClass: "text-white",
      text: "text-white",
      sub: "text-white/60",
      particles: "stars",
    }
  },
};