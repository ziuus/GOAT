'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { FeatureCard } from '@/components/ui/FeatureCard';
import { SafetyNotice } from '@/components/ui/Status';
import { Sparkles, Users, Search, TerminalSquare, BookOpen, Layers, Activity, Calendar, GitBranch } from 'lucide-react';
import Link from 'next/link';

export default function Home() {
  return (
    <PageShell>
      <PageHeader 
        title="Welcome to GOAT OS" 
        subtitle="Your local-first AI Agent Operating System for building, learning, research, and workflows."
      />

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="md:col-span-2 space-y-6">
          <SafetyNotice />

          <section>
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-500 mb-4 px-1">Quick Actions</h2>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <Link href="/cofounder">
                <FeatureCard title="Validate an idea" description="Use Cofounder to plan MVPs and features." icon={<Sparkles className="w-5 h-5" />} className="h-full" />
              </Link>
              <Link href="/learner">
                <FeatureCard title="Plan learning" description="Generate a structured roadmap to learn a new skill." icon={<BookOpen className="w-5 h-5" />} className="h-full" />
              </Link>
              <Link href="/designer">
                <FeatureCard title="Review UI design" description="Get feedback on UX, accessibility, and aesthetics." icon={<Layers className="w-5 h-5" />} className="h-full" />
              </Link>
              <Link href="/researcher">
                <FeatureCard title="Research a topic" description="Gather source-grounded insights and competitor scans." icon={<Search className="w-5 h-5" />} className="h-full" />
              </Link>
            </div>
          </section>

          <section>
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-500 mb-4 px-1">Recent Activity</h2>
            <div className="bg-white/[0.02] border border-white/5 rounded-xl p-6 flex flex-col items-center justify-center min-h-[200px] text-slate-500">
              <Activity className="w-8 h-8 mb-3 opacity-50" />
              <p>No recent activity today.</p>
              <Link href="/timeline" className="text-indigo-400 mt-2 text-sm hover:underline">View full timeline</Link>
            </div>
          </section>
        </div>

        <div className="space-y-6">
          <section>
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-500 mb-4 px-1">System Health</h2>
            <div className="space-y-3">
              <div className="bg-white/[0.02] border border-white/5 rounded-xl p-4 flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <Activity className="w-5 h-5 text-emerald-500" />
                  <span className="text-sm font-medium text-white">Daemon</span>
                </div>
                <span className="text-xs bg-emerald-500/10 text-emerald-400 px-2 py-0.5 rounded uppercase tracking-wider font-semibold">Online</span>
              </div>
              <div className="bg-white/[0.02] border border-white/5 rounded-xl p-4 flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <TerminalSquare className="w-5 h-5 text-emerald-500" />
                  <span className="text-sm font-medium text-white">ToolRegistry</span>
                </div>
                <span className="text-xs bg-emerald-500/10 text-emerald-400 px-2 py-0.5 rounded uppercase tracking-wider font-semibold">Ready</span>
              </div>
              <div className="bg-white/[0.02] border border-white/5 rounded-xl p-4 flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <GitBranch className="w-5 h-5 text-slate-500" />
                  <span className="text-sm font-medium text-white">GitHub MCP</span>
                </div>
                <span className="text-xs bg-slate-500/10 text-slate-400 px-2 py-0.5 rounded uppercase tracking-wider font-semibold">Idle</span>
              </div>
            </div>
          </section>
        </div>
      </div>
    </PageShell>
  );
}
