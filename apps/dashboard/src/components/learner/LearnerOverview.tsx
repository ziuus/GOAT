import { Clock, Target, Compass, BookOpen } from 'lucide-react';

interface Props {
  goal: any;
  onNavigate: (tab: string) => void;
}

export function LearnerOverview({ goal, onNavigate }: Props) {
  return (
    <div className="space-y-6">
      <div className="grid grid-cols-2 gap-4">
        <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-5">
          <h3 className="text-sm font-medium text-slate-400 mb-1 flex items-center gap-2">
            <Target className="w-4 h-4" /> Target Level
          </h3>
          <p className="text-xl font-bold text-white capitalize">{goal.target_level}</p>
        </div>
        <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-5">
          <h3 className="text-sm font-medium text-slate-400 mb-1 flex items-center gap-2">
            <Clock className="w-4 h-4" /> Current Level
          </h3>
          <p className="text-xl font-bold text-white capitalize">{goal.current_level}</p>
        </div>
      </div>

      <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6">
        <h3 className="font-semibold text-white mb-4">Quick Actions</h3>
        <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
          <button onClick={() => onNavigate('roadmap')} className="p-4 border border-white/10 hover:border-blue-500/50 rounded-xl bg-black/40 flex flex-col items-center justify-center gap-2 transition-all group text-slate-300 hover:text-white">
            <Compass className="w-6 h-6 group-hover:text-blue-400 transition-colors" />
            <span className="text-sm font-medium">Roadmap</span>
          </button>
          <button onClick={() => onNavigate('today')} className="p-4 border border-white/10 hover:border-blue-500/50 rounded-xl bg-black/40 flex flex-col items-center justify-center gap-2 transition-all group text-slate-300 hover:text-white">
            <BookOpen className="w-6 h-6 group-hover:text-blue-400 transition-colors" />
            <span className="text-sm font-medium">Today's Plan</span>
          </button>
          {/* Add more quick links if needed */}
        </div>
      </div>
    </div>
  );
}
