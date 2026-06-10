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

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="dark">
      <body className={`${inter.className} flex h-screen overflow-hidden bg-background`}>
        <EventStreamProvider>
          <Sidebar />
          <main className="flex-1 overflow-y-auto p-8">{children}</main>
        </EventStreamProvider>
      </body>
    </html>
  );
}
