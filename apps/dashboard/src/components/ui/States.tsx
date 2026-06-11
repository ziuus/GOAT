import React from 'react';

interface StatesProps {
  title?: string;
  description?: string;
  icon?: React.ReactNode;
  action?: React.ReactNode;
}

export function EmptyState({ title = 'No Data', description, icon, action }: StatesProps) {
  return (
    <div className="min-h-[400px] border border-white/5 border-dashed rounded-2xl flex flex-col items-center justify-center text-slate-500 p-8 text-center bg-black/20">
      {icon && <div className="mb-4 opacity-50">{icon}</div>}
      <h3 className="text-lg font-medium text-slate-300">{title}</h3>
      {description && <p className="text-sm mt-2 max-w-md">{description}</p>}
      {action && <div className="mt-6">{action}</div>}
    </div>
  );
}

export function LoadingState({ title = 'Loading...', description }: StatesProps) {
  return (
    <div className="min-h-[400px] rounded-2xl flex flex-col items-center justify-center text-slate-500 p-8 text-center">
      <div className="w-8 h-8 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin mb-4"></div>
      <h3 className="text-lg font-medium text-slate-300">{title}</h3>
      {description && <p className="text-sm mt-2 max-w-md">{description}</p>}
    </div>
  );
}

export function ErrorState({ title = 'Error', description, action }: StatesProps) {
  return (
    <div className="min-h-[400px] border border-red-500/20 bg-red-500/5 rounded-2xl flex flex-col items-center justify-center text-red-400 p-8 text-center">
      <h3 className="text-lg font-medium">{title}</h3>
      {description && <p className="text-sm mt-2 max-w-md opacity-80">{description}</p>}
      {action && <div className="mt-6">{action}</div>}
    </div>
  );
}
