import { Transaction } from './index';

export type SortKey = keyof Transaction;
export type SortOrder = 'asc' | 'desc';

export interface SortConfig {
  key: SortKey;
  order: SortOrder;
}

export interface ColumnConfig {
  key: SortKey;
  label: string;
  width: string;
  minWidth: string;
  isLastColumn?: boolean;
}
