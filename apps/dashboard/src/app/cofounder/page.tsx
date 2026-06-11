'use client';

import React, { useState } from 'react';
import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { FeatureCard } from '@/components/ui/FeatureCard';
import { EmptyState } from '@/components/ui/States';
import { StatusBadge } from '@/components/ui/Status';
import { Sparkles, Plus, Target, Presentation, Briefcase, ChevronRight, Activity } from 'lucide-react';

const MOCK_IDEAS = [
  {
    id: 'idea-1',
    title: 'AI-Powered Code Reviewer',
    description: 'An autonomous agent that reviews pull requests and suggests fixes.',
    status: 'Validating',
    score: 85,
    mvpScope: 'Basic GitHub integration to comment on PRs with linting errors.',
    competitors: ['SonarQube', 'CodeRabbit', 'ReviewPad']
  }
];

export default function CofounderPage() {
  const [ideas, setIdeas] = useState(MOCK_IDEAS);
  const [activeIdeaId, setActiveIdeaId] = useState('idea-1');
  const [isCreating, setIsCreating] = useState(false);
  const [newIdeaTitle, setNewIdeaTitle] = useState('');
  const [newIdeaDesc, setNewIdeaDesc] = useState('');

  const activeIdea = ideas.find(i => i.id === activeIdeaId);

  const handleCreate = () => {
    if (!newIdeaTitle) return;
    const newIdea = {
      id: `idea-${Date.now()}`,
      title: newIdeaTitle,
      description: newIdeaDesc,
      status: 'Draft',
      score: 0,
      mvpScope: '',
      competitors: []
    };
    setIdeas([newIdea, ...ideas]);
    setActiveIdeaId(newIdea.id);
    setIsCreating(false);
    setNewIdeaTitle('');
    setNewIdeaDesc('');
  };

  return (
    <PageShell className="!p-0 !max-w-none flex h-full">
      <div className="w-80 border-r border-white/5 bg-[#0A0A0A] flex flex-col shrink-0 h-screen overflow-y-auto">
        <div className="p-6 border-b border-white/5 sticky top-0 bg-[#0A0A0A] z-10 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-indigo-500/10 text-indigo-400 rounded-lg">
              <Sparkles className="w-5 h-5" />
            </div>
            <span className="font-semibold text-white">Cofounder</span>
          </div>
          <button onClick={() => setIsCreating(true)} className="p-1.5 hover:bg-white/5 rounded-md text-slate-400 hover:text-white transition-colors">
            <Plus className="w-4 h-4" />
          </button>
        </div>

        <div className="p-4 space-y-2">
          {isCreating && (
            <div className="bg-white/[0.02] border border-white/10 rounded-xl p-4 space-y-3 mb-4">
              <input 
                autoFocus
                placeholder="Idea title..." 
                className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-indigo-500"
                value={newIdeaTitle}
                onChange={e => setNewIdeaTitle(e.target.value)}
              />
              <textarea 
                placeholder="Short description..." 
                className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-indigo-500 resize-none h-20"
                value={newIdeaDesc}
                onChange={e => setNewIdeaDesc(e.target.value)}
              />
              <div className="flex gap-2">
                <button onClick={handleCreate} className="flex-1 bg-indigo-500/20 text-indigo-400 py-1.5 rounded-lg text-xs font-medium hover:bg-indigo-500/30">Save</button>
                <button onClick={() => setIsCreating(false)} className="flex-1 bg-white/5 text-slate-400 py-1.5 rounded-lg text-xs font-medium hover:bg-white/10">Cancel</button>
              </div>
            </div>
          )}

          {ideas.map(idea => (
            <button
              key={idea.id}
              onClick={() => setActiveIdeaId(idea.id)}
              className={`w-full text-left p-3 rounded-xl border transition-all ${
                activeIdeaId === idea.id ? 'bg-indigo-500/10 border-indigo-500/50' : 'bg-white/[0.02] border-white/5 hover:border-white/20'
              }`}
            >
              <div className="flex justify-between items-start mb-1">
                <span className="font-medium text-white text-sm truncate pr-2">{idea.title}</span>
                <StatusBadge status={idea.status} />
              </div>
              <p className="text-xs text-slate-500 line-clamp-2">{idea.description}</p>
            </button>
          ))}
        </div>
      </div>

      <div className="flex-1 h-screen overflow-y-auto p-8">
        {activeIdea ? (
          <div className="max-w-4xl mx-auto space-y-8">
            <PageHeader 
              title={activeIdea.title}
              subtitle={activeIdea.description}
              actions={
                <button disabled title="Validation API coming soon" className="flex items-center gap-2 px-4 py-2 bg-slate-500/10 text-slate-500 rounded-lg text-sm font-medium border border-slate-500/20 cursor-not-allowed">
                  <Activity className="w-4 h-4" /> Run Validation (Soon)
                </button>
              }
            />

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <FeatureCard title="Idea Scorecard" icon={<Target className="w-5 h-5" />}>
                <div className="flex items-end gap-3 mt-2">
                  <span className="text-4xl font-bold text-white">{activeIdea.score}</span>
                  <span className="text-slate-500 mb-1">/ 100</span>
                </div>
                <div className="mt-4 space-y-2">
                  <div className="flex justify-between text-xs">
                    <span className="text-slate-400">Market Need</span>
                    <span className="text-emerald-400">Strong</span>
                  </div>
                  <div className="flex justify-between text-xs">
                    <span className="text-slate-400">Technical Feasibility</span>
                    <span className="text-amber-400">Medium</span>
                  </div>
                </div>
              </FeatureCard>

              <FeatureCard title="MVP Scope" icon={<Briefcase className="w-5 h-5" />}>
                <p className="text-sm text-slate-300 leading-relaxed mt-2">
                  {activeIdea.mvpScope || 'No MVP scope generated yet.'}
                </p>
                <button disabled title="Generation coming soon" className="mt-4 text-xs text-slate-500 flex items-center gap-1 cursor-not-allowed">
                  Generate Scope <ChevronRight className="w-3 h-3" />
                </button>
              </FeatureCard>
            </div>

            <section className="space-y-4">
              <h3 className="text-sm font-bold uppercase tracking-wider text-slate-500 px-1">Market Analysis</h3>
              <div className="bg-white/[0.02] border border-white/5 rounded-xl p-6">
                <div className="flex items-center gap-2 mb-4 text-slate-300 font-medium">
                  <Presentation className="w-4 h-4 text-indigo-400" /> Known Competitors
                </div>
                {activeIdea.competitors.length > 0 ? (
                  <div className="flex flex-wrap gap-2">
                    {activeIdea.competitors.map(c => (
                      <span key={c} className="bg-white/5 border border-white/10 px-3 py-1.5 rounded-lg text-sm text-slate-300">{c}</span>
                    ))}
                  </div>
                ) : (
                  <p className="text-sm text-slate-500">Run market analysis to find competitors.</p>
                )}
              </div>
            </section>
          </div>
        ) : (
          <EmptyState 
            title="No Idea Selected" 
            description="Select an idea from the sidebar or create a new one to begin validation."
            icon={<Sparkles className="w-12 h-12" />}
          />
        )}
      </div>
    </PageShell>
  );
}
