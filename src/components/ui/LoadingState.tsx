export default function LoadingState({
  message = 'Loading...',
}: {
  message?: string;
}) {
  return (
    <div className="flex flex-col items-center justify-center py-16">
      <svg
        className="animate-spin mb-6"
        width="56"
        height="56"
        viewBox="0 0 56 56"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <circle
          cx="28"
          cy="28"
          r="24"
          stroke="var(--primary)"
          strokeWidth="5"
        />
        <path
          d="M52 28c0-13.255-10.745-24-24-24"
          stroke="var(--foreground)"
          strokeWidth="5"
          strokeLinecap="round"
        />
      </svg>

      <p className="text-[color:var(--primary)] text-lg font-semibold mb-2 tracking-wide">
        {message}
      </p>
    </div>
  );
}
