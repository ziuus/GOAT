'use client';

import { useState, useEffect } from 'react';
import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { SafetyNotice } from '@/components/ui/Status';
import { Rocket, Target, Send, Activity, FileText, CheckCircle } from 'lucide-react';
import { goatApi } from '@/lib/goat-api';

interface MissionPlan {
  goal_type: string;
  suggested_agents: string[];
  suggested_workflow: string;
  expected_artifacts: string[];
  required_approvals: string[];
  next_actions: string[];
  safety_notes: string[];
}

export default function MissionControl() {
  const [goal, setGoal] = useState('');
  const [isPlanning, setIsPlanning] = useState(false);
  const [plan, setPlan] = useState<MissionPlan | null>(null);

  const handlePlanGoal = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!goal.trim()) return;

    setIsPlanning(true);
    try {
      const res = await fetch('http://127.0.0.1:3000/v1/mission-control/plan-goal', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('goat_token')}`
        },
        body: JSON.stringify({ goal, project_id: null, constraints: null })
      });
      const data = await res.json();
      setPlan(data);
    } catch (e) {
      console.error("Failed to plan goal", e);
    } finally {
      setIsPlanning(false);
    }
  };

  return (
    <PageShell>
      <PageHeader 
        title={
          <div className="flex items-center gap-3">
            <Rocket className="w-6 h-6 text-indigo-400" />
            Mission Control
          </div>
        }
        subtitle="Your central command workspace for managing autonomous projects."
      />

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 space-y-6">
          <section className="bg-gray-800/50 border border-gray-700/50 rounded-xl p-6">
            <h2 className="text-lg font-medium text-white mb-4 flex items-center gap-2">
              <Target className="w-5 h-5 text-gray-400" />
              What are we building / solving today?
            </h2>
            <form onSubmit={handlePlanGoal} className="flex gap-3">
              <input
                type="text"
                placeholder="Describe your goal... (e.g. 'Build a React dashboard', 'Research market trends')"
                className="flex-1 bg-gray-900 border border-gray-700 text-white rounded-lg px-4 py-3 focus:outline-none focus:border-indigo-500"
                value={goal}
                onChange={(e) => setGoal(e.target.value)}
              />
              <button 
                type="submit"
                disabled={isPlanning || !goal.trim()}
                className="bg-indigo-600 hover:bg-indigo-700 text-white px-6 py-3 rounded-lg font-medium transition-colors disabled:opacity-50 flex items-center gap-2"
              >
                {isPlanning ? 'Planning...' : (
                  <>
                    <Send className="w-4 h-4" /> Plan
                  </>
                )}
              </button>
            </form>
          </section>

          {plan && (
            <section className="bg-gray-800/50 border border-indigo-500/30 rounded-xl p-6 relative overflow-hidden">
              <div className="absolute top-0 left-0 w-1 h-full bg-indigo-500" />
              <h2 className="text-lg font-medium text-white mb-4">Mission Plan Prepared</h2>
              
              <div className="grid grid-cols-2 gap-4 mb-6">
                <div>
                  <div className="text-xs text-gray-400 mb-1 uppercase tracking-wider">Goal Type</div>
                  <div className="text-sm text-gray-200">{plan.goal_type}</div>
                </div>
                <div>
                  <div className="text-xs text-gray-400 mb-1 uppercase tracking-wider">Suggested Workflow</div>
                  <div className="text-sm text-gray-200">{plan.suggested_workflow}</div>
                </div>
              </div>

              <div className="space-y-4">
                <div>
                  <div className="text-xs text-gray-400 mb-2 uppercase tracking-wider flex items-center gap-1">
                    <Rocket className="w-3 h-3" /> Recommended Team
                  </div>
                  <div className="flex flex-wrap gap-2">
                    {plan.suggested_agents.map(a => (
                      <span key={a} className="px-2 py-1 bg-gray-900 border border-gray-700 rounded-md text-xs text-gray-300">
                        @{a.toLowerCase()}
                      </span>
                    ))}
                  </div>
                </div>
                
                <div>
                  <div className="text-xs text-gray-400 mb-2 uppercase tracking-wider flex items-center gap-1">
                    <FileText className="w-3 h-3" /> Expected Artifacts
                  </div>
                  <ul className="list-disc pl-5 text-sm text-gray-300">
                    {plan.expected_artifacts.map(art => <li key={art}>{art}</li>)}
                  </ul>
                </div>

                <div>
                  <div className="text-xs text-gray-400 mb-2 uppercase tracking-wider flex items-center gap-1">
                    <CheckCircle className="w-3 h-3" /> Required Approvals
                  </div>
                  <ul className="list-disc pl-5 text-sm text-gray-300">
                    {plan.required_approvals.map(app => <li key={app}>{app}</li>)}
                  </ul>
                </div>

                <div className="pt-4 mt-4 border-t border-gray-700/50">
                  <SafetyNotice>{plan.safety_notes.join(' ')}</SafetyNotice>
                </div>
                
                <div className="pt-4">
                  <button className="bg-emerald-600 hover:bg-emerald-700 text-white px-6 py-2 rounded-lg font-medium w-full transition-colors">
                    Approve & Execute Mission
                  </button>
                </div>
              </div>
            </section>
          )}

          <section className="bg-gray-800/50 border border-gray-700/50 rounded-xl p-6">
            <h2 className="text-lg font-medium text-white mb-4 flex items-center gap-2">
              <Activity className="w-5 h-5 text-gray-400" />
              Unified Activity Feed
            </h2>
            <div className="text-sm text-gray-400 py-8 text-center italic border border-dashed border-gray-700 rounded-lg">
              No recent activity in the current workspace.
            </div>
          </section>
        </div>

        <div className="space-y-6">
          <section className="bg-gray-800/50 border border-gray-700/50 rounded-xl p-6">
            <h2 className="text-lg font-medium text-white mb-4">Active Workspace</h2>
            <div className="bg-gray-900 p-4 rounded-lg border border-gray-700">
              <div className="text-xs text-green-400 uppercase tracking-widest font-bold mb-1">Local Project</div>
              <div className="text-sm text-gray-200">No project selected</div>
              <button className="text-xs text-indigo-400 hover:text-indigo-300 mt-3">Select Project &rarr;</button>
            </div>
          </section>

          <section className="bg-gray-800/50 border border-gray-700/50 rounded-xl p-6">
            <h2 className="text-lg font-medium text-white mb-4">Pending Approvals</h2>
            <div className="text-sm text-gray-400 py-4 text-center italic border border-dashed border-gray-700 rounded-lg">
              No pending actions.
            </div>
          </section>

          <section className="bg-gray-800/50 border border-gray-700/50 rounded-xl p-6">
            <h2 className="text-lg font-medium text-white mb-4">Recent Artifacts</h2>
            <div className="text-sm text-gray-400 py-4 text-center italic border border-dashed border-gray-700 rounded-lg">
              No artifacts generated yet.
            </div>
          </section>
        </div>
      </div>
    </PageShell>
  );
}
