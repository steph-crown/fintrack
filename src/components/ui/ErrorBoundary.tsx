'use client';
import React from 'react';

interface ErrorBoundaryProps {
  children: React.ReactNode;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends React.Component<
  ErrorBoundaryProps,
  ErrorBoundaryState
> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.log({ error, errorInfo });
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="flex flex-col items-center justify-center py-16 text-center">
          <svg
            width="64"
            height="64"
            viewBox="0 0 64 64"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            className="mb-6"
          >
            <circle
              cx="32"
              cy="32"
              r="30"
              stroke="var(--error)"
              strokeWidth="4"
              fill="var(--background)"
            />
            <line
              x1="20"
              y1="20"
              x2="44"
              y2="44"
              stroke="var(--error)"
              strokeWidth="4"
              strokeLinecap="round"
            />
            <line
              x1="44"
              y1="20"
              x2="20"
              y2="44"
              stroke="var(--error)"
              strokeWidth="4"
              strokeLinecap="round"
            />
            <circle cx="32" cy="32" r="12" fill="var(--error)" opacity="0.1" />
          </svg>
          <p className="text-[color:var(--error)] text-xl font-bold mb-2">
            Oops! Something went wrong.
          </p>

          <p className="text-sm  max-w-md mx-auto  mb-8">
            {this.state.error?.message}
          </p>

          <button
            className="px-5 py-2 rounded-full bg-primary text-white font-semibold shadow hover:bg-[color:var(--primary-2)] transition-colors cursor-pointer"
            onClick={() => window.location.reload()}
          >
            Refresh Page
          </button>
        </div>
      );
    }
    return this.props.children;
  }
}
