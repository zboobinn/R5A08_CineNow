import React, { useState } from 'react';
import logo from "../assets/cinenow.png";
import WeatherCard from "./WeatherCard";

const Sidebar = () => {
  // On utilise maintenant un tableau vide au départ pour stocker PLUSIEURS humeurs
  const [selectedMoods, setSelectedMoods] = useState([]);

  // J'ai ajouté "Fatigué" et "Peur" pour arriver à 6 humeurs
  const moods = [
    { id: 'joy', label: 'Joyeux', icon: '✨' },
    { id: 'sad', label: 'Triste', icon: '😢' },
    { id: 'angry', label: 'Énervé', icon: '🔥' },
    { id: 'calm', label: 'Calme', icon: '🧘' },
    { id: 'tired', label: 'Fatigué', icon: '😴' },
    { id: 'fear', label: 'Peur', icon: '😨' }
  ];

  // Fonction pour ajouter ou retirer une humeur du tableau
  const toggleMood = (moodId) => {
    setSelectedMoods(prevSelected => {
      // Si l'humeur est déjà dans le tableau, on l'enlève (désélection)
      if (prevSelected.includes(moodId)) {
        return prevSelected.filter(id => id !== moodId);
      }
      // Sinon, on l'ajoute au tableau existant
      else {
        return [...prevSelected, moodId];
      }
    });
  };

  return (
    <aside className="w-72 bg-white border-r border-stone-200 flex flex-col p-6 shrink-0 shadow-sm overflow-y-auto">
      {/* Logo */}
      <div className="flex justify-center mb-8 shrink-0">
        <img 
          src={logo} 
          alt="CineNow" 
          className="w-28 h-auto object-contain shrink-0" 
        />
      </div>
      
      {/* Conteneur pour espacer Météo et Humeur */}
      <div className="flex flex-col gap-8 flex-1">
        
        <WeatherCard />

        {/* Section Humeur */}
        <div>
          <h3 className="text-sm font-bold text-stone-400 uppercase tracking-wider mb-3">
            Ton humeur
          </h3>
          <div className="grid grid-cols-2 gap-3">
            {moods.map(mood => {
              // On vérifie si l'humeur actuelle fait partie du tableau des sélections
              const isSelected = selectedMoods.includes(mood.id);

              return (
                <button 
                  key={mood.id} 
                  onClick={() => toggleMood(mood.id)}
                  className={`p-3 rounded-xl border transition-all text-center flex flex-col items-center justify-center gap-1
                    ${isSelected 
                      ? 'bg-orange-50 border-orange-400 shadow-sm text-orange-700' 
                      : 'bg-stone-50 border-stone-200 hover:border-orange-300 hover:bg-white text-stone-600'
                    }`}
                >
                  <span className="text-2xl">{mood.icon}</span>
                  <span className="text-xs font-semibold">{mood.label}</span>
                </button>
              );
            })}
          </div>
        </div>

      </div>
    </aside>
  );
};

export default Sidebar;