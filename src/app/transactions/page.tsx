'use client';

import LoadingState from '@/components/ui/LoadingState';
import { Suspense, lazy } from 'react';

const TransactionsContent = lazy(() => import('./TransactionsContent'));

export default function TransactionsPage() {
  return (
    <Suspense fallback={<LoadingState message="Loading..." />}>
      <TransactionsContent />
    </Suspense>
  );
}
