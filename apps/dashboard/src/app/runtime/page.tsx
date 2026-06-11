'use client';

import { useEffect, useState } from 'react';
import { runtimeApi, getGoatConfig } from '@/lib/goat-api';
import { TerminalSquare, Play, Pause, XCircle, RotateCcw, AlertCircle, CheckCircle2, Activity, Clock } from 'lucide-react';
import { EmptyState } from '@/components/ui/empty-state';

export default function RuntimePage() {
  const [jobs, setJobs] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  const load = async () => {
    try {
      if (!getGoatConfig()) return;
      const data = await runtimeApi.listJobs();
      // sort by created_at descending
      setJobs((data.jobs || []).sort((a: any, b: any) => b.created_at - a.created_at));
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
    const int = setInterval(load, 2000);
    return () => clearInterval(int);
  }, []);

  const handleAction = async (jobId: string, action: string) => {
    try {
      if (action === 'start') await runtimeApi.startJob(jobId);
      if (action === 'pause') await runtimeApi.pauseJob(jobId);
      if (action === 'resume') await runtimeApi.resumeJob(jobId);
      if (action === 'cancel') await runtimeApi.cancelJob(jobId);
      if (action === 'retry') await runtimeApi.retryJob(jobId);
      load();
    } catch (e) {
      console.error('Action failed:', e);
    }
  };

  if (loading) return <div className="animate-pulse">Loading runtime...</div>;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Agent Execution Runtime</h1>
        <p className="text-muted-foreground">Manage active Prime Agent workflows and resilient background tasks.</p>
      </div>

      {jobs.length === 0 ? (
        <EmptyState 
          icon={TerminalSquare} 
          title="No agent jobs running" 
          description="Initiate a workflow with a Prime Agent to see it appear in the Execution Runtime."
        />
      ) : (
        <div className="border border-border rounded-lg overflow-hidden bg-card">
          <table className="w-full text-sm text-left">
            <thead className="bg-muted text-muted-foreground">
              <tr>
                <th className="px-6 py-3 font-medium">Job ID</th>
                <th className="px-6 py-3 font-medium">Agent</th>
                <th className="px-6 py-3 font-medium">Task</th>
                <th className="px-6 py-3 font-medium">Status</th>
                <th className="px-6 py-3 font-medium">Created</th>
                <th className="px-6 py-3 font-medium text-right">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-border">
              {jobs.map((job) => (
                <tr key={job.id} className="hover:bg-muted/50 transition-colors">
                  <td className="px-6 py-4 font-mono text-xs">{job.id.substring(0, 8)}...</td>
                  <td className="px-6 py-4 font-medium capitalize text-indigo-400">{job.agent_id}</td>
                  <td className="px-6 py-4">
                    <div className="max-w-xs truncate" title={job.task}>{job.task}</div>
                    <div className="text-[10px] text-muted-foreground mt-1 uppercase tracking-wider">{job.job_kind}</div>
                  </td>
                  <td className="px-6 py-4">
                    <StatusBadge status={job.status} />
                  </td>
                  <td className="px-6 py-4 text-muted-foreground">
                    {new Date(job.created_at * 1000).toLocaleString()}
                  </td>
                  <td className="px-6 py-4 text-right space-x-2">
                    {job.status === 'Pending' && (
                      <button onClick={() => handleAction(job.id, 'start')} className="p-1.5 hover:bg-white/10 rounded text-emerald-400" title="Start"><Play className="w-4 h-4" /></button>
                    )}
                    {job.status === 'Running' && (
                      <button onClick={() => handleAction(job.id, 'pause')} className="p-1.5 hover:bg-white/10 rounded text-yellow-400" title="Pause"><Pause className="w-4 h-4" /></button>
                    )}
                    {job.status === 'Paused' && (
                      <button onClick={() => handleAction(job.id, 'resume')} className="p-1.5 hover:bg-white/10 rounded text-emerald-400" title="Resume"><Play className="w-4 h-4" /></button>
                    )}
                    {(job.status === 'Running' || job.status === 'Pending' || job.status === 'Paused' || job.status === 'WaitingForApproval') && (
                      <button onClick={() => handleAction(job.id, 'cancel')} className="p-1.5 hover:bg-white/10 rounded text-destructive" title="Cancel"><XCircle className="w-4 h-4" /></button>
                    )}
                    {(job.status === 'Failed' || job.status === 'Cancelled') && (
                      <button onClick={() => handleAction(job.id, 'retry')} className="p-1.5 hover:bg-white/10 rounded text-blue-400" title="Retry"><RotateCcw className="w-4 h-4" /></button>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}

function StatusBadge({ status }: { status: string }) {
  let color = 'bg-muted text-muted-foreground';
  let Icon = TerminalSquare;
  let label = status;

  if (status === 'Completed') {
    color = 'bg-emerald-500/10 text-emerald-500 border border-emerald-500/20';
    Icon = CheckCircle2;
  } else if (status === 'Failed' || status === 'Cancelled') {
    color = 'bg-destructive/10 text-destructive border border-destructive/20';
    Icon = AlertCircle;
  } else if (status === 'Running') {
    color = 'bg-blue-500/10 text-blue-500 border border-blue-500/20';
    Icon = Activity;
  } else if (status === 'WaitingForApproval') {
    color = 'bg-yellow-500/10 text-yellow-500 border border-yellow-500/20';
    Icon = AlertCircle;
    label = 'Approval Needed';
  } else if (status === 'Paused') {
    color = 'bg-orange-500/10 text-orange-500 border border-orange-500/20';
    Icon = Pause;
  } else if (status === 'Pending') {
    color = 'bg-slate-500/10 text-slate-400 border border-slate-500/20';
    Icon = Clock;
  }

  return (
    <span className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium ${color}`}>
      <Icon className="w-3.5 h-3.5" />
      {label}
    </span>
  );
}
