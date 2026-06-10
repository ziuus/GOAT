'use client';

import { useEffect, useState } from 'react';
import { goatApi, getGoatConfig } from '@/lib/goat-api';
import { Calendar, PlayCircle } from 'lucide-react';
import { EmptyState } from '@/components/ui/empty-state';

export default function SchedulePage() {
  const [schedule, setSchedule] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!getGoatConfig()) return;
    const load = async () => {
      try {
        const data = await goatApi.getSchedule();
        setSchedule(data.tasks || []);
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    };
    load();
  }, []);

  if (loading) return <div className="animate-pulse">Loading schedule...</div>;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Schedule</h1>
        <p className="text-muted-foreground">Automated tasks running on intervals or cron expressions.</p>
      </div>

      {schedule.length === 0 ? (
        <EmptyState 
          icon={Calendar} 
          title="No scheduled tasks configured" 
          description="Edit your ~/.config/goat/goat.toml to add scheduled tasks."
        />
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {schedule.map((task, i) => (
            <div key={i} className="border border-border bg-card p-6 rounded-lg">
              <div className="flex items-start justify-between mb-4">
                <h3 className="font-semibold">{task.name || 'Unnamed Task'}</h3>
                {task.enabled !== false ? (
                  <span className="bg-emerald-500/10 text-emerald-500 text-xs px-2 py-1 rounded-full border border-emerald-500/20">Active</span>
                ) : (
                  <span className="bg-muted text-muted-foreground text-xs px-2 py-1 rounded-full">Disabled</span>
                )}
              </div>
              <p className="text-sm text-muted-foreground mb-4">{task.prompt}</p>
              
              <div className="space-y-2 text-sm">
                {task.cron && <p>Cron: <code className="bg-muted px-1.5 py-0.5 rounded">{task.cron}</code></p>}
                {task.interval_secs && <p>Interval: {task.interval_secs}s</p>}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
