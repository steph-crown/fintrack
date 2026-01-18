import { useState } from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";

interface CodeBlockProps {
  code: string;
  language?: string;
}

function CodeBlock({ code }: CodeBlockProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="relative group">
      <pre className="bg-[#0d0d0d] border border-border p-4 text-sm font-mono overflow-x-auto">
        <code className="text-muted-foreground">{code}</code>
      </pre>
      <Button
        variant="ghost"
        size="sm"
        className="absolute top-2 right-2 h-8 px-2 opacity-0 group-hover:opacity-100 transition-opacity"
        onClick={handleCopy}
      >
        {copied ? (
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
            <polyline points="20 6 9 17 4 12" />
          </svg>
        ) : (
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
            <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
            <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
          </svg>
        )}
      </Button>
    </div>
  );
}

const COMMANDS = [
  {
    title: "Initialize your tracker",
    description: "Set up FinTrack with your preferred currency",
    code: "fintrack init -c NGN",
  },
  {
    title: "Add income",
    description: "Record income with optional subcategory and description",
    code: 'fintrack add Income 50000 -s Wages -d "Monthly Salary"',
  },
  {
    title: "Add expenses",
    description: "Track your spending by category",
    code: 'fintrack add Expenses 150.50 -s Groceries -d "Weekly shop"',
  },
  {
    title: "View your records",
    description: "List all transactions with filtering options",
    code: "fintrack list",
  },
  {
    title: "See your balance",
    description: "Get a financial summary with totals",
    code: "fintrack total",
  },
  {
    title: "Analyze spending",
    description: "Get insights into your financial data",
    code: "fintrack describe",
  },
];

export function GettingStartedSection() {
  return (
    <section id="getting-started" className="py-20 px-4 sm:px-6 lg:px-8">
      <div className="mx-auto max-w-6xl">
        <div className="text-center mb-12">
          <h2 className="text-3xl sm:text-4xl tracking-tight text-foreground mb-4">
            Getting Started
          </h2>
          <p className="text-muted-foreground max-w-2xl mx-auto">
            Start tracking your finances in seconds with these simple commands.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {COMMANDS.map((cmd, index) => (
            <Card key={index} className="border-border">
              <CardContent className="pt-6">
                <div className="flex items-start gap-4 mb-4">
                  <span className="flex items-center justify-center w-8 h-8 text-sm bg-primary/10 text-primary">
                    {index + 1}
                  </span>
                  <div>
                    <h3 className="text-foreground mb-1">{cmd.title}</h3>
                    <p className="text-sm text-muted-foreground">{cmd.description}</p>
                  </div>
                </div>
                <CodeBlock code={cmd.code} />
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </section>
  );
}
