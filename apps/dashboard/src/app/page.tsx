'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { ErrorState } from '@/components/ui/States';
import { goatApi } from '@/lib/goat-api';

export default function Home() {
  const router = useRouter();
  const [daemonStatus, setDaemonStatus] = useState<'checking' | 'online' | 'offline'>('checking');

  useEffect(() => {
    const checkHealth = async () => {
      try {
        await goatApi.getHealth();
        setDaemonStatus('online');
        router.push('/mission-control');
      } catch (e) {
        setDaemonStatus('offline');
      }
    };
    checkHealth();
  }, [router]);

  if (daemonStatus === 'offline') {
    return (
      <PageShell>
        <PageHeader 
          title="GOAT OS Offline" 
          subtitle="The local daemon is not running."
        />
        <ErrorState 
          title="Daemon Disconnected"
          description="GOAT requires its local Rust daemon to execute agents, access memory, and manage workflows securely."
          action={
            <button 
              onClick={() => alert("Run: cargo run --release -- daemon start")}
              className="px-4 py-2 bg-red-500/20 text-red-400 hover:bg-red-500/30 font-medium rounded-lg text-sm border border-red-500/30"
            >
              How to start the daemon
            </button>
          }
        />
      </PageShell>
    );
  }

  return (
    <PageShell>
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-400 animate-pulse">Initializing Mission Control...</div>
      </div>
    </PageShell>
  );
}
