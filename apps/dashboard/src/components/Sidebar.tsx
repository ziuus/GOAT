'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { 
  Activity, Clock, TerminalSquare, Calendar, ScrollText, 
  Settings, ShieldCheck, MessageSquare, FolderTree, GitBranch, 
  Command, FileText, Sparkles, Wand2, Library, Workflow, 
  BrainCircuit, Radio, Users, Search, BookOpen, Layers
} from 'lucide-react';

import pkg from '../../package.json';

const navGroups = [
  {
    label: "Core",
    items: [
      { name: 'Dashboard', href: '/', icon: Activity },
      { name: 'Chat', href: '/chat', icon: MessageSquare },
      { name: 'Command Palette', href: '/commands', icon: Command },
    ]
  },
  {
    label: "Agents",
    items: [
      { name: 'Prime Agents', href: '/agents', icon: Layers },
      { name: 'Cofounder', href: '/cofounder', icon: Sparkles },
      { name: 'Socializer', href: '/socializer', icon: Users },
      { name: 'Designer', href: '/designer', icon: Sparkles },
      { name: 'Researcher', href: '/researcher', icon: Search },
      { name: 'Operator', href: '/operator', icon: TerminalSquare },
      { name: 'Learner', href: '/learner', icon: BookOpen },
    ]
  },
  {
    label: "Intelligence",
    items: [
      { name: 'PromptForge', href: '/promptforge', icon: Wand2 },
      { name: 'Brain Search', href: '/brain', icon: BrainCircuit },
      { name: 'Memory Galaxy', href: '/memory', icon: Sparkles },
      { name: 'AI Studio', href: '/studio', icon: Wand2 },
    ]
  },
  {
    label: "Workflows",
    items: [
      { name: 'AgentFlow', href: '/agentflow', icon: Workflow },
      { name: 'Recipes', href: '/recipes', icon: Workflow },
      { name: 'Timeline', href: '/timeline', icon: Calendar },
      { name: 'Reports', href: '/reports', icon: FileText },
    ]
  },
  {
    label: "System",
    items: [
      { name: 'Approvals', href: '/approvals', icon: ShieldCheck },
      { name: 'Settings', href: '/settings', icon: Settings },
      { name: 'Logs', href: '/logs', icon: ScrollText },
    ]
  }
];

export default function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="w-64 border-r border-white/5 bg-[#050505] flex flex-col text-slate-300">
      <div className="p-6 border-b border-white/5 shrink-0">
        <h1 className="text-xl font-bold tracking-tight text-white flex items-center gap-2">
          <Layers className="w-5 h-5 text-indigo-500" />
          GOAT <span className="text-[10px] uppercase font-semibold text-indigo-400 bg-indigo-500/10 px-1.5 py-0.5 rounded ml-1">OS Alpha</span>
        </h1>
        <p className="text-xs text-slate-500 mt-2">Local-first AI Agent System</p>
      </div>

      <nav className="flex-1 overflow-y-auto p-4 space-y-6 scrollbar-none">
        {navGroups.map((group) => (
          <div key={group.label}>
            <h3 className="text-[10px] font-bold uppercase tracking-wider text-slate-600 mb-2 px-2">
              {group.label}
            </h3>
            <div className="space-y-0.5">
              {group.items.map((item) => {
                const isActive = pathname === item.href;
                const Icon = item.icon;
                return (
                  <Link
                    key={item.name}
                    href={item.href}
                    className={`flex items-center gap-3 px-2 py-1.5 rounded-lg text-sm font-medium transition-colors ${
                      isActive
                        ? 'bg-indigo-500/10 text-indigo-400'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-white/[0.02]'
                    }`}
                  >
                    <Icon className={`w-4 h-4 ${isActive ? 'text-indigo-400' : 'text-slate-500'}`} />
                    {item.name}
                  </Link>
                );
              })}
            </div>
          </div>
        ))}
      </nav>

      <div className="p-4 border-t border-white/5 shrink-0 bg-[#0A0A0A]">
        <div className="flex items-start gap-3 bg-emerald-500/5 border border-emerald-500/10 rounded-lg p-3">
          <ShieldCheck className="w-4 h-4 text-emerald-500 mt-0.5 shrink-0" />
          <div>
            <p className="text-xs font-medium text-emerald-400">ApprovalGate Active</p>
            <p className="text-[10px] text-emerald-500/70 mt-0.5">System is protected</p>
          </div>
        </div>
      </div>
    </aside>
  );
}
