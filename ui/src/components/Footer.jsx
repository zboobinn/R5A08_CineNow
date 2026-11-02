import tmdbLogo from "../assets/tmdb-logo.svg";

const linkClass = "font-semibold text-white underline underline-offset-2 hover:text-orange-300 transition-colors";

function Footer() {
  return (
    <footer className="bg-stone-900 text-sm text-stone-300">
      <div className="mx-auto grid max-w-[100rem] gap-6 px-4 py-8 sm:px-6 md:grid-cols-2 lg:px-10">
        <div className="flex flex-col gap-3">
          <a href="https://www.themoviedb.org" target="_blank" rel="noreferrer" className="w-fit">
            <img src={tmdbLogo} alt="The Movie Database (TMDB)" className="h-4 w-auto sm:h-5" />
          </a>
          <p lang="en">This product uses the TMDB API but is not endorsed or certified by TMDB.</p>
        </div>

        <p className="md:text-right">
          Données météo :{" "}
          <a href="https://open-meteo.com/" target="_blank" rel="noreferrer" className={linkClass}>
            Open-Meteo.com
          </a>{" "}
          (
          <a href="https://creativecommons.org/licenses/by/4.0/" target="_blank" rel="noreferrer" className={linkClass}>
            CC BY 4.0
          </a>
          )
        </p>
      </div>
    </footer>
  );
}

export default Footer;
