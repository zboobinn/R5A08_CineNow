import Sidebar from "./components/Sidebar";
import Home from "./pages/Home"; // On va créer cette page

function App() {
  return (
    <div className="flex h-screen bg-stone-50 text-stone-800">
      {/* La Sidebar reste fixe à gauche */}
      <Sidebar />

      {/* Le contenu principal change selon la "Page" importée */}
      <main className="flex-1 overflow-y-auto">
        <Home />
      </main>
    </div>
  );
}

export default App;