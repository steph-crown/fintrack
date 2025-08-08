import { ColumnConfig } from '@/interfaces/table.interface';

export const TABLE_COLUMNS: ColumnConfig[] = [
  { key: 'date', label: 'Date', width: 'w-[700px]', minWidth: 'min-w-[200px]' },
  {
    key: 'remark',
    label: 'Remark',
    width: 'w-[200px]',
    minWidth: 'min-w-[200px]',
  },
  {
    key: 'amount',
    label: 'Amount',
    width: 'w-[100px]',
    minWidth: 'min-w-[100px]',
  },
  {
    key: 'currency',
    label: 'Currency',
    width: 'w-[90px]',
    minWidth: 'min-w-[90px]',
  },
  {
    key: 'type',
    label: 'Type',
    width: 'w-[80px]',
    minWidth: 'min-w-[90px]',
    isLastColumn: true,
  },
];
