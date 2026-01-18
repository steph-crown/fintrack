import { Header } from "@/components/layout/Header";
import { Footer } from "@/components/layout/Footer";
import { HeroSection } from "@/components/sections/HeroSection";
import { FeaturesSection } from "@/components/sections/FeaturesSection";
import { GettingStartedSection } from "@/components/sections/GettingStartedSection";
import { DownloadsSection } from "@/components/sections/DownloadsSection";

function App() {
  return (
    <div className="min-h-screen bg-background">
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
