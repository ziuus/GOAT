'use client';

import { useState, useEffect } from 'react';
import { Play, Pause, Square, AlertTriangle, CheckCircle, Clock, FastForward, Activity } from 'lucide-react';

export default function AgentFlowPage() {
  const [sessions, setSessions] = useState<any[]>([]);
  const [selectedSession, setSelectedSession] = useState<any | null>(null);
  const [loading, setLoading] = useState(true);
  const [newTitle, setNewTitle] = useState('');
  const [newTemplate, setNewTemplate] = useState('startup-validation-flow');

  useEffect(() => {
    loadSessions();
  }, []);

  const loadSessions = async () => {
    try {
      setLoading(true);
      const res = await fetch('/api/proxy/v1/collaboration/sessions').then(r => r.json());
      if (res.sessions) setSessions(res.sessions);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async () => {
    try {
      const res = await fetch('/api/proxy/v1/collaboration/sessions', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ title: newTitle || 'New Flow', goal: newTitle, template: newTemplate })
      }).then(r => r.json());
      if (res.session) {
        setSessions([res.session, ...sessions]);
        setSelectedSession(res.session);
        setNewTitle('');
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleAction = async (id: string, action: string) => {
    try {
      const res = await fetch(`/api/proxy/v1/collaboration/sessions/${id}/${action}`, { method: 'POST' }).then(r => r.json());
      if (res.session) {
        setSelectedSession(res.session);
        setSessions(sessions.map(s => s.id === id ? res.session : s));
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="min-h-screen bg-[#0A0A0A] text-slate-300 p-8">
      <div className="max-w-7xl mx-auto space-y-8">
        <header className="flex justify-between items-end border-b border-white/10 pb-6">
          <div>
            <h1 className="text-3xl font-bold text-white flex items-center gap-3">
              <Activity className="w-8 h-8 text-indigo-400" />
              AgentFlow Collaboration
            </h1>
            <p className="text-sm text-slate-400 mt-2">
              Prime Agent Collaboration Layer + Live Execution Feedback
            </p>
          </div>
        </header>

        <div className="grid grid-cols-12 gap-8">
          <div className="col-span-12 md:col-span-4 space-y-6">
            <div className="bg-white/[0.02] border border-white/5 rounded-xl p-5 space-y-4">
              <h3 className="font-semibold text-white">New Workflow</h3>
              <input 
                type="text" 
                placeholder="Goal / Title" 
                value={newTitle}
                onChange={e => setNewTitle(e.target.value)}
                className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-indigo-500"
              />
              <select 
                value={newTemplate}
                onChange={e => setNewTemplate(e.target.value)}
                className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-indigo-500"
              >
                <option value="startup-validation-flow">Startup Validation Flow</option>
                <option value="launch-readiness-flow">Launch Readiness Flow</option>
                <option value="build-and-release-flow">Build & Release Flow</option>
                <option value="learning-project-flow">Learning Project Flow</option>
                <option value="incident-response-flow">Incident Response Flow</option>
              </select>
              <button 
                onClick={handleCreate}
                className="w-full bg-indigo-500/20 text-indigo-400 py-2 rounded-lg text-sm font-medium hover:bg-indigo-500/30 transition-colors border border-indigo-500/50"
              >
                Create Session
              </button>
            </div>

            <div className="space-y-2">
              <h3 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-3 px-2">Active Sessions</h3>
              {loading && <p className="px-2 text-sm">Loading...</p>}
              {sessions.map(s => (
                <button
                  key={s.id}
                  onClick={() => setSelectedSession(s)}
                  className={`w-full text-left p-4 rounded-xl border transition-all ${
                    selectedSession?.id === s.id 
                      ? 'bg-indigo-500/10 border-indigo-500/50' 
                      : 'bg-white/[0.02] border-white/5 hover:border-white/20'
                  }`}
                >
                  <div className="flex justify-between items-center mb-1">
                    <span className="font-medium text-white text-sm truncate">{s.title}</span>
                    <span className="text-[10px] uppercase bg-white/10 px-1.5 py-0.5 rounded text-slate-400 shrink-0">{s.status}</span>
                  </div>
                  <div className="text-xs text-slate-500 truncate">{s.template}</div>
                </button>
              ))}
            </div>
          </div>

          <div className="col-span-12 md:col-span-8">
            {selectedSession ? (
              <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6 h-full min-h-[600px] flex flex-col">
                <div className="flex justify-between items-start mb-6 border-b border-white/10 pb-6">
                  <div>
                    <h2 className="text-2xl font-bold text-white">{selectedSession.title}</h2>
                    <div className="flex gap-2 text-xs mt-2">
                      <span className="px-2 py-1 bg-white/5 border border-white/10 rounded-md text-slate-400 uppercase tracking-wider">
                        {selectedSession.template}
                      </span>
                      <span className="px-2 py-1 bg-white/5 border border-white/10 rounded-md text-slate-400 uppercase tracking-wider">
                        {selectedSession.status}
                      </span>
                    </div>
                  </div>
                  <div className="flex gap-2">
                    <button onClick={() => handleAction(selectedSession.id, 'start')} className="p-2 bg-white/5 hover:bg-white/10 rounded-lg text-green-400" title="Start">
                      <Play className="w-4 h-4" />
                    </button>
                    <button onClick={() => handleAction(selectedSession.id, 'step')} className="p-2 bg-white/5 hover:bg-white/10 rounded-lg text-blue-400" title="Advance Step">
                      <FastForward className="w-4 h-4" />
                    </button>
                    <button onClick={() => handleAction(selectedSession.id, 'pause')} className="p-2 bg-white/5 hover:bg-white/10 rounded-lg text-yellow-400" title="Pause">
                      <Pause className="w-4 h-4" />
                    </button>
                    <button onClick={() => handleAction(selectedSession.id, 'cancel')} className="p-2 bg-white/5 hover:bg-white/10 rounded-lg text-red-400" title="Cancel">
                      <Square className="w-4 h-4" />
                    </button>
                  </div>
                </div>

                <div className="flex-1 overflow-y-auto pr-4">
                  <h3 className="font-semibold text-white mb-4">Workflow Steps</h3>
                  <div className="space-y-4 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-white/10 before:to-transparent">
                    {selectedSession.steps.map((step: any, idx: number) => {
                      const isActive = idx === selectedSession.current_step_index;
                      const isPast = idx < selectedSession.current_step_index;
                      return (
                        <div key={step.id} className="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active">
                          <div className={`flex items-center justify-center w-10 h-10 rounded-full border-4 border-[#0A0A0A] shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 shadow ${isActive ? 'bg-blue-500 text-white' : isPast ? 'bg-green-500 text-white' : 'bg-slate-800 text-slate-500'}`}>
                            {isPast ? <CheckCircle className="w-4 h-4" /> : isActive ? <Activity className="w-4 h-4 animate-pulse" /> : <Clock className="w-4 h-4" />}
                          </div>
                          <div className="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] p-4 rounded-xl border border-white/5 bg-white/[0.02] hover:bg-white/[0.04] transition-colors">
                            <div className="flex items-center justify-between mb-1">
                              <span className="font-semibold text-white">{step.agent}</span>
                              <span className="text-[10px] uppercase tracking-wider text-slate-500">{step.status}</span>
                            </div>
                            <h4 className="text-sm text-indigo-400 mb-2">{step.action}</h4>
                            <p className="text-sm text-slate-400">{step.description}</p>
                            {step.required_approval && (
                              <div className="mt-3 flex items-center gap-2 text-xs text-yellow-500 bg-yellow-500/10 px-2 py-1.5 rounded border border-yellow-500/20">
                                <AlertTriangle className="w-3.5 h-3.5" /> Requires Approval
                              </div>
                            )}
                            {step.runtime_job_id && (
                              <div className="mt-2 text-xs text-slate-500 truncate">
                                Job Ref: {step.runtime_job_id}
                              </div>
                            )}
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>
                
                <div className="mt-6 pt-4 border-t border-white/10 flex items-center gap-2 text-xs text-slate-400">
                  <CheckCircle className="w-4 h-4 text-emerald-500" />
                  <p><strong>Safety notice:</strong> Collaboration is visible, cancellable, and runtime-backed.</p>
                </div>
              </div>
            ) : (
              <div className="h-full min-h-[600px] border border-white/5 border-dashed rounded-2xl flex items-center justify-center text-slate-500 flex-col gap-4 bg-black/20">
                <Activity className="w-16 h-16 opacity-20 text-indigo-400" />
                <p className="text-lg text-slate-400">Select an AgentFlow session to view.</p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
