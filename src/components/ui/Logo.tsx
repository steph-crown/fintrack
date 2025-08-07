import { LogoIcon } from '@/components/icons';
import Link from 'next/link';

export function Logo() {
  return (
    <Link href="/">
      <LogoIcon className="text-primary" />
    </Link>
  );
}
