'use client';

import { useEffect, useState } from 'react';
import { goatApi, getGoatConfig } from '@/lib/goat-api';
import { Server, Wrench } from 'lucide-react';

export default function MCPPage() {
  const [servers, setServers] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!getGoatConfig()) return;
    const load = async () => {
      try {
        const data = await goatApi.getMcpStatus();
        setServers(data.servers || []);
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    };
    load();
  }, []);

  if (loading) return <div className="animate-pulse">Loading MCP servers...</div>;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">MCP & Tools</h1>
        <p className="text-muted-foreground">Active Model Context Protocol servers and tool catalog.</p>
      </div>

      <div className="p-4 bg-muted/50 border border-border rounded-lg text-sm flex gap-3">
        <Wrench className="w-5 h-5 text-muted-foreground shrink-0" />
        <p className="text-muted-foreground">
          <strong className="text-foreground">Security Note:</strong> Tools remain approval-gated by the daemon. 
          Dangerous operations cannot be executed directly from this dashboard without ApprovalGate consent.
        </p>
      </div>

      {servers.length === 0 ? (
        <div className="p-12 border border-dashed border-border rounded-lg text-center text-muted-foreground">
          <Server className="w-12 h-12 mx-auto mb-4 opacity-50" />
          <p>No MCP servers active.</p>
        </div>
      ) : (
        <div className="space-y-4">
          <h2 className="font-semibold text-lg">Active Servers</h2>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            {servers.map((s, i) => (
              <div key={i} className="border border-border bg-card p-6 rounded-lg">
                <div className="flex justify-between mb-4">
                  <h3 className="font-bold">{s.name}</h3>
                  <span className="bg-emerald-500/10 text-emerald-500 text-xs px-2 py-1 rounded-full border border-emerald-500/20">Running</span>
                </div>
                <div className="space-y-2 text-sm text-muted-foreground">
                  <p>Tools Discovered: <span className="text-foreground">{s.tools_count || 0}</span></p>
                  <p>PID: <span className="text-foreground">{s.pid || 'Unknown'}</span></p>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
