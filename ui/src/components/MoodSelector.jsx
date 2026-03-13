const MOODS = [
  { id: "joy",     label: "Joyeux",  icon: '✨',    color: "text-yellow-500" },
  { id: "sad",     label: "Triste",  icon: '😢',    color: "text-blue-400" },
  { id: "angry",   label: "Énervé",  icon: '🔥',    color: "text-red-500" },
  { id: "calm",    label: "Calme",   icon: '🧘',    color: "text-green-500" },
  { id: "tired",   label: "Fatigué", icon: '😴',    color: "text-stone-400" },
  { id: "fear",    label: "Peur",    icon: '😨',    color: "text-purple-500" },
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