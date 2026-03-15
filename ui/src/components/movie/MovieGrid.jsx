import { useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFire } from "@fortawesome/free-solid-svg-icons";
import MovieCard from "./MovieCard";
import MovieModal from "./MovieModal";

function MovieGrid({ movies }) {
  const [selectedMovie, setSelectedMovie] = useState(null);

  if (movies.length === 0) return (
    <div className="flex-1 flex flex-col items-center justify-center text-stone-400 pb-20">
      <FontAwesomeIcon icon={faFire} className="text-6xl text-stone-200 mb-4" />
      <p className="text-xl font-semibold text-stone-500">Aucun film trouvé.</p>
    </div>
  );

  return (
    <>
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6 pb-10">
        {movies.map((movie, index) => (
          <MovieCard key={index} movie={movie} onClick={setSelectedMovie} />
        ))}
      </div>
      <MovieModal movie={selectedMovie} onClose={() => setSelectedMovie(null)} />
    </>
  );
}

export default MovieGrid; 