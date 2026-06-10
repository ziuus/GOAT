'use client';

import { useEffect, useState } from 'react';
import { goatApi, getGoatConfig } from '@/lib/goat-api';
import { Clock } from 'lucide-react';
import { EmptyState } from '@/components/ui/empty-state';

export default function HooksPage() {
  const [hooks, setHooks] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!getGoatConfig()) return;
    const load = async () => {
      try {
        const data = await goatApi.getHooks();
        setHooks(data.hooks || []);
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    };
    load();
  }, []);

  if (loading) return <div className="animate-pulse">Loading hooks...</div>;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Hooks</h1>
        <p className="text-muted-foreground">Automated triggers executing before or after events.</p>
      </div>

      {hooks.length === 0 ? (
        <EmptyState 
          icon={Clock} 
          title="No hooks configured" 
          description="Automated triggers will appear here once configured."
        />
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {hooks.map((hook, i) => (
            <div key={i} className="border border-border bg-card p-6 rounded-lg">
              <div className="flex items-start justify-between mb-4">
                <div>
                  <h3 className="font-semibold">{hook.name || 'Unnamed Hook'}</h3>
                  {hook.source_recipe && (
                    <div className="mt-1">
                      <span className="px-1.5 py-0.5 rounded text-[10px] uppercase font-bold bg-violet-500/10 text-violet-500 border border-violet-500/20">
                        {hook.source_recipe}
                      </span>
                    </div>
                  )}
                </div>
                {hook.enabled !== false ? (
                  <span className="bg-emerald-500/10 text-emerald-500 text-xs px-2 py-1 rounded-full border border-emerald-500/20">Active</span>
                ) : (
                  <span className="bg-muted text-muted-foreground text-xs px-2 py-1 rounded-full">Disabled</span>
                )}
              </div>
              <div className="space-y-2 text-sm">
                <p>Event: <span className="text-foreground">{hook.event}</span></p>
                <p className="text-muted-foreground">{hook.action_prompt}</p>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
