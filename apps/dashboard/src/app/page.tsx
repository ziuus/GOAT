'use client';

import { useEffect, useState } from 'react';
import { goatApi, getGoatConfig, daemonFetch } from '@/lib/goat-api';
import { Activity, Server, Box, Cpu, ShieldAlert, ShieldCheck, BrainCircuit } from 'lucide-react';
import { useRouter } from 'next/navigation';

export default function OverviewPage() {
  const [status, setStatus] = useState<any>(null);
  const [health, setHealth] = useState<any>(null);
  const [approvals, setApprovals] = useState<any[]>([]);
  const [history, setHistory] = useState<any[]>([]);
  const [learning, setLearning] = useState<any>({ pending_count: 0 });
  const [brainStats, setBrainStats] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const router = useRouter();

  useEffect(() => {
    if (!getGoatConfig()) {
      router.push('/settings');
      return;
    }

    const load = async () => {
      try {
        const [h, s, a, hRes, lRes] = await Promise.all([
          goatApi.getHealth(),
          goatApi.getStatus(),
          goatApi.getApprovals(),
          daemonFetch('/v1/approvals/history').then(res => res.ok ? res.json() : { history: [] }).catch(() => ({ history: [] })),
          daemonFetch('/v1/learning/candidates').then(res => res.ok ? res.json() : { candidates: [] }).catch(() => ({ candidates: [] })),
          goatApi.getBrainStatus()
        ]);
        setHealth(h);
        setStatus(s);
        setApprovals(a.approvals || []);
        setHistory(hRes.history || []);
        setLearning({ pending_count: lRes.candidates?.length || 0 });
        setBrainStats(arguments[0][5] || { total_documents: 0 });
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

  const lastDecision = history.length > 0 ? history[history.length - 1] : null;

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

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mt-6">
        <div className="border border-border bg-card rounded-lg p-6 lg:col-span-2 flex flex-col justify-between">
          <div>
            <h3 className="font-semibold mb-4">Memory & Context</h3>
            <div className="space-y-2 text-sm text-muted-foreground">
              <p>Provider: <span className="text-foreground">{status?.provider || 'Unknown'}</span></p>
              <p>History Length: <span className="text-foreground">{status?.history_len || 0} messages</span></p>
              <p>Brain Status: <span className="text-foreground">{status?.brain_status || 'Unknown'}</span></p>
            </div>
          </div>
          <div className="mt-6 pt-6 border-t border-border">
            <h3 className="font-semibold mb-4">System Details</h3>
            <div className="grid grid-cols-2 gap-4 text-sm text-muted-foreground">
              <p>OS: <span className="text-foreground">{health?.os || 'Unknown'}</span></p>
              <p>Uptime: <span className="text-foreground">{health?.uptime_secs || 0}s</span></p>
            </div>
          </div>
        </div>
        
        <div className="border border-border bg-card rounded-lg p-6 flex flex-col h-full">
          <h3 className="font-semibold mb-4 flex items-center gap-2">
            <ShieldAlert className="w-4 h-4 text-amber-500" /> Security & Approvals
          </h3>
          <div className="flex-1 flex flex-col space-y-6">
            <div className="bg-muted rounded-md p-4 flex flex-col items-center justify-center border border-border flex-1">
              <span className="text-4xl font-bold">{approvals.length}</span>
              <span className="text-sm text-muted-foreground">Pending Approvals</span>
            </div>
            
            <div className="bg-muted rounded-md p-4 border border-border space-y-2">
              <div className="text-xs text-muted-foreground uppercase font-semibold">Last Decision</div>
              {lastDecision ? (
                <div className="flex items-center gap-2">
                  {lastDecision.decision === 'y' ? (
                    <ShieldCheck className="w-5 h-5 text-green-500" />
                  ) : (
                    <ShieldAlert className="w-5 h-5 text-red-500" />
                  )}
                  <div className="flex flex-col overflow-hidden">
                    <span className="text-sm font-medium truncate">{lastDecision.request.tool_name}</span>
                    <span className="text-xs text-muted-foreground truncate">{lastDecision.request.action_summary}</span>
                  </div>
                </div>
              ) : (
                <div className="text-sm text-muted-foreground italic">No recent history</div>
              )}
            </div>
            
            <button 
              onClick={() => router.push('/approvals')}
              className="w-full text-sm py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
            >
              View Approvals
            </button>
            <div className="bg-muted rounded-md p-4 border border-border flex flex-col items-center justify-center space-y-2 mt-4">
               <span className="text-2xl font-bold text-indigo-400">{learning.pending_count}</span>
               <span className="text-sm text-muted-foreground">Pending Memories</span>
               <button onClick={() => router.push('/memory')} className="w-full text-xs py-1.5 bg-indigo-500/10 text-indigo-400 rounded-md hover:bg-indigo-500/20 transition-colors">
                 Go to Memory Galaxy
               </button>
            </div>
            
            <div className="bg-muted rounded-md p-4 border border-border flex flex-col items-center justify-center space-y-2 mt-4">
               <span className="text-2xl font-bold text-fuchsia-400">{brainStats?.total_documents || 0}</span>
               <span className="text-sm text-muted-foreground flex items-center gap-1"><BrainCircuit className="w-4 h-4"/> Docs Indexed</span>
               <button onClick={() => router.push('/brain')} className="w-full text-xs py-1.5 bg-fuchsia-500/10 text-fuchsia-400 rounded-md hover:bg-fuchsia-500/20 transition-colors">
                 Search Brain
               </button>
            </div>
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
