import React from 'react';
import logo from "../assets/cinenow.png";
import WeatherCard from "./WeatherCard";
import MoodSelector from "./MoodSelector";

const Sidebar = ({ selectedMoods, onToggle, onRecommend, onReset }) => {
  return (
    <aside className="w-72 bg-white border-r border-stone-200 flex flex-col p-6 shrink-0 shadow-sm overflow-y-auto">
      <div className="flex justify-center mb-8 shrink-0">
        <img src={logo} alt="CineNow" className="w-28 h-auto object-contain shrink-0" />
      </div>

      <div className="flex flex-col gap-8 flex-1">
        <WeatherCard />
        <MoodSelector selectedMoods={selectedMoods} onToggle={onToggle} />

        {selectedMoods.length > 0 && (
          <div className="flex flex-col gap-2">
            <button
              onClick={onRecommend}
              className="w-full py-3 bg-orange-500 hover:bg-orange-600 text-white font-bold rounded-xl transition-all shadow-sm"
            >
              Recommander
            </button>
            <button
              onClick={onReset}
              className="w-full py-2 text-sm text-stone-400 hover:text-stone-600 transition-colors"
            >
              Réinitialiser
            </button>
          </div>
        )}
      </div>
    </aside>
  );
};

export default Sidebar;