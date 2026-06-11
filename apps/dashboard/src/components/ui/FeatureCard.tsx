import React from 'react';

interface FeatureCardProps {
  title: string;
  description?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  className?: string;
  onClick?: () => void;
}

export function FeatureCard({ title, description, icon, children, className = '', onClick }: FeatureCardProps) {
  const isClickable = !!onClick;
  return (
    <div 
      onClick={onClick}
      className={`bg-white/[0.02] border border-white/5 rounded-xl p-5 md:p-6 transition-all ${isClickable ? 'cursor-pointer hover:bg-white/[0.04] hover:border-white/10 active:scale-[0.99]' : ''} ${className}`}
    >
      <div className="flex items-start gap-4">
        {icon && (
          <div className="p-2.5 bg-indigo-500/10 text-indigo-400 rounded-lg shrink-0">
            {icon}
          </div>
        )}
        <div className="flex-1 min-w-0">
          <h3 className="font-semibold text-white text-base truncate">{title}</h3>
          {description && <p className="text-sm text-slate-400 mt-1 line-clamp-2">{description}</p>}
          {children && <div className="mt-4">{children}</div>}
        </div>
      </div>
    </div>
  );
}
