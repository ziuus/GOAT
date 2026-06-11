import { Code, Lightbulb, CheckCircle2, RefreshCw } from 'lucide-react';

interface Props {
  data: any | null;
  loading: boolean;
  onGenerate: () => void;
}

export function LearnerPracticePanel({ data, loading, onGenerate }: Props) {
  if (loading) {
    return <div className="p-8 text-center text-slate-500 animate-pulse">Generating practice task...</div>;
  }

  if (!data || !data.practice_task) {
    return (
      <div className="flex flex-col items-center justify-center py-16 border border-white/5 border-dashed rounded-2xl">
        <Code className="w-12 h-12 text-slate-600 mb-4" />
        <p className="text-slate-400 mb-6">No practice task generated yet.</p>
        <button 
          onClick={onGenerate}
          className="px-4 py-2 bg-blue-500/20 text-blue-400 rounded-lg hover:bg-blue-500/30 transition-colors border border-blue-500/50"
        >
          Generate Practice Task
        </button>
      </div>
    );
  }

  const task = data.practice_task;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-bold text-white flex items-center gap-2">
          <Code className="w-6 h-6 text-blue-400" /> Practice Task
        </h2>
        <button 
          onClick={onGenerate}
          className="flex items-center gap-2 text-xs px-3 py-1.5 bg-white/5 hover:bg-white/10 rounded-lg text-slate-300 transition-colors"
        >
          <RefreshCw className="w-3.5 h-3.5" /> Another Task
        </button>
      </div>

      <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6 relative overflow-hidden">
        <div className="absolute top-0 right-0 p-4 opacity-5 pointer-events-none">
          <Code className="w-32 h-32" />
        </div>

        <div className="relative z-10">
          <div className="mb-6">
            <h3 className="font-semibold text-white text-lg mb-2">Problem Statement</h3>
            <p className="text-slate-300 leading-relaxed">{task.problem_statement}</p>
          </div>

          {task.constraints && task.constraints.length > 0 && (
            <div className="mb-6">
              <h4 className="text-sm font-semibold text-slate-400 mb-2 uppercase tracking-wider">Constraints</h4>
              <ul className="list-disc list-inside space-y-1 text-slate-300 text-sm">
                {task.constraints.map((c: string, idx: number) => (
                  <li key={idx}>{c}</li>
                ))}
              </ul>
            </div>
          )}

          {task.hints && task.hints.length > 0 && (
            <div className="mb-8">
              <h4 className="text-sm font-semibold text-slate-400 mb-2 flex items-center gap-2 uppercase tracking-wider">
                <Lightbulb className="w-4 h-4 text-yellow-500" /> Hints
              </h4>
              <ul className="space-y-2 text-slate-300 text-sm">
                {task.hints.map((h: string, idx: number) => (
                  <li key={idx} className="bg-white/5 p-3 rounded-lg border border-white/5">{h}</li>
                ))}
              </ul>
            </div>
          )}

          <div className="flex gap-4 pt-4 border-t border-white/10">
            <button className="flex-1 flex items-center justify-center gap-2 bg-blue-500/10 hover:bg-blue-500/20 text-blue-400 py-3 rounded-xl transition-colors border border-blue-500/20 font-medium">
              <Code className="w-5 h-5" /> Open Workspace (TODO)
            </button>
            <button className="flex-1 flex items-center justify-center gap-2 bg-green-500/10 hover:bg-green-500/20 text-green-400 py-3 rounded-xl transition-colors border border-green-500/20 font-medium">
              <CheckCircle2 className="w-5 h-5" /> Mark Completed
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
