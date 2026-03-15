const MOODS = [
  { id: "joy",        label: "Joyeux",     icon: '✨' },
  { id: "sad",        label: "Triste",     icon: '😢' },
  { id: "angry",      label: "Énervé",     icon: '🔥' },
  { id: "calm",       label: "Calme",      icon: '🧘' },
  { id: "tired",      label: "Fatigué",    icon: '😴' },
  { id: "fear",       label: "Peur",       icon: '😨' },
  { id: "adventure",  label: "Aventure",   icon: '🗺️' },
  { id: "reflection", label: "Réflexion",  icon: '🔮' },
];


function MoodSelector({ selectedMoods, onToggle }) {
  return (
    <div>
      <h3 className="text-xs font-bold text-stone-400 uppercase tracking-wider mb-3">
        Ton humeur
      </h3>
      <div className="grid grid-cols-2 gap-2">
        {MOODS.map((mood) => {
          const isSelected = selectedMoods.includes(mood.id);
          return (
            <button
              key={mood.id}
              onClick={() => onToggle(mood.id)}
              className={`flex flex-col items-center justify-center gap-1 p-3 rounded-xl border transition-all
                ${isSelected
                  ? "bg-orange-50 border-orange-400 shadow-sm"
                  : "bg-stone-50 border-stone-200 hover:border-orange-300 hover:bg-white"
                }`}
            >
              <span className="text-2xl"> {mood.icon}</span>
              <span className={`text-xs font-semibold ${isSelected ? "text-amber-700" : "text-stone-500"}`}>
                {mood.label}
              </span>
            </button>
          );
        })}
      </div>
    </div>
  );
}

export default MoodSelector;