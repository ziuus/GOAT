'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { EmptyState } from '@/components/ui/States';
import { Users, Plus, Target } from 'lucide-react';

export default function SocializerPage() {
  return (
    <PageShell>
      <PageHeader 
        title="Socializer"
        subtitle="Ethical distribution, content strategy, and launch drafts."
        actions={
          <button className="flex items-center gap-2 px-4 py-2 bg-indigo-500/10 text-indigo-400 rounded-lg text-sm font-medium border border-indigo-500/20 hover:bg-indigo-500/20 transition-colors">
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
