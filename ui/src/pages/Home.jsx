import React, { useState, useMemo, useEffect } from 'react';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass } from "@fortawesome/free-solid-svg-icons";
import MovieGrid from "../components/movie/MovieGrid";

const normalizeText = (text) =>
  text.toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g, "");

const Home = ({ recommendations }) => {
  const [searchTerm, setSearchTerm] = useState("");
  const [moviesByGenre, setMoviesByGenre] = useState({});
  const [loading, setLoading] = useState(true);

  const [activeTab, setActiveTab] = useState("recent"); // "recent" | "upcoming"

// En mode recommandation, pas d'onglets
const tabs = [
  { id: "recent", label: "Sorties récentes" },
  { id: "upcoming", label: "À venir" },
];


  useEffect(() => {
    fetch("http://localhost:3001/movies")
      .then((res) => res.json())
      .then((data) => { setMoviesByGenre(data); setLoading(false); })
      .catch(() => setLoading(false));
  }, []);

  const allMovies = useMemo(() => {
    const seen = new Set();
    return Object.values(moviesByGenre)
      .flat()
      .filter((movie) => {
        if (seen.has(movie.id)) return false;
        seen.add(movie.id);
        return true;
      });
  }, [moviesByGenre]);

const today = new Date().toISOString().slice(0, 10); // "2025-03-15"

const recentMovies = useMemo(() => {
  return [...allMovies]
    .filter((m) => m.release_date && m.release_date <= today)
    .sort((a, b) => b.release_date.localeCompare(a.release_date))
    .slice(0, 20);
}, [allMovies]);

const upcomingMovies = useMemo(() => {
  return [...allMovies]
    .filter((m) => m.release_date && m.release_date > today)
    .sort((a, b) => a.release_date.localeCompare(b.release_date))
    .slice(0, 10);
}, [allMovies]);

  const isRecommendationMode = recommendations !== null;

  const displayedMovies = isRecommendationMode
    ? recommendations
    : activeTab === "recent" ? recentMovies : upcomingMovies;
    
  const filteredMovies = useMemo(() => {
    if (!searchTerm) return displayedMovies;
    const searchNormalized = normalizeText(searchTerm);
    return allMovies.filter((movie) =>
      normalizeText(movie.title).includes(searchNormalized)
    );
  }, [searchTerm, displayedMovies, allMovies]);

  if (loading) return (
    <div className="flex-1 flex items-center justify-center text-stone-400">
      Chargement des films...
    </div>
  );

  return (
  <div className="p-8 h-full flex flex-col max-w-7xl mx-auto">
    <header className="mb-10 flex flex-col md:flex-row md:items-end justify-between gap-6">
      <div>
        {isRecommendationMode ? (
          <>
            <h1 className="text-4xl font-bold text-stone-900 tracking-tight mb-2">Pour toi ce soir</h1>
            <p className="text-stone-500 text-lg">{recommendations.length} films sélectionnés selon ton humeur et la météo.</p>
          </>
        ) : (
          <>
            <h1 className="text-4xl font-bold text-stone-900 tracking-tight mb-2">
              {activeTab === "recent" ? "Sorties récentes" : "À venir"}
            </h1>
            <div className="flex gap-2 mt-3">
              {tabs.map((tab) => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`px-4 py-1.5 rounded-full text-sm font-semibold transition-all
                    ${activeTab === tab.id
                      ? "bg-orange-500 text-white shadow-sm"
                      : "bg-stone-100 text-stone-500 hover:bg-stone-200"
                    }`}
                >
                  {tab.label}
                </button>
              ))}
            </div>
          </>
        )}
      </div>

      <div className="relative w-full md:w-96 group">
        <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
          <FontAwesomeIcon icon={faMagnifyingGlass} className="text-stone-400 group-focus-within:text-orange-500 transition-colors" />
        </div>
        <input
          type="text"
          placeholder="Rechercher un film..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="w-full pl-11 pr-4 py-3.5 bg-white border border-stone-200 rounded-2xl text-stone-700 shadow-sm focus:outline-none focus:border-orange-400 focus:ring-4 focus:ring-orange-400/10 transition-all placeholder:text-stone-400"
        />
      </div>
    </header>

    <MovieGrid movies={filteredMovies} />
  </div>
);
};

export default Home;