import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFilm } from "@fortawesome/free-solid-svg-icons";
import WeatherCard from "./components/WeatherCard";

function App() {
  return (
    <div className="flex h-screen bg-stone-50 text-stone-800">

      {/* Sidebar gauche */}
      <aside className="w-56 bg-white border-r border-stone-200 flex flex-col gap-6 p-6 shrink-0">
        <h1 className="text-xl font-bold flex items-center gap-2">
          <FontAwesomeIcon icon={faFilm} className="text-stone-600" />
          Ciné-Now
        </h1>
        <WeatherCard />
      </aside>

      {/* Contenu principal */}
      <main className="flex-1 p-6 overflow-y-auto">
        <p className="text-stone-400">Les films et l'humeur arrivent bientôt...</p>
      </main>

    </div>
  );
}

export default App;