import { useCallback, useEffect, useState } from "react";

const REFRESH_MS = 30000;
const WARMUP_MS = 3000; // Services encore en train de publier leurs données sur MQTT

// status : "loading" | "warming" (réponse vide, on réessaie vite) | "ready" | "error"
export function usePolledResource(fetcher, isEmpty) {
  const [state, setState] = useState({ data: null, status: "loading", error: null });
  const [attempt, setAttempt] = useState(0);

  useEffect(() => {
    const controller = new AbortController();
    let timer;

    const load = async () => {
      let delay = REFRESH_MS;
      try {
        const data = await fetcher({ signal: controller.signal });
        const empty = isEmpty(data);
        if (empty) delay = WARMUP_MS;
        setState({ data, status: empty ? "warming" : "ready", error: null });
      } catch (err) {
        if (controller.signal.aborted) return;
        // Une panne pendant un rafraîchissement n'efface pas ce qui est déjà affiché
        setState((prev) =>
          prev.status === "ready" ? prev : { data: null, status: "error", error: err.message }
        );
      }
      timer = setTimeout(load, delay);
    };

    load();
    return () => {
      controller.abort();
      clearTimeout(timer);
    };
  }, [fetcher, isEmpty, attempt]);

  const refresh = useCallback(() => setAttempt((n) => n + 1), []);

  const retry = useCallback(() => {
    setState({ data: null, status: "loading", error: null });
    refresh();
  }, [refresh]);

  return { ...state, retry, refresh };
}
