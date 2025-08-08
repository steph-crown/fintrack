export default function EmptyState({
  message = 'No data available.',
}: {
  message?: string;
}) {
  return (
    <div className="flex flex-col items-center justify-center py-16">
      <svg
        width="80"
        height="80"
        viewBox="0 0 80 80"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        className="mb-6"
      >
        <circle
          cx="40"
          cy="40"
          r="38"
          stroke="#E5E7EB"
          strokeWidth="4"
          fill="#F9FAFB"
        />
        <rect x="22" y="32" width="36" height="24" rx="6" fill="#E5E7EB" />
        <rect x="28" y="38" width="24" height="4" rx="2" fill="#F3F4F6" />
        <rect x="28" y="46" width="16" height="4" rx="2" fill="#F3F4F6" />
      </svg>

      <p className="text-black-3/80 text-sm sm:text-base font-medium mb-2">
        {message}
      </p>
    </div>
  );
}
