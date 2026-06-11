'use client';

import React, { useState } from 'react';
import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { EmptyState } from '@/components/ui/States';
import { 
  Search, Plus, BookOpen, ShieldCheck, 
  Target, Zap, ChevronRight, FileText, Globe
} from 'lucide-react';

const DUMMY_PROJECTS = [
  {
    id: "proj-1",
    name: "LiteLLM vs Native Providers",
    question: "Should we use LiteLLM proxy or integrate providers directly?",
    type: "technology_comparison",
    sources: 12,
    claims: 8,
    status: "Completed",
    confidence: "High"
  },
  {
    id: "proj-2",
    name: "AI Agent Frameworks 2026",
    question: "What are the leading AI agent frameworks for desktop automation?",
    type: "competitor_research",
    sources: 34,
    claims: 15,
    status: "In Progress",
    confidence: "Medium"
  }
];

export default function ResearcherPage() {
  const [showNewModal, setShowNewModal] = useState(false);
  const [activeTab, setActiveTab] = useState("projects");
  const [selectedProject, setSelectedProject] = useState<any>(null);

  const handleNewProject = () => {
    setShowNewModal(true);
  };

  return (
    <PageShell>
      <PageHeader 
        title={
          <div className="flex items-center gap-3">
            Researcher <span className="bg-sky-500/10 text-sky-400 text-xs px-2 py-0.5 rounded font-medium border border-sky-500/20">Phase 7.5</span>
          </div>
        }
        subtitle="Source-grounded briefs, competitor scans, and evidence-backed comparisons."
        actions={
          <button 
            onClick={handleNewProject}
            className="flex items-center gap-2 px-4 py-2 bg-sky-500 hover:bg-sky-400 text-white rounded-lg text-sm font-medium transition-colors"
          >
            <Plus className="w-4 h-4" /> New Research Project
          </button>
        }
      />
      
      {!selectedProject ? (
        <div className="space-y-6">
          <div className="flex items-center gap-2 border-b border-white/5 pb-2">
            <button 
              onClick={() => setActiveTab('projects')}
              className={`px-4 py-2 text-sm font-medium rounded-lg transition-colors ${activeTab === 'projects' ? 'bg-white/10 text-white' : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'}`}
            >
              Active Projects
            </button>
            <button 
              onClick={() => setActiveTab('sources')}
              className={`px-4 py-2 text-sm font-medium rounded-lg transition-colors ${activeTab === 'sources' ? 'bg-white/10 text-white' : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'}`}
            >
              Global Source Library
            </button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {DUMMY_PROJECTS.map(proj => (
              <div 
                key={proj.id} 
                className="p-5 rounded-xl border border-white/10 bg-white/[0.02] hover:bg-white/[0.04] transition-colors cursor-pointer group"
                onClick={() => setSelectedProject(proj)}
              >
                <div className="flex items-start justify-between mb-3">
                  <div className="p-2 bg-sky-500/10 text-sky-400 rounded-lg">
                    {proj.type === 'technology_comparison' ? <Zap className="w-5 h-5" /> : <Target className="w-5 h-5" />}
                  </div>
                  <span className="text-xs font-medium px-2 py-1 bg-white/5 text-slate-300 rounded-full border border-white/10">
                    {proj.status}
                  </span>
                </div>
                <h3 className="text-lg font-medium text-white mb-1 group-hover:text-sky-400 transition-colors">{proj.name}</h3>
                <p className="text-sm text-slate-400 line-clamp-2 mb-4">{proj.question}</p>
                
                <div className="flex items-center gap-4 text-xs text-slate-500 border-t border-white/5 pt-4">
                  <div className="flex items-center gap-1">
                    <BookOpen className="w-3.5 h-3.5" />
                    {proj.sources} Sources
                  </div>
                  <div className="flex items-center gap-1">
                    <ShieldCheck className="w-3.5 h-3.5 text-emerald-400/70" />
                    {proj.claims} Claims
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      ) : (
        <div className="space-y-6">
          <button 
            onClick={() => setSelectedProject(null)}
            className="flex items-center gap-2 text-sm text-slate-400 hover:text-white transition-colors"
          >
            ← Back to Projects
          </button>
          
          <div className="p-6 rounded-2xl border border-white/10 bg-white/[0.02]">
            <div className="flex items-start justify-between mb-6">
              <div>
                <h2 className="text-2xl font-semibold text-white mb-2">{selectedProject.name}</h2>
                <p className="text-slate-400">{selectedProject.question}</p>
              </div>
              <div className="flex gap-2">
                <button className="px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white flex items-center gap-2">
                  <Globe className="w-4 h-4" /> Ingest Browser Artifact
                </button>
                <button className="px-3 py-1.5 bg-sky-500/10 hover:bg-sky-500/20 text-sky-400 border border-sky-500/20 rounded-lg text-sm font-medium flex items-center gap-2">
                  <FileText className="w-4 h-4" /> Generate Report
                </button>
              </div>
            </div>

            <div className="grid grid-cols-3 gap-6">
              <div className="col-span-2 space-y-4">
                <div className="p-4 rounded-xl border border-white/5 bg-black/20">
                  <h3 className="text-sm font-medium text-white mb-3 flex items-center gap-2">
                    <BookOpen className="w-4 h-4 text-sky-400" /> Source Material
                  </h3>
                  <div className="space-y-2">
                    <div className="p-3 rounded-lg border border-white/5 bg-white/[0.01] flex justify-between items-center">
                      <span className="text-sm text-slate-300">LiteLLM Documentation - Supported Models</span>
                      <span className="text-xs text-emerald-400 bg-emerald-400/10 px-2 py-0.5 rounded border border-emerald-400/20">Official</span>
                    </div>
                    <div className="p-3 rounded-lg border border-white/5 bg-white/[0.01] flex justify-between items-center">
                      <span className="text-sm text-slate-300">HackerNews Thread: "Moving off proxy providers"</span>
                      <span className="text-xs text-amber-400 bg-amber-400/10 px-2 py-0.5 rounded border border-amber-400/20">Community</span>
                    </div>
                  </div>
                </div>

                <div className="p-4 rounded-xl border border-white/5 bg-black/20">
                  <h3 className="text-sm font-medium text-white mb-3 flex items-center gap-2">
                    <ShieldCheck className="w-4 h-4 text-emerald-400" /> Evidence-Backed Claims
                  </h3>
                  <div className="space-y-3">
                    <div className="p-3 rounded-lg border border-white/5 bg-white/[0.01]">
                      <p className="text-sm text-slate-200 mb-2">"Direct integration reduces latency by an average of 15ms compared to proxy solutions."</p>
                      <div className="flex gap-2">
                        <span className="text-xs px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">Strong Evidence</span>
                        <span className="text-xs text-slate-500">2 Independent Sources</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div className="space-y-4">
                <div className="p-4 rounded-xl border border-emerald-500/10 bg-emerald-500/5">
                  <h3 className="text-sm font-medium text-emerald-400 mb-2">Trust & Safety</h3>
                  <ul className="text-xs text-slate-400 space-y-2">
                    <li className="flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-emerald-400"/> Zero fake citations detected</li>
                    <li className="flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-emerald-400"/> All claims source-grounded</li>
                    <li className="flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-amber-400"/> 1 conflicting opinion flagged</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </PageShell>
  );
}
