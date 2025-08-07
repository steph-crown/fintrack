import { User } from '@/interfaces';
import Image from 'next/image';

interface AvatarProps {
  user: User;
  size?: number;
  className?: string;
}

export function Avatar({ user, size = 40, className }: AvatarProps) {
  return (
    <div className="relative group">
      <Image
        src={user.profilePicture}
        alt={user.username}
        width={size}
        height={size}
        className={`rounded-full object-cover border-2 border-white ${className}`}
      />

      <div className="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-2 py-1 bg-gray-900 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity duration-200 whitespace-nowrap pointer-events-none z-10">
        {user.username}

        <div className="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900"></div>
      </div>
    </div>
  );
}
