import React, { useState, useMemo } from 'react';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass, faStar, faFire } from "@fortawesome/free-solid-svg-icons";

// 🎬 Fausses données
const MOCK_MOVIES = [
  { id: 1, title: "Dune: Deuxième Partie", year: 2024, rating: 8.8, mood: "Épique", image: "https://image.tmdb.org/t/p/w500/1pdfLvkbY9ohJlCjQH2JGjjcNsV.jpg" },
  { id: 2, title: "Pauvres Créatures", year: 2023, rating: 8.4, mood: "Étrange", image: "https://image.tmdb.org/t/p/w500/kCGlIMHnOm8JPXq3rXM6c5RkEPK.jpg" },
  { id: 3, title: "Interstellar", year: 2014, rating: 8.7, mood: "Réflexion", image: "https://image.tmdb.org/t/p/w500/gEU2QniE6E77NI6lCU6MxlNBvIx.jpg" },
  { id: 4, title: "Spider-Man: Across the Spider-Verse", year: 2023, rating: 8.9, mood: "Joyeux", image: "https://image.tmdb.org/t/p/w500/8Vt6mWEReuy4Of61Lnj5Xj704m8.jpg" },
  { id: 5, title: "Le Garçon et le Héron", year: 2023, rating: 7.9, mood: "Calme", image: "https://image.tmdb.org/t/p/w500/A2ZzH2hA3Z2qZqE5Ff01NnsB2aC.jpg" },
  { id: 6, title: "La La Land", year: 2016, rating: 8.0, mood: "Triste", image: "https://image.tmdb.org/t/p/w500/uDO8zWDhfWwoFdKS4fzkUJt0Vr0.jpg" },
];

const normalizeText = (text) => {
  return text.toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g, "");
};

const Home = () => {
  const [searchTerm, setSearchTerm] = useState("");

  const filteredMovies = useMemo(() => {
    if (!searchTerm) return MOCK_MOVIES;
    
    const searchNormalized = normalizeText(searchTerm);
    return MOCK_MOVIES.filter(movie => 
      normalizeText(movie.title).includes(searchNormalized) ||
      normalizeText(movie.mood).includes(searchNormalized)
    );
  }, [searchTerm]);

  return (
    <div className="p-8 fade-in h-full flex flex-col max-w-7xl mx-auto">
      
      {/* 🔝 En-tête + Barre de recherche */}
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
            placeholder="Rechercher un film ou un mood..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full pl-11 pr-4 py-3.5 bg-white border border-stone-200 rounded-2xl text-stone-700 shadow-sm focus:outline-none focus:border-orange-400 focus:ring-4 focus:ring-orange-400/10 transition-all placeholder:text-stone-400"
          />
        </div>
      </header>

      {/* 🎬 Grille de films */}
      {filteredMovies.length > 0 ? (
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6 pb-10">
          {filteredMovies.map((movie) => (
            <div key={movie.id} className="group relative flex flex-col bg-white rounded-2xl overflow-hidden shadow-sm hover:shadow-2xl transition-all duration-300 hover:-translate-y-2 cursor-pointer border border-stone-100">
              
              <div className="relative aspect-[2/3] overflow-hidden bg-stone-200">
                <img src={movie.image} alt={movie.title} className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
                <div className="absolute top-3 left-3 px-2.5 py-1 bg-white/90 backdrop-blur-sm rounded-lg text-xs font-bold text-stone-800 shadow-sm">{movie.mood}</div>
                <button className="absolute top-3 right-3 w-8 h-8 bg-black/40 hover:bg-orange-500 backdrop-blur-md rounded-full text-white opacity-0 group-hover:opacity-100 transition-all duration-300 flex items-center justify-center">+</button>
              </div>

              <div className="p-4 flex flex-col flex-1 justify-between">
                <div>
                  <h3 className="font-bold text-stone-800 leading-tight mb-1 line-clamp-1 group-hover:text-orange-600 transition-colors">{movie.title}</h3>
                  <p className="text-sm text-stone-400">{movie.year}</p>
                </div>
                
                <div className="flex items-center gap-1.5 mt-3">
                  <FontAwesomeIcon icon={faStar} className="text-yellow-400 text-sm" />
                  <span className="font-semibold text-stone-700 text-sm">{movie.rating}</span>
                </div>
              </div>

            </div>
          ))}
        </div>
      ) : (
        <div className="flex-1 flex flex-col items-center justify-center text-stone-400 pb-20 fade-in">
          <FontAwesomeIcon icon={faFire} className="text-6xl text-stone-200 mb-4" />
          <p className="text-xl font-semibold text-stone-500">Oups, aucun film trouvé.</p>
          <p>Essayez une autre recherche comme "Dune" ou "Joyeux".</p>
        </div>
      )}

    </div>
  );
};

export default Home;