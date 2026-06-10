"use client";

import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  Search, 
  Download, 
  ShieldCheck, 
  ShieldAlert, 
  TerminalSquare, 
  Globe, 
  BrainCircuit, 
  Library,
  GitBranch,
  Trash2,
  RefreshCw,
  Info
} from "lucide-react";

const tabs = [
  { id: "marketplace", label: "Remote Marketplace", icon: Globe },
  { id: "installed", label: "Installed", icon: Download },
  { id: "local", label: "Local", icon: TerminalSquare },
  { id: "learned", label: "Learned", icon: BrainCircuit },
  { id: "drafts", label: "Studio Drafts", icon: GitBranch },
  { id: "audit", label: "Audit Reports", icon: ShieldCheck },
];

const mockRemoteSkills = [
  { id: "sk_1", name: "rust-test-generator", author: "goat-community", downloads: 1402, rating: 4.8, description: "Automatically generates idiomatic Rust tests for any selected struct or function.", risk: "low" },
  { id: "sk_2", name: "nextjs-component-scaffold", author: "frontend-wizards", downloads: 853, rating: 4.5, description: "Scaffolds a Next.js component with Framer Motion and TailwindCSS.", risk: "low" },
  { id: "sk_3", name: "aws-deploy-script", author: "devops-guru", downloads: 312, rating: 4.1, description: "Deploys the current project to AWS using AWS CLI. Uses credentials.", risk: "high" },
];

