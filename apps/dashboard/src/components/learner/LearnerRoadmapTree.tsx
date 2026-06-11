import { Compass, CheckCircle2, Circle } from 'lucide-react';

interface Props {
  data: any | null;
  loading: boolean;
  onGenerate: () => void;
}

export function LearnerRoadmapTree({ data, loading, onGenerate }: Props) {
  if (loading) {
    return <div className="p-8 text-center text-slate-500 animate-pulse">Loading roadmap...</div>;
  }

  if (!data) {
    return (
      <div className="flex flex-col items-center justify-center py-16 border border-white/5 border-dashed rounded-2xl">
        <Compass className="w-12 h-12 text-slate-600 mb-4" />
        <p className="text-slate-400 mb-6">No roadmap generated for this goal yet.</p>
        <button 
          onClick={onGenerate}
          className="px-4 py-2 bg-blue-500/20 text-blue-400 rounded-lg hover:bg-blue-500/30 transition-colors border border-blue-500/50"
        >
          Generate Roadmap
        </button>
      </div>
    );
  }

  return (
    <div className="space-y-8 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-white/10 before:to-transparent">
      {data.roadmap?.phases?.map((phase: any, idx: number) => (
        <div key={idx} className="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active">
          <div className="flex items-center justify-center w-10 h-10 rounded-full border-4 border-[#0A0A0A] bg-blue-500/20 text-blue-400 shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 z-10">
            {phase.status === 'Completed' ? <CheckCircle2 className="w-5 h-5" /> : <Circle className="w-4 h-4" />}
          </div>
          
          <div className="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-white/[0.02] border border-white/5 hover:border-white/20 p-5 rounded-2xl shadow transition-colors">
            <div className="flex items-center justify-between mb-2">
              <h3 className="font-bold text-white text-lg">{phase.title}</h3>
              <span className="text-xs px-2 py-1 bg-black rounded-md border border-white/10 text-slate-400">
                {phase.estimated_hours}h
              </span>
            </div>
            <p className="text-sm text-slate-400 mb-4">{phase.description}</p>
            
            <div className="space-y-2 mt-4 border-t border-white/10 pt-4">
              {phase.modules?.map((mod: any, mIdx: number) => (
                <div key={mIdx} className="flex items-start gap-2 text-sm">
                  <span className={`w-1.5 h-1.5 mt-1.5 rounded-full shrink-0 ${mod.status === 'Completed' ? 'bg-green-400' : 'bg-blue-500/50'}`} />
                  <span className="text-slate-300">{mod.title}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
