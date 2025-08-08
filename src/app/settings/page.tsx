import LoadingState from '@/components/ui/LoadingState';
import { Suspense, lazy } from 'react';

const SettingsContent = lazy(() => import('./SettingsContent'));

export default function SettingsPage() {
  return (
    <Suspense fallback={<LoadingState message="Loading..." />}>
      <SettingsContent />
    </Suspense>
  );
}
