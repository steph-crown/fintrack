import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Button } from "@/components/ui/button";

interface CodeBlockProps {
  code: string;
  title?: string;
}

function CodeBlock({ code, title }: CodeBlockProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="relative group mb-4">
      {title && (
        <div className="text-sm text-muted-foreground mb-2">{title}</div>
      )}
      <pre className="bg-[#0d0d0d] border border-border p-4 text-sm font-mono overflow-x-auto max-w-full">
        <code className="text-green-400 whitespace-pre">{code}</code>
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

export function DownloadsSection() {
  return (
    <section id="downloads" className="py-20 px-4 sm:px-6 lg:px-8 bg-card">
      <div className="mx-auto max-w-6xl">
        <div className="text-center mb-12">
          <h2 className="text-3xl sm:text-4xl tracking-tight text-foreground mb-4">
            Downloads
          </h2>
          <p className="text-muted-foreground max-w-2xl mx-auto">
            Install FinTrack on your system using your preferred method.
          </p>
        </div>

        <Tabs defaultValue="macos" className="w-full">
          <TabsList className="grid w-full max-w-lg mx-auto grid-cols-4 mb-8">
            <TabsTrigger value="macos">macOS</TabsTrigger>
            <TabsTrigger value="windows">Windows</TabsTrigger>
            <TabsTrigger value="linux">Linux</TabsTrigger>
            <TabsTrigger value="other">Other</TabsTrigger>
          </TabsList>

          <TabsContent value="macos">
            <div className="grid gap-6 md:grid-cols-2">
              <Card className="overflow-hidden">
                <CardHeader>
                  <CardTitle className="text-lg">Homebrew (Recommended)</CardTitle>
                  <CardDescription>
                    The easiest way to install on macOS
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <CodeBlock code="brew install steph-crown/fintrack/fintrack" />
                </CardContent>
              </Card>

              <Card className="overflow-hidden">
                <CardHeader>
                  <CardTitle className="text-lg">Installer Script</CardTitle>
                  <CardDescription>
                    One-line installation via curl
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <CodeBlock code='curl --proto "=https" --tlsv1.2 -LsSf https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-installer.sh | sh' />
                </CardContent>
              </Card>

              <Card className="overflow-hidden md:col-span-2">
                <CardHeader>
                  <CardTitle className="text-lg">Manual Installation</CardTitle>
                  <CardDescription>
                    Download the appropriate binary for your Mac
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="grid gap-4 md:grid-cols-2 mb-6">
                    <div>
                      <p className="text-sm text-muted-foreground mb-2">Apple Silicon (M1/M2/M3):</p>
                      <a
                        href="https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-aarch64-apple-darwin.tar.xz"
                        className="text-primary hover:underline text-sm"
                      >
                        fintrack-aarch64-apple-darwin.tar.xz
                      </a>
                    </div>
                    <div>
                      <p className="text-sm text-muted-foreground mb-2">Intel Macs:</p>
                      <a
                        href="https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-x86_64-apple-darwin.tar.xz"
                        className="text-primary hover:underline text-sm"
                      >
                        fintrack-x86_64-apple-darwin.tar.xz
                      </a>
                    </div>
                  </div>
                  <div className="border-t border-border pt-4">
                    <p className="text-sm text-muted-foreground mb-3">After downloading:</p>
                    <ol className="list-decimal list-inside space-y-2 text-sm text-muted-foreground">
                      <li>Extract: <code className="bg-muted px-1.5 py-0.5 text-foreground">tar -xf fintrack-*.tar.xz</code></li>
                      <li>Move to PATH: <code className="bg-muted px-1.5 py-0.5 text-foreground">mv fintrack /usr/local/bin/</code></li>
                      <li>Make executable: <code className="bg-muted px-1.5 py-0.5 text-foreground">chmod +x /usr/local/bin/fintrack</code></li>
                    </ol>
                  </div>
                </CardContent>
              </Card>
            </div>
          </TabsContent>

          <TabsContent value="windows">
            <div className="grid gap-6 md:grid-cols-2">
              <Card className="overflow-hidden">
                <CardHeader>
                  <CardTitle className="text-lg">PowerShell (Recommended)</CardTitle>
                  <CardDescription>
                    Quick installation via PowerShell
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <CodeBlock code='powershell -ExecutionPolicy Bypass -c "irm https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-installer.ps1 | iex"' />
                </CardContent>
              </Card>

              <Card className="overflow-hidden">
                <CardHeader>
                  <CardTitle className="text-lg">MSI Installer</CardTitle>
                  <CardDescription>
                    Traditional Windows installer with wizard
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <a
                    href="https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-x86_64-pc-windows-msvc.msi"
                    className="inline-flex items-center gap-2 text-primary hover:underline"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                      <polyline points="7 10 12 15 17 10" />
                      <line x1="12" x2="12" y1="15" y2="3" />
                    </svg>
                    Download MSI Installer
                  </a>
                </CardContent>
              </Card>

              <Card className="overflow-hidden md:col-span-2">
                <CardHeader>
                  <CardTitle className="text-lg">Manual Installation</CardTitle>
                  <CardDescription>
                    Download the ZIP and add to your PATH
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <ol className="list-decimal list-inside space-y-2 text-sm text-muted-foreground">
                    <li>
                      Download{" "}
                      <a
                        href="https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-x86_64-pc-windows-msvc.zip"
                        className="text-primary hover:underline"
                      >
                        fintrack-x86_64-pc-windows-msvc.zip
                      </a>
                    </li>
                    <li>Extract to a location like C:\Program Files\fintrack\</li>
                    <li>Add the folder to your system PATH via Environment Variables</li>
                    <li>Restart your terminal</li>
                  </ol>
                </CardContent>
              </Card>
            </div>
          </TabsContent>

          <TabsContent value="linux">
            <div className="grid gap-6 md:grid-cols-2">
              <Card className="overflow-hidden">
                <CardHeader>
                  <CardTitle className="text-lg">Installer Script (Recommended)</CardTitle>
                  <CardDescription>
                    Works on most Linux distributions
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <CodeBlock code='curl --proto "=https" --tlsv1.2 -LsSf https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-installer.sh | sh' />
                </CardContent>
              </Card>

              <Card className="overflow-hidden">
                <CardHeader>
                  <CardTitle className="text-lg">Manual Download</CardTitle>
                  <CardDescription>
                    Choose the right binary for your system
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="space-y-3 mb-6">
                    <div>
                      <p className="text-sm text-muted-foreground mb-1">Linux x64:</p>
                      <a
                        href="https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-x86_64-unknown-linux-gnu.tar.xz"
                        className="text-primary hover:underline text-sm"
                      >
                        fintrack-x86_64-unknown-linux-gnu.tar.xz
                      </a>
                    </div>
                    <div>
                      <p className="text-sm text-muted-foreground mb-1">Linux ARM64:</p>
                      <a
                        href="https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-aarch64-unknown-linux-gnu.tar.xz"
                        className="text-primary hover:underline text-sm"
                      >
                        fintrack-aarch64-unknown-linux-gnu.tar.xz
                      </a>
                    </div>
                    <div>
                      <p className="text-sm text-muted-foreground mb-1">Alpine Linux:</p>
                      <a
                        href="https://github.com/steph-crown/fintrack/releases/latest/download/fintrack-x86_64-unknown-linux-musl.tar.xz"
                        className="text-primary hover:underline text-sm"
                      >
                        fintrack-x86_64-unknown-linux-musl.tar.xz
                      </a>
                    </div>
                  </div>
                  <div className="border-t border-border pt-4">
                    <p className="text-sm text-muted-foreground mb-3">After downloading:</p>
                    <ol className="list-decimal list-inside space-y-2 text-sm text-muted-foreground">
                      <li>Extract: <code className="bg-muted px-1.5 py-0.5 text-foreground">tar -xf fintrack-*.tar.xz</code></li>
                      <li>Move to PATH: <code className="bg-muted px-1.5 py-0.5 text-foreground">mv fintrack /usr/local/bin/</code></li>
                      <li>Make executable: <code className="bg-muted px-1.5 py-0.5 text-foreground">chmod +x /usr/local/bin/fintrack</code></li>
                    </ol>
                  </div>
                </CardContent>
              </Card>
            </div>
          </TabsContent>

          <TabsContent value="other">
            <div className="grid gap-6 md:grid-cols-2">
              <Card className="overflow-hidden">
                <CardHeader>
                  <CardTitle className="text-lg">npm (All Platforms)</CardTitle>
                  <CardDescription>
                    Install via npm if you have Node.js
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <CodeBlock code="npm install -g fintrack" />
                </CardContent>
              </Card>

              <Card className="overflow-hidden">
                <CardHeader>
                  <CardTitle className="text-lg">Cargo (Rust Users)</CardTitle>
                  <CardDescription>
                    Install from crates.io with Cargo
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <CodeBlock code="cargo install fintrack" />
                  <p className="text-xs text-muted-foreground mt-2">
                    Requires Rust 1.70+
                  </p>
                </CardContent>
              </Card>
            </div>
          </TabsContent>
        </Tabs>

        <div className="mt-8 text-center">
          <p className="text-sm text-muted-foreground">
            After installation, verify it works with:{" "}
            <code className="bg-muted px-2 py-1 text-foreground">fintrack --version</code>
          </p>
        </div>
      </div>
    </section>
  );
}
