'use client';

import { useState, useEffect } from 'react';
import { promptforgeApi, PromptForgeHistoryEntry } from '@/lib/goat-api';
import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { FeatureCard } from '@/components/ui/FeatureCard';
import { EmptyState, ErrorState, LoadingState } from '@/components/ui/States';
import { StatusBadge, SafetyNotice } from '@/components/ui/Status';
import { Wand2, Activity, List, Code, FileText, CheckCircle2 } from 'lucide-react';

export default function PromptForgePage() {
  const [status, setStatus] = useState<any>(null);
  const [config, setConfig] = useState<any>(null);
  const [history, setHistory] = useState<PromptForgeHistoryEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [prompt, setPrompt] = useState('');
  const [refinedPrompt, setRefinedPrompt] = useState('');
  const [refining, setRefining] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [sRes, cRes, hRes] = await Promise.all([
        promptforgeApi.getStatus(),
        promptforgeApi.getConfig(),
        promptforgeApi.getHistory()
      ]);
      setStatus(sRes);
      setConfig(cRes.config);
      setHistory(hRes.history || []);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleRefine = async () => {
    if (!prompt.trim()) return;
    try {
      setRefining(true);
      setError(null);
      const res = await promptforgeApi.refine({
        original_prompt: prompt,
        target_agent: 'user',
        target_format: config?.default_target || 'goat',
        domain: 'general',
        complexity: 'medium',
        safe_context: '',
        constraints: [],
        mode: config?.mode || 'mock',
      });
      if (res.result?.refined_prompt) {
        setRefinedPrompt(res.result.refined_prompt);
      } else if (res.error) {
        setError(res.error);
      }
      await loadData();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setRefining(false);
    }
  };

  if (loading && !status) return <PageShell><LoadingState title="Loading PromptForge" /></PageShell>;

  return (
    <PageShell>
      <PageHeader 
        title="PromptForge" 
        subtitle="Improve rough prompts before agents use them. PromptForge only refines text—it does not execute tasks."
      />

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <FeatureCard title="Status" icon={<Activity className="w-5 h-5" />}>
          <div className="mt-2">
            <StatusBadge status={status?.enabled ? 'Online' : 'Offline'} />
          </div>
        </FeatureCard>
        <FeatureCard title="Mode" icon={<Wand2 className="w-5 h-5" />}>
          <p className="mt-2 text-sm text-slate-300 capitalize">{status?.mode || 'Mock'}</p>
        </FeatureCard>
        <FeatureCard title="Auto-Refine" icon={<CheckCircle2 className="w-5 h-5" />}>
          <p className="mt-2 text-sm text-slate-300">{status?.auto_refine ? 'Enabled' : 'Disabled (Requires manual click)'}</p>
        </FeatureCard>
      </div>

      <SafetyNotice>
        PromptForge helps clarify intent. It cannot bypass the ApprovalGate or start agent tasks without your permission.
      </SafetyNotice>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6 space-y-4">
          <div className="flex items-center gap-2 text-sm font-semibold text-white">
            <FileText className="w-4 h-4 text-indigo-400" />
            Rough Draft
          </div>
          <textarea
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="Type your rough idea here..."
            className="w-full h-48 bg-black/50 border border-white/10 rounded-xl p-4 text-sm text-slate-300 focus:outline-none focus:border-indigo-500 resize-none transition-colors"
          />
          <button
            onClick={handleRefine}
            disabled={refining || !prompt.trim()}
            className="w-full py-3 bg-indigo-500/20 text-indigo-400 font-semibold rounded-xl hover:bg-indigo-500/30 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex justify-center items-center gap-2"
          >
            {refining ? <div className="w-4 h-4 border-2 border-indigo-400 border-t-transparent rounded-full animate-spin"></div> : <Wand2 className="w-4 h-4" />}
            {refining ? 'Refining...' : 'Improve Prompt'}
          </button>
        </div>

        <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6 space-y-4">
          <div className="flex items-center gap-2 text-sm font-semibold text-white">
            <Code className="w-4 h-4 text-emerald-400" />
            Refined Output
          </div>
          {error && <div className="p-3 bg-red-500/10 text-red-400 text-sm rounded-lg">{error}</div>}
          <div className={`w-full h-64 bg-black/50 border border-white/10 rounded-xl p-4 text-sm overflow-y-auto ${refinedPrompt ? 'text-slate-200 font-mono text-xs' : 'text-slate-500 flex items-center justify-center'}`}>
            {refinedPrompt || "The improved prompt will appear here."}
          </div>
        </div>
      </div>

      <section>
        <h2 className="text-sm font-bold uppercase tracking-wider text-slate-500 mb-4 px-1 flex items-center gap-2">
          <List className="w-4 h-4" /> History
        </h2>
        {history.length > 0 ? (
          <div className="space-y-3">
            {history.slice(0, 5).map((h, i) => (
              <div key={i} className="bg-white/[0.02] border border-white/5 p-4 rounded-xl flex flex-col gap-2 cursor-pointer hover:bg-white/[0.04]" onClick={() => { setPrompt(h.original_prompt); setRefinedPrompt(h.refined_prompt); }}>
                <div className="flex justify-between items-start">
                  <p className="text-sm text-slate-300 font-medium truncate max-w-lg">{h.original_prompt}</p>
                  <StatusBadge status={h.refined_prompt ? 'Online' : 'Waiting'} />
                </div>
              </div>
            ))}
          </div>
        ) : (
          <EmptyState title="No History" description="Your prompt refinement history will appear here." />
        )}
      </section>
    </PageShell>
  );
}
