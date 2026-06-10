'use client';

import { useState, useEffect } from 'react';
import { daemonFetch } from '@/lib/goat-api';
import { GitBranch, Loader2 } from 'lucide-react';

export default function DiffsPage() {
  const [diff, setDiff] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchDiff();
  }, []);

  const fetchDiff = async () => {
    try {
      const res = await daemonFetch('/v1/diffs');
      if (res.ok) {
        const data = await res.json();
        setDiff(data.diff);
      } else {
        const data = await res.json();
        setDiff(`[Error]: ${data.error}`);
      }
    } catch (e: any) {
      setDiff(`[Error]: ${e.message}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col h-[calc(100vh-4rem)] p-6 space-y-4">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Diffs Viewer</h1>
        <p className="text-muted-foreground">View uncommitted changes safely.</p>
      </div>

      <div className="flex-1 bg-card border border-border rounded-md flex flex-col min-h-0">
        <div className="p-3 border-b border-border bg-muted/30 font-medium text-sm flex items-center justify-between">
          <div className="flex items-center gap-2">
            <GitBranch className="w-4 h-4" />
            Workspace Git Diff
          </div>
          <button
            onClick={fetchDiff}
            className="text-xs bg-muted hover:bg-muted/80 px-2 py-1 rounded-md text-foreground border border-border"
          >
            Refresh
          </button>
        </div>
        <div className="flex-1 overflow-auto p-4 bg-[#0d1117] text-[#c9d1d9] font-mono text-sm whitespace-pre">
          {loading ? (
            <div className="flex items-center justify-center h-full">
              <Loader2 className="w-5 h-5 animate-spin" />
            </div>
          ) : !diff || diff.trim() === '' ? (
            <div className="text-muted-foreground">No changes in workspace.</div>
          ) : (
            diff.split('\n').map((line, i) => {
              let colorClass = '';
              let bgClass = '';
              if (line.startsWith('+') && !line.startsWith('+++')) {
                colorClass = 'text-[#3fb950]';
                bgClass = 'bg-[#2ea04326]';
              } else if (line.startsWith('-') && !line.startsWith('---')) {
                colorClass = 'text-[#f85149]';
                bgClass = 'bg-[#f8514926]';
              } else if (line.startsWith('@@')) {
                colorClass = 'text-[#d2a8ff]';
                bgClass = 'bg-[#388bfd1a]';
              }
              return (
                <div key={i} className={`px-2 ${colorClass} ${bgClass} ${!bgClass && 'hover:bg-[#161b22]'}`}>
                  {line || ' '}
                </div>
              );
            })
          )}
        </div>
      </div>
    </div>
  );
}
