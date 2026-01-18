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
          className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[600px] opacity-30"
          style={{
            background: "radial-gradient(ellipse at center top, hsl(187, 85%, 43%) 0%, transparent 70%)"
          }}
        />
        {/* Secondary gradient glow */}
        <div
          className="absolute bottom-0 right-0 w-[500px] h-[400px] opacity-20"
          style={{
            background: "radial-gradient(ellipse at bottom right, hsl(187, 85%, 43%) 0%, transparent 60%)"
          }}
        />
        {/* Dot grid pattern */}
        <div
          className="absolute inset-0 opacity-[0.15]"
          style={{
            backgroundImage: `radial-gradient(circle, hsl(187, 85%, 43%) 1px, transparent 1px)`,
            backgroundSize: "32px 32px"
          }}
        />

        {/* Floating decorative shapes */}
        {/* Top left circle */}
        <div className="absolute top-20 left-[10%] w-4 h-4 rounded-full bg-primary/60 animate-bounce" style={{ animationDuration: "3s" }} />
        {/* Top right square */}
        <div className="absolute top-32 right-[15%] w-5 h-5 bg-primary/50 rotate-45 animate-pulse" />
        {/* Mid left diamond */}
        <div className="absolute top-1/2 left-[8%] w-3 h-3 bg-cyan-400/60 rotate-45 animate-bounce" style={{ animationDuration: "4s", animationDelay: "0.5s" }} />
        {/* Bottom right circle */}
        <div className="absolute bottom-40 right-[10%] w-6 h-6 rounded-full border-2 border-primary/60 animate-pulse" style={{ animationDuration: "2.5s" }} />
        {/* Mid right dot */}
        <div className="absolute top-1/3 right-[8%] w-3 h-3 rounded-full bg-primary/70 animate-bounce" style={{ animationDuration: "2.8s", animationDelay: "1s" }} />
        {/* Bottom left shape */}
        <div className="absolute bottom-60 left-[15%] w-4 h-4 border-2 border-cyan-400/60 rotate-45 animate-pulse" style={{ animationDuration: "3.5s" }} />
        {/* Extra floating elements */}
        <div className="absolute top-40 left-[20%] w-2 h-2 rounded-full bg-primary/80 animate-ping" style={{ animationDuration: "2s" }} />
        <div className="absolute bottom-32 right-[20%] w-3 h-3 bg-cyan-400/50 rotate-12 animate-bounce" style={{ animationDuration: "3.2s" }} />
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
