'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { EmptyState } from '@/components/ui/States';
import { Layers, Plus } from 'lucide-react';

export default function DesignerPage() {
  return (
    <PageShell>
      <PageHeader 
        title="Designer"
        subtitle="UI/UX review, accessibility checks, and Builder handoffs."
        actions={
          <button className="flex items-center gap-2 px-4 py-2 bg-indigo-500/10 text-indigo-400 rounded-lg text-sm font-medium border border-indigo-500/20 hover:bg-indigo-500/20 transition-colors">
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
