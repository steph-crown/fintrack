import { DashboardSummary as DashboardSummaryType } from '@/interfaces';
import { DashboardCard } from './DashboardCard';

interface DashboardSummaryProps {
  data: DashboardSummaryType;
  className?: string;
}

export function DashboardSummary({ data, className }: DashboardSummaryProps) {
  const cards = [
    {
      title: 'Total Balance',
      value: data.totalBalance,
      change: data.balanceChange,
    },
    {
      title: 'Total Credits',
      value: data.totalCredits,
      change: data.creditsChange,
    },
    {
      title: 'Total Debits',
      value: data.totalDebits,
      change: data.debitsChange,
    },
    {
      title: 'Transactions',
      value: data.transactionCount,
      change: data.transactionChange,
    },
  ];

  return (
    <div className={className}>
      <h2 className="text-xl font-bold text-foreground mb-[1.125rem]">
        Summary
      </h2>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 lg:gap-7">
        {cards.map((card) => (
          <DashboardCard
            key={card.title}
            title={card.title}
            value={card.value}
            change={card.change}
          />
        ))}
      </div>
    </div>
  );
}
