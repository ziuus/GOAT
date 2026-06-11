import React from 'react';

export function PageShell({ children, className = '' }: { children: React.ReactNode; className?: string }) {
  return (
    <div className={`min-h-screen bg-[#0A0A0A] text-slate-300 p-4 md:p-8 ${className}`}>
      <div className="max-w-7xl mx-auto space-y-8">
        {children}
      </div>
    </div>
  );
}
