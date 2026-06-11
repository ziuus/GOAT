'use client';

import { useState, useEffect } from 'react';
import { learnerApi } from '@/lib/goat-api';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  BookOpen, Target, Brain, FileText, Calendar, CheckCircle2, ListTodo, Activity, Compass, Code, Database, Clock
} from 'lucide-react';

export default function LearnerPage() {
  const [goals, setGoals] = useState<any[]>([]);
  const [selectedGoal, setSelectedGoal] = useState<any | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

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
        setNewTitle('');
      }
    } catch (err: any) {
      setError(err.message);
    }
  };

  const handleAction = async (id: string, action: string) => {
    try {
      let res: any;
      if (action === 'roadmap') res = await learnerApi.roadmap(id);
      else if (action === 'week') res = await learnerApi.week(id);
      else if (action === 'today') res = await learnerApi.today(id);
      else if (action === 'practice') res = await learnerApi.practice(id);
      else if (action === 'revise') res = await learnerApi.revise(id);
      else if (action === 'project') res = await learnerApi.project(id);
      else if (action === 'exam') res = await learnerApi.exam(id);
      else if (action === 'progress') res = await learnerApi.progress(id);
      else if (action === 'report') res = await learnerApi.report(id);

      if (res) alert(`Action ${action} completed successfully.`);
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <div className="min-h-screen bg-[#0A0A0A] text-slate-300 p-8">
      <div className="max-w-6xl mx-auto space-y-8">
        
        <header className="flex justify-between items-end border-b border-white/10 pb-6">
          <div>
            <h1 className="text-3xl font-bold text-white flex items-center gap-3">
              <BookOpen className="w-8 h-8 text-blue-400" />
              Learner Prime
            </h1>
            <p className="text-sm text-slate-400 mt-2">
              Structured Learning Plans, Roadmaps, Practice Tasks, and Revision Checkpoints.
            </p>
          </div>
          <div className="flex items-center gap-2">
            <span className="flex items-center gap-1.5 px-3 py-1 bg-blue-500/10 text-blue-400 rounded-full text-xs font-medium border border-blue-500/20">
              <Activity className="w-3.5 h-3.5" /> Realistic Schedules Active
            </span>
          </div>
        </header>

        {error && (
          <div className="p-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm">
            {error}
          </div>
        )}

        <div className="grid grid-cols-12 gap-8">
          
          <div className="col-span-4 space-y-4">
            <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-5">
              <h2 className="text-sm font-semibold text-white mb-4 flex items-center gap-2">
                <Target className="w-4 h-4" /> New Learning Goal
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
                <button 
                  onClick={handleCreate}
                  className="w-full bg-blue-500/20 hover:bg-blue-500/30 text-blue-400 font-medium text-sm py-2 rounded-lg transition-colors border border-blue-500/50"
                >
                  Create Goal
                </button>
              </div>
            </div>

            <div className="space-y-2">
              <h3 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-3">Active Goals</h3>
              {goals.map(g => (
                <button
                  key={g.id}
                  onClick={() => setSelectedGoal(g)}
                  className={`w-full text-left p-4 rounded-xl border transition-all ${
                    selectedGoal?.id === g.id 
                      ? 'bg-blue-500/10 border-blue-500/50' 
                      : 'bg-white/[0.02] border-white/5 hover:border-white/20'
                  }`}
                >
                  <div className="flex items-center justify-between mb-1">
                    <h4 className="font-medium text-white truncate">{g.title}</h4>
                    <span className="text-[10px] uppercase bg-black px-2 py-0.5 rounded text-slate-400 border border-white/10">{g.domain}</span>
                  </div>
                  <div className="flex items-center gap-1.5 text-xs text-slate-500 mt-2">
                    <Clock className="w-3.5 h-3.5" />
                    <span>Level: {g.current_level}</span>
                  </div>
                </button>
              ))}
            </div>
          </div>

          <div className="col-span-8">
            {selectedGoal ? (
              <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6">
                <div className="mb-6 pb-6 border-b border-white/10">
                  <h2 className="text-2xl font-bold text-white mb-2 flex items-center gap-3">
                    {selectedGoal.title}
                  </h2>
                  <p className="text-slate-400 text-sm">Target: {selectedGoal.target_level}</p>
                </div>

                <div className="bg-blue-500/5 border border-blue-500/20 p-4 rounded-xl mb-6 flex gap-3 text-blue-200 text-sm">
                  <BookOpen className="w-5 h-5 shrink-0 text-blue-400" />
                  <p><strong>Note:</strong> Learner generates realistic study plans. We do not over-promise mastery in unrealistic timeframes. Be honest with your time budget!</p>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <button onClick={() => handleAction(selectedGoal.id, 'roadmap')} className="p-4 border border-white/10 hover:border-blue-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <Compass className="w-5 h-5 text-blue-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Generate Roadmap</h3>
                    <p className="text-xs text-slate-500 mt-1">Structured learning phases and modules.</p>
                  </button>
                  <button onClick={() => handleAction(selectedGoal.id, 'today')} className="p-4 border border-white/10 hover:border-blue-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <ListTodo className="w-5 h-5 text-blue-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Today's Plan</h3>
                    <p className="text-xs text-slate-500 mt-1">Get 1-2 hour concentrated daily tasks.</p>
                  </button>
                  <button onClick={() => handleAction(selectedGoal.id, 'practice')} className="p-4 border border-white/10 hover:border-blue-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <Code className="w-5 h-5 text-blue-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Practice Set</h3>
                    <p className="text-xs text-slate-500 mt-1">Generate practical problems to solve.</p>
                  </button>
                  <button onClick={() => handleAction(selectedGoal.id, 'revise')} className="p-4 border border-white/10 hover:border-blue-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <Brain className="w-5 h-5 text-blue-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Revision Checkpoint</h3>
                    <p className="text-xs text-slate-500 mt-1">Check confidence and catch mistakes.</p>
                  </button>
                  <button onClick={() => handleAction(selectedGoal.id, 'progress')} className="p-4 border border-white/10 hover:border-blue-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <CheckCircle2 className="w-5 h-5 text-blue-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Log Progress</h3>
                    <p className="text-xs text-slate-500 mt-1">Track what you learned today.</p>
                  </button>
                  <button onClick={() => handleAction(selectedGoal.id, 'report')} className="p-4 border border-white/10 hover:border-blue-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <FileText className="w-5 h-5 text-blue-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Learning Report</h3>
                    <p className="text-xs text-slate-500 mt-1">Generate a summary of weak/strong areas.</p>
                  </button>
                </div>
              </div>
            ) : (
              <div className="h-full min-h-[400px] border border-white/5 border-dashed rounded-2xl flex items-center justify-center text-slate-500 flex-col gap-4">
                <BookOpen className="w-12 h-12 opacity-20" />
                <p>Select a goal or create a new one.</p>
              </div>
            )}
          </div>

        </div>
      </div>
    </div>
  );
}
