'use client';

import { navigationItems } from '@/data/navigation.data';
import { NavigationItem } from '@/interfaces';
import Link from 'next/link';
import { usePathname } from 'next/navigation';

export function Sidebar() {
  const pathname = usePathname();

  const isActiveLink = (href: string): boolean => {
    if (href === '/') {
      return pathname === '/';
    }
    return pathname.startsWith(href);
  };

  return (
    <nav className="py-7">
      <ul className="flex flex-col">
        {navigationItems.map((item: NavigationItem) => {
          const isActive = isActiveLink(item.href);

          return (
            <li key={item.href}>
              <Link
                href={item.href}
                className={`
                    block px-4 py-2 leading-5 text-sm text-[0.9375rem] font-medium transition-all duration-200 rounded-2xl
                    ${
                      isActive
                        ? 'bg-primary-3/16 text-primary-2'
                        : 'text-foreground hover:bg-gray-50 hover:text-gray-900'
                    }
                  `}
              >
                {item.label}
              </Link>
            </li>
          );
        })}
      </ul>
    </nav>
  );
}
