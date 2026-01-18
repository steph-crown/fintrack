import { Button } from "@/components/ui/button";
import { AnimatedTerminal } from "@/components/terminal/AnimatedTerminal";

export function HeroSection() {
  const scrollToDownloads = () => {
    const element = document.getElementById("downloads");
    if (element) {
      element.scrollIntoView({ behavior: "smooth" });
    }
  };

  return (
    <section className="min-h-screen pt-32 pb-20 px-4 sm:px-6 lg:px-8 relative overflow-hidden flex flex-col justify-center">
      <div className="mx-auto max-w-6xl relative z-10 w-full">
        <div className="text-center mb-12">
          <h1 className="text-4xl sm:text-5xl lg:text-6xl tracking-tight text-foreground mb-6">
            <span className="relative inline-block">
              FinTrack
              {/* Decorative underline */}
              <svg
                className="absolute -bottom-2 left-0 w-full text-primary"
                viewBox="0 0 200 12"
                fill="none"
                preserveAspectRatio="none"
              >
                <path
                  d="M2 8C30 4 60 2 100 6C140 10 170 8 198 4"
                  stroke="currentColor"
                  strokeWidth="3"
                  strokeLinecap="round"
                />
              </svg>
            </span>
          </h1>
          <p className="text-lg sm:text-xl text-primary mb-4">
            A local-first CLI financial tracker
          </p>
          <p className="text-base sm:text-lg text-muted-foreground max-w-2xl mx-auto mb-8">
            Track your income and expenses on your own machine. Zero cloud dependencies.
            Complete data ownership. Your finances, your data, your privacy.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Button onClick={scrollToDownloads} size="lg" className="px-8 group">
              Get Started
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" className="ml-2 group-hover:translate-y-0.5 transition-transform">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" x2="12" y1="15" y2="3" />
              </svg>
            </Button>
            <Button
              variant="outline"
              size="lg"
              className="px-8 group"
              asChild
            >
              <a
                href="https://github.com/steph-crown/fintrack"
                target="_blank"
                rel="noopener noreferrer"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" className="mr-2">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
                Star on GitHub
              </a>
            </Button>
          </div>
        </div>

        <AnimatedTerminal />
      </div>
    </section>
  );
}
