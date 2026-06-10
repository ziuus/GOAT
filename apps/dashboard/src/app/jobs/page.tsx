'use client';

import { useEffect, useState } from 'react';
import { goatApi, getGoatConfig } from '@/lib/goat-api';
import { TerminalSquare, Clock, AlertCircle, CheckCircle2 } from 'lucide-react';

export default function JobsPage() {
  const [jobs, setJobs] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!getGoatConfig()) return;
    const load = async () => {
      try {
        const data = await goatApi.getJobs();
        setJobs(data.jobs || []);
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    };
    load();
    const int = setInterval(load, 5000);
    return () => clearInterval(int);
  }, []);

  if (loading) return <div className="animate-pulse">Loading jobs...</div>;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Background Jobs</h1>
        <p className="text-muted-foreground">Active and recent scheduled background tasks.</p>
      </div>

      {jobs.length === 0 ? (
        <div className="p-12 border border-dashed border-border rounded-lg text-center text-muted-foreground">
          <TerminalSquare className="w-12 h-12 mx-auto mb-4 opacity-50" />
          <p>No background jobs found.</p>
        </div>
      ) : (
        <div className="border border-border rounded-lg overflow-hidden bg-card">
          <table className="w-full text-sm text-left">
            <thead className="bg-muted text-muted-foreground">
              <tr>
                <th className="px-6 py-3 font-medium">Job ID</th>
                <th className="px-6 py-3 font-medium">Type</th>
                <th className="px-6 py-3 font-medium">Status</th>
                <th className="px-6 py-3 font-medium">Created</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-border">
              {jobs.map((job) => (
                <tr key={job.id} className="hover:bg-muted/50 transition-colors">
                  <td className="px-6 py-4 font-mono text-xs">{job.id}</td>
                  <td className="px-6 py-4 font-medium">{job.type}</td>
                  <td className="px-6 py-4">
                    <StatusBadge status={job.status} />
                  </td>
                  <td className="px-6 py-4 text-muted-foreground">
                    {new Date(job.created_at * 1000).toLocaleString()}
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
  let Icon = Clock;

  if (status === 'Completed') {
    color = 'bg-emerald-500/10 text-emerald-500 border border-emerald-500/20';
    Icon = CheckCircle2;
  } else if (status === 'Failed' || status === 'Cancelled') {
    color = 'bg-destructive/10 text-destructive border border-destructive/20';
    Icon = AlertCircle;
  } else if (status === 'Running') {
    color = 'bg-blue-500/10 text-blue-500 border border-blue-500/20';
    Icon = Activity;
  } else if (status === 'RequiresApproval') {
    color = 'bg-yellow-500/10 text-yellow-500 border border-yellow-500/20';
    Icon = AlertCircle;
  }

  return (
    <span className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium ${color}`}>
      <Icon className="w-3.5 h-3.5" />
      {status}
    </span>
  );
}

import { Activity } from 'lucide-react';
