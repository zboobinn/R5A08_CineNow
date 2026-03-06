import React from 'react';

const Home = () => {
  return (
    <div className="p-8 fade-in h-full flex flex-col">
      <header className="mb-8">
        <h1 className="text-3xl font-bold text-stone-900">Recommandations</h1>
        <p className="text-stone-500">Les films de ton mood.</p>
      </header>

      {/* Zone d'affichage des futurs films */}
      <div className="flex-1 bg-stone-100 border-2 border-dashed border-stone-300 rounded-3xl p-12 flex items-center justify-center text-stone-400">
        <p>...</p>
      </div>
    </div>
  );
};

export default Home;