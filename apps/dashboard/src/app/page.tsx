'use client';

import { useEffect, useState } from 'react';
import { goatApi, getGoatConfig } from '@/lib/goat-api';
import { Activity, Server, Box, Cpu } from 'lucide-react';
import { useRouter } from 'next/navigation';

export default function OverviewPage() {
  const [status, setStatus] = useState<any>(null);
  const [health, setHealth] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const router = useRouter();

  useEffect(() => {
    if (!getGoatConfig()) {
      router.push('/settings');
      return;
    }

    const load = async () => {
      try {
        const [h, s] = await Promise.all([goatApi.getHealth(), goatApi.getStatus()]);
        setHealth(h);
        setStatus(s);
      } catch (e: any) {
        setError(e.message);
      }
    };
    load();
    const interval = setInterval(load, 5000);
    return () => clearInterval(interval);
  }, [router]);

  if (error) {
    return (
      <div className="p-6 bg-destructive/10 border border-destructive rounded-lg text-destructive">
        <h2 className="font-bold text-lg mb-2">Connection Error</h2>
        <p>{error}</p>
        <button onClick={() => router.push('/settings')} className="mt-4 underline">Go to Settings</button>
      </div>
    );
  }

  if (!status) return <div className="text-muted-foreground animate-pulse">Loading daemon status...</div>;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Overview</h1>
        <p className="text-muted-foreground">System status and metrics.</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard title="Daemon Health" value={health?.status || 'Unknown'} icon={Activity} />
        <StatCard title="GOAT Version" value={health?.version || '0.13.0'} icon={Box} />
        <StatCard title="Active Profile" value={status?.profile || 'Unknown'} icon={Cpu} />
        <StatCard title="MCP Servers" value={status?.mcp_server_count || 0} icon={Server} />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
        <div className="border border-border bg-card rounded-lg p-6">
          <h3 className="font-semibold mb-4">Memory & Context</h3>
          <div className="space-y-2 text-sm text-muted-foreground">
            <p>Provider: <span className="text-foreground">{status?.provider || 'Unknown'}</span></p>
            <p>History Length: <span className="text-foreground">{status?.history_len || 0} messages</span></p>
            <p>Brain Status: <span className="text-foreground">{status?.brain_status || 'Unknown'}</span></p>
          </div>
        </div>
        
        <div className="border border-border bg-card rounded-lg p-6">
          <h3 className="font-semibold mb-4">System Details</h3>
          <div className="space-y-2 text-sm text-muted-foreground">
            <p>OS: <span className="text-foreground">{health?.os || 'Unknown'}</span></p>
            <p>Uptime: <span className="text-foreground">{health?.uptime_secs || 0}s</span></p>
          </div>
        </div>
      </div>
    </div>
  );
}

function StatCard({ title, value, icon: Icon }: { title: string; value: string | number; icon: any }) {
  return (
    <div className="border border-border bg-card p-6 rounded-lg shadow-sm">
      <div className="flex items-center gap-4">
        <div className="p-3 bg-primary/10 text-primary rounded-md">
          <Icon className="w-5 h-5" />
        </div>
        <div>
          <p className="text-sm text-muted-foreground">{title}</p>
          <p className="text-2xl font-bold">{value}</p>
        </div>
      </div>
    </div>
  );
}
