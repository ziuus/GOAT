"use client";

import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  Users, Bot, Workflow, BrainCircuit, Activity, 
  Power, FileText, Link as LinkIcon, Search, 
  TerminalSquare, ShieldCheck, Zap
} from "lucide-react";

// Mock data for agents
const MOCK_AGENTS = [
  {
    id: "agent-1",
    name: "Architect Prime",
    tier: "Prime",
    status: "active",
    description: "Main orchestrator for system design and module planning.",
    domainResponsibilities: ["System Architecture", "Security", "Scalability"],
    parentPrime: null
  },
  {
    id: "agent-2",
    name: "Frontend Specialist",
    tier: "Specialist",
    status: "experimental",
    description: "Handles UI/UX implementations, React/Next.js components.",
    domainResponsibilities: ["UI Components", "State Management", "Styling"],
    parentPrime: "Architect Prime"
  },
  {
    id: "agent-3",
    name: "DB Subagent",
    tier: "Subagent",
    status: "planned",
    description: "Temporary agent spawned for specific DB migration tasks.",
    domainResponsibilities: ["Schema Updates", "Query Optimization"],
    parentPrime: "Backend Prime"
  },
  {
    id: "agent-4",
    name: "Security Prime",
    tier: "Prime",
    status: "active",
    description: "Monitors overall system security, reviews PRs for vulnerabilities.",
    domainResponsibilities: ["AuthZ/AuthN", "Audit Logs", "Compliance"],
    parentPrime: null
  },
  {
    id: "agent-5",
    name: "Cofounder Prime",
    tier: "Prime",
    status: "active",
    description: "Evaluates ideas, generates MVP specs, and creates validation plans.",
    domainResponsibilities: ["Ideation", "Validation", "Market Research"],
    parentPrime: null
  },
  {
    id: "agent-6",
    name: "Researcher Prime",
    tier: "Prime",
    status: "active",
    description: "Deep source-grounded research, technology comparisons, and competitor analysis.",
    domainResponsibilities: ["Research", "Comparisons", "Brief Generation"],
    parentPrime: null
  }
];

const tiers = [
  { id: "all", label: "All Agents" },
  { id: "Prime", label: "Prime Agents" },
  { id: "Specialist", label: "Specialist Agents" },
  { id: "Subagent", label: "Subagents" }
];

