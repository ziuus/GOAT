'use client';

import { useState, useEffect } from 'react';
import { researcherApi } from '@/lib/goat-api';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  Search, BookOpen, Layers, Target, FileText, Bot, 
  ChevronRight, ShieldAlert, AlertTriangle, AlertCircle, Plus
} from 'lucide-react';

export default function ResearcherPage() {
  const [topics, setTopics] = useState<any[]>([]);
  const [selectedTopic, setSelectedTopic] = useState<any | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [newTitle, setNewTitle] = useState('');
  const [newQuestion, setNewQuestion] = useState('');
  const [newDomain, setNewDomain] = useState('');

  useEffect(() => {
    loadTopics();
  }, []);

  const loadTopics = async () => {
    try {
      setLoading(true);
      const res = await researcherApi.listTopics();
      if (res.topics) setTopics(res.topics);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async () => {
    try {
      const res = await researcherApi.createTopic({
        title: newTitle || 'Untitled Topic',
        research_question: newQuestion || 'What is the current state of art?',
        domain: newDomain || 'Technology',
      });
      if (res.topic) {
        setTopics([...topics, res.topic]);
        setSelectedTopic(res.topic);
        setNewTitle('');
        setNewQuestion('');
        setNewDomain('');
      }
    } catch (err: any) {
      setError(err.message);
    }
  };

  const handleAction = async (id: string, action: string) => {
    try {
      let res: any;
      if (action === 'plan') res = await researcherApi.createPlan(id);
      else if (action === 'competitors') res = await researcherApi.generateCompetitors(id);
      else if (action === 'compare') res = await researcherApi.generateCompare(id);
      else if (action === 'market') res = await researcherApi.generateMarket(id);
      else if (action === 'brief') res = await researcherApi.generateBrief(id);
      else if (action === 'report') res = await researcherApi.generateReport(id);

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
              <Search className="w-8 h-8 text-indigo-400" />
              Researcher
            </h1>
            <p className="text-sm text-slate-400 mt-2">
              Source-Grounded Research Briefs & Technology Comparisons. (Note: Claims are evidence-based but not guaranteed truth).
            </p>
          </div>
          <div className="flex items-center gap-2">
            <span className="flex items-center gap-1.5 px-3 py-1 bg-amber-500/10 text-amber-400 rounded-full text-xs font-medium border border-amber-500/20">
              <AlertTriangle className="w-3.5 h-3.5" /> Source Checking Active
            </span>
            <span className="flex items-center gap-1.5 px-3 py-1 bg-indigo-500/10 text-indigo-400 rounded-full text-xs font-medium border border-indigo-500/20">
              <Bot className="w-3.5 h-3.5" /> Prime Agent
            </span>
          </div>
        </header>

        {error && (
          <div className="p-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm flex items-center gap-2">
            <AlertCircle className="w-4 h-4" /> {error}
          </div>
        )}

        <div className="grid grid-cols-12 gap-8">
          
          <div className="col-span-4 space-y-4">
            <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-5">
              <h2 className="text-sm font-semibold text-white mb-4 flex items-center gap-2">
                <Plus className="w-4 h-4" /> New Topic
              </h2>
              <div className="space-y-3">
                <input 
                  type="text" 
                  placeholder="Topic Title (e.g. AI IDEs)" 
                  value={newTitle}
                  onChange={e => setNewTitle(e.target.value)}
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-indigo-500 transition-colors"
                />
                <input 
                  type="text" 
                  placeholder="Domain (e.g. Developer Tools)" 
                  value={newDomain}
                  onChange={e => setNewDomain(e.target.value)}
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-indigo-500 transition-colors"
                />
                <textarea 
                  placeholder="Research Question..." 
                  value={newQuestion}
                  onChange={e => setNewQuestion(e.target.value)}
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-indigo-500 transition-colors h-24 resize-none"
                />
                <button 
                  onClick={handleCreate}
                  className="w-full bg-indigo-500 hover:bg-indigo-600 text-white font-medium text-sm py-2 rounded-lg transition-colors"
                >
                  Create Topic
                </button>
              </div>
            </div>

            <div className="space-y-2">
              <h3 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-3">Active Topics</h3>
              {topics.map(t => (
                <button
                  key={t.id}
                  onClick={() => setSelectedTopic(t)}
                  className={`w-full text-left p-4 rounded-xl border transition-all ${
                    selectedTopic?.id === t.id 
                      ? 'bg-indigo-500/10 border-indigo-500/50' 
                      : 'bg-white/[0.02] border-white/5 hover:border-white/20'
                  }`}
                >
                  <div className="flex items-center justify-between mb-1">
                    <h4 className="font-medium text-white truncate">{t.title}</h4>
                  </div>
                  <p className="text-xs text-slate-400 truncate">{t.research_question}</p>
                </button>
              ))}
            </div>
          </div>

          <div className="col-span-8">
            {selectedTopic ? (
              <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6">
                <div className="mb-6 pb-6 border-b border-white/10">
                  <h2 className="text-2xl font-bold text-white mb-2">{selectedTopic.title}</h2>
                  <p className="text-slate-400 text-sm">{selectedTopic.research_question}</p>
                  <div className="flex gap-2 mt-4">
                    <span className="px-2 py-1 bg-white/5 rounded text-xs">{selectedTopic.domain}</span>
                    <span className="px-2 py-1 bg-white/5 rounded text-xs">{selectedTopic.status}</span>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <button onClick={() => handleAction(selectedTopic.id, 'plan')} className="p-4 border border-white/10 hover:border-indigo-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <Layers className="w-5 h-5 text-indigo-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Generate Plan</h3>
                    <p className="text-xs text-slate-500 mt-1">Scope keywords & source priorities.</p>
                  </button>
                  <button onClick={() => handleAction(selectedTopic.id, 'competitors')} className="p-4 border border-white/10 hover:border-indigo-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <Target className="w-5 h-5 text-indigo-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Competitor Scan</h3>
                    <p className="text-xs text-slate-500 mt-1">Analyze market competitors.</p>
                  </button>
                  <button onClick={() => handleAction(selectedTopic.id, 'compare')} className="p-4 border border-white/10 hover:border-indigo-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <BookOpen className="w-5 h-5 text-indigo-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Tech Comparison</h3>
                    <p className="text-xs text-slate-500 mt-1">Evaluate frameworks or tools.</p>
                  </button>
                  <button onClick={() => handleAction(selectedTopic.id, 'brief')} className="p-4 border border-white/10 hover:border-indigo-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <FileText className="w-5 h-5 text-indigo-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Research Brief</h3>
                    <p className="text-xs text-slate-500 mt-1">Source-grounded executive summary.</p>
                  </button>
                  <button onClick={() => handleAction(selectedTopic.id, 'report')} className="p-4 border border-white/10 hover:border-indigo-500/50 rounded-xl bg-black/40 text-left group transition-all col-span-2">
                    <ShieldAlert className="w-5 h-5 text-indigo-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Generate Final Report</h3>
                    <p className="text-xs text-slate-500 mt-1">Produce full markdown document into GOAT reporting system.</p>
                  </button>
                </div>
              </div>
            ) : (
              <div className="h-full min-h-[400px] border border-white/5 border-dashed rounded-2xl flex items-center justify-center text-slate-500 flex-col gap-4">
                <Search className="w-12 h-12 opacity-20" />
                <p>Select a topic or create a new one to begin researching.</p>
              </div>
            )}
          </div>

        </div>
      </div>
    </div>
  );
}
