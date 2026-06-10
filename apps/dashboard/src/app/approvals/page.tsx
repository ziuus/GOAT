'use client';

import { useEffect, useState } from 'react';
import { ShieldCheck, ShieldAlert, Check, X, ShieldQuestion } from 'lucide-react';
import { goatApi } from '@/lib/goat-api';

export default function ApprovalsPage() {
  const [approvals, setApprovals] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  const fetchApprovals = async () => {
    try {
      const data = await goatApi.getApprovals();
      setApprovals(data.approvals || []);
      setError('');
    } catch (err: any) {
      setError(err.message || 'Failed to fetch approvals');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchApprovals();
    const interval = setInterval(fetchApprovals, 3000); // Poll for now as fallback
    return () => clearInterval(interval);
  }, []);

  const handleApprove = async (id: string) => {
    try {
      await goatApi.approveRequest(id);
      fetchApprovals();
    } catch (err: any) {
      alert(`Approval failed: ${err.message}`);
    }
  };

  const handleDeny = async (id: string) => {
    try {
      await goatApi.denyRequest(id);
      fetchApprovals();
    } catch (err: any) {
      alert(`Deny failed: ${err.message}`);
    }
  };

  const getRiskIcon = (risk: string) => {
    switch (risk.toLowerCase()) {
      case 'low': return <ShieldCheck className="w-5 h-5 text-green-500" />;
      case 'high': return <ShieldAlert className="w-5 h-5 text-red-500" />;
      default: return <ShieldQuestion className="w-5 h-5 text-yellow-500" />;
    }
  };

  const getRiskColor = (risk: string) => {
    switch (risk.toLowerCase()) {
      case 'low': return 'bg-green-500/10 text-green-500 border-green-500/20';
      case 'high': return 'bg-red-500/10 text-red-500 border-red-500/20';
      default: return 'bg-yellow-500/10 text-yellow-500 border-yellow-500/20';
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Approval Queue</h1>
          <p className="text-muted-foreground mt-2">Manage pending operations requesting elevated permissions.</p>
        </div>
      </div>

      {error && (
        <div className="p-4 bg-red-500/10 text-red-500 border border-red-500/20 rounded-lg">
          {error}
        </div>
      )}

      {loading ? (
        <div className="animate-pulse space-y-4">
          {[1, 2, 3].map(i => (
            <div key={i} className="h-24 bg-card rounded-xl border border-border" />
          ))}
        </div>
      ) : approvals.length === 0 ? (
        <div className="p-8 text-center bg-card rounded-xl border border-border text-muted-foreground">
          <ShieldCheck className="w-12 h-12 mx-auto mb-4 opacity-50" />
          <p>No pending approvals in the queue.</p>
        </div>
      ) : (
        <div className="space-y-4">
          {approvals.map((item) => (
            <div key={item.id} className="p-6 bg-card rounded-xl border border-border flex items-start justify-between gap-6 hover:border-primary/50 transition-colors">
              <div className="flex-1 space-y-4">
                <div className="flex items-center gap-3">
                  {getRiskIcon(item.request.risk_level)}
                  <h3 className="font-medium text-lg">{item.request.tool_name}</h3>
                  <span className={`px-2.5 py-0.5 rounded-full text-xs font-medium border ${getRiskColor(item.request.risk_level)}`}>
                    {item.request.risk_level.toUpperCase()} RISK
                  </span>
                  <span className="text-xs text-muted-foreground ml-auto bg-muted px-2 py-1 rounded">Source: {item.source}</span>
                </div>
                
                <div className="text-sm text-foreground bg-muted p-3 rounded-md font-mono border border-border/50">
                  {item.request.action_summary}
                </div>
              </div>

              <div className="flex flex-col gap-2">
                <button
                  onClick={() => handleApprove(item.id)}
                  className="flex items-center justify-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg font-medium hover:bg-primary/90 transition-colors"
                >
                  <Check className="w-4 h-4" /> Approve
                </button>
                <button
                  onClick={() => handleDeny(item.id)}
                  className="flex items-center justify-center gap-2 px-4 py-2 bg-destructive/10 text-destructive border border-destructive/20 rounded-lg font-medium hover:bg-destructive/20 transition-colors"
                >
                  <X className="w-4 h-4" /> Deny
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
