"use client";

import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  Activity, Shield, Terminal, Zap, BookOpen, Clock, Play,
  Search, Download, Eye, FileText, ChevronRight
} from "lucide-react";
import { goatApi } from '@/lib/goat-api';

interface TimelineEvent {
  id: string;
  timestamp: number;
  kind: string;
  title: string;
  summary: string;
  actor: string;
  risk_level: string;
  source: string;
}

export default function TimelinePage() {
  const [events, setEvents] = useState<TimelineEvent[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  const [replayMode, setReplayMode] = useState(false);

  useEffect(() => {
    fetchEvents();
  }, []);

  const fetchEvents = async (query?: string) => {
    setLoading(true);
    try {
      const url = query 
        ? `/v1/timeline/search?q=${encodeURIComponent(query)}`
        : "/v1/timeline/recent";
      
      const data = await goatApi.get(url);
      setEvents(data.events || []);
    } catch (e) {
      console.error("Failed to fetch timeline", e);
    }
    setLoading(false);
  };

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    fetchEvents(searchQuery);
  };

  const handleExport = async () => {
    try {
      await goatApi.post("/v1/timeline/export", {});
      alert("Timeline exported successfully.");
    } catch (e) {
      console.error(e);
    }
  };

  const getIcon = (kind: string) => {
    if (kind.includes("Approval")) return <Shield className="w-5 h-5 text-blue-400" />;
    if (kind.includes("Job") || kind.includes("Tool")) return <Terminal className="w-5 h-5 text-green-400" />;
    if (kind.includes("Skill") || kind.includes("Recipe")) return <Zap className="w-5 h-5 text-yellow-400" />;
    if (kind.includes("Memory")) return <BookOpen className="w-5 h-5 text-purple-400" />;
    return <Activity className="w-5 h-5 text-gray-400" />;
  };

  return (
    <div className="flex-1 p-8 text-white min-h-screen bg-[#050505]">
      <div className="max-w-6xl mx-auto space-y-8">
        
        {/* Header */}
        <div className="flex items-end justify-between border-b border-white/10 pb-6">
          <div>
            <h1 className="text-3xl font-light tracking-tight flex items-center gap-3">
              <Clock className="text-indigo-400" />
              Project Timeline
            </h1>
            <p className="text-gray-400 mt-2 font-mono text-sm">
              Work history replay, approvals, and context reconstruction.
            </p>
          </div>
          <div className="flex items-center gap-4">
            <button 
              onClick={handleExport}
              className="px-4 py-2 bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 flex items-center gap-2 transition-colors"
            >
              <Download className="w-4 h-4" /> Export
            </button>
            <button 
              onClick={() => setReplayMode(!replayMode)}
              className={`px-4 py-2 rounded-lg flex items-center gap-2 transition-all ${
                replayMode ? 'bg-indigo-500 text-white' : 'bg-white/5 border border-white/10 hover:bg-white/10'
              }`}
            >
              <Play className="w-4 h-4" /> Replay Mode
            </button>
          </div>
        </div>

        {/* Toolbar */}
        <div className="flex gap-4">
          <form onSubmit={handleSearch} className="flex-1 relative">
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
            <input 
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search timeline (e.g., 'approvals', 'dashboard polish')..."
              className="w-full bg-white/5 border border-white/10 rounded-xl py-3 pl-12 pr-4 focus:outline-none focus:border-indigo-500/50 focus:bg-white/10 transition-all font-mono text-sm"
            />
          </form>
          <div className="flex items-center gap-2 px-4 py-2 bg-indigo-500/10 border border-indigo-500/20 text-indigo-400 rounded-xl text-sm font-mono cursor-pointer hover:bg-indigo-500/20 transition-colors">
            <Eye className="w-4 h-4" /> Privacy: Standard
          </div>
        </div>

        {/* Timeline Feed */}
        <div className="relative pt-4">
          {/* Vertical Line */}
          <div className="absolute left-8 top-0 bottom-0 w-[1px] bg-gradient-to-b from-indigo-500/50 via-white/10 to-transparent"></div>

          {loading ? (
            <div className="pl-20 py-8 text-gray-500 font-mono animate-pulse">Reconstructing timeline...</div>
          ) : events.length === 0 ? (
            <div className="pl-20 py-8 text-gray-500 font-mono">No events found matching your query.</div>
          ) : (
            <div className="space-y-6">
              <AnimatePresence>
                {events.map((event, idx) => (
                  <motion.div 
                    initial={{ opacity: 0, x: -20 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: idx * 0.05 }}
                    key={event.id}
                    className="relative pl-20 group"
                  >
                    {/* Node Dot */}
                    <div className="absolute left-[31px] top-4 w-2 h-2 rounded-full bg-indigo-400 ring-4 ring-[#050505] group-hover:bg-white transition-colors"></div>

                    {/* Content Card */}
                    <div className="p-5 rounded-2xl bg-white/[0.02] border border-white/[0.05] hover:bg-white/[0.04] transition-all backdrop-blur-md">
                      <div className="flex justify-between items-start mb-3">
                        <div className="flex items-center gap-3">
                          <div className="p-2 bg-white/5 rounded-lg border border-white/5">
                            {getIcon(event.kind)}
                          </div>
                          <div>
                            <h3 className="font-medium text-gray-200">{event.title}</h3>
                            <div className="flex items-center gap-2 mt-1">
                              <span className="text-xs font-mono text-gray-500 bg-black/50 px-2 py-0.5 rounded border border-white/5">
                                {event.kind}
                              </span>
                              <span className="text-xs font-mono text-gray-500">
                                {new Date(event.timestamp * 1000).toLocaleTimeString()}
                              </span>
                            </div>
                          </div>
                        </div>
                        {event.risk_level && event.risk_level !== 'None' && (
                          <span className={`text-xs px-2 py-1 rounded font-mono border ${
                            event.risk_level === 'High' ? 'bg-red-500/10 text-red-400 border-red-500/20' : 'bg-yellow-500/10 text-yellow-400 border-yellow-500/20'
                          }`}>
                            Risk: {event.risk_level}
                          </span>
                        )}
                      </div>
                      <p className="text-gray-400 text-sm leading-relaxed mb-4">
                        {event.summary}
                      </p>
                      
                      <div className="flex gap-3">
                        <button className="text-xs font-mono text-indigo-400 hover:text-indigo-300 flex items-center gap-1 transition-colors">
                          <FileText className="w-3 h-3" /> Related Context
                        </button>
                        <button className="text-xs font-mono text-indigo-400 hover:text-indigo-300 flex items-center gap-1 transition-colors">
                          <Search className="w-3 h-3" /> Open in Brain
                        </button>
                      </div>
                    </div>
                  </motion.div>
                ))}
              </AnimatePresence>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
