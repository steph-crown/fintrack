import { IconProps } from '@/interfaces';

export function ChevronDownIcon({
  className,
  width = 24,
  height = 25,
}: IconProps) {
  return (
    <svg
      width={width}
      height={height}
      viewBox="0 0 24 25"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      <path
        d="M9.86274 9.25C8.65111 9.25 8.04529 9.25 7.76477 9.48959C7.52136 9.69749 7.39218 10.0093 7.4173 10.3285C7.44624 10.6962 7.87462 11.1246 8.73137 11.9814L10.8686 14.1186C11.2646 14.5146 11.4627 14.7127 11.691 14.7868C11.8918 14.8521 12.1082 14.8521 12.309 14.7868C12.5373 14.7127 12.7354 14.5146 13.1314 14.1186L15.2686 11.9814C16.1254 11.1246 16.5538 10.6962 16.5827 10.3285C16.6078 10.0093 16.4786 9.69749 16.2352 9.48959C15.9547 9.25 15.3489 9.25 14.1373 9.25H9.86274Z"
        fill="currentColor"
      />
    </svg>
  );
}
