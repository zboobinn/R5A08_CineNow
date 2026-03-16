import { useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faXmark, faStar, faClock, faUser, faFilm, faFire } from "@fortawesome/free-solid-svg-icons";

function MovieModal({ movie, onClose }) {
  const [details, setDetails] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!movie) return;
    setLoading(true);
    setDetails(null);
    fetch(`http://localhost:3001/movie/${movie.id}`)
      .then((res) => res.json())
      .then((data) => { setDetails(data); setLoading(false); })
      .catch(() => setLoading(false));
  }, [movie]);

  if (!movie) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4"
      onClick={onClose}
    >
      <div
        className="bg-white rounded-3xl shadow-2xl max-w-4xl w-full max-h-[90vh] overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center gap-6 p-6">

          {/* Poster */}
          <div className="w-48 shrink-0">
            <div className="aspect-2/3 rounded-2xl overflow-hidden bg-stone-200 shadow-md">
              {movie.poster_path ? (
                <img src={movie.poster_path} alt={movie.title} className="w-full h-full object-cover" />
              ) : (
                <div className="w-full h-full flex items-center justify-center text-stone-400">
                  <FontAwesomeIcon icon={faFire} className="text-4xl" />
                </div>
              )}
            </div>
          </div>

          {/* Détails */}
          <div className="flex-1 overflow-y-auto relative pr-2">

            {/* Bouton fermer */}
            <button
              onClick={onClose}
              className="absolute top-0 right-0 w-9 h-9 bg-stone-100 hover:bg-stone-200 rounded-full text-stone-600 flex items-center justify-center transition-colors"
            >
              <FontAwesomeIcon icon={faXmark} />
            </button>

            {/* Titre */}
            <h2 className="text-2xl font-bold text-stone-900 leading-tight pr-10">{movie.title}</h2>
            {movie.release_date && (
              <p className="text-stone-400 text-sm mt-1">{movie.release_date.slice(0, 4)}</p>
            )}

            {loading ? (
              <p className="text-stone-400 text-center py-8">Chargement des détails...</p>
            ) : details ? (
              <div className="flex flex-col gap-4 mt-4">

                {/* Note + durée */}
                <div className="flex items-center gap-4">
                  <div className="flex items-center gap-1.5">
                    <FontAwesomeIcon icon={faStar} className="text-yellow-400" />
                    <span className="font-bold text-stone-800">{details.rating.toFixed(1)}</span>
                    <span className="text-stone-400 text-sm">/ 10</span>
                  </div>
                  {details.duration && (
                    <div className="flex items-center gap-1.5 text-stone-500 text-sm">
                      <FontAwesomeIcon icon={faClock} />
                      <span>{details.duration} min</span>
                    </div>
                  )}
                </div>

                {/* Genres */}
                {details.genres?.length > 0 && (
                  <div className="flex flex-wrap gap-2">
                    {details.genres.map((g) => (
                      <span key={g} className="px-3 py-1 bg-orange-50 text-orange-700 text-xs font-semibold rounded-full border border-orange-200">
                        {g}
                      </span>
                    ))}
                  </div>
                )}

                {/* Synopsis */}
                {details.overview && (
                  <div>
                    <h3 className="text-sm font-bold text-stone-400 uppercase tracking-wider mb-2">Synopsis</h3>
                    <p className="text-stone-700 text-sm leading-relaxed">{details.overview}</p>
                  </div>
                )}

                {/* Réalisateur */}
                {details.director && (
                  <div className="flex items-center gap-2 text-sm">
                    <FontAwesomeIcon icon={faFilm} className="text-stone-400" />
                    <span className="text-stone-500">Réalisateur :</span>
                    <span className="font-semibold text-stone-800">{details.director}</span>
                  </div>
                )}

                {/* Acteurs */}
                {details.actors?.length > 0 && (
                  <div>
                    <h3 className="text-sm font-bold text-stone-400 uppercase tracking-wider mb-2">Acteurs</h3>
                    <div className="flex flex-wrap gap-2">
                      {details.actors.map((a) => (
                        <span key={a} className="flex items-center gap-1.5 px-3 py-1 bg-stone-100 text-stone-700 text-xs font-medium rounded-full">
                          <FontAwesomeIcon icon={faUser} className="text-stone-400 text-xs" />
                          {a}
                        </span>
                      ))}
                    </div>
                  </div>
                )}

              </div>
            ) : (
              <p className="text-stone-400 text-center py-8">Impossible de charger les détails.</p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default MovieModal;