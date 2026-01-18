import { Card, CardContent } from "@/components/ui/card";
import type { ReactNode } from "react";

interface Feature {
  icon: ReactNode;
  title: string;
  description: ReactNode;
}

const Code = ({ children }: { children: ReactNode }) => (
  <code className="font-mono text-primary bg-primary/10 px-1 py-0.5">{children}</code>
);

const FEATURES: Feature[] = [
  {
    icon: (
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
        <rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
        <path d="M7 11V7a5 5 0 0 1 10 0v4" />
      </svg>
    ),
    title: "Your Data Stays Yours",
    description: <>Everything is stored locally in <Code>~/.fintrack/</Code>. No remote servers, no accounts, no privacy concerns.</>,
  },
  {
    icon: (
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
        <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
      </svg>
    ),
    title: "Fast & Lightweight",
    description: "Simple Rust-powered CLI tool that runs instantly and gets out of your way.",
  },
  {
    icon: (
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
        <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
        <path d="M14 2v4a2 2 0 0 0 2 2h4" />
        <path d="M10 9H8" />
        <path d="M16 13H8" />
        <path d="M16 17H8" />
      </svg>
    ),
    title: "Human-Readable Storage",
    description: "All your financial data is in plain JSON. Easy to inspect, backup, and migrate.",
  },
  {
    icon: (
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
        <circle cx="12" cy="12" r="10" />
        <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" />
        <path d="M2 12h20" />
      </svg>
    ),
    title: "Cross-Platform",
    description: "Works on macOS, Windows, and Linux. Install via Homebrew, npm, Cargo, or direct download.",
  },
  {
    icon: (
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
        <circle cx="12" cy="12" r="10" />
        <path d="M15 9.5c-.5-1-1.5-1.5-3-1.5-2 0-3.5 1-3.5 2.5 0 2 2.5 2.5 4 3 1.5.5 2.5 1.5 2.5 3 0 1.5-1.5 2.5-3.5 2.5-1.5 0-2.5-.5-3-1.5" />
        <path d="M12 5v2" />
        <path d="M12 17v2" />
      </svg>
    ),
    title: "Multi-Currency",
    description: "Support for NGN, USD, GBP, EUR, CAD, AUD, and JPY currencies out of the box.",
  },
  {
    icon: (
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
        <path d="M3 3v16a2 2 0 0 0 2 2h16" />
        <path d="m19 9-5 5-4-4-3 3" />
      </svg>
    ),
    title: "Built-in Analytics",
    description: <>Use <Code>fintrack describe</Code> for spending breakdown, top subcategories, and transaction insights.</>,
  },
];

export function FeaturesSection() {
  return (
    <section id="features" className="py-20 px-4 sm:px-6 lg:px-8 bg-card">
      <div className="mx-auto max-w-6xl">
        <div className="text-center mb-12">
          <h2 className="text-3xl sm:text-4xl tracking-tight text-foreground mb-4">
            Why FinTrack?
          </h2>
          <p className="text-muted-foreground max-w-2xl mx-auto">
            Simple, reliable, and transparent financial tracking that respects your privacy.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {FEATURES.map((feature, index) => (
            <Card key={index} className="border-border bg-background">
              <CardContent className="pt-6">
                <div className="text-primary mb-4">{feature.icon}</div>
                <h3 className="text-lg text-foreground mb-2">{feature.title}</h3>
                <p className="text-sm text-muted-foreground">{feature.description}</p>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </section>
  );
}
