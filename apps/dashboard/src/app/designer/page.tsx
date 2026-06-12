'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { Search, History, Brush, LayoutPanelLeft, Accessibility, Palette } from 'lucide-react';

export default function DesignerPage() {
  return (
    <PageShell>
      <PageHeader 
        title={
          <div className="flex items-center gap-3">
            Designer <span className="bg-amber-500/10 text-amber-500 text-xs px-2 py-0.5 rounded font-medium border border-amber-500/20">Deep Quality</span>
          </div>
        }
        subtitle="Visual critique, accessibility reviews, design system audits, and Builder handoffs."
        actions={
          <button className="flex items-center gap-2 px-4 py-2 bg-amber-500/10 hover:bg-amber-500/20 text-amber-500 rounded-lg text-sm font-medium border border-amber-500/20 transition-colors">
            <Search className="w-4 h-4" /> Start Review
          </button>
        }
      />
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        {/* Left Column: Reviews & Handoffs */}
        <div className="lg:col-span-2 space-y-6">
          <div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden shadow-sm">
            <div className="border-b border-slate-800 p-4 bg-slate-900/50 flex items-center justify-between">
              <h3 className="font-semibold text-slate-100 flex items-center gap-2">
                <History className="w-4 h-4 text-amber-500" /> Recent Audits
              </h3>
            </div>
            <div className="p-4 space-y-4">
              
              <div className="p-4 border border-slate-800 rounded-lg bg-slate-800/30">
                <div className="flex justify-between items-start mb-2">
                  <div className="flex items-center gap-2">
                    <span className="text-xs font-medium px-2 py-1 bg-purple-500/10 text-purple-400 rounded border border-purple-500/20 flex items-center gap-1">
                      <Accessibility className="w-3 h-3" /> Accessibility Risk
                    </span>
                    <span className="text-xs font-medium px-2 py-1 bg-slate-700 text-slate-300 rounded">Dashboard UI</span>
                  </div>
                  <span className="text-xs font-medium px-2 py-1 bg-red-500/10 text-red-400 rounded border border-red-500/20">Risk: High</span>
                </div>
                <h4 className="text-sm font-medium text-slate-200 mb-2">Color contrast failures on secondary text</h4>
                <p className="text-sm text-slate-400 line-clamp-3">The secondary text color `text-slate-500` against the `bg-slate-900` background fails WCAG AA guidelines with a ratio of 2.8:1. Recommendation: increase lightness to `text-slate-400`.</p>
                <div className="mt-3 flex justify-end">
                  <button className="text-xs text-amber-400 hover:text-amber-300 font-medium">Create Builder Handoff &rarr;</button>
                </div>
              </div>

              <div className="p-4 border border-slate-800 rounded-lg bg-slate-800/30">
                <div className="flex justify-between items-start mb-2">
                  <div className="flex items-center gap-2">
                    <span className="text-xs font-medium px-2 py-1 bg-blue-500/10 text-blue-400 rounded border border-blue-500/20 flex items-center gap-1">
                      <LayoutPanelLeft className="w-3 h-3" /> Landing Page
                    </span>
                    <span className="text-xs font-medium px-2 py-1 bg-slate-700 text-slate-300 rounded">Visual Hierarchy</span>
                  </div>
                  <span className="text-xs font-medium px-2 py-1 bg-emerald-500/10 text-emerald-400 rounded border border-emerald-500/20">Risk: Low</span>
                </div>
                <h4 className="text-sm font-medium text-slate-200 mb-2">Hero section CTA spacing</h4>
                <p className="text-sm text-slate-400 line-clamp-3">The gap between the primary action button and secondary link in the hero section is too tight. Consider increasing the gap from 2 to 4 to improve touch target separation.</p>
                <div className="mt-3 flex justify-end">
                  <button className="text-xs text-amber-400 hover:text-amber-300 font-medium">Create Builder Handoff &rarr;</button>
                </div>
              </div>

            </div>
          </div>
        </div>

        {/* Right Column: Review Criteria */}
        <div className="space-y-6">
          <div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden shadow-sm">
            <div className="border-b border-slate-800 p-4 bg-slate-900/50">
              <h3 className="font-semibold text-slate-100 flex items-center gap-2">
                <Brush className="w-4 h-4 text-amber-500" /> Quality Guidelines
              </h3>
            </div>
            <div className="p-4">
              <ul className="space-y-4">
                <li className="flex items-start gap-3">
                  <div className="p-1.5 bg-amber-500/10 rounded-md shrink-0 mt-0.5">
                    <Accessibility className="w-4 h-4 text-amber-400" />
                  </div>
                  <div>
                    <h4 className="text-sm font-medium text-slate-200">No Fake WCAG</h4>
                    <p className="text-xs text-slate-400 mt-1">Designer identifies "risks", not compliance. It relies on DOM mapping and vision models.</p>
                  </div>
                </li>
                <li className="flex items-start gap-3">
                  <div className="p-1.5 bg-amber-500/10 rounded-md shrink-0 mt-0.5">
                    <Search className="w-4 h-4 text-amber-400" />
                  </div>
                  <div>
                    <h4 className="text-sm font-medium text-slate-200">Visual Evidence</h4>
                    <p className="text-xs text-slate-400 mt-1">Critiques without screenshots explicitly state their limitations.</p>
                  </div>
                </li>
                <li className="flex items-start gap-3">
                  <div className="p-1.5 bg-blue-500/10 rounded-md shrink-0 mt-0.5">
                    <Palette className="w-4 h-4 text-blue-400" />
                  </div>
                  <div>
                    <h4 className="text-sm font-medium text-slate-200">Builder Handoffs</h4>
                    <p className="text-xs text-slate-400 mt-1">Designer plans layout fixes; Builder writes the code.</p>
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
