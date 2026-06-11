import React from 'react';
import { ShieldCheck, AlertTriangle } from 'lucide-react';

export function SafetyNotice({ protected: isProtected = true, children }: { protected?: boolean; children?: React.ReactNode }) {
  if (isProtected) {
    return (
      <div className="flex items-start gap-3 bg-emerald-500/5 border border-emerald-500/20 rounded-lg p-4 text-emerald-400/90 text-sm">
        <ShieldCheck className="w-5 h-5 shrink-0 mt-0.5" />
        <div>
          <p className="font-medium text-emerald-400">ApprovalGate Protected</p>
          <p className="opacity-80 mt-0.5 leading-relaxed">
            {children || 'Destructive actions and external network calls require your explicit approval before running.'}
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex items-start gap-3 bg-yellow-500/5 border border-yellow-500/20 rounded-lg p-4 text-yellow-500/90 text-sm">
      <AlertTriangle className="w-5 h-5 shrink-0 mt-0.5" />
      <div>
        <p className="font-medium text-yellow-500">Auto-Approve Enabled</p>
        <p className="opacity-80 mt-0.5 leading-relaxed">
          {children || 'Dangerous actions will execute automatically. Please proceed with extreme caution.'}
        </p>
      </div>
    </div>
  );
}

export function StatusBadge({ status, className = '' }: { status: string; className?: string }) {
  const getColors = (s: string) => {
    switch (s.toLowerCase()) {
      case 'online':
      case 'active':
      case 'running':
      case 'completed':
        return 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20';
      case 'offline':
      case 'error':
      case 'failed':
      case 'cancelled':
        return 'bg-red-500/10 text-red-400 border-red-500/20';
      case 'idle':
      case 'planned':
      case 'paused':
        return 'bg-slate-500/10 text-slate-400 border-slate-500/20';
      case 'waiting':
      case 'waitingforapproval':
        return 'bg-amber-500/10 text-amber-400 border-amber-500/20';
      default:
        return 'bg-indigo-500/10 text-indigo-400 border-indigo-500/20';
    }
  };

  return (
    <span className={`px-2 py-0.5 text-[10px] font-medium uppercase tracking-wider rounded border ${getColors(status)} ${className}`}>
      {status}
    </span>
  );
}
