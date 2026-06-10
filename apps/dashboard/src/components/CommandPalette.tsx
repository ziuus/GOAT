'use client';

import * as React from 'react';
import { useRouter } from 'next/navigation';
import { MessageSquare, FolderTree, GitBranch, ShieldCheck, Activity, Search, Command as CmdIcon, Settings } from 'lucide-react';
import { cn } from './ui/card';

export function CommandPalette() {
  const [open, setOpen] = React.useState(false);
  const [search, setSearch] = React.useState('');
  const router = useRouter();
  const inputRef = React.useRef<HTMLInputElement>(null);

  React.useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === 'k' && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen((open) => !open);
      }
      if (e.key === 'Escape' && open) {
        setOpen(false);
      }
    };

    document.addEventListener('keydown', down);
    return () => document.removeEventListener('keydown', down);
  }, [open]);

  React.useEffect(() => {
    if (open && inputRef.current) {
      inputRef.current.focus();
    }
  }, [open]);

  if (!open) return null;

  const items = [
    { name: 'Go to Overview', path: '/', icon: Activity },
    { name: 'Go to Chat', path: '/chat', icon: MessageSquare },
    { name: 'Go to Repo Explorer', path: '/repo', icon: FolderTree },
    { name: 'Go to Diffs', path: '/diffs', icon: GitBranch },
    { name: 'Go to Approvals', path: '/approvals', icon: ShieldCheck },
    { name: 'Go to Commands', path: '/commands', icon: CmdIcon },
    { name: 'Open Settings', path: '/settings', icon: Settings },
  ];

  const filtered = items.filter(i => i.name.toLowerCase().includes(search.toLowerCase()));

  return (
    <div className="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm flex items-start justify-center pt-[15vh]">
      <div className="w-full max-w-xl bg-card border border-border rounded-xl shadow-2xl overflow-hidden text-foreground animate-in fade-in zoom-in-95 duration-200">
        <div className="flex flex-col h-full w-full">
          <div className="flex items-center border-b border-border px-4 py-3 gap-2">
            <Search className="w-4 h-4 text-muted-foreground" />
            <input 
              ref={inputRef}
              value={search}
              onChange={e => setSearch(e.target.value)}
              placeholder="Type a command or search..." 
              className="flex-1 bg-transparent border-none outline-none text-sm placeholder:text-muted-foreground"
            />
            <kbd className="hidden sm:inline-flex items-center gap-1 px-1.5 py-0.5 rounded border border-border bg-muted text-[10px] font-medium text-muted-foreground">
              ESC
            </kbd>
          </div>
          
          <div className="max-h-[300px] overflow-y-auto p-2">
            {filtered.length === 0 ? (
              <div className="py-6 text-center text-sm text-muted-foreground">No results found.</div>
            ) : (
              <div className="text-xs font-medium text-muted-foreground mb-2 px-2 pt-2">
                {filtered.map(item => {
                  const Icon = item.icon;
                  return (
                    <button
                      key={item.path}
                      onClick={() => { router.push(item.path); setOpen(false); }}
                      className="w-full flex items-center gap-2 px-2 py-2 rounded-md text-sm text-foreground cursor-pointer hover:bg-muted focus:bg-muted outline-none transition-colors"
                    >
                      <Icon className="w-4 h-4" /> {item.name}
                    </button>
                  );
                })}
              </div>
            )}
          </div>
        </div>
        {/* Overlay to close when clicking outside */}
        <div className="fixed inset-0 -z-10" onClick={() => setOpen(false)} />
      </div>
    </div>
  );
}
