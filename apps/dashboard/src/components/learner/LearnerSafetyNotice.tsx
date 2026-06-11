import { BookOpen } from 'lucide-react';

export function LearnerSafetyNotice() {
  return (
    <div className="bg-blue-500/5 border border-blue-500/20 p-4 rounded-xl mb-6 flex gap-3 text-blue-200 text-sm">
      <BookOpen className="w-5 h-5 shrink-0 text-blue-400" />
      <div>
        <strong>Realistic Schedules Active:</strong> LearnerAgent builds project-based, trackable study paths. 
        It explicitly avoids 12-hour/day extreme burn-out schedules unless forced. Take 1-2 hour focused blocks.
      </div>
    </div>
  );
}
