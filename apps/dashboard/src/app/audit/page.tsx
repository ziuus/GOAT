'use client';

import { useState, useEffect } from 'react';
import { daemonFetch } from '@/lib/goat-api';
import { Search, Loader2, ListFilter, CalendarClock, Shield } from 'lucide-react';

export default function AuditPage() {
  const [logs, setLogs] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [category, setCategory] = useState<string>('tool');
  const [searchTerm, setSearchTerm] = useState('');

  useEffect(() => {
    fetchAuditLogs();
  }, [category]);

  const fetchAuditLogs = async () => {
    setLoading(true);
    try {
      const res = await daemonFetch(`/v1/audit?category=${category}`);
      if (res.ok) {
        const data = await res.json();
        setLogs(data.audit || []);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const filteredLogs = logs.filter(l => l.toLowerCase().includes(searchTerm.toLowerCase()));

  return (
    <div className="flex flex-col h-[calc(100vh-4rem)] p-6 space-y-4">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Audit Explorer</h1>
        <p className="text-muted-foreground">Browse redacted system audit logs securely.</p>
      </div>

      <div className="flex flex-col md:flex-row gap-4">
        <div className="flex-1 flex gap-2">
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
            <input
              type="text"
              placeholder="Search audit events..."
              value={searchTerm}
              onChange={e => setSearchTerm(e.target.value)}
              className="w-full pl-9 pr-4 py-2 bg-background border border-border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
            />
          </div>
          <select
            value={category}
            onChange={e => setCategory(e.target.value)}
            className="px-3 py-2 bg-background border border-border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
          >
            <option value="tool">Tool Execution Logs</option>
            <option value="scheduler">Scheduler Logs</option>
          </select>
          <button
            onClick={fetchAuditLogs}
            className="px-4 py-2 bg-muted text-foreground border border-border rounded-md text-sm hover:bg-muted/80"
          >
            Refresh
          </button>
        </div>
      </div>

      <div className="flex-1 bg-card border border-border rounded-md overflow-hidden flex flex-col min-h-0">
        <div className="p-3 border-b border-border bg-muted/30 font-medium text-sm flex items-center gap-2">
          <ListFilter className="w-4 h-4" /> Log Entries ({filteredLogs.length})
        </div>
        <div className="flex-1 overflow-auto p-4 space-y-1 bg-[#0d1117] text-[#c9d1d9] font-mono text-sm">
          {loading ? (
            <div className="flex items-center justify-center h-full">
              <Loader2 className="w-6 h-6 animate-spin" />
            </div>
          ) : filteredLogs.length === 0 ? (
            <div className="text-muted-foreground text-center mt-10">No logs found matching criteria.</div>
          ) : (
            filteredLogs.map((log, i) => (
              <div key={i} className="py-1 px-2 hover:bg-[#161b22] rounded whitespace-pre-wrap break-all border-b border-[#21262d] pb-2 mb-2 last:border-0">
                {log}
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
