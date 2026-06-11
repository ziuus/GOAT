'use client';

import React, { useState, useEffect } from 'react';
import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { FeatureCard } from '@/components/ui/FeatureCard';
import { EmptyState } from '@/components/ui/States';
import { StatusBadge } from '@/components/ui/Status';
import { Sparkles, Plus, Target, Presentation, Briefcase, ChevronRight, Activity, FileText } from 'lucide-react';
import { cofounderApi } from '@/lib/goat-api';

interface Idea {
  id: string;
  title: string;
  description: string;
  status: string;
  score: number;
  mvpScope: string;
  state?: string;
  target_audience?: string;
}

export default function CofounderPage() {
  const [ideas, setIdeas] = useState<Idea[]>([]);
  const [activeIdeaId, setActiveIdeaId] = useState<string | null>(null);
  const [isCreating, setIsCreating] = useState(false);
  const [newIdeaTitle, setNewIdeaTitle] = useState('');
  const [newIdeaDesc, setNewIdeaDesc] = useState('');
  const [isLoading, setIsLoading] = useState(true);
  const [isProcessing, setIsProcessing] = useState(false);

  useEffect(() => {
    loadIdeas();
  }, []);

  const loadIdeas = async () => {
    try {
      setIsLoading(true);
      const res = await cofounderApi.getIdeas();
      const loadedIdeas = res.ideas || [];
      // Map backend fields to UI if needed, assume backend state string is the status
      const mapped = loadedIdeas.map((i: any) => ({
        id: i.id,
        title: i.title,
        description: i.description,
        status: i.state || 'Draft',
        score: i.score || 0,
        mvpScope: i.mvpScope || '',
        ...i
      }));
      // Sort newest first
      mapped.sort((a: any, b: any) => (b.created_at || 0) - (a.created_at || 0));
      setIdeas(mapped);
      if (mapped.length > 0 && !activeIdeaId) {
        setActiveIdeaId(mapped[0].id);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setIsLoading(false);
    }
  };

  const activeIdea = ideas.find(i => i.id === activeIdeaId);

  const handleCreate = async () => {
    if (!newIdeaTitle) return;
    setIsProcessing(true);
    try {
      const res = await cofounderApi.createIdea({
        title: newIdeaTitle,
        description: newIdeaDesc,
        target_audience: 'General'
      });
      if (res.idea) {
        await loadIdeas();
        setActiveIdeaId(res.idea.id);
        setIsCreating(false);
        setNewIdeaTitle('');
        setNewIdeaDesc('');
      }
    } catch (e) {
      console.error(e);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleAction = async (actionFn: () => Promise<any>) => {
    setIsProcessing(true);
    try {
      await actionFn();
      await loadIdeas();
    } catch (e) {
      console.error(e);
    } finally {
      setIsProcessing(false);
    }
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
                disabled={isProcessing}
              />
              <textarea 
                placeholder="Short description..." 
                className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-indigo-500 resize-none h-20"
                value={newIdeaDesc}
                onChange={e => setNewIdeaDesc(e.target.value)}
                disabled={isProcessing}
              />
              <div className="flex gap-2">
                <button onClick={handleCreate} disabled={isProcessing} className="flex-1 bg-indigo-500/20 text-indigo-400 py-1.5 rounded-lg text-xs font-medium hover:bg-indigo-500/30 disabled:opacity-50">Save</button>
                <button onClick={() => setIsCreating(false)} disabled={isProcessing} className="flex-1 bg-white/5 text-slate-400 py-1.5 rounded-lg text-xs font-medium hover:bg-white/10 disabled:opacity-50">Cancel</button>
              </div>
            </div>
          )}

          {isLoading ? (
            <div className="p-4 text-center text-sm text-slate-500">Loading ideas...</div>
          ) : ideas.length === 0 && !isCreating ? (
            <div className="p-4 text-center text-sm text-slate-500">No ideas yet. Create one!</div>
          ) : (
            ideas.map(idea => (
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
            ))
          )}
        </div>
      </div>

      <div className="flex-1 h-screen overflow-y-auto p-8">
        {activeIdea ? (
          <div className="max-w-4xl mx-auto space-y-8">
            <PageHeader 
              title={activeIdea.title}
              subtitle={activeIdea.description}
              actions={
                <button 
                  onClick={() => handleAction(() => cofounderApi.validateIdea(activeIdea.id))}
                  disabled={isProcessing} 
                  className="flex items-center gap-2 px-4 py-2 bg-indigo-500/20 text-indigo-400 hover:bg-indigo-500/30 rounded-lg text-sm font-medium border border-indigo-500/20 transition-colors disabled:opacity-50"
                >
                  <Activity className="w-4 h-4" /> 
                  {isProcessing ? 'Working...' : 'Run Validation'}
                </button>
              }
            />

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <FeatureCard title="Idea Scorecard" icon={<Target className="w-5 h-5" />}>
                <div className="flex items-end gap-3 mt-2">
                  <span className="text-4xl font-bold text-white">{activeIdea.score || '--'}</span>
                  <span className="text-slate-500 mb-1">/ 100</span>
                </div>
                <div className="mt-4 space-y-2">
                  <button onClick={() => handleAction(() => cofounderApi.scoreIdea(activeIdea.id))} disabled={isProcessing} className="text-xs text-indigo-400 flex items-center gap-1 hover:underline disabled:opacity-50">
                    Generate Score <ChevronRight className="w-3 h-3" />
                  </button>
                </div>
              </FeatureCard>

              <FeatureCard title="MVP Scope" icon={<Briefcase className="w-5 h-5" />}>
                <p className="text-sm text-slate-300 leading-relaxed mt-2 whitespace-pre-wrap">
                  {activeIdea.mvpScope || 'No MVP scope generated yet.'}
                </p>
                <button 
                  onClick={() => handleAction(() => cofounderApi.generateMvp(activeIdea.id))}
                  disabled={isProcessing} 
                  className="mt-4 text-xs text-indigo-400 flex items-center gap-1 hover:underline disabled:opacity-50"
                >
                  Generate Scope <ChevronRight className="w-3 h-3" />
                </button>
              </FeatureCard>
            </div>

            <section className="space-y-4">
              <div className="flex items-center justify-between px-1">
                <h3 className="text-sm font-bold uppercase tracking-wider text-slate-500">Reports & Artifacts</h3>
                <button 
                  onClick={() => handleAction(() => cofounderApi.generateReport(activeIdea.id))}
                  disabled={isProcessing}
                  className="text-xs text-indigo-400 hover:underline disabled:opacity-50"
                >
                  Generate Report
                </button>
              </div>
              <div className="bg-white/[0.02] border border-white/5 rounded-xl p-6">
                <div className="flex items-center gap-2 mb-4 text-slate-300 font-medium">
                  <FileText className="w-4 h-4 text-indigo-400" />
                  Generated Content
                </div>
                <div className="text-sm text-slate-400">
                  Reports will be available in the Reports section once generated.
                </div>
              </div>
            </section>
          </div>
        ) : (
          !isLoading && (
            <div className="h-full flex items-center justify-center">
              <EmptyState 
                title="No Idea Selected" 
                description="Select an idea from the sidebar or create a new one to get started."
                action={
                  <button onClick={() => setIsCreating(true)} className="flex items-center gap-2 px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white rounded-lg text-sm font-medium transition-colors">
                    <Plus className="w-4 h-4" /> New Idea
                  </button>
                }
              />
            </div>
          )
        )}
      </div>
    </PageShell>
  );
}
