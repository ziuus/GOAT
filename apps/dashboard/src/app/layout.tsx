import type { Metadata } from 'next';
import './globals.css';
import { Inter } from 'next/font/google';
import Sidebar from '@/components/Sidebar';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'GOAT Dashboard',
  description: 'Control center for the General Omniscient Agentic Tool',
};

import EventStreamProvider from '@/components/EventStreamProvider';
import { ThemeProvider } from '@/components/ThemeProvider';
import { CommandPalette } from '@/components/CommandPalette';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={`${inter.className} flex h-screen overflow-hidden bg-background`}>
        <ThemeProvider>
          <EventStreamProvider>
            <CommandPalette />
            <Sidebar />
            <main className="flex-1 overflow-y-auto bg-background text-foreground">{children}</main>
          </EventStreamProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
