'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Activity, Clock, TerminalSquare, Calendar, ScrollText, Settings, ShieldCheck } from 'lucide-react';

const navItems = [
  { name: 'Overview', href: '/', icon: Activity },
  { name: 'Approvals', href: '/approvals', icon: ShieldCheck },
  { name: 'Jobs', href: '/jobs', icon: TerminalSquare },
  { name: 'Schedule', href: '/schedule', icon: Calendar },
  { name: 'Hooks', href: '/hooks', icon: Clock },
  { name: 'MCP & Tools', href: '/mcp', icon: TerminalSquare },
  { name: 'Logs', href: '/logs', icon: ScrollText },
  { name: 'Settings', href: '/settings', icon: Settings },
];

export default function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="w-64 border-r border-border bg-card flex flex-col">
      <div className="p-6 border-b border-border">
        <h1 className="text-lg font-bold tracking-tight">GOAT Dashboard</h1>
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
