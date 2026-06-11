import { FileText, Download, Target, ShieldAlert } from 'lucide-react';

interface Props {
  data: any | null;
  loading: boolean;
  onGenerate: () => void;
}

export function LearnerReportPanel({ data, loading, onGenerate }: Props) {
  if (loading) {
    return <div className="p-8 text-center text-slate-500 animate-pulse">Generating learning report...</div>;
  }

  if (!data || !data.report) {
    return (
      <div className="flex flex-col items-center justify-center py-16 border border-white/5 border-dashed rounded-2xl">
        <FileText className="w-12 h-12 text-slate-600 mb-4" />
        <p className="text-slate-400 mb-6">No report generated for this goal.</p>
        <button 
          onClick={onGenerate}
          className="px-4 py-2 bg-blue-500/20 text-blue-400 rounded-lg hover:bg-blue-500/30 transition-colors border border-blue-500/50"
        >
          Generate Report
        </button>
      </div>
    );
  }

  const r = data.report;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-bold text-white flex items-center gap-2">
          <FileText className="w-6 h-6 text-blue-400" /> Learning Report
        </h2>
        <button className="flex items-center gap-2 text-xs px-3 py-1.5 bg-white/5 hover:bg-white/10 rounded-lg text-slate-300 transition-colors border border-white/10">
          <Download className="w-3.5 h-3.5" /> Export PDF (TODO)
        </button>
      </div>

      <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6 space-y-6">
        <div>
          <h3 className="font-semibold text-white mb-2">Summary</h3>
          <p className="text-slate-300 text-sm leading-relaxed">{r.summary}</p>
        </div>

        <div className="grid md:grid-cols-2 gap-6 pt-4 border-t border-white/10">
          <div>
            <h4 className="text-sm font-semibold text-green-400 mb-3 flex items-center gap-2">
              <Target className="w-4 h-4" /> Strong Areas
            </h4>
            <ul className="space-y-2">
              {r.completed_modules?.map((m: string, i: number) => (
                <li key={i} className="text-sm text-slate-300 flex items-start gap-2">
                  <span className="w-1 h-1 rounded-full bg-green-400 mt-2 shrink-0" /> {m}
                </li>
              ))}
            </ul>
          </div>
          <div>
            <h4 className="text-sm font-semibold text-red-400 mb-3 flex items-center gap-2">
              <ShieldAlert className="w-4 h-4" /> Weak Areas
            </h4>
            <ul className="space-y-2">
              {r.weak_areas?.map((m: string, i: number) => (
                <li key={i} className="text-sm text-slate-300 flex items-start gap-2">
                  <span className="w-1 h-1 rounded-full bg-red-400 mt-2 shrink-0" /> {m}
                </li>
              ))}
            </ul>
          </div>
        </div>

        <div className="pt-4 border-t border-white/10">
          <h3 className="font-semibold text-white mb-3">Recommended Next Steps</h3>
          <ul className="space-y-2">
            {r.next_steps?.map((step: string, idx: number) => (
              <li key={idx} className="bg-white/5 border border-white/5 p-3 rounded-lg text-sm text-slate-300">
                {step}
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}
