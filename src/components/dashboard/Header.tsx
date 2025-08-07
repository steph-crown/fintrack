import { GridIcon, MenuIcon } from '@/components/icons';
import { Avatar, Logo, Search } from '@/components/ui';
import { currentUser } from '@/data/users.data';
import Link from 'next/link';

export function Header() {
  return (
    <header className="flex justify-between items-center main-px py-3 ">
      <div className="flex items-center gap-2 sm:gap-7">
        <button>
          <MenuIcon />
        </button>

        <Logo />
      </div>

      <div className="flex items-center max-[300px]:gap-0 gap-1 sm:gap-7">
        <Search />

        <button className="icon-btn">
          <GridIcon />
        </button>

        <Link href="/">
          <Avatar user={currentUser} />
        </Link>
      </div>
    </header>
  );
}
