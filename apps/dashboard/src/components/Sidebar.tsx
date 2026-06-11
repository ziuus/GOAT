'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Activity, Clock, TerminalSquare, Calendar, ScrollText, Settings, ShieldCheck, MessageSquare, FolderTree, GitBranch, Command, FileText, Sparkles, Wand2, Library, Workflow, BrainCircuit, Radio, Users } from 'lucide-react';

import pkg from '../../package.json';

const navItems = [
  { name: 'Overview', href: '/', icon: Activity },
  { name: 'Agents', href: '/agents', icon: Users },
  { name: 'Reports', href: '/reports', icon: FileText },
  { name: 'Cofounder', href: '/cofounder', icon: Sparkles },
  { name: 'Commands', href: '/commands', icon: Command },
  { name: 'Chat', href: '/chat', icon: MessageSquare },
  { name: 'Skill Directory', href: '/skills', icon: Library },
  { name: 'AI Studio', href: '/studio', icon: Wand2 },
  { name: 'Recipes', href: '/recipes', icon: Workflow },
  { name: 'Onboarding', href: '/onboarding', icon: Activity },
  { name: 'Project Profile', href: '/project', icon: FolderTree },
  { name: 'Repo Explorer', href: '/repo', icon: FolderTree },
  { name: 'Diffs', href: '/diffs', icon: GitBranch },
  { name: 'Approvals', href: '/approvals', icon: ShieldCheck },
  { name: 'Memory Galaxy', href: '/memory', icon: Sparkles },
  { name: 'Brain Search', href: '/brain', icon: BrainCircuit },
  { name: 'Timeline', href: '/timeline', icon: Calendar },
  { name: 'Audit Log', href: '/audit', icon: FileText },
  { name: 'Jobs', href: '/jobs', icon: TerminalSquare },
  { name: 'Schedule', href: '/schedule', icon: Calendar },
  { name: 'Hooks', href: '/hooks', icon: Clock },
  { name: 'GitHub Workflow', href: '/github', icon: GitBranch },
  { name: 'Browser QA', href: '/browser', icon: Activity },
  { name: 'Transports & Voice', href: '/transports', icon: Radio },
  { name: 'MCP & Tools', href: '/mcp', icon: TerminalSquare },
  { name: 'Logs', href: '/logs', icon: ScrollText },
  { name: 'Settings', href: '/settings', icon: Settings },
];

export default function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="w-64 border-r border-border bg-card flex flex-col">
      <div className="p-6 border-b border-border">
        <h1 className="text-lg font-bold tracking-tight">GOAT Dashboard <span className="text-xs font-normal text-muted-foreground ml-1">v{pkg.version}</span></h1>
        <p className="text-xs text-muted-foreground mt-1">General Omniscient Agentic Tool</p>
      </div>
      <nav className="flex-1 p-4 space-y-1">
        {navItems.map((item) => {
          const isActive = pathname === item.href;
          const Icon = item.icon;
          return (
            <Link
              key={item.name}
              href={item.href}
              className={`flex items-center gap-3 px-3 py-2 rounded-md text-sm transition-colors ${
                isActive
                  ? 'bg-primary/10 text-primary font-medium'
                  : 'text-muted-foreground hover:bg-muted hover:text-foreground'
              }`}
            >
              <Icon className="w-4 h-4" />
              {item.name}
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}
