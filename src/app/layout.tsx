import { Header, MobileSidebar, Sidebar } from '@/components';
import { ErrorBoundary } from '@/components/ui/ErrorBoundary';
import { SearchProvider, SidebarProvider } from '@/contexts';
import type { Metadata } from 'next';
import { Public_Sans } from 'next/font/google';
import './globals.css';

const publicSans = Public_Sans({
  variable: '--font-public-sans',
  subsets: ['latin'],
  display: 'swap',
});

export const metadata: Metadata = {
  title: 'FinTrack - Wallet Ledger Dashboard',
  description:
    'A comprehensive wallet ledger dashboard for tracking financial transactions',
  icons: {
    icon: [{ url: '/favicon.svg', type: 'image/svg+xml' }],
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${publicSans.variable} font-sans antialiased`}>
        <SearchProvider>
          <SidebarProvider>
            <ErrorBoundary>
              <Header />
              <MobileSidebar />
              <main className="main-x-pad flex gap-8 lg:gap-12 ">
                <aside className="hidden lg:block w-[15rem] lg:w-[17.5rem] xl:w-80 max-w-full h-full shrink-0">
                  <Sidebar />
                </aside>
                <div className=" py-4 sm:py-7 w-full">{children}</div>
              </main>
            </ErrorBoundary>
          </SidebarProvider>
        </SearchProvider>
      </body>
    </html>
  );
}
