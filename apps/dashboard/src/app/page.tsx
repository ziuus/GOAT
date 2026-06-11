'use client';

import { useState, useEffect } from 'react';
import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { FeatureCard } from '@/components/ui/FeatureCard';
import { SafetyNotice } from '@/components/ui/Status';
import { ErrorState } from '@/components/ui/States';
import { Sparkles, Search, TerminalSquare, BookOpen, Layers, Activity, GitBranch, PlayCircle, AlertTriangle } from 'lucide-react';
import Link from 'next/link';
import { goatApi } from '@/lib/goat-api';

export default function Home() {
  const [daemonStatus, setDaemonStatus] = useState<'checking' | 'online' | 'offline'>('checking');
  const [mcpStatus, setMcpStatus] = useState<'idle' | 'connected'>('idle');

  useEffect(() => {
    const checkHealth = async () => {
      try {
        await goatApi.getHealth();
        setDaemonStatus('online');
        const mcp = await goatApi.getMcpStatus();
        if (mcp && mcp.servers && mcp.servers.length > 0) {
          setMcpStatus('connected');
        }
      } catch (e) {
        setDaemonStatus('offline');
      }
    };
    checkHealth();
  }, []);

  if (daemonStatus === 'offline') {
    return (
      <PageShell>
        <PageHeader 
          title="GOAT OS Offline" 
          subtitle="The local daemon is not running."
        />
        <ErrorState 
          title="Daemon Disconnected"
          description="GOAT requires its local Rust daemon to execute agents, access memory, and manage workflows securely."
          action={
            <button 
              onClick={() => alert("Run: cargo run --release -- daemon start")}
              className="px-4 py-2 bg-red-500/20 text-red-400 hover:bg-red-500/30 font-medium rounded-lg text-sm border border-red-500/30"
            >
              How to start the daemon
            </button>
          }
        />
      </PageShell>
    );
  }

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
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-500 mb-4 px-1">Start Here</h2>
            <div className="bg-gradient-to-r from-indigo-500/10 to-emerald-500/10 border border-white/10 rounded-xl p-6 mb-6 flex flex-col md:flex-row items-center justify-between gap-4">
              <div>
                <h3 className="text-lg font-semibold text-white flex items-center gap-2">
                  <PlayCircle className="w-5 h-5 text-indigo-400" /> Let's get started
                </h3>
                <p className="text-sm text-slate-400 mt-1">Configure your environment or dive straight into your first workflow.</p>
              </div>
              <div className="flex gap-3">
                <Link href="/onboarding" className="px-4 py-2 bg-white/5 border border-white/10 hover:bg-white/10 text-white rounded-lg text-sm font-medium transition-colors">
                  Setup & Doctor
                </Link>
                <Link href="/cofounder" className="px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white rounded-lg text-sm font-medium transition-colors">
                  Try Cofounder
                </Link>
              </div>
            </div>

            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-500 mb-4 px-1">Quick Actions</h2>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 mb-6">
              <Link href="/agentflow">
                <FeatureCard title="Start AgentFlow" description="Collaborate across multiple agents in a workflow." icon={<Layers className="w-5 h-5 text-indigo-400" />} className="h-full border-indigo-500/20 bg-indigo-500/5" />
              </Link>
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
            
            <h2 className="text-sm font-bold uppercase tracking-wider text-slate-500 mb-4 px-1">Active Collaborations</h2>
            <div className="bg-white/[0.02] border border-white/5 rounded-xl p-6 flex flex-col items-center justify-center min-h-[150px] text-slate-500 mb-6">
              <Activity className="w-8 h-8 mb-3 opacity-50" />
              <p>No active AgentFlow sessions.</p>
              <Link href="/agentflow" className="text-indigo-400 mt-2 text-sm hover:underline">View AgentFlow</Link>
            </div>

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
                <span className="text-xs bg-emerald-500/10 text-emerald-400 px-2 py-0.5 rounded uppercase tracking-wider font-semibold">
                  {daemonStatus === 'online' ? 'Online' : 'Checking'}
                </span>
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
                  <GitBranch className={`w-5 h-5 ${mcpStatus === 'connected' ? 'text-emerald-500' : 'text-slate-500'}`} />
                  <span className="text-sm font-medium text-white">GitHub MCP</span>
                </div>
                <span className={`text-xs px-2 py-0.5 rounded uppercase tracking-wider font-semibold ${mcpStatus === 'connected' ? 'bg-emerald-500/10 text-emerald-400' : 'bg-slate-500/10 text-slate-400'}`}>
                  {mcpStatus === 'connected' ? 'Connected' : 'Idle'}
                </span>
              </div>
            </div>
          </section>
        </div>
      </div>
    </PageShell>
  );
}
