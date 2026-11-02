export const MOODS = [
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
    <div role="group" aria-labelledby="mood-title">
      <h2 id="mood-title" className="mb-3 text-xs font-bold tracking-wider text-stone-600 uppercase">
        Ton humeur
      </h2>
      <div className="grid grid-cols-4 gap-2 lg:grid-cols-2">
        {MOODS.map((mood) => {
          const isSelected = selectedMoods.includes(mood.id);
          return (
            <button
              key={mood.id}
              type="button"
              aria-pressed={isSelected}
              onClick={() => onToggle(mood.id)}
              className={`flex min-h-16 flex-col items-center justify-center gap-1 rounded-xl border p-2 transition-colors
                ${isSelected
                  ? "border-orange-500 bg-orange-100 shadow-[inset_0_0_0_1px_var(--color-orange-500)]"
                  : "border-stone-200 bg-stone-50 hover:border-orange-400 hover:bg-white"
                }`}
            >
              <span aria-hidden="true" className="text-2xl leading-none">{mood.icon}</span>
              <span className={`text-xs font-semibold ${isSelected ? "text-orange-900" : "text-stone-600"}`}>
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
