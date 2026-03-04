import WeatherCard from "./components/WeatherCard";
import logo from "./assets/cinenow.png";

function App() {
  return (
    <div className="flex h-screen bg-stone-50 text-stone-800">

      <aside className="w-50 bg-white border-r border-stone-200 flex flex-col gap-6 p-6 shrink-0">
        <img src={logo} alt="CineNow" className="w-24 h-24 mx-auto" />
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