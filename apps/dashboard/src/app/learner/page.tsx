'use client';

import { useState, useEffect } from 'react';
import { learnerApi } from '@/lib/goat-api';
import { BookOpen, Target, Clock, Plus, Compass } from 'lucide-react';
import { LearnerShell } from '@/components/learner/LearnerShell';

export default function LearnerPage() {
  const [goals, setGoals] = useState<any[]>([]);
  const [selectedGoal, setSelectedGoal] = useState<any | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [isCreating, setIsCreating] = useState(false);
  const [newTitle, setNewTitle] = useState('');
  const [newDomain, setNewDomain] = useState('DSA');

  useEffect(() => {
    loadGoals();
  }, []);

  const loadGoals = async () => {
    try {
      setLoading(true);
      const res = await learnerApi.listGoals();
      if (res.goals) setGoals(res.goals);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async () => {
    try {
      const res = await learnerApi.createGoal({
        title: newTitle || 'New Learning Goal',
        domain: newDomain,
      });
      if (res.goal) {
        setGoals([...goals, res.goal]);
        setSelectedGoal(res.goal);
        setIsCreating(false);
        setNewTitle('');
      }
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <div className="min-h-screen bg-[#0A0A0A] text-slate-300 p-8">
      <div className="max-w-7xl mx-auto space-y-8">
        
        <header className="flex justify-between items-end border-b border-white/10 pb-6">
          <div>
            <h1 className="text-3xl font-bold text-white flex items-center gap-3">
              <BookOpen className="w-8 h-8 text-blue-400" />
              Learner OS
            </h1>
            <p className="text-sm text-slate-400 mt-2">
              Structured Journey, Roadmap Tracks, and Realistic AI Scheduling.
            </p>
          </div>
          <button 
            onClick={() => setIsCreating(!isCreating)}
            className="flex items-center gap-2 px-4 py-2 bg-blue-500/10 text-blue-400 rounded-lg text-sm font-medium border border-blue-500/20 hover:bg-blue-500/20 transition-colors"
          >
            <Plus className="w-4 h-4" /> New Track
          </button>
        </header>

        {error && (
          <div className="p-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm">
            {error}
          </div>
        )}

        <div className="grid grid-cols-12 gap-8">
          
          <div className="col-span-12 md:col-span-3 space-y-4">
            {isCreating && (
              <div className="bg-white/[0.02] border border-white/5 hover:border-blue-500/30 transition-colors rounded-2xl p-5 mb-4 shadow-lg shadow-blue-500/5">
                <h2 className="text-sm font-semibold text-white mb-4 flex items-center gap-2">
                  <Target className="w-4 h-4 text-blue-400" /> Configure Track
                </h2>
                <div className="space-y-3">
                  <input 
                    type="text" 
                    placeholder="e.g. Master Rust Concurrency" 
                    value={newTitle}
                    onChange={e => setNewTitle(e.target.value)}
                    className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-blue-500 transition-colors"
                  />
                  <select 
                    value={newDomain}
                    onChange={e => setNewDomain(e.target.value)}
                    className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-blue-500 transition-colors"
                  >
                    <option value="DSA">Data Structures & Algorithms</option>
                    <option value="AIML">AI / Machine Learning</option>
                    <option value="Rust">Rust Programming</option>
                    <option value="Web3">Web3 & DApps</option>
                    <option value="FullStack">Full-Stack Development</option>
                    <option value="SystemDesign">System Design</option>
                    <option value="ExamPrep">Exam Preparation</option>
                    <option value="ProjectBased">Project-Based Learning</option>
                    <option value="General">General Study</option>
                  </select>
                  <div className="flex gap-2">
                    <button 
                      onClick={() => setIsCreating(false)}
                      className="flex-1 bg-white/5 hover:bg-white/10 text-slate-300 font-medium text-xs py-2 rounded-lg transition-colors border border-white/10"
                    >
                      Cancel
                    </button>
                    <button 
                      onClick={handleCreate}
                      className="flex-1 bg-blue-500/20 hover:bg-blue-500/30 text-blue-400 font-medium text-xs py-2 rounded-lg transition-colors border border-blue-500/50"
                    >
                      Start Journey
                    </button>
                  </div>
                </div>
              </div>
            )}

            <div className="space-y-2">
              <h3 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-3 px-2">Active Tracks</h3>
              {goals.length === 0 && !isCreating && !loading && (
                <p className="text-sm text-slate-500 px-2 italic">No active tracks. Create one to start learning.</p>
              )}
              {goals.map(g => (
                <button
                  key={g.id}
                  onClick={() => setSelectedGoal(g)}
                  className={`w-full text-left p-4 rounded-xl border transition-all ${
                    selectedGoal?.id === g.id 
                      ? 'bg-blue-500/10 border-blue-500/50 shadow-[0_0_15px_rgba(59,130,246,0.1)]' 
                      : 'bg-white/[0.02] border-white/5 hover:border-white/20'
                  }`}
                >
                  <div className="flex items-center justify-between mb-1">
                    <h4 className="font-medium text-white truncate max-w-[70%]">{g.title}</h4>
                    <span className="text-[9px] uppercase bg-black px-1.5 py-0.5 rounded text-slate-400 border border-white/10 shrink-0">{g.domain}</span>
                  </div>
                  <div className="flex items-center gap-1.5 text-xs text-slate-500 mt-2">
                    <Clock className="w-3.5 h-3.5" />
                    <span>Lvl: {g.current_level}</span>
                  </div>
                </button>
              ))}
            </div>
            
            <div className="mt-8 pt-6 border-t border-white/10">
              <h3 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-3 px-2">Templates</h3>
              <button 
                onClick={() => { setNewTitle('DSA Masterclass'); setNewDomain('DSA'); setIsCreating(true); }}
                className="w-full text-left px-4 py-3 bg-white/[0.01] hover:bg-white/[0.03] border border-white/5 rounded-xl transition-colors text-sm text-slate-400"
              >
                DSA Masterclass <span className="text-xs bg-blue-500/10 text-blue-400 px-1.5 py-0.5 rounded ml-2">Popular</span>
              </button>
            </div>
          </div>

          <div className="col-span-12 md:col-span-9">
            {selectedGoal ? (
              <LearnerShell goal={selectedGoal} onUpdate={loadGoals} />
            ) : (
              <div className="h-full min-h-[600px] border border-white/5 border-dashed rounded-2xl flex items-center justify-center text-slate-500 flex-col gap-4 bg-black/20">
                <Compass className="w-16 h-16 opacity-20 text-blue-400" />
                <p className="text-lg text-slate-400">Select a learning track to open the OS.</p>
              </div>
            )}
          </div>

        </div>
      </div>
    </div>
  );
}
