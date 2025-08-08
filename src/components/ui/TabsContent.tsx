'use client';

import { ReactNode } from 'react';
import { useTabs } from './Tabs';

interface TabsContentProps {
  children: ReactNode;
  value: string;
  className?: string;
}

export function TabsContent({ children, value, className }: TabsContentProps) {
  const { activeTab } = useTabs();

  if (activeTab !== value) {
    return null;
  }

  return <div className={`py-4 sm:py-7 ${className || ''}`}>{children}</div>;
}
