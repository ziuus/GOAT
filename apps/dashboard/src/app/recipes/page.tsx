"use client";

import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  Search, 
  Download, 
  ShieldCheck, 
  Globe, 
  BrainCircuit, 
  Library,
  GitBranch,
  Info,
  Power,
  Workflow,
  Sparkles,
  UserCheck
} from "lucide-react";

const tabs = [
  { id: "builtin", label: "Built-In", icon: Library },
  { id: "installed", label: "Installed", icon: Download },
  { id: "marketplace", label: "Remote Marketplace", icon: Globe },
  { id: "learned", label: "Learned", icon: BrainCircuit },
  { id: "drafts", label: "Studio Drafts", icon: GitBranch },
  { id: "agents", label: "Agent Templates", icon: UserCheck },
  { id: "audit", label: "Audit Reports", icon: ShieldCheck },
];

const mockRecipes = [
  { id: "builtin_1", name: "cargo-check-on-save", author: "goat-core", source: "builtin", risk: "low", enabled: false, description: "Runs cargo check whenever a Rust file is modified." },
  { id: "builtin_2", name: "checkpoint-before-write", author: "goat-core", source: "builtin", risk: "low", enabled: false, description: "Automatically creates a git checkpoint before executing any risky write." },
  { id: "rem_1", name: "aws-s3-sync", author: "devops-guru", source: "marketplace", risk: "high", enabled: false, description: "Syncs build artifacts to an AWS S3 bucket. Requires credentials." },
];

export default function RecipesPage() {
  const [activeTab, setActiveTab] = useState("builtin");
  const [searchQuery, setSearchQuery] = useState("");
  const [isSearching, setIsSearching] = useState(false);

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
      <div className="absolute top-[20%] left-[10%] w-[50%] h-[50%] rounded-full bg-violet-600/10 blur-[150px] pointer-events-none" />
      <div className="absolute bottom-[10%] right-[10%] w-[40%] h-[40%] rounded-full bg-fuchsia-600/10 blur-[120px] pointer-events-none" />

      {/* Header */}
      <header className="px-8 py-6 border-b border-white/5 bg-white/[0.02] backdrop-blur-md z-10 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-violet-500/20 to-fuchsia-500/20 border border-white/10 flex items-center justify-center shadow-lg shadow-violet-500/10">
            <Workflow className="w-6 h-6 text-violet-400" />
          </div>
          <div>
            <h1 className="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-white/60">Recipes & Automations</h1>
            <p className="text-sm text-slate-400">Discover and manage workflow recipes and agent templates</p>
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
                  ? "text-white bg-violet-500/10 border border-violet-500/20" 
                  : "text-slate-400 hover:text-white hover:bg-white/5 border border-transparent"
              }`}
            >
              {activeTab === tab.id && (
                <motion.div 
                  layoutId="recipesTabIndicator"
                  className="absolute left-0 top-0 bottom-0 w-1 bg-violet-500 rounded-r-full shadow-[0_0_10px_rgba(139,92,246,0.5)]"
                />
              )}
              <tab.icon className={`w-5 h-5 transition-colors ${activeTab === tab.id ? 'text-violet-400' : 'group-hover:text-slate-300'}`} />
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
              
              {/* BUILT-IN RECIPES */}
              {activeTab === "builtin" && (
                <>
                  <div className="flex items-center justify-between mb-2">
                    <h2 className="text-xl font-semibold flex items-center gap-2 text-white">
                      <Library className="text-violet-400" />
                      Built-In Recipes
                    </h2>
                  </div>

                  <div className="grid gap-4 mt-4">
                    {mockRecipes.filter(r => r.source === "builtin").map((recipe) => (
                      <motion.div variants={itemVariants} key={recipe.id} className="bg-white/5 border border-white/10 rounded-2xl p-5 hover:bg-white/[0.07] hover:border-white/20 transition-all flex flex-col gap-4 group">
                        <div className="flex justify-between items-start">
                          <div>
                            <h3 className="text-lg font-bold text-white flex items-center gap-2">
                              {recipe.name}
                              <span className="px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-violet-500/20 text-violet-400 border border-violet-500/30">Safe</span>
                            </h3>
                            <div className="text-xs text-slate-400 mt-1">by {recipe.author}</div>
                          </div>
                          <div className="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                            <button className="px-3 py-1.5 rounded-lg bg-black/40 hover:bg-black/60 border border-white/10 text-xs font-medium flex items-center gap-2">
                              <Info className="w-3.5 h-3.5" /> Details
                            </button>
                            <button className="px-3 py-1.5 rounded-lg bg-violet-500/20 hover:bg-violet-500/30 text-violet-300 border border-violet-500/30 text-xs font-medium flex items-center gap-2">
                              <Power className="w-3.5 h-3.5" /> Enable
                            </button>
                          </div>
                        </div>
                        <p className="text-sm text-slate-300 leading-relaxed">
                          {recipe.description}
                        </p>
                      </motion.div>
                    ))}
                  </div>
                </>
              )}

              {/* MARKETPLACE SEARCH PLACEHOLDER */}
              {activeTab === "marketplace" && (
                 <>
                   <div className="flex items-center justify-between mb-2">
                     <h2 className="text-xl font-semibold flex items-center gap-2 text-white">
                       <Globe className="text-violet-400" />
                       Remote Recipe Marketplace
                     </h2>
                   </div>
                   {/* Search Bar */}
                   <div className="flex gap-4">
                     <div className="flex-1 relative group">
                       <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                         <Search className="w-5 h-5 text-slate-500 group-focus-within:text-violet-400 transition-colors" />
                       </div>
                       <input
                         type="text"
                         value={searchQuery}
                         onChange={(e) => setSearchQuery(e.target.value)}
                         placeholder="Search for remote automation recipes..."
                         className="w-full bg-black/40 border border-white/10 rounded-xl py-3 pl-12 pr-4 text-sm text-white placeholder:text-slate-500 focus:outline-none focus:border-violet-500/50 transition-colors shadow-inner"
                         onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
                       />
                     </div>
                     <button 
                       onClick={handleSearch}
                       disabled={isSearching}
                       className="px-6 py-3 rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-600 hover:from-violet-400 hover:to-fuchsia-500 text-white font-medium shadow-lg shadow-violet-500/20 disabled:opacity-50 transition-all hover:scale-105 active:scale-95 flex items-center justify-center min-w-[120px]"
                     >
                       {isSearching ? <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" /> : "Search"}
                     </button>
                   </div>
                 </>
              )}

              {/* OTHER TABS PLACEHOLDER */}
              {activeTab !== "builtin" && activeTab !== "marketplace" && (
                <div className="flex-1 flex flex-col items-center justify-center text-center max-w-md mx-auto">
                  <motion.div variants={itemVariants} className="w-20 h-20 rounded-3xl bg-white/5 border border-white/10 flex items-center justify-center mb-6 shadow-2xl">
                    <Workflow className="w-10 h-10 text-violet-400 opacity-50" />
                  </motion.div>
                  <motion.h2 variants={itemVariants} className="text-2xl font-bold text-white mb-3">
                    {tabs.find(t => t.id === activeTab)?.label}
                  </motion.h2>
                  <motion.p variants={itemVariants} className="text-slate-400 mb-8 leading-relaxed">
                    View and manage your workflow recipes and templates. Installed recipes are disabled by default and require approval for high-risk actions.
                  </motion.p>
                </div>
              )}

            </motion.div>
          </AnimatePresence>
        </div>
      </div>
    </div>
  );
}
