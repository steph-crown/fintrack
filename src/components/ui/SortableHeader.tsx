import { ChevronDownIcon } from '@/components/icons';
import { Transaction } from '@/interfaces';

type SortKey = keyof Transaction;
type SortOrder = 'asc' | 'desc';

interface SortConfig {
  key: SortKey;
  order: SortOrder;
}

interface SortableHeaderProps {
  label: string;
  sortKey: SortKey;
  sortConfig: SortConfig;
  onSort: (key: SortKey) => void;
  className?: string;
}

export function SortableHeader({
  label,
  sortKey,
  sortConfig,
  onSort,
  className,
}: SortableHeaderProps) {
  const isActive = sortConfig.key === sortKey;
  const isDesc = isActive && sortConfig.order === 'desc';

  return (
    <th className={`text-left py-1 pr-3 ${className || ''}`}>
      <button
        onClick={() => onSort(sortKey)}
        className={`flex items-center gap-1 text-black-2/62 hover:text-black-2/70 hover:font-semibold transition-all leading-4 text-[0.8125rem] cursor-pointer ${isActive ? 'font-semibold text-black-2/70 ' : ' font-medium'}`}
      >
        {label}
        <ChevronDownIcon
          width={24}
          height={24}
          className={`transition-transform duration-200 hover:scale-150 ${
            isActive
              ? isDesc
                ? 'rotate-0 text-primary'
                : 'rotate-180 text-primary'
              : 'text-black-2/62'
          } ${isActive ? 'scale-125' : ''}`}
        />
      </button>
    </th>
  );
}
