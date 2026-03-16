import React from 'react';

export function Particles({ type }) {
  // 1. SOLAIL (Jour) et ÉTOILES (Nuit)
  if (type === "stars") return (
    <div className="absolute inset-0 z-0 overflow-hidden">
      {[...Array(8)].map((_, i) => (
        <div key={i} className="star" style={{
          top: `${10 + (i * 13) % 50}%`,
          left: `${5 + (i * 17) % 80}%`,
          animationDelay: `${i * 0.4}s`,
        }} />
      ))}
    </div>
  );

  // 2. NUAGES (Nuageux)
  if (type === "clouds") return (
    <div className="absolute inset-0 z-0 overflow-hidden">
      {[...Array(4)].map((_, i) => (
        <div key={i} className="cloud-particle" style={{
          top: `${20 + i * 15}%`,
          animationDelay: `${i * 2}s`,
          left: '-10%'
        }}>☁️</div>
      ))}
    </div>
  );

  // 3. BROUILLARD
  if (type === "fog") return (
    <div className="absolute inset-0 z-0 fog-layer" />
  );

  // 4. PLUIE (Pluie, Averses)
  if (type === "rain" || type === "showers") return (
    <div className="absolute inset-0 z-0 overflow-hidden">
      {[...Array(8)].map((_, i) => (
        <div key={i} className="rain-drop" style={{
          left: `${10 + i * 11}%`,
          animationDelay: `${i * 0.15}s`,
        }} />
      ))}
    </div>
  );

  // 5. NEIGE (Neige)
  if (type === "snow") return (
    <div className="absolute inset-0 z-0 overflow-hidden">
      {[...Array(6)].map((_, i) => (
        <div key={i} className="snow-flake" style={{
          left: `${10 + i * 14}%`,
          animationDelay: `${i * 0.3}s`,
        }}>❄</div>
      ))}
    </div>
  );

  // 6. ORAGE (Orageux)
  if (type === "thunder") return (
    <div className="absolute inset-0 z-0 overflow-hidden">
      <div className="lightning" />
      <Particles type="rain" /> {/* On réutilise la pluie comme fond */}
    </div>
  );

  return null;
}