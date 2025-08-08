import { Transaction } from '@/interfaces';
import { ColumnConfig, SortKey } from '@/interfaces/table.interface';

export const formatAmount = (amount: number): string => {
  const absAmount = Math.abs(amount);
  return `${amount < 0 ? '-' : ''}$${absAmount.toLocaleString()}`;
};

export const formatDate = (dateString: string): string => {
  return dateString;
};

export const getCellValue = (
  transaction: Transaction,
  key: SortKey
): string | number => {
  switch (key) {
    case 'date':
      return formatDate(transaction.date);
    case 'amount':
      return formatAmount(transaction.amount);
    default:
      return transaction[key];
  }
};

export const getCellClassName = (
  column: ColumnConfig,
  isLastRow: boolean
): string => {
  const baseClass = `${column.key === 'type' ? 'py-3' : 'py-[1.125rem]'} pr-3 ${column.width} ${column.minWidth} relative`;

  const borderClass = !isLastRow
    ? `after:content-[""] after:absolute after:bottom-0 after:left-0 after:h-px after:bg-gray-200 after:${column.isLastColumn ? 'right-[0.001px]' : 'right-[18px]'}`
    : '';

  return `${baseClass} ${borderClass}`;
};

export const getColumnHeaderClassName = (column: ColumnConfig): string => {
  return `py-1 pr-3 ${column.width} ${column.minWidth} after:content-[''] after:absolute after:bottom-0 after:left-0 after:${column.isLastColumn ? 'right-0' : 'right-[18px]'} after:right-[0.001px]  after:h-px after:bg-gray-200 relative`;
};
