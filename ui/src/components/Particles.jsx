export function Particles({ type }) {
  if (type === "rain") return (
    <>
      {[...Array(8)].map((_, i) => (
        <div key={i} className="rain-drop" style={{
          left: `${10 + i * 11}%`,
          animationDelay: `${i * 0.15}s`,
        }} />
      ))}
    </>
  );
  if (type === "snow") return (
    <>
      {[...Array(6)].map((_, i) => (
        <div key={i} className="snow-flake" style={{
          left: `${10 + i * 14}%`,
          animationDelay: `${i * 0.3}s`,
        }}>❄</div>
      ))}
    </>
  );
  if (type === "stars") return (
    <>
      {[...Array(8)].map((_, i) => (
        <div key={i} className="star" style={{
          top: `${10 + (i * 13) % 50}%`,
          left: `${5 + (i * 17) % 80}%`,
          animationDelay: `${i * 0.4}s`,
        }} />
      ))}
    </>
  );
  return null;
}