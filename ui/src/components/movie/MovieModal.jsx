import { useEffect, useRef, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faXmark, faStar, faClock, faUser, faFilm } from "@fortawesome/free-solid-svg-icons";
import { getMovieDetails } from "../../api";
import Poster from "./Poster";

function DetailsSkeleton() {
  return (
    <div role="status" className="mt-4 flex flex-col gap-3">
      <span className="sr-only">Chargement des détails</span>
      <div className="h-5 w-1/3 animate-pulse rounded bg-stone-200" />
      <div className="h-4 w-full animate-pulse rounded bg-stone-200" />
      <div className="h-4 w-full animate-pulse rounded bg-stone-200" />
      <div className="h-4 w-2/3 animate-pulse rounded bg-stone-200" />
    </div>
  );
}

function MovieModal({ movie, onClose }) {
  const dialogRef = useRef(null);
  const [details, setDetails] = useState(null);
  const [error, setError] = useState(null);
  const [attempt, setAttempt] = useState(0);

  // <dialog> en modal gère Échap, le piège de focus et le retour du focus sur la carte
  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog.open) dialog.showModal();
  }, []);

  useEffect(() => {
    const controller = new AbortController();
    setDetails(null);
    setError(null);
    getMovieDetails(movie.id, { signal: controller.signal })
      .then(setDetails)
      .catch((err) => {
        if (!controller.signal.aborted) setError(err.message);
      });
    return () => controller.abort();
  }, [movie.id, attempt]);

  const close = () => dialogRef.current.close();

  return (
    <dialog
      ref={dialogRef}
      aria-labelledby="movie-title"
      onClose={onClose}
      onClick={(e) => { if (e.target === dialogRef.current) close(); }}
      className="movie-dialog m-auto max-h-[calc(100dvh-2rem)] w-[calc(100%-2rem)] max-w-3xl overflow-y-auto rounded-3xl bg-white text-stone-800 shadow-2xl backdrop:bg-stone-950/70"
    >
      <div className="relative flex flex-col gap-5 p-5 sm:flex-row sm:gap-6 sm:p-6">
        <button
          type="button"
          onClick={close}
          aria-label="Fermer la fiche du film"
          className="absolute top-3 right-3 flex h-10 w-10 items-center justify-center rounded-full bg-stone-100 text-stone-700 transition-colors hover:bg-stone-200"
        >
          <FontAwesomeIcon icon={faXmark} />
        </button>

        {/* Poster */}
        <div className="mx-auto w-36 shrink-0 sm:mx-0 sm:w-52">
          <div className="aspect-2/3 overflow-hidden rounded-2xl bg-stone-200 shadow-md">
            <Poster movie={movie} />
          </div>
        </div>

        {/* Détails */}
        <div className="min-w-0 flex-1">
          <h2 id="movie-title" className="pr-10 font-display text-3xl leading-tight font-extrabold text-stone-900">
            {movie.title}
          </h2>
          {movie.release_date && (
            <p className="mt-1 text-sm text-stone-600">{movie.release_date.slice(0, 4)}</p>
          )}

          {error && (
            <div role="alert" className="mt-4 rounded-2xl bg-red-50 p-4 text-sm text-red-900">
              <p>{error}</p>
              <button
                type="button"
                onClick={() => setAttempt((n) => n + 1)}
                className="mt-2 font-semibold underline underline-offset-2"
              >
                Réessayer
              </button>
            </div>
          )}

          {!error && !details && <DetailsSkeleton />}

          {details && (
            <div className="fade-in mt-4 flex flex-col gap-4">

              {/* Note + durée */}
              <div className="flex flex-wrap items-center gap-4">
                <div className="flex items-center gap-1.5">
                  <FontAwesomeIcon icon={faStar} className="text-orange-500" />
                  {details.rating > 0 ? (
                    <>
                      <span className="font-bold text-stone-900">{details.rating.toFixed(1)}</span>
                      <span className="text-sm text-stone-600">/ 10</span>
                    </>
                  ) : (
                    <span className="text-sm text-stone-600">Pas encore noté</span>
                  )}
                </div>
                {details.duration > 0 && (
                  <div className="flex items-center gap-1.5 text-sm text-stone-600">
                    <FontAwesomeIcon icon={faClock} />
                    <span>{details.duration} min</span>
                  </div>
                )}
              </div>

              {/* Genres */}
              {details.genres?.length > 0 && (
                <div className="flex flex-wrap gap-2">
                  {details.genres.map((g) => (
                    <span key={g} className="rounded-full border border-orange-300 bg-orange-50 px-3 py-1 text-xs font-semibold text-orange-900">
                      {g}
                    </span>
                  ))}
                </div>
              )}

              {/* Synopsis */}
              {details.overview && (
                <div>
                  <h3 className="mb-2 text-xs font-bold tracking-wider text-stone-600 uppercase">Synopsis</h3>
                  <p className="text-sm leading-relaxed text-stone-700">{details.overview}</p>
                </div>
              )}

              {/* Réalisateur */}
              {details.director && (
                <div className="flex items-center gap-2 text-sm">
                  <FontAwesomeIcon icon={faFilm} className="text-stone-500" />
                  <span className="text-stone-600">Réalisation :</span>
                  <span className="font-semibold text-stone-800">{details.director}</span>
                </div>
              )}

              {/* Acteurs */}
              {details.actors?.length > 0 && (
                <div>
                  <h3 className="mb-2 text-xs font-bold tracking-wider text-stone-600 uppercase">Distribution</h3>
                  <div className="flex flex-wrap gap-2">
                    {details.actors.map((a) => (
                      <span key={a} className="flex items-center gap-1.5 rounded-full bg-stone-100 px-3 py-1 text-xs font-medium text-stone-700">
                        <FontAwesomeIcon icon={faUser} className="text-xs text-stone-500" />
                        {a}
                      </span>
                    ))}
                  </div>
                </div>
              )}

            </div>
          )}
        </div>
      </div>
    </dialog>
  );
}

export default MovieModal;
