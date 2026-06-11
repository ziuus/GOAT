'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { EmptyState } from '@/components/ui/States';
import { Layers, Plus } from 'lucide-react';

export default function DesignerPage() {
  return (
    <PageShell>
      <PageHeader 
        title={
          <div className="flex items-center gap-3">
            Designer <span className="bg-amber-500/10 text-amber-500 text-xs px-2 py-0.5 rounded font-medium border border-amber-500/20">Experimental</span>
          </div>
        }
        subtitle="UI/UX review, accessibility checks, and Builder handoffs."
        actions={
          <button disabled title="Coming soon" className="flex items-center gap-2 px-4 py-2 bg-slate-500/10 text-slate-500 rounded-lg text-sm font-medium border border-slate-500/20 cursor-not-allowed">
            <Plus className="w-4 h-4" /> New UI Review
          </button>
        }
      />
      
      <EmptyState 
        title="No Reviews Yet" 
        description="Start a new UI/UX review to get feedback on accessibility and aesthetics." 
        icon={<Layers className="w-12 h-12" />} 
      />
    </PageShell>
  );
}
