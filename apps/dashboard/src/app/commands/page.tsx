'use client';

import { useState, useEffect } from 'react';
import { daemonFetch } from '@/lib/goat-api';
import { Terminal, ShieldAlert, CheckCircle2, Loader2, Send } from 'lucide-react';

export default function CommandsPage() {
  const [command, setCommand] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<any>(null);

  const safeExamples = ['/status', '/jobs', '/schedule', '/hooks', '/mcp list', '/repo', '/context show'];

  const executeCommand = async (cmdToRun: string) => {
    if (!cmdToRun.trim() || loading) return;
    setLoading(true);
    setResult(null);
    setCommand(cmdToRun);

    try {
      const res = await daemonFetch('/v1/command', {
        method: 'POST',
        body: JSON.stringify({ command: cmdToRun }),
      });
      const data = await res.json();
      setResult({ status: res.status, data });
    } catch (e: any) {
      setResult({ status: 500, data: { error: e.message } });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col h-[calc(100vh-4rem)] p-6 space-y-4">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Command Center</h1>
        <p className="text-muted-foreground">Safely execute GOAT commands. Dangerous commands require approval.</p>
      </div>

      <div className="flex flex-col md:flex-row gap-4 flex-1 min-h-0">
        <div className="md:w-1/3 flex flex-col space-y-4">
          <div className="bg-card border border-border rounded-md p-4 space-y-3">
            <h2 className="text-sm font-medium flex items-center gap-2">
              <Terminal className="w-4 h-4" /> Execute Command
            </h2>
            <div className="flex gap-2">
              <input
                type="text"
                value={command}
                onChange={(e) => setCommand(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && executeCommand(command)}
                placeholder="/status"
                className="flex-1 px-3 py-2 bg-background border border-border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 font-mono"
              />
              <button
                onClick={() => executeCommand(command)}
                disabled={loading || !command.trim()}
                className="px-4 py-2 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
              >
                {loading ? <Loader2 className="w-4 h-4 animate-spin" /> : <Send className="w-4 h-4" />}
                Run
              </button>
            </div>
          </div>

          <div className="bg-card border border-border rounded-md p-4 flex-1">
            <h2 className="text-sm font-medium mb-3">Safe Examples</h2>
            <div className="flex flex-wrap gap-2">
              {safeExamples.map((cmd) => (
                <button
                  key={cmd}
                  onClick={() => executeCommand(cmd)}
                  className="px-2 py-1 bg-muted hover:bg-muted/80 border border-border rounded-md text-xs font-mono transition-colors"
                >
                  {cmd}
                </button>
              ))}
            </div>
            <div className="mt-4 text-xs text-muted-foreground bg-primary/5 p-3 rounded-md border border-primary/10">
              <span className="font-medium text-foreground">Security Note:</span> Destructive commands (e.g. /bash, /write) will not execute directly. They will generate an approval request that you can review in the Approvals tab.
            </div>
          </div>
        </div>

        <div className="md:w-2/3 bg-card border border-border rounded-md flex flex-col">
          <div className="p-3 border-b border-border bg-muted/30 font-medium text-sm">
            Execution Result
          </div>
          <div className="flex-1 p-4 overflow-auto font-mono text-sm">
            {!result ? (
              <div className="flex items-center justify-center h-full text-muted-foreground">
                Enter a command to see results
              </div>
            ) : result.data.approval_required ? (
              <div className="flex flex-col items-center justify-center h-full text-center space-y-3">
                <ShieldAlert className="w-12 h-12 text-yellow-500" />
                <div>
                  <h3 className="font-bold text-lg text-foreground">Approval Required</h3>
                  <p className="text-muted-foreground">{result.data.message}</p>
                  <div className="mt-4 p-3 bg-muted rounded-md inline-block">
                    ID: {result.data.approval_id}
                  </div>
                </div>
              </div>
            ) : result.status >= 400 ? (
              <div className="text-destructive whitespace-pre-wrap">
                Error: {JSON.stringify(result.data, null, 2)}
              </div>
            ) : (
              <div className="text-foreground whitespace-pre-wrap">
                <div className="flex items-center gap-2 text-green-500 mb-4 pb-2 border-b border-border/50">
                  <CheckCircle2 className="w-4 h-4" /> Success
                </div>
                {JSON.stringify(result.data, null, 2)}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