export default function AgentsPage() {
  const [activeTab, setActiveTab] = useState("all");
  const [searchQuery, setSearchQuery] = useState("");
  const [agents, setAgents] = useState(MOCK_AGENTS);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // Simulate API fetch: GET /v1/agents
    const timer = setTimeout(() => {
      setAgents(MOCK_AGENTS);
      setIsLoading(false);
    }, 800);
    return () => clearTimeout(timer);
  }, []);

  const filteredAgents = agents.filter(agent => {
    const matchesTab = activeTab === "all" || agent.tier === activeTab;
    const matchesSearch = agent.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
                          agent.description.toLowerCase().includes(searchQuery.toLowerCase());
    return matchesTab && matchesSearch;
  });

  const getStatusColor = (status: string) => {
    switch (status) {
      case "active": return "text-emerald-400 bg-emerald-400/10 border-emerald-400/20";
      case "experimental": return "text-amber-400 bg-amber-400/10 border-amber-400/20";
      case "planned": return "text-slate-400 bg-slate-400/10 border-slate-400/20";
      default: return "text-slate-400 bg-slate-400/10 border-slate-400/20";
    }
  };

  const containerVariants = {
    hidden: { opacity: 0 },
    visible: { 
      opacity: 1, 
      transition: { staggerChildren: 0.1 }
    }
  };

  const cardVariants = {
    hidden: { opacity: 0, y: 20 },
    visible: { opacity: 1, y: 0 }
  };

  return (
    <div className="flex-1 h-full flex flex-col relative overflow-hidden bg-[#0A0A0B] text-slate-200">
      {/* Background Orbs */}
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] rounded-full bg-indigo-600/10 blur-[120px] pointer-events-none" />
      <div className="absolute bottom-[-10%] right-[-10%] w-[30%] h-[30%] rounded-full bg-violet-600/10 blur-[100px] pointer-events-none" />

      {/* Header */}
      <header className="px-8 py-6 border-b border-white/5 bg-white/[0.02] backdrop-blur-md z-10 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-indigo-500/20 to-purple-500/20 border border-white/10 flex items-center justify-center shadow-lg shadow-indigo-500/10">
            <Users className="w-6 h-6 text-indigo-400" />
          </div>
          <div>
            <h1 className="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-white/60">Agents</h1>
            <p className="text-sm text-slate-400">Manage and monitor GOAT Agent Architecture</p>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <div className="relative">
            <Search className="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
            <input 
              type="text" 
              placeholder="Search agents..." 
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="bg-black/40 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-white placeholder:text-slate-500 focus:outline-none focus:border-indigo-500/50 w-64"
            />
          </div>
          <button className="px-4 py-2 rounded-lg bg-indigo-500 hover:bg-indigo-600 text-white transition-colors flex items-center gap-2 text-sm shadow-lg shadow-indigo-500/20 font-medium">
            <Bot className="w-4 h-4" />
            <span>New Agent</span>
          </button>
        </div>
      </header>

      {/* Main Content */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-8 z-10">
        <div className="max-w-7xl mx-auto flex flex-col gap-6">
          
          {/* Tabs */}
          <div className="flex items-center gap-2 border-b border-white/5 pb-4">
            {tiers.map(tab => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                  activeTab === tab.id
                    ? "bg-indigo-500/20 text-indigo-300 border border-indigo-500/30"
                    : "text-slate-400 hover:text-white hover:bg-white/5 border border-transparent"
                }`}
              >
                {tab.label}
              </button>
            ))}
          </div>

          {/* Agents Grid */}
          {isLoading ? (
            <div className="flex items-center justify-center h-64">
              <span className="w-8 h-8 border-4 border-indigo-500/30 border-t-indigo-500 rounded-full animate-spin" />
            </div>
          ) : (
            <motion.div 
              variants={containerVariants}
              initial="hidden"
              animate="visible"
              className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6"
            >
              <AnimatePresence>
                {filteredAgents.map(agent => (
                  <motion.div
                    key={agent.id}
                    variants={cardVariants}
                    layout
                    initial={{ opacity: 0, scale: 0.9 }}
                    animate={{ opacity: 1, scale: 1 }}
                    exit={{ opacity: 0, scale: 0.9 }}
                    transition={{ duration: 0.2 }}
                    className="bg-white/[0.03] border border-white/10 rounded-2xl p-6 hover:border-indigo-500/50 transition-colors group relative overflow-hidden flex flex-col"
                  >
                    <div className="absolute top-0 left-0 w-1 h-full bg-gradient-to-b from-indigo-500 to-violet-500 opacity-0 group-hover:opacity-100 transition-opacity" />
                    
                    <div className="flex justify-between items-start mb-4">
                      <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded-xl bg-black/40 border border-white/10 flex items-center justify-center">
                          {agent.tier === "Prime" ? <BrainCircuit className="w-5 h-5 text-indigo-400" /> :
                           agent.tier === "Specialist" ? <TerminalSquare className="w-5 h-5 text-violet-400" /> :
                           <Workflow className="w-5 h-5 text-slate-400" />}
                        </div>
                        <div>
                          <h3 className="font-semibold text-white">{agent.name}</h3>
                          <span className="text-xs text-slate-400">{agent.tier}</span>
                        </div>
                      </div>
                      <span className={`text-[10px] uppercase font-bold px-2 py-1 rounded-md border ${getStatusColor(agent.status)}`}>
                        {agent.status}
                      </span>
                    </div>

                    <p className="text-sm text-slate-300 mb-4 flex-1">{agent.description}</p>

                    <div className="space-y-3 mb-6">
                      <div>
                        <span className="text-xs font-medium text-slate-500 uppercase tracking-wider block mb-1.5">Domains</span>
                        <div className="flex flex-wrap gap-1.5">
                          {agent.domainResponsibilities.map(domain => (
                            <span key={domain} className="text-xs px-2 py-1 bg-white/5 border border-white/10 rounded-md text-slate-300">
                              {domain}
                            </span>
                          ))}
                        </div>
                      </div>
                      {agent.parentPrime && (
                        <div className="flex items-center gap-2 text-xs text-slate-400">
                          <LinkIcon className="w-3 h-3" />
                          <span>Reports to: <span className="text-indigo-300">{agent.parentPrime}</span></span>
                        </div>
                      )}
                    </div>

                    {/* Actions */}
                    
                    {agent.id === "agent-6" ? (
                      <div className="grid grid-cols-2 gap-2 pt-4 border-t border-white/5 mt-auto">
                        <button 
                          onClick={() => window.location.href = '/designer'}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 rounded-lg text-xs font-medium transition-colors border border-indigo-500/20"
                        >
                          <Zap className="w-3.5 h-3.5" /> Start Review
                        </button>
                        <button 
                          onClick={() => alert(`Reviewing Dashboard`)}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-medium transition-colors border border-white/10"
                        >
                          <ShieldCheck className="w-3.5 h-3.5" /> Review Dashboard
                        </button>
                        <button 
                          onClick={() => alert(`Reviewing Landing Page`)}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-medium transition-colors border border-white/10"
                        >
                          <FileText className="w-3.5 h-3.5" /> Review Landing
                        </button>
                        <button 
                          onClick={() => window.location.href = '/designer'}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-medium transition-colors border border-white/10"
                        >
                          <Workflow className="w-3.5 h-3.5" /> Open Designer
                        </button>
                      </div>
                    ) : agent.id === "agent-5" ? (
                      <div className="grid grid-cols-2 gap-2 pt-4 border-t border-white/5 mt-auto">
                        <button 
                          onClick={() => alert(`Creating new idea`)}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 rounded-lg text-xs font-medium transition-colors border border-indigo-500/20"
                        >
                          <Zap className="w-3.5 h-3.5" /> New Idea
                        </button>
                        <button 
                          onClick={() => alert(`Validating idea`)}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-medium transition-colors border border-white/10"
                        >
                          <ShieldCheck className="w-3.5 h-3.5" /> Validate Idea
                        </button>
                        <button 
                          onClick={() => alert(`Generating report`)}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-medium transition-colors border border-white/10"
                        >
                          <FileText className="w-3.5 h-3.5" /> Generate Report
                        </button>
                        <button 
                          onClick={() => window.location.href = '/cofounder'}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-medium transition-colors border border-white/10"
                        >
                          <Workflow className="w-3.5 h-3.5" /> Open Dashboard
                        </button>
                      </div>
                    ) : (
                      <div className="grid grid-cols-2 gap-2 pt-4 border-t border-white/5 mt-auto">
                        <button 
                          onClick={() => alert(`Running task on ${agent.name}`)}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 rounded-lg text-xs font-medium transition-colors border border-indigo-500/20"
                        >
                          <Zap className="w-3.5 h-3.5" /> Run Task
                        </button>
                        <button 
                          onClick={() => alert(`Toggling state for ${agent.name}`)}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-medium transition-colors border border-white/10"
                        >
                          <Power className="w-3.5 h-3.5" /> Toggle
                        </button>
                        <button 
                          onClick={() => alert(`Viewing reports for ${agent.name}`)}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-medium transition-colors border border-white/10"
                        >
                          <FileText className="w-3.5 h-3.5" /> Reports
                        </button>
                        <button 
                          onClick={() => alert(`Attaching specialist to ${agent.name}`)}
                          className="flex items-center justify-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-medium transition-colors border border-white/10"
                        >
                          <ShieldCheck className="w-3.5 h-3.5" /> Attach
                        </button>
                      </div>
                    )}

                  </motion.div>
                ))}
              </AnimatePresence>
            </motion.div>
          )}

          {!isLoading && filteredAgents.length === 0 && (
            <div className="text-center py-20 text-slate-500">
              <Bot className="w-12 h-12 mx-auto mb-4 opacity-20" />
              <p>No agents found matching your criteria.</p>
            </div>
          )}

        </div>
      </div>
    </div>
  );
}
