import { User } from '@/interfaces';
import { Avatar } from './Avatar';

interface AvatarGroupProps {
  users: User[];
  size?: number;
  className?: string;
}

export function AvatarGroup({ users, size = 36, className }: AvatarGroupProps) {
  return (
    <div className={`flex -space-x-2 ${className}`}>
      {users.map((user, index) => (
        <Avatar
          key={user.username}
          user={user}
          size={size}
          className={`relative hover:z-20`}
          style={{
            zIndex: 10 - index,
          }}
        />
      ))}
    </div>
  );
}
