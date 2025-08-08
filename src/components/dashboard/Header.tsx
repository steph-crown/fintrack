'use client';

import { GridIcon, MenuIcon } from '@/components/icons';
import { Avatar, Logo, Search } from '@/components/ui';
import { useSidebar } from '@/contexts';
import { currentUser } from '@/data/users.data';
import Link from 'next/link';

export function Header() {
  const { toggle } = useSidebar();

  return (
    <header className="flex justify-between items-center main-x-pad py-3 ">
      <div className="flex items-center gap-2 sm:gap-7">
        <button onClick={toggle}>
          <MenuIcon />
        </button>

        <Logo />
      </div>

      <div className="flex items-center max-[300px]:gap-0 gap-1 sm:gap-3">
        <Search />

        <button className="icon-btn mr-2">
          <GridIcon />
        </button>

        <Link href="/">
          <Avatar user={currentUser} />
        </Link>
      </div>
    </header>
  );
}
