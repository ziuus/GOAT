'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { EmptyState } from '@/components/ui/States';
import { Users, Plus, Target } from 'lucide-react';

export default function SocializerPage() {
  return (
    <PageShell>
      <PageHeader 
        title={
          <div className="flex items-center gap-3">
            Socializer <span className="bg-amber-500/10 text-amber-500 text-xs px-2 py-0.5 rounded font-medium border border-amber-500/20">Experimental</span>
          </div>
        }
        subtitle="Ethical distribution, content strategy, and launch drafts."
        actions={
          <button disabled title="Coming soon" className="flex items-center gap-2 px-4 py-2 bg-slate-500/10 text-slate-500 rounded-lg text-sm font-medium border border-slate-500/20 cursor-not-allowed">
            <Plus className="w-4 h-4" /> New Campaign
          </button>
        }
      />
      
      <EmptyState 
        title="No Active Campaigns" 
        description="Create a distribution campaign to plan content and outreach." 
        icon={<Users className="w-12 h-12" />} 
      />
    </PageShell>
  );
}
