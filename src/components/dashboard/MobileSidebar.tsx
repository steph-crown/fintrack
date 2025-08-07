'use client';

import { Logo } from '@/components/ui';
import { useSidebar } from '@/contexts';
import { useEffect } from 'react';
import { Sidebar } from './Sidebar';

export function MobileSidebar() {
  const { isOpen, close } = useSidebar();

  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }

    return () => {
      document.body.style.overflow = '';
    };
  }, [isOpen]);

  return (
    <>
      <div
        className={`fixed inset-0 bg-black bg-opacity-50 z-40 md:hidden transition-opacity duration-300 ${
          isOpen ? 'opacity-100 visible' : 'opacity-0 invisible'
        }`}
        onClick={close}
      />

      <aside
        className={`fixed top-0 left-0 h-full w-full bg-white z-50 md:hidden transform transition-transform duration-300 ease-in-out ${
          isOpen ? 'translate-x-0' : '-translate-x-full'
        }`}
      >
        <div className="p-4">
          <div className="flex justify-between items-center">
            <Logo />

            <button
              onClick={close}
              className="flex justify-center items-center text-2xl leading-none icon-btn !rounded-full w-8 h-8"
            >
              ×
            </button>
          </div>
          <Sidebar />
        </div>
      </aside>
    </>
  );
}
