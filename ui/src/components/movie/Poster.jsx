import { useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFilm } from "@fortawesome/free-solid-svg-icons";

function Poster({ movie, className = "" }) {
  const [failed, setFailed] = useState(false);

  if (!movie.poster_path || failed) {
    return (
      <span
        role="img"
        aria-label={`Affiche indisponible pour ${movie.title}`}
        className="flex h-full w-full items-center justify-center text-4xl text-stone-500"
      >
        <FontAwesomeIcon icon={faFilm} />
      </span>
    );
  }

  return (
    <img
      src={movie.poster_path}
      alt={`Affiche de ${movie.title}`}
      loading="lazy"
      decoding="async"
      onError={() => setFailed(true)}
      className={`h-full w-full object-cover ${className}`}
    />
  );
}

export default Poster;
