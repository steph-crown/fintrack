import { Header } from "@/components/layout/Header";
import { Footer } from "@/components/layout/Footer";
import { HeroSection } from "@/components/sections/HeroSection";
import { FeaturesSection } from "@/components/sections/FeaturesSection";
import { GettingStartedSection } from "@/components/sections/GettingStartedSection";
import { DownloadsSection } from "@/components/sections/DownloadsSection";

function App() {
  return (
    <div className="min-h-screen bg-background relative selection:bg-cyan-500/30">
      {/* Global Hero Background Pattern */}
      <div className="absolute top-0 left-0 right-0 min-h-screen overflow-hidden pointer-events-none z-0">
        {/* Radial gradient from top */}
        <div
          className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[600px] opacity-40 dark:opacity-30 mix-blend-multiply dark:mix-blend-normal"
          style={{
            background: "radial-gradient(ellipse at center top, hsl(187, 85%, 45%) 0%, transparent 70%)"
          }}
        />
        {/* Secondary gradient glow */}
        <div
          className="absolute bottom-0 right-0 w-[500px] h-[400px] opacity-30 dark:opacity-20 mix-blend-multiply dark:mix-blend-normal"
          style={{
            background: "radial-gradient(ellipse at bottom right, hsl(187, 85%, 45%) 0%, transparent 60%)"
          }}
        />
        {/* Dot grid pattern */}
        <div
          className="absolute inset-0 opacity-[0.2] dark:opacity-[0.2] text-[hsl(187,90%,15%)] dark:text-[hsl(187,85%,36%)]"
          style={{
            backgroundImage: `radial-gradient(circle, currentColor 1px, transparent 1px)`,
            backgroundSize: "32px 32px"
          }}
        />

        {/* Floating decorative shapes */}
        {/* Top left circle */}
        <div className="absolute top-20 left-[10%] w-4 h-4 rounded-full bg-cyan-600/60 dark:bg-primary/60 animate-bounce" style={{ animationDuration: "3s" }} />
        {/* Top right square */}
        <div className="absolute top-32 right-[15%] w-5 h-5 bg-cyan-600/50 dark:bg-primary/50 rotate-45 animate-pulse" />
        {/* Mid left diamond */}
        <div className="absolute top-1/2 left-[8%] w-3 h-3 bg-cyan-500/60 dark:bg-cyan-400/60 rotate-45 animate-bounce" style={{ animationDuration: "4s", animationDelay: "0.5s" }} />
        {/* Bottom right circle */}
        <div className="absolute bottom-40 right-[10%] w-6 h-6 rounded-full border-2 border-cyan-600/60 dark:border-primary/60 animate-pulse" style={{ animationDuration: "2.5s" }} />
        {/* Mid right dot */}
        <div className="absolute top-1/3 right-[8%] w-3 h-3 rounded-full bg-cyan-600/70 dark:bg-primary/70 animate-bounce" style={{ animationDuration: "2.8s", animationDelay: "1s" }} />
        {/* Bottom left shape */}
        <div className="absolute bottom-60 left-[15%] w-4 h-4 border-2 border-cyan-500/60 dark:border-cyan-400/60 rotate-45 animate-pulse" style={{ animationDuration: "3.5s" }} />
        {/* Extra floating elements */}
        <div className="absolute top-40 left-[20%] w-2 h-2 rounded-full bg-cyan-600/80 dark:bg-primary/80 animate-ping" style={{ animationDuration: "2s" }} />
        <div className="absolute bottom-32 right-[20%] w-3 h-3 bg-cyan-500/50 dark:bg-cyan-400/50 rotate-12 animate-bounce" style={{ animationDuration: "3.2s" }} />
      </div>

      <Header />
      <main>
        <HeroSection />
        <FeaturesSection />
        <GettingStartedSection />
        <DownloadsSection />
      </main>
      <Footer />
    </div>
  );
}

export default App;
