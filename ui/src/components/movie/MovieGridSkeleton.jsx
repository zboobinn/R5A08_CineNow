function MovieGridSkeleton({ count = 10, label = "Chargement des films" }) {
  return (
    <div role="status">
      <span className="sr-only">{label}</span>
      <div aria-hidden="true" className="movie-grid pb-10">
        {Array.from({ length: count }, (_, i) => (
          <div key={i} className="overflow-hidden rounded-2xl border border-stone-200 bg-white">
            <div className="aspect-2/3 animate-pulse bg-stone-200" />
            <div className="flex flex-col gap-2 p-3 sm:p-4">
              <div className="h-4 w-4/5 animate-pulse rounded bg-stone-200" />
              <div className="h-4 w-2/5 animate-pulse rounded bg-stone-200" />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default MovieGridSkeleton;