export default function SkillsPage() {
  const [activeTab, setActiveTab] = useState("marketplace");
  const [searchQuery, setSearchQuery] = useState("");
  const [isSearching, setIsSearching] = useState(false);
  const [selectedSkill, setSelectedSkill] = useState<any>(null);

  const handleSearch = () => {
    setIsSearching(true);
    setTimeout(() => {
      setIsSearching(false);
    }, 800);
  };

  const containerVariants = {
    hidden: { opacity: 0, y: 20 },
    visible: { 
      opacity: 1, 
      y: 0,
      transition: { duration: 0.5, staggerChildren: 0.1 }
    }
  };

  const itemVariants = {
    hidden: { opacity: 0, x: -20 },
    visible: { opacity: 1, x: 0 }
  };

  return (
    <div className="flex-1 h-full flex flex-col relative overflow-hidden bg-[#0A0A0B] text-slate-200">
      {/* Background Orbs */}
      <div className="absolute top-[20%] left-[10%] w-[50%] h-[50%] rounded-full bg-emerald-600/10 blur-[150px] pointer-events-none" />
      <div className="absolute bottom-[10%] right-[10%] w-[40%] h-[40%] rounded-full bg-blue-600/10 blur-[120px] pointer-events-none" />

      {/* Header */}
      <header className="px-8 py-6 border-b border-white/5 bg-white/[0.02] backdrop-blur-md z-10 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-emerald-500/20 to-blue-500/20 border border-white/10 flex items-center justify-center shadow-lg shadow-emerald-500/10">
            <Library className="w-6 h-6 text-emerald-400" />
          </div>
          <div>
            <h1 className="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-white/60">Skill Directory</h1>
            <p className="text-sm text-slate-400">Discover, audit, and install skills</p>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <div className="flex-1 flex min-h-0 z-10">
        {/* Sidebar Tabs */}
        <div className="w-64 border-r border-white/5 bg-black/20 p-4 flex flex-col gap-2 overflow-y-auto">
          {tabs.map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-300 relative group ${
                activeTab === tab.id 
                  ? "text-white bg-emerald-500/10 border border-emerald-500/20" 
                  : "text-slate-400 hover:text-white hover:bg-white/5 border border-transparent"
              }`}
            >
              {activeTab === tab.id && (
                <motion.div 
                  layoutId="skillsTabIndicator"
                  className="absolute left-0 top-0 bottom-0 w-1 bg-emerald-500 rounded-r-full shadow-[0_0_10px_rgba(16,185,129,0.5)]"
                />
              )}
              <tab.icon className={`w-5 h-5 transition-colors ${activeTab === tab.id ? 'text-emerald-400' : 'group-hover:text-slate-300'}`} />
              <span className="font-medium text-sm">{tab.label}</span>
            </button>
          ))}
        </div>

        {/* Tab Content Area */}
        <div className="flex-1 p-8 overflow-y-auto custom-scrollbar relative">
          <AnimatePresence mode="wait">
            <motion.div
              key={activeTab}
              variants={containerVariants}
              initial="hidden"
              animate="visible"
              exit={{ opacity: 0, y: -20, transition: { duration: 0.2 } }}
              className="h-full flex flex-col gap-6 max-w-5xl mx-auto"
            >
              
              {/* REMOTE MARKETPLACE */}
              {activeTab === "marketplace" && (
                <>
                  <div className="flex items-center justify-between mb-2">
                    <h2 className="text-xl font-semibold flex items-center gap-2 text-white">
                      <Globe className="text-emerald-400" />
                      skills.sh Marketplace
                    </h2>
                    <div className="text-sm px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-300 flex items-center gap-2">
                      <div className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                      Connected to API
                    </div>
                  </div>

                  {/* Search Bar */}
                  <div className="flex gap-4">
                    <div className="flex-1 relative group">
                      <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                        <Search className="w-5 h-5 text-slate-500 group-focus-within:text-emerald-400 transition-colors" />
                      </div>
                      <input
                        type="text"
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                        placeholder="Search for remote skills... (e.g. 'rust', 'react', 'aws')"
                        className="w-full bg-black/40 border border-white/10 rounded-xl py-3 pl-12 pr-4 text-sm text-white placeholder:text-slate-500 focus:outline-none focus:border-emerald-500/50 transition-colors shadow-inner"
                        onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
                      />
                    </div>
                    <button 
                      onClick={handleSearch}
                      disabled={isSearching}
                      className="px-6 py-3 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 text-white font-medium shadow-lg shadow-emerald-500/20 disabled:opacity-50 transition-all hover:scale-105 active:scale-95 flex items-center justify-center min-w-[120px]"
                    >
                      {isSearching ? <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" /> : "Search"}
                    </button>
                  </div>

                  {/* Results List */}
                  <div className="grid gap-4 mt-4">
                    {mockRemoteSkills.map((skill) => (
                      <motion.div variants={itemVariants} key={skill.id} className="bg-white/5 border border-white/10 rounded-2xl p-5 hover:bg-white/[0.07] hover:border-white/20 transition-all flex flex-col gap-4 group">
                        <div className="flex justify-between items-start">
                          <div>
                            <h3 className="text-lg font-bold text-white flex items-center gap-2">
                              {skill.name}
                              {skill.risk === "high" ? (
                                <span className="px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-red-500/20 text-red-400 border border-red-500/30">High Risk</span>
                              ) : (
                                <span className="px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">Safe</span>
                              )}
                            </h3>
                            <div className="text-xs text-slate-400 mt-1">by {skill.author} • {skill.downloads} downloads • ⭐ {skill.rating}</div>
                          </div>
                          <div className="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                            <button className="px-3 py-1.5 rounded-lg bg-black/40 hover:bg-black/60 border border-white/10 text-xs font-medium flex items-center gap-2">
                              <Info className="w-3.5 h-3.5" /> Details
                            </button>
                            <button className="px-3 py-1.5 rounded-lg bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-300 border border-indigo-500/30 text-xs font-medium flex items-center gap-2">
                              <ShieldCheck className="w-3.5 h-3.5" /> Audit
                            </button>
                            <button className="px-3 py-1.5 rounded-lg bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-300 border border-emerald-500/30 text-xs font-medium flex items-center gap-2">
                              <Download className="w-3.5 h-3.5" /> Install
                            </button>
                          </div>
                        </div>
                        <p className="text-sm text-slate-300 leading-relaxed">
                          {skill.description}
                        </p>
                      </motion.div>
                    ))}
                  </div>
                </>
              )}

              {/* OTHER TABS PLACEHOLDER */}
              {activeTab !== "marketplace" && (
                <div className="flex-1 flex flex-col items-center justify-center text-center max-w-md mx-auto">
                  <motion.div variants={itemVariants} className="w-20 h-20 rounded-3xl bg-white/5 border border-white/10 flex items-center justify-center mb-6 shadow-2xl">
                    <Library className="w-10 h-10 text-emerald-400 opacity-50" />
                  </motion.div>
                  <motion.h2 variants={itemVariants} className="text-2xl font-bold text-white mb-3">
                    {tabs.find(t => t.id === activeTab)?.label}
                  </motion.h2>
                  <motion.p variants={itemVariants} className="text-slate-400 mb-8 leading-relaxed">
                    View and manage your local and installed skills. Ensure you regularly audit imported skills for safety and updates.
                  </motion.p>
                  
                  {activeTab === "installed" && (
                     <motion.div variants={itemVariants} className="p-4 rounded-xl bg-indigo-500/10 border border-indigo-500/20 text-indigo-200/80 text-sm flex items-start gap-3 text-left w-full">
                     <ShieldCheck className="w-5 h-5 shrink-0 mt-0.5 text-indigo-400" />
                     <p>
                       Installed skills have passed Audit and ApprovalGate. They are stored locally at <code>~/.config/goat/skills/</code> and have full offline availability.
                     </p>
                   </motion.div>
                  )}
                </div>
              )}

            </motion.div>
          </AnimatePresence>
        </div>
      </div>
    </div>
  );
}
