import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faStar } from "@fortawesome/free-solid-svg-icons";
import Poster from "./Poster";

// Date complète pour un film à venir, année seule sinon
function formatRelease(date) {
  if (!date) return null;
  if (date <= new Date().toISOString().slice(0, 10)) return date.slice(0, 4);
  return new Date(date).toLocaleDateString("fr-FR", { day: "numeric", month: "short", year: "numeric" });
}

function MovieCard({ movie, onClick }) {
  const release = formatRelease(movie.release_date);
  const rating = movie.rating > 0 ? movie.rating.toFixed(1) : null;
  const description = [
    movie.title,
    movie.genres?.[0],
    release,
    rating ? `note ${rating} sur 10` : "pas encore noté",
  ].filter(Boolean).join(", ");

  return (
    <li className="min-w-0">
      <button
        type="button"
        onClick={() => onClick?.(movie)}
        aria-label={description}
        className="group flex h-full w-full cursor-pointer flex-col overflow-hidden rounded-2xl border border-stone-200 bg-white text-left shadow-sm transition duration-200 hover:-translate-y-1 hover:border-orange-400 hover:shadow-lg"
      >
        <span className="relative block aspect-2/3 w-full overflow-hidden bg-stone-200">
          <Poster movie={movie} className="transition-transform duration-300 group-hover:scale-105" />

          {movie.genres?.length > 0 && (
            <span className="absolute top-2 left-2 rounded-lg bg-stone-950/80 px-2 py-1 text-xs font-semibold text-white capitalize">
              {movie.genres[0]}
            </span>
          )}
        </span>

        <span className="flex flex-1 flex-col justify-between gap-2 p-3 sm:p-4">
          <span className="line-clamp-2 leading-snug font-semibold text-stone-900 transition-colors group-hover:text-orange-800">
            {movie.title}
          </span>
          <span className="flex items-center justify-between gap-2 text-sm text-stone-600">
            {rating && (
              <span className="flex items-center gap-1.5">
                <FontAwesomeIcon icon={faStar} className="text-orange-500" />
                <span className="font-semibold text-stone-800">{rating}</span>
              </span>
            )}
            {release && <span className="whitespace-nowrap">{release}</span>}
          </span>
        </span>
      </button>
    </li>
  );
}

export default MovieCard;
