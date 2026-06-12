'use client';

import { useState, useEffect } from 'react';
import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { SafetyNotice } from '@/components/ui/Status';
import { Rocket, Target, Send, Activity, FileText, CheckCircle, Bot, AlertTriangle, ListTodo, Archive, Play, Pause, ChevronRight } from 'lucide-react';

interface AgentRef {
  name: string;
  role: string;
  status: string;
}

interface MissionPlanStep {
  id: string;
  title: string;
  description: string;
  assigned_agent?: string;
  status: string;
}

interface Mission {
  mission_id: string;
  title: string;
  raw_goal: string;
  mission_type: string;
  recommended_agents: AgentRef[];
  plan_steps: MissionPlanStep[];
  expected_artifacts: string[];
  status: string;
  created_at: number;
  updated_at: number;
  linked_project?: string;
  progress: number;
  notes: string[];
  risks: string[];
  next_actions: string[];
}

export default function MissionControl() {
  const [goal, setGoal] = useState('');
  const [isPlanning, setIsPlanning] = useState(false);
  const [plan, setPlan] = useState<Mission | null>(null);
  const [feed, setFeed] = useState<Mission[]>([]);
  const [projects, setProjects] = useState<any[]>([]);
  const [loadingFeed, setLoadingFeed] = useState(true);

  useEffect(() => {
    fetchFeed();
    fetchProjects();
  }, []);

  const fetchFeed = async () => {
    try {
      const res = await fetch('http://127.0.0.1:3000/v1/mission-control/feed', {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('goat_token')}` }
      });
      if (res.ok) {
        const data = await res.json();
        setFeed(data.feed || []);
      }
    } catch (e) {
      console.error("Failed to fetch feed", e);
    } finally {
      setLoadingFeed(false);
    }
  };

  const fetchProjects = async () => {
    try {
      const res = await fetch('http://127.0.0.1:3000/v1/projects', {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('goat_token')}` }
      });
      if (res.ok) {
        const data = await res.json();
        setProjects(data.projects || []);
      }
    } catch (e) {
      console.error("Failed to fetch projects", e);
    }
  };

  const handlePlanGoal = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!goal.trim()) return;

    let matchedProjectId = null;
    if (projects.length === 1) {
      matchedProjectId = projects[0].project_id;
    } else if (projects.length > 1) {
      const match = projects.find(p => goal.toLowerCase().includes(p.name.toLowerCase()));
      if (match) matchedProjectId = match.project_id;
    }

    setIsPlanning(true);
    setPlan(null);
    try {
      const res = await fetch('http://127.0.0.1:3000/v1/mission-control/plan-goal', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('goat_token')}`
        },
        body: JSON.stringify({ goal, project_id: matchedProjectId, constraints: null })
      });
      const data = await res.json();
      setPlan(data);
      fetchFeed(); // Refresh feed to show the newly planned mission
    } catch (e) {
      console.error("Failed to plan goal", e);
    } finally {
      setIsPlanning(false);
    }
  };

  const renderStatusBadge = (status: string) => {
    const colors: Record<string, string> = {
      'draft': 'bg-gray-500/20 text-gray-400 border-gray-500/30',
      'planned': 'bg-blue-500/20 text-blue-400 border-blue-500/30',
      'running': 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30',
      'blocked': 'bg-rose-500/20 text-rose-400 border-rose-500/30',
      'completed': 'bg-purple-500/20 text-purple-400 border-purple-500/30',
      'archived': 'bg-gray-800/50 text-gray-500 border-gray-800',
    };
    const c = colors[status] || colors['draft'];
    return (
      <span className={`px-2.5 py-0.5 rounded-full text-[10px] uppercase tracking-wider font-bold border ${c}`}>
        {status}
      </span>
    );
  };

  return (
    <PageShell>
      <PageHeader 
        title={
          <div className="flex items-center gap-3 relative group">
            <div className="absolute -inset-2 bg-indigo-500/20 blur-lg rounded-full opacity-0 group-hover:opacity-100 transition duration-500"></div>
            <Rocket className="w-8 h-8 text-indigo-400 relative z-10 animate-pulse" />
            <span className="bg-clip-text text-transparent bg-gradient-to-r from-white via-indigo-100 to-indigo-300 relative z-10 font-bold text-3xl">
              Mission Control
            </span>
          </div>
        }
        subtitle="Command center for autonomous operations and agent orchestration."
      />

      <div className="grid grid-cols-1 xl:grid-cols-3 gap-8">
        <div className="xl:col-span-2 space-y-8">
          
          {/* Main Input Form */}
          <section className="relative overflow-hidden bg-gray-900/40 backdrop-blur-xl border border-white/5 rounded-2xl p-8 shadow-2xl">
            <div className="absolute top-0 inset-x-0 h-px bg-gradient-to-r from-transparent via-indigo-500/50 to-transparent"></div>
            <h2 className="text-xl font-semibold text-white mb-6 flex items-center gap-3">
              <Target className="w-6 h-6 text-indigo-400" />
              What are we building or solving today?
            </h2>
            <form onSubmit={handlePlanGoal} className="flex gap-4">
              <div className="relative flex-1 group">
                <div className="absolute -inset-0.5 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-xl blur opacity-20 group-hover:opacity-40 transition duration-500"></div>
                <input
                  type="text"
                  placeholder="Describe your goal... (e.g. 'Build a Next.js dashboard', 'Audit the database')"
                  className="relative w-full bg-black/50 border border-white/10 text-white placeholder-gray-500 rounded-xl px-5 py-4 focus:outline-none focus:ring-2 focus:ring-indigo-500/50 transition-all text-lg"
                  value={goal}
                  onChange={(e) => setGoal(e.target.value)}
                />
              </div>
              <button 
                type="submit"
                disabled={isPlanning || !goal.trim()}
                className="relative overflow-hidden bg-gradient-to-br from-indigo-600 to-violet-700 hover:from-indigo-500 hover:to-violet-600 text-white px-8 py-4 rounded-xl font-bold transition-all disabled:opacity-50 disabled:grayscale flex items-center gap-3 shadow-lg hover:shadow-indigo-500/25 border border-white/10"
              >
                {isPlanning ? 'Planning...' : (
                  <>
                    <Send className="w-5 h-5" /> Plan Mission
                  </>
                )}
              </button>
            </form>
          </section>

          {/* Mission Plan Details */}
          {plan && (
            <section className="bg-gray-900/40 backdrop-blur-xl border border-indigo-500/20 rounded-2xl p-8 relative overflow-hidden shadow-2xl animate-in fade-in slide-in-from-bottom-4 duration-500">
              <div className="absolute top-0 left-0 w-1.5 h-full bg-gradient-to-b from-indigo-500 to-violet-600" />
              <div className="flex justify-between items-start mb-8">
                <div>
                  <h2 className="text-2xl font-bold text-white mb-2">{plan.title}</h2>
                  <p className="text-gray-400 italic">"{plan.raw_goal}"</p>
                </div>
                {renderStatusBadge(plan.status)}
              </div>
              
              <div className="grid grid-cols-2 md:grid-cols-4 gap-6 mb-8 bg-black/20 p-5 rounded-xl border border-white/5">
                <div>
                  <div className="text-[10px] text-gray-500 mb-1 uppercase tracking-widest font-bold">Mission Type</div>
                  <div className="text-sm text-indigo-300 font-medium">{plan.mission_type}</div>
                </div>
                <div>
                  <div className="text-[10px] text-gray-500 mb-1 uppercase tracking-widest font-bold">Progress</div>
                  <div className="text-sm text-white font-medium">{plan.progress}%</div>
                </div>
                <div>
                  <div className="text-[10px] text-gray-500 mb-1 uppercase tracking-widest font-bold">Created</div>
                  <div className="text-sm text-gray-300 font-medium">{new Date(plan.created_at).toLocaleTimeString()}</div>
                </div>
                <div>
                  <div className="text-[10px] text-gray-500 mb-1 uppercase tracking-widest font-bold">Project</div>
                  <div className="text-sm text-gray-300 font-medium">{plan.linked_project || 'None'}</div>
                </div>
              </div>

              <div className="space-y-8">
                <div>
                  <div className="text-xs text-indigo-400 mb-4 uppercase tracking-widest font-bold flex items-center gap-2">
                    <Bot className="w-4 h-4" /> Recommended Prime Agents
                  </div>
                  <div className="flex flex-wrap gap-3">
                    {plan.recommended_agents.map((a, i) => (
                      <div key={i} className={`px-4 py-2 bg-gray-950/50 border ${a.status === 'implemented' ? 'border-indigo-500/30' : 'border-gray-700/50 opacity-60'} rounded-lg flex flex-col`}>
                        <span className="text-sm font-bold text-gray-200">@{a.name.toLowerCase()}</span>
                        <span className="text-[10px] text-gray-500 uppercase">{a.role}</span>
                        {a.status !== 'implemented' && <span className="text-[9px] text-rose-400 mt-1">{a.status}</span>}
                      </div>
                    ))}
                  </div>
                </div>

                <div>
                  <div className="text-xs text-indigo-400 mb-4 uppercase tracking-widest font-bold flex items-center gap-2">
                    <ListTodo className="w-4 h-4" /> Execution Plan
                  </div>
                  <div className="space-y-3">
                    {plan.plan_steps.map((step, i) => (
                      <div key={step.id} className="flex gap-4 p-4 bg-black/30 border border-white/5 rounded-xl">
                        <div className="flex flex-col items-center gap-2">
                          <div className="w-6 h-6 rounded-full bg-gray-800 text-gray-400 flex items-center justify-center text-xs font-bold">{i + 1}</div>
                          {i !== plan.plan_steps.length - 1 && <div className="w-px h-full bg-gray-800"></div>}
                        </div>
                        <div className="flex-1 pb-2">
                          <div className="flex justify-between items-start mb-1">
                            <h4 className="text-sm font-bold text-gray-200">{step.title}</h4>
                            <span className="text-[10px] text-gray-500 bg-gray-900 px-2 py-0.5 rounded border border-gray-800 uppercase">{step.status}</span>
                          </div>
                          <p className="text-sm text-gray-400 mb-2">{step.description}</p>
                          {step.assigned_agent && (
                            <div className="text-xs text-indigo-400 font-medium">Assigned: @{step.assigned_agent.toLowerCase()}</div>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
                
                <div className="grid md:grid-cols-2 gap-6">
                  <div>
                    <div className="text-xs text-indigo-400 mb-3 uppercase tracking-widest font-bold flex items-center gap-2">
                      <FileText className="w-4 h-4" /> Expected Artifacts
                    </div>
                    <ul className="space-y-2">
                      {plan.expected_artifacts.map((art, i) => (
                        <li key={i} className="flex items-center gap-2 text-sm text-gray-300">
                          <ChevronRight className="w-3 h-3 text-gray-600" /> {art}
                        </li>
                      ))}
                    </ul>
                  </div>

                  <div>
                    <div className="text-xs text-amber-400 mb-3 uppercase tracking-widest font-bold flex items-center gap-2">
                      <AlertTriangle className="w-4 h-4" /> Identified Risks
                    </div>
                    <ul className="space-y-2">
                      {plan.risks.map((risk, i) => (
                        <li key={i} className="flex items-center gap-2 text-sm text-gray-300">
                          <span className="w-1.5 h-1.5 rounded-full bg-amber-500/50"></span> {risk}
                        </li>
                      ))}
                    </ul>
                  </div>
                </div>

                <div className="pt-6 mt-6 border-t border-white/5">
                  <SafetyNotice>{plan.notes.join(' ')}</SafetyNotice>
                </div>
                
                <div className="pt-6 flex gap-4">
                  <button className="flex-1 bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white px-6 py-4 rounded-xl font-bold transition-all shadow-lg hover:shadow-emerald-500/20 flex items-center justify-center gap-2">
                    <Play className="w-5 h-5" /> Start Mission
                  </button>
                  <button className="px-6 py-4 rounded-xl font-bold text-gray-400 hover:text-white hover:bg-gray-800 transition-colors border border-gray-800 flex items-center justify-center gap-2">
                    <Archive className="w-5 h-5" /> Save to Backlog
                  </button>
                </div>
              </div>
            </section>
          )}

          {/* Activity Feed */}
          <section className="bg-gray-900/40 backdrop-blur-xl border border-white/5 rounded-2xl p-8">
            <h2 className="text-lg font-semibold text-white mb-6 flex items-center gap-3">
              <Activity className="w-5 h-5 text-gray-400" />
              Mission Timeline
            </h2>
            {loadingFeed ? (
              <div className="text-sm text-gray-500 py-8 text-center animate-pulse">Loading mission data...</div>
            ) : feed.length === 0 ? (
              <div className="text-sm text-gray-400 py-12 text-center border border-dashed border-gray-800 rounded-xl bg-black/20">
                No missions created yet. Plan your first mission above!
              </div>
            ) : (
              <div className="space-y-4">
                {feed.map(m => (
                  <div key={m.mission_id} className="p-4 bg-black/30 border border-white/5 rounded-xl hover:border-indigo-500/30 transition-colors cursor-pointer group">
                    <div className="flex justify-between items-start mb-2">
                      <h3 className="font-medium text-gray-200 group-hover:text-indigo-300 transition-colors">{m.title}</h3>
                      {renderStatusBadge(m.status)}
                    </div>
                    <p className="text-xs text-gray-500 line-clamp-1 mb-3">{m.raw_goal}</p>
                    <div className="flex items-center gap-4 text-[10px] text-gray-600 font-bold uppercase tracking-wider">
                      <span>{new Date(m.created_at).toLocaleDateString()}</span>
                      <span>{m.mission_type}</span>
                      <span>{m.progress}% Done</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </section>
        </div>

        <div className="space-y-8">
          <section className="bg-gray-900/40 backdrop-blur-xl border border-white/5 rounded-2xl p-6 shadow-xl">
            <h2 className="text-sm font-bold text-gray-400 uppercase tracking-widest mb-4 flex justify-between items-center">
              <span>Active Workspace Context</span>
            </h2>
            {projects.length === 0 ? (
              <div className="bg-black/40 p-5 rounded-xl border border-dashed border-gray-800 text-center">
                <div className="text-sm text-gray-500 mb-2">No projects learned yet.</div>
                <div className="text-xs text-indigo-400 font-medium">Run `goat learn &lt;path&gt;` in your terminal.</div>
              </div>
            ) : (
              <div className="space-y-4">
                {projects.map((p, i) => (
                  <div key={p.project_id} className="bg-black/40 p-5 rounded-xl border border-white/5 relative overflow-hidden group">
                    <div className="absolute top-0 right-0 w-24 h-24 bg-indigo-500/10 blur-2xl rounded-full opacity-0 group-hover:opacity-100 transition duration-500" />
                    <div className="text-[10px] text-emerald-400 uppercase tracking-widest font-bold mb-2">Local Project</div>
                    <div className="text-sm text-white font-medium mb-1">{p.name}</div>
                    <div className="text-xs text-gray-500 mb-2">{p.root_path}</div>
                    <div className="text-[10px] text-gray-400 uppercase tracking-wider font-bold mb-2">Stack: {p.detected_stack.join(', ')}</div>
                    {p.available_commands.slice(0,3).map((cmd: string) => (
                      <div key={cmd} className="text-xs text-indigo-300 font-mono mb-1">&gt; {cmd}</div>
                    ))}
                    {p.available_commands.length > 3 && (
                      <div className="text-xs text-gray-600 font-mono">+{p.available_commands.length - 3} more...</div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </section>

          <section className="bg-gray-900/40 backdrop-blur-xl border border-white/5 rounded-2xl p-6 shadow-xl">
            <h2 className="text-sm font-bold text-gray-400 uppercase tracking-widest mb-4">Pending Approvals</h2>
            <div className="text-sm text-gray-500 py-6 text-center border border-dashed border-gray-800 rounded-xl bg-black/20">
              No actions pending approval.
            </div>
          </section>

          <section className="bg-gray-900/40 backdrop-blur-xl border border-white/5 rounded-2xl p-6 shadow-xl">
            <h2 className="text-sm font-bold text-gray-400 uppercase tracking-widest mb-4">Recent Artifacts</h2>
            <div className="text-sm text-gray-500 py-6 text-center border border-dashed border-gray-800 rounded-xl bg-black/20">
              No artifacts generated in this session.
            </div>
          </section>
        </div>
      </div>
    </PageShell>
  );
}
