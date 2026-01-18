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
    <section className="pt-32 pb-20 px-4 sm:px-6 lg:px-8">
      <div className="mx-auto max-w-6xl">
        <div className="text-center mb-12">
          <h1 className="text-4xl sm:text-5xl lg:text-6xl tracking-tight text-foreground mb-6">
            FinTrack
          </h1>
          <p className="text-lg sm:text-xl text-primary mb-4">
            A local-first CLI financial tracker
          </p>
          <p className="text-base sm:text-lg text-muted-foreground max-w-2xl mx-auto mb-8">
            Track your income and expenses on your own machine. Zero cloud dependencies.
            Complete data ownership. Your finances, your data, your privacy.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Button onClick={scrollToDownloads} size="lg" className="px-8">
              Get Started
            </Button>
            <Button
              variant="outline"
              size="lg"
              className="px-8"
              asChild
            >
              <a
                href="https://github.com/steph-crown/fintrack"
                target="_blank"
                rel="noopener noreferrer"
              >
                View on GitHub
              </a>
            </Button>
          </div>
        </div>

        <AnimatedTerminal />
      </div>
    </section>
  );
}
