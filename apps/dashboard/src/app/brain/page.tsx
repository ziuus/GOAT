"use client";

import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  Search, 
  BrainCircuit, 
  Lock, 
  BookOpen, 
  Database,
  ShieldCheck,
  RefreshCw,
  Library,
  Workflow,
  Download,
  TerminalSquare
} from "lucide-react";
import { goatApi } from "@/lib/goat-api";

export default function BrainSearchPage() {
  const [query, setQuery] = useState("");
  const [isSearching, setIsSearching] = useState(false);
  const [results, setResults] = useState<any[]>([]);
  const [status, setStatus] = useState<any>(null);
  const [embedStatus, setEmbedStatus] = useState<any>(null);
  const [activeTab, setActiveTab] = useState("all");
  const [searchMode, setSearchMode] = useState("keyword");

  useEffect(() => {
    fetchStatus();
    
    // Check for query in URL
    if (typeof window !== 'undefined') {
      const params = new URLSearchParams(window.location.search);
      const q = params.get('q');
      const m = params.get('mode');
      if (q) {
        setQuery(q);
        if (m) setSearchMode(m);
        // We delay the search slightly to let state update
        setTimeout(() => {
          triggerSearch(q, m || 'keyword');
        }, 100);
      }
    }
  }, []);

  const triggerSearch = async (q: string, mode: string) => {
    if (!q) return;
    setIsSearching(true);
    try {
      const res = await goatApi.searchBrain(q, mode);
      setResults(res.results || []);
    } catch (e) {
      console.error(e);
    } finally {
      setIsSearching(false);
    }
  };

  const fetchStatus = async () => {
    try {
      const res = await goatApi.getBrainStatus();
      setStatus(res);
      const eres = await goatApi.getEmbeddingsStatus();
      setEmbedStatus(eres);
    } catch (e) {
      console.error(e);
    }
  };

  const handleSearch = () => {
    triggerSearch(query, searchMode);
  };

  const handleReindex = async () => {
    try {
      await goatApi.reindexBrain();
      fetchStatus();
    } catch (e) {
      console.error(e);
    }
  };

  const handleRebuildEmbeddings = async () => {
    try {
      await goatApi.rebuildEmbeddings();
      fetchStatus();
    } catch (e) {
      console.error(e);
    }
  };

  const getIcon = (kind: string) => {
    switch (kind) {
      case "skill": return <Library className="w-5 h-5 text-fuchsia-400" />;
      case "recipe": return <Workflow className="w-5 h-5 text-violet-400" />;
      case "memory": return <BrainCircuit className="w-5 h-5 text-blue-400" />;
      case "job": return <TerminalSquare className="w-5 h-5 text-emerald-400" />;
      case "installed": return <Download className="w-5 h-5 text-slate-400" />;
      case "builder_validation_failure": 
      case "builder_retry_plan":
      case "builder_fix_outcome":
      case "builder_fix_lesson":
      case "builder_recurring_mistake":
      case "builder_project_learning":
        return <BrainCircuit className="w-5 h-5 text-amber-400" />;
      case "researcher_project":
      case "researcher_source":
      case "researcher_citation":
      case "researcher_claim":
      case "researcher_finding":
      case "researcher_brief":
      case "researcher_competitor_profile":
      case "researcher_competitor_report":
      case "researcher_technology_comparison":
        return <BookOpen className="w-5 h-5 text-sky-400" />;
      default: return <Database className="w-5 h-5 text-slate-400" />;
    }
  };

  return (
    <div className="flex-1 h-full flex flex-col relative overflow-hidden bg-[#0A0A0B] text-slate-200">
      {/* Background Effects */}
      <div className="absolute top-[10%] left-[20%] w-[40%] h-[40%] rounded-full bg-violet-600/10 blur-[150px] pointer-events-none" />
      <div className="absolute bottom-[20%] right-[10%] w-[30%] h-[30%] rounded-full bg-blue-600/10 blur-[120px] pointer-events-none" />

      {/* Header */}
      <header className="px-8 py-6 border-b border-white/5 bg-white/[0.02] backdrop-blur-md z-10 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-violet-500/20 to-blue-500/20 border border-white/10 flex items-center justify-center shadow-lg shadow-violet-500/10">
            <BrainCircuit className="w-6 h-6 text-violet-400" />
          </div>
          <div>
            <h1 className="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-white/60">Brain Search & Recall</h1>
            <p className="text-sm text-slate-400">Search memories, skills, recipes, and more</p>
          </div>
        </div>
        <div className="flex gap-4">
          <div className="flex flex-col items-end">
            <div className="text-sm font-medium text-white">{status?.total_documents || 0} Docs Indexed</div>
            <div className="text-xs text-slate-400 flex items-center gap-1">
              <ShieldCheck className="w-3 h-3 text-emerald-400" /> Local Only
            </div>
            {embedStatus?.enabled && (
              <div className="text-xs text-violet-400 mt-1">
                {embedStatus.total_vectors} Vectors ({embedStatus.provider})
              </div>
            )}
          </div>
          <div className="flex gap-2">
            <button onClick={handleRebuildEmbeddings} title="Rebuild Embeddings" className="p-3 bg-white/5 hover:bg-white/10 rounded-xl border border-white/10 transition-colors group">
              <Database className="w-5 h-5 text-slate-400 group-hover:text-violet-400" />
            </button>
            <button onClick={handleReindex} title="Deep Reindex" className="p-3 bg-white/5 hover:bg-white/10 rounded-xl border border-white/10 transition-colors group">
              <RefreshCw className="w-5 h-5 text-slate-400 group-hover:text-white" />
            </button>
          </div>
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 p-8 overflow-y-auto custom-scrollbar relative z-10 flex flex-col items-center">
        <div className="w-full max-w-4xl space-y-8">
          
          {/* Search Bar */}
          <div className="flex gap-4 w-full">
            <div className="flex-1 relative group">
              <div className="absolute inset-y-0 left-0 pl-5 flex items-center pointer-events-none">
                <Search className="w-6 h-6 text-slate-500 group-focus-within:text-violet-400 transition-colors" />
              </div>
              <input
                type="text"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
                placeholder="Ask your brain anything..."
                className="w-full bg-black/40 border-2 border-white/10 rounded-2xl py-4 pl-14 pr-6 text-lg text-white placeholder:text-slate-500 focus:outline-none focus:border-violet-500/50 transition-colors shadow-inner"
              />
            </div>
            <select
              value={searchMode}
              onChange={(e) => setSearchMode(e.target.value)}
              className="bg-black/40 border-2 border-white/10 rounded-2xl px-4 py-4 text-white focus:outline-none focus:border-violet-500/50 appearance-none"
            >
              <option value="keyword">Keyword</option>
              <option value="fuzzy">Fuzzy</option>
              {embedStatus?.enabled && <option value="semantic">Semantic</option>}
              {embedStatus?.enabled && <option value="hybrid">Hybrid</option>}
            </select>
            <button  
              onClick={handleSearch}
              disabled={isSearching}
              className="px-8 py-4 rounded-2xl bg-gradient-to-r from-violet-600 to-blue-600 hover:from-violet-500 hover:to-blue-500 text-white font-medium text-lg shadow-lg shadow-violet-500/20 disabled:opacity-50 transition-all hover:scale-105 active:scale-95 flex items-center gap-2"
            >
              {isSearching ? <RefreshCw className="w-5 h-5 animate-spin" /> : "Recall"}
            </button>
          </div>

          {/* Privacy Note */}
          <div className="flex items-center justify-center gap-2 text-xs text-slate-500 bg-white/5 py-2 px-4 rounded-full w-fit mx-auto border border-white/5">
            <Lock className="w-3.5 h-3.5" />
            Secrets and tokens are automatically redacted before indexing.
          </div>

          {/* Results */}
          <div className="space-y-4">
            {results.map((res: any, idx) => (
              <motion.div 
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: idx * 0.05 }}
                key={res.document.id} 
                className="bg-white/5 border border-white/10 rounded-2xl p-6 hover:bg-white/[0.07] transition-colors"
              >
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <div className="p-2 bg-black/30 rounded-lg border border-white/5">
                      {getIcon(res.document.kind)}
                    </div>
                    <div>
                      <h3 className="text-lg font-bold text-white flex items-center gap-2">
                        {res.document.title}
                        <span className="px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-white/10 text-slate-300">
                          {res.document.kind}
                        </span>
                      </h3>
                      <div className="text-xs text-slate-400">
                        Score: {res.score.toFixed(2)} (Kw: {res.keyword_score?.toFixed(1) || 0}, Sem: {res.semantic_score?.toFixed(2) || 0}) • {res.match_reason}
                      </div>
                    </div>
                  </div>
                  <div className="text-xs text-slate-500">{new Date(res.document.created_at).toLocaleString()}</div>
                </div>
                <p className="text-sm text-slate-300 mb-4">{res.document.summary}</p>
                
                <div className="bg-black/50 p-4 rounded-xl border border-white/5 text-sm font-mono text-slate-400 overflow-hidden text-ellipsis whitespace-nowrap">
                  {res.document.body}
                </div>
              </motion.div>
            ))}
            
            {results.length === 0 && !isSearching && query && (
              <div className="text-center py-20 text-slate-500">
                <BrainCircuit className="w-12 h-12 mx-auto mb-4 opacity-20" />
                No related memories or artifacts found.
              </div>
            )}
          </div>

        </div>
      </div>
    </div>
  );
}
