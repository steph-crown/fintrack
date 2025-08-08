'use client';

import { TABLE_COLUMNS } from '@/constants';
import { SortConfig, SortKey } from '@/interfaces';
import { useSearch } from '@/contexts';
import EmptyState from '@/components/ui/EmptyState';
import {
  getCellClassName,
  getCellValue,
  getColumnHeaderClassName,
} from '@/utils';
import { useCallback, useMemo, useState } from 'react';
import { SortableHeader } from './SortableHeader';
import { StatusTag } from './StatusTag';

interface TransactionsTableProps {
  className?: string;
}

export function TransactionsTable({ className }: TransactionsTableProps) {
  const { filteredTransactions, isSearching, searchQuery } = useSearch();
  const [sortConfig, setSortConfig] = useState<SortConfig>({
    key: 'date',
    order: 'desc',
  });

  const handleSort = useCallback((key: SortKey) => {
    setSortConfig((current) => ({
      key,
      order: current.key === key && current.order === 'desc' ? 'asc' : 'desc',
    }));
  }, []);

  const sortedData = useMemo(() => {
    return [...filteredTransactions].sort((a, b) => {
      const aValue = a[sortConfig.key];
      const bValue = b[sortConfig.key];

      if (aValue < bValue) {
        return sortConfig.order === 'asc' ? -1 : 1;
      }

      if (aValue > bValue) {
        return sortConfig.order === 'asc' ? 1 : -1;
      }

      return 0;
    });
  }, [filteredTransactions, sortConfig]);

  return (
    <div
      id="transactions-table-scroll"
      className={`overflow-x-auto ${className || ''}`}
    >
      {isSearching && (
        <div className="mb-4 p-3 bg-primary-4/10 border border-primary-4/30 rounded-lg">
          <p className="text-sm text-primary">
            Found {filteredTransactions.length} transaction
            {filteredTransactions.length !== 1 ? 's' : ''} matching &ldquo;
            {searchQuery}&rdquo;
          </p>
        </div>
      )}
      {sortedData.length === 0 ? (
        <div className="py-8">
          <EmptyState
            message={
              isSearching
                ? `No transactions found matching "${searchQuery}"`
                : 'No transactions available'
            }
          />
        </div>
      ) : (
        <table className="w-full min-w-[700px] border-collapse">
          <thead>
            <tr>
              {TABLE_COLUMNS.map((column) => (
                <SortableHeader
                  key={column.key}
                  label={column.label}
                  sortKey={column.key}
                  sortConfig={sortConfig}
                  onSort={handleSort}
                  className={getColumnHeaderClassName(column)}
                />
              ))}
            </tr>
          </thead>
          <tbody>
            {sortedData.map((transaction, index) => (
              <tr
                key={transaction.id}
                className="hover:bg-gray-50 transition-colors"
              >
                {TABLE_COLUMNS.map((column) => {
                  const isLastRow = index === sortedData.length - 1;
                  let cellValue;
                  if (column.key === 'type') {
                    cellValue = (
                      <StatusTag
                        status={
                          transaction.type === 'Credit' ? 'success' : 'error'
                        }
                      >
                        {transaction.type}
                      </StatusTag>
                    );
                  } else {
                    cellValue = getCellValue(transaction, column.key);
                  }
                  return (
                    <td
                      key={column.key}
                      className={`${getCellClassName(column, isLastRow)}  !text-[0.9375rem]`}
                    >
                      {cellValue}
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
