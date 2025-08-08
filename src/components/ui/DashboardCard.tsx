import { MoreIcon } from '@/components/icons';

interface DashboardCardProps {
  title: string;
  value: string | number;
  change: number;
  className?: string;
}

export function DashboardCard({
  title,
  value,
  change,
  className,
}: DashboardCardProps) {
  const isDebitCard = title.toLowerCase().includes('debit');
  const isPositive = isDebitCard ? change <= 0 : change >= 0;
  const changeColor = isPositive ? 'text-success' : 'text-error';
  const changeSign = change >= 0 ? '+' : '-';

  return (
    <div
      className={`bg-primary-4/9 rounded-2xl p-7 transition-shadow hover:shadow-lg hover:bg-primary-4/20 cursor-pointer ${className || ''}`}
    >
      <div className="flex items-center justify-between mb-[1.125rem]">
        <h3 className="font-bold text-[1.0625rem] leading-6 -tracking-[0.5%]">
          {title}
        </h3>

        <button className=" hover:text-gray-600 transition-colors cursor-pointer">
          <MoreIcon width={20} height={20} />
        </button>
      </div>

      <div className="space-y-2">
        <h1 className="font-bold card-header mb-1">
          {typeof value === 'number'
            ? title.toLowerCase().includes('transaction')
              ? value.toLocaleString()
              : `$${value.toLocaleString()}`
            : value}
        </h1>

        <div className={`text-[0.8125rem] font-medium ${changeColor}`}>
          {changeSign}
          {Math.abs(change)}%
        </div>
      </div>
    </div>
  );
}
