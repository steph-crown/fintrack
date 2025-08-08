import { ChevronDownIcon, MoreIcon } from '@/components/icons';
import {
  AvatarGroup,
  DashboardSummary,
  StatusTag,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  TransactionsTable,
} from '@/components/ui';
import { dashboardData } from '@/data/dashboard.data';
import { teamUsers } from '@/data/users.data';

export default function HomeContent() {
  const displayedTeamUsersNames = teamUsers
    .slice(0, 3)
    .map((user) => user.username)
    .join(', ');

  return (
    <div className="flex flex-col gap-y-7 w-full">
      <div className="flex sm:items-center justify-between flex-col sm:flex-row w-full gap-3">
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-1">
            <h1>Wallet Ledger</h1>
            <ChevronDownIcon />
          </div>
          <StatusTag status="success">Active</StatusTag>
        </div>
        <div className="flex items-center gap-3">
          <button className=" rounded-2xl bg-primary px-[1.125rem] py-2 font-medium text-[0.9375rem] leading-5 text-black-3">
            Share
          </button>
          <button className="rounded-2xl p-2 border border-solid border-primary-6/20">
            <MoreIcon />
          </button>
        </div>
      </div>

      <div className="flex flex-col min-[350px]:flex-row gap-x-3 gap-y-2">
        <AvatarGroup users={teamUsers} />
        <div className="flex items-center gap-1 text-black-2/62 leading-[-0.5%] text-[0.9375rem]">
          <p>{displayedTeamUsersNames}</p>
          <p>+12 others</p>
        </div>
      </div>

      <Tabs defaultValue="overview" className="w-full">
        <TabsList>
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="transactions">Transactions</TabsTrigger>
        </TabsList>

        <TabsContent value="overview">
          <div className="flex flex-col gap-y-8">
            <DashboardSummary data={dashboardData} className="py-3" />
            <TransactionsTable />
          </div>
        </TabsContent>

        <TabsContent value="transactions">
          <div className="py-6">
            <h3 className="text-lg font-semibold mb-4">Transaction History</h3>

            <p className="text-gray-600">
              Your transaction history will be displayed here. This could
              include a list of all transactions, filters, and search
              functionality.
            </p>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
