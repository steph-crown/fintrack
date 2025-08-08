'use client';

import { transactionsData } from '@/data/transactions.data';
import { Transaction } from '@/interfaces';
import { createContext, ReactNode, useContext, useMemo, useState } from 'react';

interface SearchContextType {
  searchQuery: string;
  setSearchQuery: (query: string) => void;
  filteredTransactions: Transaction[];
  isSearching: boolean;
  clearSearch: () => void;
}

const SearchContext = createContext<SearchContextType | undefined>(undefined);

interface SearchProviderProps {
  children: ReactNode;
}

export function SearchProvider({ children }: SearchProviderProps) {
  const [searchQuery, setSearchQuery] = useState('');

  const filteredTransactions = useMemo(() => {
    if (!searchQuery.trim()) {
      return transactionsData;
    }

    const query = searchQuery.toLowerCase().trim();

    return transactionsData.filter((transaction) => {
      const searchableFields = [
        transaction.remark.toLowerCase(),
        transaction.type.toLowerCase(),
        transaction.currency.toLowerCase(),
        transaction.amount.toString(),
        new Date(transaction.date).toLocaleDateString().toLowerCase(),
        new Date(transaction.date)
          .toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
            year: 'numeric',
          })
          .toLowerCase(),
        transaction.date,
      ];

      return searchableFields.some((field) => field.includes(query));
    });
  }, [searchQuery]);

  const isSearching = searchQuery.trim().length > 0;

  const clearSearch = () => {
    setSearchQuery('');
  };

  return (
    <SearchContext.Provider
      value={{
        searchQuery,
        setSearchQuery,
        filteredTransactions,
        isSearching,
        clearSearch,
      }}
    >
      {children}
    </SearchContext.Provider>
  );
}

export function useSearch() {
  const context = useContext(SearchContext);
  if (context === undefined) {
    throw new Error('useSearch must be used within a SearchProvider');
  }
  return context;
}
