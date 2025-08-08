import { ReactNode } from 'react';

interface TabsListProps {
  children: ReactNode;
  className?: string;
}

export function TabsList({ children, className }: TabsListProps) {
  return (
    <div className={`flex border-b border-gray-200 ${className || ''}`}>
      {children}
    </div>
  );
}
