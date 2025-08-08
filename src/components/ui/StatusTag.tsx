import { StatusDot } from './StatusDot';

interface StatusTagProps {
  status: 'success' | 'error';
  children: React.ReactNode;
}

export function StatusTag({ status, children }: StatusTagProps) {
  return (
    <div className="inline-flex items-center gap-[0.5rem] px-2 py-[0.25rem] bg-primary-4/9 rounded-[1rem] text-[0.9375rem] leading-[1.25rem] font-medium">
      <StatusDot status={status} />
      {children}
    </div>
  );
}
