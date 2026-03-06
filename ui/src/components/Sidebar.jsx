import React from 'react';
import logo from "../assets/cinenow.png";
import WeatherCard from "./WeatherCard";

const Sidebar = () => {
  return (
    <aside className="w-72 bg-white border-r border-stone-200 flex flex-col p-6 shrink-0 shadow-sm">
      {/* Logo avec taille fixe et respect des proportions */}
      <div className="flex justify-center mb-8">
        <img 
          src={logo} 
          alt="CineNow" 
          className="w-40 h-auto object-contain shrink-0" 
        />
      </div>
      
      <nav className="flex-1">
        <WeatherCard />
        {/* Tu pourras ajouter tes liens de navigation ici plus tard */}
      </nav>
    </aside>
  );
};

export default Sidebar;