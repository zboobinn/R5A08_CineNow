import React, { useState } from 'react';
import logo from "../assets/cinenow.png";
import WeatherCard from "./WeatherCard";
import MoodSelector from "./MoodSelector";



const Sidebar = () => {
  const [selectedMoods, setSelectedMoods] = useState([]);

  const toggleMood = (moodId) => {
    setSelectedMoods((prev) =>
      prev.includes(moodId) ? prev.filter((id) => id !== moodId) : [...prev, moodId]
    );
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
        
        <MoodSelector selectedMoods={selectedMoods} onToggle={toggleMood} />
      </div>
    </aside>
  );
};

export default Sidebar;