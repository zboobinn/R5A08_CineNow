import { useRef, useState } from "react";
import { getRecommendations } from "./api";
import Footer from "./components/Footer";
import Sidebar from "./components/Sidebar";
import Home from "./pages/Home";

// status : "idle" = catalogue affiché, sinon mode recommandation
const NO_RECOMMENDATION = { status: "idle", movies: [], moods: [], error: null };

function App() {
  const [selectedMoods, setSelectedMoods] = useState([]);
  const [recommendation, setRecommendation] = useState(NO_RECOMMENDATION);
  const pendingRequest = useRef(null);

  const toggleMood = (moodId) => {
    setSelectedMoods((prev) =>
      prev.includes(moodId) ? prev.filter((id) => id !== moodId) : [...prev, moodId]
    );
  };

  const requestRecommendations = async (moods) => {
    pendingRequest.current?.abort();
    const controller = new AbortController();
    pendingRequest.current = controller;

    setRecommendation({ status: "loading", movies: [], moods, error: null });
    try {
      const movies = await getRecommendations(moods, { signal: controller.signal });
      setRecommendation({ status: "ready", movies, moods, error: null });
    } catch (err) {
      if (controller.signal.aborted) return;
      setRecommendation({ status: "error", movies: [], moods, error: err.message });
    }
  };

  const handleExit = () => {
    pendingRequest.current?.abort();
    setRecommendation(NO_RECOMMENDATION);
  };

  const handleReset = () => {
    handleExit();
    setSelectedMoods([]);
  };

  return (
    <div className="min-h-dvh bg-stone-50 text-stone-800 lg:flex">
      <a href="#contenu" className="skip-link">Aller aux films</a>
      <Sidebar
        selectedMoods={selectedMoods}
        onToggle={toggleMood}
        onRecommend={() => requestRecommendations(selectedMoods)}
        onReset={handleReset}
        isLoading={recommendation.status === "loading"}
      />
      {/* La marge laisse la place à la barre d'action fixe sous lg */}
      <div className={`flex min-w-0 flex-1 flex-col ${selectedMoods.length > 0 ? "pb-18 lg:pb-0" : ""}`}>
        <main id="contenu" tabIndex={-1} className="flex-1 outline-none">
          <Home
            recommendation={recommendation}
            onExit={handleExit}
            onRetry={() => requestRecommendations(recommendation.moods)}
          />
        </main>
        <Footer />
      </div>
    </div>
  );
}

export default App;
