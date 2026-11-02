import { useState } from "react";
import MovieCard from "./MovieCard";
import MovieModal from "./MovieModal";

function MovieGrid({ movies }) {
  const [selectedMovie, setSelectedMovie] = useState(null);

  return (
    <>
      <ul className="movie-grid fade-in pb-10">
        {movies.map((movie) => (
          <MovieCard key={movie.id} movie={movie} onClick={setSelectedMovie} />
        ))}
      </ul>
      {selectedMovie && (
        <MovieModal key={selectedMovie.id} movie={selectedMovie} onClose={() => setSelectedMovie(null)} />
      )}
    </>
  );
}

export default MovieGrid;
