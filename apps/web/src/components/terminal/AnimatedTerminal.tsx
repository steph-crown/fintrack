import { useEffect, useState, useRef } from "react";

interface TerminalLine {
  type: "command" | "output" | "success" | "table" | "header";
  content: string;
  color?: string;
}

interface TerminalStep {
  command: string;
  output: TerminalLine[];
}

const TERMINAL_STEPS: TerminalStep[] = [
  {
    command: "fintrack init -c NGN",
    output: [
      { type: "success", content: "✓ Success" },
      { type: "output", content: "Tracker initialized successfully!" },
    ],
  },
  {
    command: "fintrack add Income 50000 -s Wages -d \"Monthly Salary\"",
    output: [
      { type: "success", content: "✓ Record created:" },
      { type: "output", content: "ID: 1 | income | Wages | 50,000.00 NGN | Monthly Salary" },
    ],
  },
  {
    command: "fintrack add Expenses 1200 -s Housing -d \"Rent\"",
    output: [
      { type: "success", content: "✓ Record created:" },
      { type: "output", content: "ID: 2 | expenses | Housing | 1,200.00 NGN | Rent" },
    ],
  },
  {
    command: "fintrack list",
    output: [
      { type: "header", content: "┌────┬──────────┬───────────────┬─────────────────┬────────────┬────────────────┐" },
      { type: "header", content: "│ ID │ Category │ Subcategory   │ Amount          │ Date       │ Description    │" },
      { type: "header", content: "├────┼──────────┼───────────────┼─────────────────┼────────────┼────────────────┤" },
      { type: "table", content: "│  1 │ income   │ Wages         │   50,000.00 NGN │ 18-01-2026 │ Monthly Salary │" },
      { type: "table", content: "│  2 │ expenses │ Housing       │    1,200.00 NGN │ 18-01-2026 │ Rent           │" },
      { type: "header", content: "└────┴──────────┴───────────────┴─────────────────┴────────────┴────────────────┘" },
    ],
  },
  {
    command: "fintrack total",
    output: [
      { type: "header", content: "Financial Summary:" },
      { type: "output", content: "  Opening Balance: 0.00 NGN" },
      { type: "success", content: "  Total Income: 50,000.00 NGN" },
      { type: "output", content: "  Total Expenses: 1,200.00 NGN", color: "text-red-400" },
      { type: "header", content: "" },
      { type: "success", content: "  Net Balance: 48,800.00 NGN" },
    ],
  },
];

const TYPING_SPEED = 50;
const OUTPUT_DELAY = 100;
const STEP_DELAY = 1500;

export function AnimatedTerminal() {
  const [lines, setLines] = useState<TerminalLine[]>([]);
  const [currentCommand, setCurrentCommand] = useState("");
  const [stepIndex, setStepIndex] = useState(0);
  const [isTyping, setIsTyping] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  // Animation ID: increments on each effect run, invalidating previous animations
  const animationIdRef = useRef(0);

  // Auto-scroll to bottom when content changes
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [lines, currentCommand]);

  useEffect(() => {
    // Increment animation ID - any animation with old ID will stop
    const myId = ++animationIdRef.current;

    // Helper to check if this animation is still valid
    const isValid = () => animationIdRef.current === myId;

    const typeCommand = async (command: string) => {
      setIsTyping(true);
      for (let i = 0; i <= command.length; i++) {
        if (!isValid()) return false;
        setCurrentCommand(command.slice(0, i));
        await new Promise((r) => setTimeout(r, TYPING_SPEED));
      }
      if (!isValid()) return false;
      setIsTyping(false);
      return true;
    };

    const showOutput = async (output: TerminalLine[]) => {
      for (const line of output) {
        if (!isValid()) return false;
        await new Promise((r) => setTimeout(r, OUTPUT_DELAY));
        if (!isValid()) return false;
        setLines((prev) => [...prev, line]);
      }
      return true;
    };

    const runStep = async (step: TerminalStep) => {
      if (!await typeCommand(step.command)) return false;
      if (!isValid()) return false;
      setLines((prev) => [...prev, { type: "command", content: `$ ${step.command}` }]);
      setCurrentCommand("");
      await new Promise((r) => setTimeout(r, 300));
      if (!isValid()) return false;
      if (!await showOutput(step.output)) return false;
      if (!isValid()) return false;
      setLines((prev) => [...prev, { type: "output", content: "" }]);
      return true;
    };

    const runAnimation = async () => {
      if (stepIndex < TERMINAL_STEPS.length) {
        if (!await runStep(TERMINAL_STEPS[stepIndex])) return;
        if (!isValid()) return;
        await new Promise((r) => setTimeout(r, STEP_DELAY));
        if (!isValid()) return;
        setStepIndex((prev) => prev + 1);
      } else {
        // Reset and loop
        await new Promise((r) => setTimeout(r, 3000));
        if (!isValid()) return;
        setLines([]);
        setStepIndex(0);
      }
    };

    runAnimation();
  }, [stepIndex]);

  const getLineClass = (line: TerminalLine) => {
    switch (line.type) {
      case "command":
        return "text-cyan-400";
      case "success":
        return "text-green-400";
      case "header":
        return "text-muted-foreground";
      case "table":
        return "text-foreground";
      default:
        return line.color || "text-muted-foreground";
    }
  };

  return (
    <div className="w-full max-w-3xl mx-auto">
      <div className="border border-border bg-[#0d0d0d] overflow-hidden">
        {/* Terminal header */}
        <div className="flex items-center gap-2 px-4 py-3 bg-[#1a1a1a] border-b border-border">
          <div className="flex items-center gap-1.5">
            <div className="w-3 h-3 rounded-full bg-red-500/80" />
            <div className="w-3 h-3 rounded-full bg-yellow-500/80" />
            <div className="w-3 h-3 rounded-full bg-green-500/80" />
          </div>
          <span className="text-xs text-muted-foreground ml-2">terminal</span>
        </div>

        {/* Terminal content - scrollable */}
        <div
          ref={scrollRef}
          className="p-4 h-[380px] overflow-y-auto font-mono text-sm scrollbar-thin scrollbar-thumb-border scrollbar-track-transparent"
        >
          <div className="space-y-0.5">
            {lines.map((line, i) => (
              <div key={i} className={getLineClass(line)}>
                {line.content}
              </div>
            ))}
            {/* Current typing line */}
            <div className="flex items-center text-cyan-400">
              <span className="text-muted-foreground mr-1">$</span>
              <span>{currentCommand}</span>
              {isTyping && (
                <span className="inline-block w-2 h-4 bg-cyan-400 ml-0.5 animate-pulse" />
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
