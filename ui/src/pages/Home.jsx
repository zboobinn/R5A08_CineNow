import React, { useState, useMemo, useEffect } from 'react';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass, faStar, faFire } from "@fortawesome/free-solid-svg-icons";

const normalizeText = (text) => {
  return text.toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g, "");
};

const Home = () => {
  const [searchTerm, setSearchTerm] = useState("");
  const [moviesByGenre, setMoviesByGenre] = useState({});
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch("http://localhost:3001/movies")
      .then((res) => res.json())
      .then((data) => {
        setMoviesByGenre(data);
        setLoading(false);
      })
      .catch((err) => {
        console.error("Erreur films:", err);
        setLoading(false);
      });
  }, []);

  // Aplatir tous les films avec leur genre
  const allMovies = useMemo(() => {
  const seen = new Set();
  return Object.entries(moviesByGenre).flatMap(([genre, movies]) =>
    movies.map((movie) => ({ ...movie, genre }))
  ).filter((movie) => {
    if (seen.has(movie.id)) return false;
    seen.add(movie.id);
    return true;
  });
}, [moviesByGenre]);

  const filteredMovies = useMemo(() => {
    if (!searchTerm) return allMovies;
    const searchNormalized = normalizeText(searchTerm);
    return allMovies.filter(movie =>
      normalizeText(movie.title).includes(searchNormalized) ||
      normalizeText(movie.genre).includes(searchNormalized)
    );
  }, [searchTerm, allMovies]);

  if (loading) return (
    <div className="flex-1 flex items-center justify-center text-stone-400">
      Chargement des films...
    </div>
  );

  return (
    <div className="p-8 fade-in h-full flex flex-col max-w-7xl mx-auto">

      <header className="mb-10 flex flex-col md:flex-row md:items-end justify-between gap-6">
        <div>
          <h1 className="text-4xl font-bold text-stone-900 tracking-tight mb-2">À l'affiche</h1>
          <p className="text-stone-500 text-lg">Des films parfaits pour ton mood du jour.</p>
        </div>

        <div className="relative w-full md:w-96 group">
          <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
            <FontAwesomeIcon icon={faMagnifyingGlass} className="text-stone-400 group-focus-within:text-orange-500 transition-colors" />
          </div>
          <input
            type="text"
            placeholder="Rechercher un film ou un genre..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full pl-11 pr-4 py-3.5 bg-white border border-stone-200 rounded-2xl text-stone-700 shadow-sm focus:outline-none focus:border-orange-400 focus:ring-4 focus:ring-orange-400/10 transition-all placeholder:text-stone-400"
          />
        </div>
      </header>

      {filteredMovies.length > 0 ? (
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6 pb-10">
          {filteredMovies.map((movie, index) => (
            <div key={`${movie.id}-${index}`} className="group relative flex flex-col bg-white rounded-2xl overflow-hidden shadow-sm hover:shadow-2xl transition-all duration-300 hover:-translate-y-2 cursor-pointer border border-stone-100">

              <div className="relative aspect-2/3 overflow-hidden bg-stone-200">
                {movie.poster_path ? (
                  <img src={movie.poster_path} alt={movie.title} className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
                ) : (
                  <div className="w-full h-full flex items-center justify-center text-stone-400">
                    <FontAwesomeIcon icon={faFire} className="text-4xl" />
                  </div>
                )}
                <div className="absolute inset-0 bg-linear-to-t from-black/80 via-black/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
                <div className="absolute top-3 left-3 px-2.5 py-1 bg-white/90 backdrop-blur-sm rounded-lg text-xs font-bold text-stone-800 shadow-sm capitalize">{movie.genre}</div>
              </div>

              <div className="p-4 flex flex-col flex-1 justify-between">
                <h3 className="font-bold text-stone-800 leading-tight mb-1 line-clamp-2 group-hover:text-orange-600 transition-colors">{movie.title}</h3>
                <div className="flex items-center gap-1.5 mt-3">
                  <FontAwesomeIcon icon={faStar} className="text-yellow-400 text-sm" />
                  <span className="font-semibold text-stone-700 text-sm">{movie.rating.toFixed(1)}</span>
                </div>
              </div>

            </div>
          ))}
        </div>
      ) : (
        <div className="flex-1 flex flex-col items-center justify-center text-stone-400 pb-20">
          <FontAwesomeIcon icon={faFire} className="text-6xl text-stone-200 mb-4" />
          <p className="text-xl font-semibold text-stone-500">Aucun film trouvé.</p>
          <p>Essayez une autre recherche.</p>
        </div>
      )}

    </div>
  );
};

export default Home;