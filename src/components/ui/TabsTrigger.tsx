'use client';

import { ReactNode } from 'react';
import { useTabs } from './Tabs';

interface TabsTriggerProps {
  children: ReactNode;
  value: string;
  className?: string;
}

export function TabsTrigger({ children, value, className }: TabsTriggerProps) {
  const { activeTab, setActiveTab } = useTabs();
  const isActive = activeTab === value;

  return (
    <button
      onClick={() => setActiveTab(value)}
      className={`px-7 py-3 text-[0.9375rem] leading-5 font-medium transition-colors border-b-[1.5px] -mb-[1px] cursor-pointer ${
        isActive
          ? 'text-primary border-primary'
          : 'text-gray-600 border-transparent hover:text-gray-800'
      } ${className || ''}`}
    >
      {children}
    </button>
  );
}
