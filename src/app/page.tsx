import { Suspense, lazy } from 'react';
import LoadingState from '@/components/ui/LoadingState';

const HomeContent = lazy(() => import('./HomeContent'));

export default function Home() {
  return (
    <Suspense fallback={<LoadingState message="Loading..." />}>
      <HomeContent />
    </Suspense>
  );
}
