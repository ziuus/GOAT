'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { EmptyState } from '@/components/ui/States';
import { Search, Plus } from 'lucide-react';

export default function ResearcherPage() {
  return (
    <PageShell>
      <PageHeader 
        title="Researcher"
        subtitle="Source-grounded briefs, competitor scans, and comparisons."
        actions={
          <button className="flex items-center gap-2 px-4 py-2 bg-indigo-500/10 text-indigo-400 rounded-lg text-sm font-medium border border-indigo-500/20 hover:bg-indigo-500/20 transition-colors">
            <Plus className="w-4 h-4" /> New Research Task
          </button>
        }
      />
      
      <EmptyState 
        title="No Research Tasks" 
        description="Start a new research task to gather insights." 
        icon={<Search className="w-12 h-12" />} 
      />
    </PageShell>
  );
}
