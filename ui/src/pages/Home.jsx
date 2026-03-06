import React from 'react';

const Home = () => {
  return (
    <div className="p-8 fade-in">
      <header className="mb-10">
        <h1 className="text-3xl font-bold text-stone-900">Ciné-Now</h1>
        <p className="text-stone-500">Comment te sents-tu aujourd'hui ?</p>
      </header>

      {/* Grille d'humeurs (Placeholder) */}
      <section className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-12">
        {['Joyeur', 'Triste', 'Énervé', 'Calme'].map(mood => (
          <button key={mood} className="p-6 bg-white rounded-2xl border border-stone-200 hover:border-orange-400 hover:shadow-md transition-all text-center">
            <span className="text-2xl block mb-2">✨</span>
            <span className="font-semibold text-stone-700">{mood}</span>
          </button>
        ))}
      </section>

      <div className="bg-stone-100 border-2 border-dashed border-stone-300 rounded-3xl p-12 text-center text-stone-400">
        Les recommandations de films apparaîtront ici...
      </div>
    </div>
  );
};

export default Home;