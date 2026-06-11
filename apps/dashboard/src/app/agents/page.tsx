'use client';

import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { FeatureCard } from '@/components/ui/FeatureCard';
import { StatusBadge } from '@/components/ui/Status';
import { Sparkles, Users, Search, TerminalSquare, BookOpen, Layers, ArrowRight } from 'lucide-react';
import Link from 'next/link';

export default function AgentsOverview() {
  const agents = [
    {
      name: 'Cofounder',
      href: '/cofounder',
      icon: <Sparkles className="w-6 h-6" />,
      description: 'Idea validation, MVP scoping, and founder reports.',
      status: 'Online',
      bestFor: 'Planning new products',
      specialists: ['Product Manager', 'Business Analyst']
    },
    {
      name: 'Socializer',
      href: '/socializer',
      icon: <Users className="w-6 h-6" />,
      description: 'Ethical distribution, content strategy, and launch drafts.',
      status: 'Online',
      bestFor: 'Launch and marketing',
      specialists: ['Content Creator', 'SEO Specialist']
    },
    {
      name: 'Designer',
      href: '/designer',
      icon: <Layers className="w-6 h-6" />,
      description: 'UI/UX review, accessibility checks, and Builder handoffs.',
      status: 'Online',
      bestFor: 'Improving UI/UX',
      specialists: ['UX Critic', 'Accessibility Auditor']
    },
    {
      name: 'Researcher',
      href: '/researcher',
      icon: <Search className="w-6 h-6" />,
      description: 'Source-grounded briefs, competitor scans, and comparisons.',
      status: 'Online',
      bestFor: 'Deep dives and analysis',
      specialists: ['Data Miner', 'Tech Scout']
    },
    {
      name: 'Operator',
      href: '/operator',
      icon: <TerminalSquare className="w-6 h-6" />,
      description: 'Health checks, logs, deployment plans, and rollbacks.',
      status: 'Online',
      bestFor: 'DevOps and Reliability',
      specialists: ['SRE', 'Security Auditor']
    },
    {
      name: 'Learner',
      href: '/learner',
      icon: <BookOpen className="w-6 h-6" />,
      description: 'Roadmaps, daily plans, practice, and revision.',
      status: 'Online',
      bestFor: 'Skill acquisition',
      specialists: ['Tutor', 'Quiz Master']
    }
  ];

  return (
    <PageShell>
      <PageHeader 
        title="Prime Agents Command Center" 
        subtitle="Manage and direct your specialized high-level strategy agents."
      />

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {agents.map((agent) => (
          <div key={agent.name} className="flex flex-col bg-white/[0.02] border border-white/5 rounded-2xl overflow-hidden hover:bg-white/[0.04] transition-colors group">
            <div className="p-6 pb-4">
              <div className="flex justify-between items-start mb-4">
                <div className="p-3 bg-indigo-500/10 text-indigo-400 rounded-xl">
                  {agent.icon}
                </div>
                <StatusBadge status={agent.status} />
              </div>
              <h3 className="text-xl font-bold text-white mb-2">{agent.name}</h3>
              <p className="text-sm text-slate-400 line-clamp-2 min-h-[40px]">{agent.description}</p>
            </div>
            
            <div className="px-6 py-4 bg-black/20 mt-auto border-t border-white/5 space-y-3">
              <div className="flex justify-between items-center text-xs">
                <span className="text-slate-500 font-medium">Best for</span>
                <span className="text-slate-300">{agent.bestFor}</span>
              </div>
              <div className="flex justify-between items-start text-xs">
                <span className="text-slate-500 font-medium">Specialists</span>
                <div className="flex flex-wrap justify-end gap-1 max-w-[150px]">
                  {agent.specialists.map(s => (
                    <span key={s} className="bg-white/5 px-1.5 py-0.5 rounded text-[10px] text-slate-400">{s}</span>
                  ))}
                </div>
              </div>
            </div>

            <Link href={agent.href} className="w-full flex items-center justify-center gap-2 py-3 bg-indigo-500/10 text-indigo-400 text-sm font-semibold hover:bg-indigo-500/20 transition-colors">
              Open Workspace <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
            </Link>
          </div>
        ))}
      </div>
    </PageShell>
  );
}
