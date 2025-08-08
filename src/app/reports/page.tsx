import LoadingState from '@/components/ui/LoadingState';
import { Suspense, lazy } from 'react';

const ReportsContent = lazy(() => import('./ReportsContent'));

export default function ReportsPage() {
  return (
    <Suspense fallback={<LoadingState message="Loading..." />}>
      <ReportsContent />
    </Suspense>
  );
}
