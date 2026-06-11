import { Brain, Star, Clock } from 'lucide-react';

interface Props {
  data: any | null;
  loading: boolean;
  onGenerate: () => void;
}

export function LearnerRevisionPanel({ data, loading, onGenerate }: Props) {
  if (loading) {
    return <div className="p-8 text-center text-slate-500 animate-pulse">Loading revision checkpoint...</div>;
  }

  if (!data || !data.checkpoint) {
    return (
      <div className="flex flex-col items-center justify-center py-16 border border-white/5 border-dashed rounded-2xl">
        <Brain className="w-12 h-12 text-slate-600 mb-4" />
        <p className="text-slate-400 mb-6">No revision checkpoint active.</p>
        <button 
          onClick={onGenerate}
          className="px-4 py-2 bg-blue-500/20 text-blue-400 rounded-lg hover:bg-blue-500/30 transition-colors border border-blue-500/50"
        >
          Generate Revision
        </button>
      </div>
    );
  }

  const cp = data.checkpoint;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-bold text-white flex items-center gap-2">
          <Brain className="w-6 h-6 text-blue-400" /> Active Revision: {cp.topic}
        </h2>
        <span className="text-xs px-2 py-1 bg-white/5 rounded border border-white/10 text-slate-400">
          Last reviewed: {new Date(cp.last_reviewed_at).toLocaleDateString()}
        </span>
      </div>

      <div className="grid md:grid-cols-2 gap-4">
        {cp.weak_areas?.map((area: string, idx: number) => (
          <div key={idx} className="bg-white/[0.02] border border-white/5 rounded-xl p-5 hover:border-white/20 transition-colors">
            <h3 className="font-medium text-white mb-3">{area}</h3>
            
            <div className="flex items-center justify-between mb-4">
              <span className="text-xs text-slate-500 uppercase">Confidence</span>
              <div className="flex gap-1">
                {[1, 2, 3, 4, 5].map((star) => (
                  <Star 
                    key={star} 
                    className={`w-4 h-4 ${star <= cp.confidence_rating ? 'text-yellow-500 fill-yellow-500' : 'text-slate-600'}`} 
                  />
                ))}
              </div>
            </div>

            <div className="flex gap-2">
              <button className="flex-1 px-3 py-2 bg-blue-500/10 hover:bg-blue-500/20 text-blue-400 rounded-lg border border-blue-500/20 text-xs transition-colors">
                Revise Now
              </button>
              <button className="px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg border border-white/10 text-xs transition-colors">
                Mark Stable
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
