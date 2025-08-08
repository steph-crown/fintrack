interface StatusDotProps {
  status: 'success' | 'error';
}

export function StatusDot({ status }: StatusDotProps) {
  const colorClass = status === 'success' ? 'bg-success' : 'bg-error';

  return (
    <div
      className={`w-[0.375rem] h-[0.375rem] rounded-full ${colorClass}`}
      aria-hidden="true"
    />
  );
}
