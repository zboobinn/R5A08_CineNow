import React from 'react';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSpinner } from "@fortawesome/free-solid-svg-icons";
import logo from "../assets/cinenow.png";
import WeatherCard from "./WeatherCard";
import MoodSelector from "./MoodSelector";

const Sidebar = ({ selectedMoods, onToggle, onRecommend, onReset, isLoading }) => {
  return (
    <aside className="shrink-0 border-b border-stone-200 bg-white lg:sticky lg:top-0 lg:h-dvh lg:w-80 lg:overflow-y-auto lg:border-r lg:border-b-0">
      <div className="popcorn-stripes" aria-hidden="true" />

      <div className="flex flex-col gap-5 p-4 sm:p-6 lg:gap-6">
        <div className="flex items-center gap-3">
          <img src={logo} alt="" className="h-14 w-14 shrink-0 object-contain" />
          <div>
            <p className="font-display text-3xl leading-none font-extrabold tracking-tight text-stone-900">Ciné-Now</p>
            <p className="mt-1 text-sm text-stone-600">Ton film, selon ton humeur et la météo</p>
          </div>
        </div>

        <div className="grid gap-5 md:grid-cols-[minmax(0,17rem)_minmax(0,1fr)] md:items-start lg:grid-cols-1 lg:gap-6">
          <WeatherCard />
          <MoodSelector selectedMoods={selectedMoods} onToggle={onToggle} />
        </div>

        {selectedMoods.length > 0 && (
          // Barre fixe sous lg pour garder l'action à portée de pouce
          <div className="fixed inset-x-0 bottom-0 z-30 flex items-center gap-2 border-t border-stone-200 bg-white p-3 shadow-[0_-4px_16px_rgb(0_0_0/0.08)] lg:sticky lg:-mx-6 lg:-mb-6 lg:px-6 lg:shadow-none">
            <button
              type="button"
              onClick={onRecommend}
              disabled={isLoading}
              aria-busy={isLoading}
              className="btn-primary flex flex-1 items-center justify-center gap-2 py-3"
            >
              {isLoading && <FontAwesomeIcon icon={faSpinner} spin />}
              {isLoading ? "Recherche…" : "Recommander"}
            </button>
            <button
              type="button"
              onClick={onReset}
              className="rounded-xl px-3 py-2 text-sm font-medium text-stone-600 underline-offset-2 transition-colors hover:text-stone-900 hover:underline"
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
