'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { ShieldCheck, AlertTriangle, Send, History } from 'lucide-react';

export default function SocializerPage() {
  return (
    <PageShell>
      <PageHeader 
        title={
          <div className="flex items-center gap-3">
            Socializer <span className="bg-emerald-500/10 text-emerald-500 text-xs px-2 py-0.5 rounded font-medium border border-emerald-500/20">Ethical Engine</span>
          </div>
        }
        subtitle="Source-backed distribution assets, safety reviews, and community drafts."
        actions={
          <button className="flex items-center gap-2 px-4 py-2 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 rounded-lg text-sm font-medium border border-emerald-500/20 transition-colors">
            <Send className="w-4 h-4" /> Generate Draft
          </button>
        }
      />
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        {/* Left Column: Drafts & Content */}
        <div className="lg:col-span-2 space-y-6">
          <div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden shadow-sm">
            <div className="border-b border-slate-800 p-4 bg-slate-900/50 flex items-center justify-between">
              <h3 className="font-semibold text-slate-100 flex items-center gap-2">
                <History className="w-4 h-4 text-emerald-500" /> Recent Drafts
              </h3>
            </div>
            <div className="p-4 space-y-4">
              
              <div className="p-4 border border-slate-800 rounded-lg bg-slate-800/30">
                <div className="flex justify-between items-start mb-2">
                  <div className="flex items-center gap-2">
                    <span className="text-xs font-medium px-2 py-1 bg-blue-500/10 text-blue-400 rounded border border-blue-500/20">LinkedIn</span>
                    <span className="text-xs font-medium px-2 py-1 bg-slate-700 text-slate-300 rounded">Founder Story</span>
                  </div>
                  <span className="text-xs font-medium px-2 py-1 bg-emerald-500/10 text-emerald-400 rounded border border-emerald-500/20">Spam Risk: Low</span>
                </div>
                <h4 className="text-sm font-medium text-slate-200 mb-2">How we built the new GOAT engine</h4>
                <p className="text-sm text-slate-400 line-clamp-3">We spent the last 3 weeks ripping out our old orchestration layer. Here is what we learned about LLM routing and safety...</p>
              </div>

              <div className="p-4 border border-slate-800 rounded-lg bg-slate-800/30">
                <div className="flex justify-between items-start mb-2">
                  <div className="flex items-center gap-2">
                    <span className="text-xs font-medium px-2 py-1 bg-orange-500/10 text-orange-400 rounded border border-orange-500/20">Reddit</span>
                    <span className="text-xs font-medium px-2 py-1 bg-slate-700 text-slate-300 rounded">Architecture Dive</span>
                  </div>
                  <span className="text-xs font-medium px-2 py-1 bg-amber-500/10 text-amber-500 rounded border border-amber-500/20">Spam Risk: Medium</span>
                </div>
                <h4 className="text-sm font-medium text-slate-200 mb-2">Show r/rust: GOAT Agent Framework</h4>
                <p className="text-sm text-slate-400 line-clamp-3">Hey everyone, I wanted to share an open-source agent framework I've been working on. It uses a unique pattern for...</p>
              </div>

            </div>
          </div>
        </div>

        {/* Right Column: Safety Review */}
        <div className="space-y-6">
          <div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden shadow-sm">
            <div className="border-b border-slate-800 p-4 bg-slate-900/50">
              <h3 className="font-semibold text-slate-100 flex items-center gap-2">
                <ShieldCheck className="w-4 h-4 text-emerald-500" /> Safety & Ethics
              </h3>
            </div>
            <div className="p-4">
              <ul className="space-y-3">
                <li className="flex items-start gap-3">
                  <div className="p-1.5 bg-emerald-500/10 rounded-md shrink-0 mt-0.5">
                    <ShieldCheck className="w-4 h-4 text-emerald-400" />
                  </div>
                  <div>
                    <h4 className="text-sm font-medium text-slate-200">Source Backing</h4>
                    <p className="text-xs text-slate-400 mt-1">Claims must reference Researcher artifacts or codebase evidence.</p>
                  </div>
                </li>
                <li className="flex items-start gap-3">
                  <div className="p-1.5 bg-emerald-500/10 rounded-md shrink-0 mt-0.5">
                    <ShieldCheck className="w-4 h-4 text-emerald-400" />
                  </div>
                  <div>
                    <h4 className="text-sm font-medium text-slate-200">No Auto-DMs</h4>
                    <p className="text-xs text-slate-400 mt-1">Outreach is draft-only. No automated messaging allowed.</p>
                  </div>
                </li>
                <li className="flex items-start gap-3">
                  <div className="p-1.5 bg-amber-500/10 rounded-md shrink-0 mt-0.5">
                    <AlertTriangle className="w-4 h-4 text-amber-400" />
                  </div>
                  <div>
                    <h4 className="text-sm font-medium text-slate-200">Platform Etiquette</h4>
                    <p className="text-xs text-slate-400 mt-1">Active warning on Reddit posts containing direct product links.</p>
                  </div>
                </li>
              </ul>
            </div>
          </div>
        </div>

      </div>
    </PageShell>
  );
}
