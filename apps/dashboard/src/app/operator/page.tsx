'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { EmptyState } from '@/components/ui/States';
import { TerminalSquare, Plus } from 'lucide-react';

export default function OperatorPage() {
  return (
    <PageShell>
      <PageHeader 
        title="Operator"
        subtitle="Health checks, logs, deployment plans, and rollbacks."
        actions={
          <button className="flex items-center gap-2 px-4 py-2 bg-indigo-500/10 text-indigo-400 rounded-lg text-sm font-medium border border-indigo-500/20 hover:bg-indigo-500/20 transition-colors">
            <Plus className="w-4 h-4" /> New Operation
          </button>
        }
      />
      
      <EmptyState 
        title="No Operations" 
        description="Plan a deployment or run a health check." 
        icon={<TerminalSquare className="w-12 h-12" />} 
      />
    </PageShell>
  );
}
