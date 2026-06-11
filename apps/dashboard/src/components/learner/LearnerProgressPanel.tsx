import { Activity, Clock, CheckCircle2, TrendingUp } from 'lucide-react';

interface Props {
  data: any | null;
  loading: boolean;
  onGenerate: () => void;
}

export function LearnerProgressPanel({ data, loading, onGenerate }: Props) {
  if (loading) {
    return <div className="p-8 text-center text-slate-500 animate-pulse">Loading progress...</div>;
  }

  if (!data || !data.progress) {
    return (
      <div className="flex flex-col items-center justify-center py-16 border border-white/5 border-dashed rounded-2xl">
        <Activity className="w-12 h-12 text-slate-600 mb-4" />
        <p className="text-slate-400 mb-6">No progress logged recently.</p>
        <button 
          onClick={onGenerate}
          className="px-4 py-2 bg-blue-500/20 text-blue-400 rounded-lg hover:bg-blue-500/30 transition-colors border border-blue-500/50"
        >
          Log Progress Now
        </button>
      </div>
    );
  }

  const p = data.progress;

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-bold text-white flex items-center gap-2">
        <Activity className="w-6 h-6 text-blue-400" /> Recent Progress
      </h2>

      <div className="grid grid-cols-3 gap-4 mb-6">
        <div className="bg-white/[0.02] border border-white/5 rounded-xl p-4">
          <Clock className="w-5 h-5 text-slate-400 mb-2" />
          <div className="text-2xl font-bold text-white">{p.time_spent_minutes}</div>
          <div className="text-xs text-slate-500 uppercase tracking-wider">Minutes Spent</div>
        </div>
        <div className="bg-white/[0.02] border border-white/5 rounded-xl p-4">
          <CheckCircle2 className="w-5 h-5 text-green-400 mb-2" />
          <div className="text-2xl font-bold text-white">{p.completed_tasks?.length || 0}</div>
          <div className="text-xs text-slate-500 uppercase tracking-wider">Completed Tasks</div>
        </div>
        <div className="bg-white/[0.02] border border-white/5 rounded-xl p-4">
          <TrendingUp className="w-5 h-5 text-blue-400 mb-2" />
          <div className="text-2xl font-bold text-white">{p.skipped_tasks?.length || 0}</div>
          <div className="text-xs text-slate-500 uppercase tracking-wider">Skipped Tasks</div>
        </div>
      </div>

      <div className="bg-white/[0.02] border border-white/5 rounded-xl p-6">
        <h3 className="font-semibold text-white mb-4">Completed Items</h3>
        {p.completed_tasks && p.completed_tasks.length > 0 ? (
          <ul className="space-y-2">
            {p.completed_tasks.map((task: string, idx: number) => (
              <li key={idx} className="flex items-center gap-2 text-sm text-slate-300">
                <CheckCircle2 className="w-4 h-4 text-green-400 shrink-0" />
                {task}
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-sm text-slate-500">None logged in this session.</p>
        )}
      </div>
    </div>
  );
}
