import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

function StateMessage({ icon, title, message, actionLabel, onAction, isError = false }) {
  return (
    <div
      role={isError ? "alert" : "status"}
      className="fade-in mx-auto flex max-w-md flex-col items-center px-4 py-16 text-center"
    >
      <span
        className={`mb-5 flex h-16 w-16 items-center justify-center rounded-full text-2xl
          ${isError ? "bg-red-100 text-red-700" : "bg-orange-100 text-orange-700"}`}
      >
        <FontAwesomeIcon icon={icon} />
      </span>
      <h2 className="font-display text-2xl font-extrabold text-stone-900">{title}</h2>
      {message && <p className="mt-2 text-stone-600">{message}</p>}
      {actionLabel && (
        <button type="button" onClick={onAction} className="btn-primary mt-6 px-6 py-2.5">
          {actionLabel}
        </button>
      )}
    </div>
  );
}

export default StateMessage;
