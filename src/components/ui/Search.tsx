'use client';

import { SearchIcon } from '@/components/icons';
import { useState } from 'react';

export function Search() {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div className="relative">
      {!isExpanded ? (
        <button
          onClick={() => setIsExpanded(true)}
          className="icon-btn"
          aria-label="Search"
        >
          <SearchIcon />
        </button>
      ) : (
        <div className="flex items-center bg-white border border-gray-200 rounded-lg shadow-sm focus-within:ring-2 focus-within:ring-primary focus-within:border-primary transition-all">
          <div className="pl-3 pr-2">
            <SearchIcon width={18} height={18} className="text-gray-400" />
          </div>

          <input
            type="text"
            placeholder="Search transactions..."
            className="w-64 py-2 pr-3 text-sm text-gray-900 placeholder-gray-500 bg-transparent border-none outline-none"
            autoFocus
            onBlur={() => setIsExpanded(false)}
          />
        </div>
      )}
    </div>
  );
}
