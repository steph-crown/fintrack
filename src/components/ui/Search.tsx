'use client';

import { SearchIcon } from '@/components/icons';
import { useSearch } from '@/contexts';
import { useEffect, useRef, useState } from 'react';

export function Search() {
  const [isExpanded, setIsExpanded] = useState(false);
  const { searchQuery, setSearchQuery, clearSearch } = useSearch();
  const inputRef = useRef<HTMLInputElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        containerRef.current &&
        !containerRef.current.contains(event.target as Node)
      ) {
        if (!searchQuery.trim()) {
          setIsExpanded(false);
        }
      }
    }

    if (isExpanded) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isExpanded, searchQuery]);

  const handleClose = () => {
    clearSearch();
    setIsExpanded(false);
  };

  const handleExpand = () => {
    setIsExpanded(true);
    if (window.innerWidth < 640) {
      setTimeout(() => {
        const tableEl = document.getElementById('transactions-table-scroll');
        if (tableEl) {
          tableEl.scrollIntoView({ behavior: 'smooth', block: 'start' });
        }
      }, 100);
    }
  };

  return (
    <div className="relative" ref={containerRef}>
      {!isExpanded ? (
        <button onClick={handleExpand} className="icon-btn" aria-label="Search">
          <SearchIcon />
        </button>
      ) : (
        <>
          <div className="fixed inset-0 bg-white z-40 p-4 sm:hidden h-20 border-b border-solid border-black-3/10 shadow-2xs">
            <div className="flex items-center gap-3 mb-4">
              <div className="flex-1 flex items-center bg-gray-50 border border-gray-200 rounded-lg shadow-sm focus-within:ring-2 focus-within:ring-primary focus-within:border-primary transition-all">
                <div className="pl-3 pr-2">
                  <SearchIcon
                    width={18}
                    height={18}
                    className="text-gray-400"
                  />
                </div>

                <input
                  ref={inputRef}
                  type="text"
                  placeholder="Search transactions..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="flex-1 py-3 pr-3 text-sm text-gray-900 placeholder-gray-500 bg-transparent border-none outline-none"
                  autoFocus
                />

                {searchQuery && (
                  <button
                    onClick={handleClose}
                    className="mr-3 p-1 text-gray-400 hover:text-gray-600 transition-colors"
                    aria-label="Clear search"
                  >
                    ×
                  </button>
                )}
              </div>

              <button
                onClick={handleClose}
                className="px-4 py-2 text-sm font-medium text-primary hover:text-primary-dark transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>

          <div className="hidden sm:flex items-center bg-white border border-gray-200 rounded-lg shadow-sm focus-within:ring-2 focus-within:ring-primary focus-within:border-primary transition-all">
            <div className="pl-3 pr-2">
              <SearchIcon width={18} height={18} className="text-gray-400" />
            </div>

            <input
              ref={inputRef}
              type="text"
              placeholder="Search transactions..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-64 py-2 pr-2 text-sm text-gray-900 placeholder-gray-500 bg-transparent border-none outline-none"
              autoFocus
            />

            {searchQuery && (
              <button
                onClick={handleClose}
                className="mr-3 p-1 text-gray-400 hover:text-gray-600 transition-colors rounded-full hover:bg-gray-100 w-6 h-6 flex items-center justify-center cursor-pointer"
                aria-label="Clear search"
              >
                <span className="text-lg leading-none">×</span>
              </button>
            )}
          </div>
        </>
      )}
    </div>
  );
}
