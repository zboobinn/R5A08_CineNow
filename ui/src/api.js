const API_URL = (import.meta.env.VITE_API_URL ?? "http://localhost:3001").replace(/\/$/, "");
const TIMEOUT_MS = 10000;

export class ApiError extends Error {
  constructor(message, status = null) {
    super(message);
    this.status = status;
  }
}

async function request(path, { signal, ...options } = {}) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), TIMEOUT_MS);
  signal?.addEventListener("abort", () => controller.abort());

  try {
    const res = await fetch(`${API_URL}${path}`, { ...options, signal: controller.signal });
    if (!res.ok) {
      const body = await res.json().catch(() => null);
      throw new ApiError(body?.error ?? `Erreur serveur (${res.status}).`, res.status);
    }
    return await res.json();
  } catch (err) {
    if (err instanceof ApiError || signal?.aborted) throw err;
    throw new ApiError("Impossible de joindre le serveur Ciné-Now.");
  } finally {
    clearTimeout(timer);
  }
}

const postJson = (path, body, options) =>
  request(path, {
    ...options,
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

export const getWeather = (options) => request("/weather", options);

export const getMovies = (options) => request("/movies", options);

export const getMovieDetails = (id, options) => request(`/movie/${id}`, options);

export const getRecommendations = (moods, options) =>
  postJson("/recommendations", { moods }, options);

export const sendLocation = (lat, lon) => postJson("/location", { lat, lon });
