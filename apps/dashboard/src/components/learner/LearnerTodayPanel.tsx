import { ListTodo, CheckCircle2, PlayCircle, SkipForward } from 'lucide-react';

interface Props {
  data: any | null;
  loading: boolean;
  onGenerate: () => void;
}

export function LearnerTodayPanel({ data, loading, onGenerate }: Props) {
  if (loading) {
    return <div className="p-8 text-center text-slate-500 animate-pulse">Loading today's plan...</div>;
  }

  if (!data || !data.tasks || data.tasks.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-16 border border-white/5 border-dashed rounded-2xl">
        <ListTodo className="w-12 h-12 text-slate-600 mb-4" />
        <p className="text-slate-400 mb-6">No tasks scheduled for today.</p>
        <button 
          onClick={onGenerate}
          className="px-4 py-2 bg-blue-500/20 text-blue-400 rounded-lg hover:bg-blue-500/30 transition-colors border border-blue-500/50"
        >
          Generate Today's Plan
        </button>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-bold text-white flex items-center gap-2">
          <ListTodo className="w-6 h-6 text-blue-400" /> Focus for Today
        </h2>
        <span className="text-sm px-3 py-1 bg-white/5 rounded-full text-slate-300">
          Estimated: ~{data.tasks.reduce((acc: number, t: any) => acc + t.estimated_minutes, 0)} mins
        </span>
      </div>

      <div className="grid gap-4">
        {data.tasks.map((task: any, idx: number) => (
          <div key={idx} className="bg-white/[0.02] border border-white/5 rounded-2xl p-5 hover:border-white/20 transition-colors">
            <div className="flex justify-between items-start mb-3">
              <div>
                <h3 className="font-semibold text-white text-lg">{task.title}</h3>
                <p className="text-sm text-slate-400 mt-1">{task.description}</p>
              </div>
              <span className="shrink-0 text-xs px-2 py-1 bg-black rounded-md border border-white/10 text-slate-400">
                {task.estimated_minutes}m
              </span>
            </div>
            
            {task.resources && task.resources.length > 0 && (
              <div className="mt-4 pt-4 border-t border-white/5 flex gap-2 overflow-x-auto">
                {task.resources.map((r: string, i: number) => (
                  <span key={i} className="text-xs px-2 py-1 bg-blue-500/10 text-blue-300 rounded border border-blue-500/20 whitespace-nowrap">
                    {r}
                  </span>
                ))}
              </div>
            )}

            <div className="mt-5 flex gap-3">
              <button className="flex-1 flex items-center justify-center gap-2 bg-blue-500/10 hover:bg-blue-500/20 text-blue-400 py-2 rounded-lg transition-colors border border-blue-500/20 text-sm">
                <PlayCircle className="w-4 h-4" /> Start
              </button>
              <button className="flex-1 flex items-center justify-center gap-2 bg-green-500/10 hover:bg-green-500/20 text-green-400 py-2 rounded-lg transition-colors border border-green-500/20 text-sm">
                <CheckCircle2 className="w-4 h-4" /> Done
              </button>
              <button className="flex items-center justify-center gap-2 bg-white/5 hover:bg-white/10 text-slate-400 px-4 py-2 rounded-lg transition-colors text-sm">
                <SkipForward className="w-4 h-4" />
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
