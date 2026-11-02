import React, { useState, useMemo, useEffect, useRef } from 'react';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faArrowLeft, faBoxOpen, faCalendarDays, faMagnifyingGlass, faPlugCircleXmark,
} from "@fortawesome/free-solid-svg-icons";
import { getMovies } from "../api";
import { MOODS } from "../components/MoodSelector";
import StateMessage from "../components/StateMessage";
import MovieGrid from "../components/movie/MovieGrid";
import MovieGridSkeleton from "../components/movie/MovieGridSkeleton";
import { usePolledResource } from "../hooks/usePolledResource";

const TABS = [
  { id: "recent", label: "Sorties récentes" },
  { id: "upcoming", label: "À venir" },
];

const normalizeText = (text) =>
  text.toLowerCase().normalize("NFD").replace(/[̀-ͯ]/g, "");

const isCatalogEmpty = (catalog) => Object.keys(catalog).length === 0;

const moodLabels = (moods) =>
  moods.map((id) => MOODS.find((mood) => mood.id === id)?.label ?? id).join(", ");

const countLabel = (count) => `${count} film${count > 1 ? "s" : ""}`;

const Home = ({ recommendation, onExit, onRetry }) => {
  const [searchTerm, setSearchTerm] = useState("");
  const [activeTab, setActiveTab] = useState("recent"); // "recent" | "upcoming"
  const catalog = usePolledResource(getMovies, isCatalogEmpty);
  const headingRef = useRef(null);

  const isRecommendationMode = recommendation.status !== "idle";
  const isRecommendationLoading = recommendation.status === "loading";
  const search = searchTerm.trim();

  // Une nouvelle demande remplace la recherche et amène le focus sur les résultats
  useEffect(() => {
    if (!isRecommendationLoading) return;
    setSearchTerm("");
    headingRef.current?.focus({ preventScroll: true });
    headingRef.current?.scrollIntoView({ block: "start" });
  }, [isRecommendationLoading]);

  const allMovies = useMemo(() => {
    const seen = new Set();
    return Object.values(catalog.data ?? {})
      .flat()
      .filter((movie) => {
        if (seen.has(movie.id)) return false;
        seen.add(movie.id);
        return true;
      });
  }, [catalog.data]);

  const today = new Date().toISOString().slice(0, 10); // "2025-03-15"

  const recentMovies = useMemo(() => {
    return allMovies
      .filter((m) => m.release_date && m.release_date <= today)
      .sort((a, b) => b.release_date.localeCompare(a.release_date))
      .slice(0, 20);
  }, [allMovies, today]);

  const upcomingMovies = useMemo(() => {
    return allMovies
      .filter((m) => m.release_date && m.release_date > today)
      .sort((a, b) => a.release_date.localeCompare(b.release_date))
      .slice(0, 10);
  }, [allMovies, today]);

  const searchResults = useMemo(() => {
    if (!search) return [];
    const searchNormalized = normalizeText(search);
    return allMovies.filter((movie) => normalizeText(movie.title).includes(searchNormalized));
  }, [search, allMovies]);

  const catalogPending = catalog.status === "loading" || catalog.status === "warming";

  const renderCatalogProblem = () => {
    if (catalogPending) {
      return (
        <MovieGridSkeleton
          label={catalog.status === "warming" ? "Le catalogue est en cours de chargement" : undefined}
        />
      );
    }
    if (catalog.status === "error") {
      return (
        <StateMessage
          isError
          icon={faPlugCircleXmark}
          title="Le catalogue ne répond pas"
          message={`${catalog.error} Vérifie que les services sont lancés, puis réessaie.`}
          actionLabel="Réessayer"
          onAction={catalog.retry}
        />
      );
    }
    return null;
  };

  const renderSearch = () => {
    const problem = renderCatalogProblem();
    if (problem) return problem;
    if (searchResults.length === 0) {
      return (
        <StateMessage
          icon={faMagnifyingGlass}
          title="Aucun film trouvé"
          message={`Rien dans le catalogue pour « ${search} ».`}
          actionLabel="Effacer la recherche"
          onAction={() => setSearchTerm("")}
        />
      );
    }
    return <MovieGrid movies={searchResults} />;
  };

  const renderRecommendations = () => {
    if (isRecommendationLoading) {
      return <MovieGridSkeleton label="Recherche de recommandations" />;
    }
    if (recommendation.status === "error") {
      return (
        <StateMessage
          isError
          icon={faPlugCircleXmark}
          title="Recommandation impossible"
          message={recommendation.error}
          actionLabel="Réessayer"
          onAction={onRetry}
        />
      );
    }
    if (recommendation.movies.length === 0) {
      return (
        <StateMessage
          icon={faBoxOpen}
          title="Aucune recommandation pour l'instant"
          message="Le catalogue est peut-être encore en cours de chargement. Réessaie dans quelques secondes."
          actionLabel="Réessayer"
          onAction={onRetry}
        />
      );
    }
    return <MovieGrid movies={recommendation.movies} />;
  };

  const renderCatalog = () => {
    const problem = renderCatalogProblem();
    if (problem) return problem;

    const movies = activeTab === "recent" ? recentMovies : upcomingMovies;
    if (movies.length === 0) {
      return (
        <StateMessage
          icon={faCalendarDays}
          title={activeTab === "recent" ? "Aucune sortie récente" : "Aucune sortie annoncée"}
          message="Le catalogue ne contient aucun film dans cette catégorie pour le moment."
        />
      );
    }
    return <MovieGrid movies={movies} />;
  };

  let title = TABS.find((tab) => tab.id === activeTab).label;
  let subtitle = null;
  if (search) {
    title = `Résultats pour « ${search} »`;
    subtitle = searchResults.length > 0 ? countLabel(searchResults.length) : null;
  } else if (isRecommendationMode) {
    title = "Pour toi ce soir";
    if (isRecommendationLoading) subtitle = "On croise ton humeur et la météo…";
    else if (recommendation.status === "ready") {
      subtitle = `${countLabel(recommendation.movies.length)} selon ton humeur et la météo : ${moodLabels(recommendation.moods)}.`;
    }
  }

  return (
    <div className="mx-auto max-w-[100rem] px-4 py-6 sm:px-6 lg:px-10 lg:py-10">
      <header className="mb-8 flex flex-col gap-5 xl:flex-row xl:items-end xl:justify-between">
        <div className="min-w-0">
          <h1
            ref={headingRef}
            tabIndex={-1}
            className="scroll-mt-4 font-display text-4xl font-extrabold tracking-tight wrap-anywhere text-stone-900 outline-none lg:text-5xl"
          >
            {title}
          </h1>
          <p aria-live="polite" className="mt-2 text-stone-600 empty:hidden sm:text-lg">{subtitle}</p>

          {isRecommendationMode && (
            <button
              type="button"
              onClick={onExit}
              className="mt-4 inline-flex items-center gap-2 rounded-full border-2 border-stone-900 px-4 py-2 text-sm font-semibold text-stone-900 transition-colors hover:bg-stone-900 hover:text-white"
            >
              <FontAwesomeIcon icon={faArrowLeft} />
              Retour au catalogue
            </button>
          )}

          {!isRecommendationMode && !search && (
            <div role="group" aria-label="Catégorie de films" className="mt-4 flex flex-wrap gap-2">
              {TABS.map((tab) => (
                <button
                  key={tab.id}
                  type="button"
                  aria-pressed={activeTab === tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`rounded-full px-4 py-2 text-sm font-semibold transition-colors
                    ${activeTab === tab.id
                      ? "bg-orange-500 text-stone-950"
                      : "bg-stone-200/70 text-stone-700 hover:bg-stone-200"
                    }`}
                >
                  {tab.label}
                </button>
              ))}
            </div>
          )}
        </div>

        <div className="group relative w-full max-w-xl shrink-0 xl:w-80">
          <div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-4">
            <FontAwesomeIcon icon={faMagnifyingGlass} className="text-stone-500 transition-colors group-focus-within:text-orange-700" />
          </div>
          <input
            type="search"
            aria-label="Rechercher un film"
            placeholder="Rechercher un film..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full rounded-2xl border border-stone-300 bg-white py-3 pr-4 pl-11 text-stone-800 shadow-sm transition-colors placeholder:text-stone-500 hover:border-stone-400"
          />
        </div>
      </header>

      {search ? renderSearch() : isRecommendationMode ? renderRecommendations() : renderCatalog()}
    </div>
  );
};

export default Home;
