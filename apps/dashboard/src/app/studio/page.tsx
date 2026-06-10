"use client";

import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  Wand2, 
  TerminalSquare, 
  GitCompare, 
  BrainCircuit, 
  Bot, 
  Workflow, 
  History, 
  Play, 
  Save,
  ShieldAlert,
  ChevronRight,
  Code2,
  Sparkles,
  Search,
  LayoutDashboard,
  Globe
} from "lucide-react";

const tabs = [
  { id: "prompt", label: "Prompt Lab", icon: TerminalSquare },
  { id: "compare", label: "Model Compare", icon: GitCompare },
  { id: "skill", label: "Skill Builder", icon: BrainCircuit },
  { id: "agent", label: "Agent Builder", icon: Bot },
  { id: "workflow", label: "Workflow Builder", icon: Workflow },
  { id: "memory", label: "From Memory", icon: History },
];

const mockMemoryCandidates = [
  { id: "1", title: "Auth flow fix", type: "Skill", confidence: 0.95 },
  { id: "2", title: "React expert session", type: "Agent", confidence: 0.88 },
  { id: "3", title: "Release checklist", type: "Workflow", confidence: 0.91 }
];

export default function StudioPage() {
  const [activeTab, setActiveTab] = useState("prompt");
  const [promptText, setPromptText] = useState("");
  const [output, setOutput] = useState("");
  const [isProcessing, setIsProcessing] = useState(false);

  const runPrompt = () => {
    setIsProcessing(true);
    setTimeout(() => {
      setOutput(`Simulated output for:\n\n${promptText}`);
      setIsProcessing(false);
    }, 1500);
  };

  const containerVariants = {
    hidden: { opacity: 0, y: 20 },
    visible: { 
      opacity: 1, 
      y: 0,
      transition: { duration: 0.6, staggerChildren: 0.1 }
    }
  };

  const itemVariants = {
    hidden: { opacity: 0, x: -20 },
    visible: { opacity: 1, x: 0 }
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
            <Sparkles className="w-6 h-6 text-indigo-400" />
          </div>
          <div>
            <h1 className="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-white/60">AI Studio</h1>
            <p className="text-sm text-slate-400">Design, test, and deploy intelligent assets</p>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <button className="px-4 py-2 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 transition-colors flex items-center gap-2 text-sm">
            <Save className="w-4 h-4" />
            <span>Saved Drafts</span>
          </button>
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
                  ? "text-white bg-indigo-500/10 border border-indigo-500/20" 
                  : "text-slate-400 hover:text-white hover:bg-white/5 border border-transparent"
              }`}
            >
              {activeTab === tab.id && (
                <motion.div 
                  layoutId="activeTabIndicator"
                  className="absolute left-0 top-0 bottom-0 w-1 bg-indigo-500 rounded-r-full shadow-[0_0_10px_rgba(99,102,241,0.5)]"
                />
              )}
              <tab.icon className={`w-5 h-5 transition-colors ${activeTab === tab.id ? 'text-indigo-400' : 'group-hover:text-slate-300'}`} />
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
              className="h-full flex flex-col gap-6 max-w-6xl mx-auto"
            >
              
              {/* PROMPT LAB */}
              {activeTab === "prompt" && (
                <>
                  <div className="flex items-center justify-between">
                    <h2 className="text-xl font-semibold flex items-center gap-2">
                      <TerminalSquare className="text-indigo-400" />
                      Prompt Lab
                    </h2>
                    <div className="flex gap-2">
                      <select className="bg-black/40 border border-white/10 rounded-lg px-3 py-1.5 text-sm focus:outline-none focus:border-indigo-500/50">
                        <option>balanced (Default)</option>
                        <option>coder</option>
                        <option>architect</option>
                      </select>
                      <select className="bg-black/40 border border-white/10 rounded-lg px-3 py-1.5 text-sm focus:outline-none focus:border-indigo-500/50">
                        <option>Chat Mode</option>
                        <option>Plan Mode</option>
                        <option>Act Mode</option>
                      </select>
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-6 flex-1 min-h-[400px]">
                    {/* Input */}
                    <div className="flex flex-col gap-3">
                      <div className="flex-1 bg-black/20 border border-white/10 rounded-2xl p-1 relative group overflow-hidden">
                        <div className="absolute inset-0 bg-gradient-to-br from-indigo-500/5 to-purple-500/5 opacity-0 group-hover:opacity-100 transition-opacity" />
                        <textarea
                          value={promptText}
                          onChange={(e) => setPromptText(e.target.value)}
                          placeholder="Enter your system prompt or instruction here..."
                          className="w-full h-full bg-transparent resize-none outline-none p-4 text-sm leading-relaxed"
                        />
                      </div>
                      <div className="flex justify-end">
                        <button 
                          onClick={runPrompt}
                          disabled={isProcessing || !promptText}
                          className="px-6 py-2.5 rounded-xl bg-gradient-to-r from-indigo-500 to-violet-600 hover:from-indigo-400 hover:to-violet-500 text-white font-medium shadow-lg shadow-indigo-500/25 flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all hover:scale-105 active:scale-95"
                        >
                          {isProcessing ? (
                            <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                          ) : (
                            <Play className="w-4 h-4 fill-current" />
                          )}
                          <span>Run Prompt</span>
                        </button>
                      </div>
                    </div>

                    {/* Output */}
                    <div className="bg-black/40 border border-white/10 rounded-2xl p-5 flex flex-col shadow-inner relative overflow-hidden">
                      <div className="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-indigo-500/0 via-indigo-500/50 to-indigo-500/0 opacity-50" />
                      <h3 className="text-sm font-medium text-slate-400 mb-4 flex items-center gap-2">
                        <Code2 className="w-4 h-4" /> Output
                      </h3>
                      {output ? (
                        <div className="flex-1 overflow-y-auto text-sm text-slate-300 whitespace-pre-wrap font-mono">
                          {output}
                        </div>
                      ) : (
                        <div className="flex-1 flex items-center justify-center text-slate-600 text-sm">
                          Run a prompt to see the output here
                        </div>
                      )}
                    </div>
                  </div>
                </>
              )}

              {/* BUILDERS & OTHERS PLACEHOLDERS */}
              {activeTab !== "prompt" && (
                <div className="flex-1 flex flex-col items-center justify-center text-center max-w-md mx-auto">
                  <motion.div variants={itemVariants} className="w-20 h-20 rounded-3xl bg-white/5 border border-white/10 flex items-center justify-center mb-6 shadow-2xl">
                    <Sparkles className="w-10 h-10 text-indigo-400 opacity-50" />
                  </motion.div>
                  <motion.h2 variants={itemVariants} className="text-2xl font-bold text-white mb-3">
                    {tabs.find(t => t.id === activeTab)?.label}
                  </motion.h2>
                  <motion.p variants={itemVariants} className="text-slate-400 mb-8 leading-relaxed">
                    Visual creation studio for intelligent assets. Draft, test, and deploy directly to your GOAT brain.
                  </motion.p>
                  
                  {activeTab === "memory" && (
                    <motion.div variants={itemVariants} className="w-full text-left space-y-3">
                      <h4 className="text-sm font-medium text-slate-300 mb-4 uppercase tracking-wider">Learning Candidates</h4>
                      {mockMemoryCandidates.map(c => (
                        <div key={c.id} className="p-4 rounded-xl bg-white/5 border border-white/10 hover:border-indigo-500/50 transition-colors flex items-center justify-between group cursor-pointer">
                          <div className="flex items-center gap-3">
                            <div className="w-8 h-8 rounded-lg bg-indigo-500/20 text-indigo-400 flex items-center justify-center text-xs font-bold">
                              {c.type[0]}
                            </div>
                            <div>
                              <div className="text-sm font-medium text-white">{c.title}</div>
                              <div className="text-xs text-slate-500">Confidence: {(c.confidence * 100).toFixed()}%</div>
                            </div>
                          </div>
                          <ChevronRight className="w-5 h-5 text-slate-500 group-hover:text-white transition-colors" />
                        </div>
                      ))}
                    </motion.div>
                  )}

                  {activeTab === "skill" && (
                     <motion.div variants={itemVariants} className="w-full flex justify-center mt-4">
                        <button className="px-6 py-2 rounded-xl bg-indigo-500/20 hover:bg-indigo-500/30 border border-indigo-500/30 text-indigo-300 transition-colors flex items-center gap-2 text-sm shadow-lg shadow-indigo-500/10">
                           <Globe className="w-4 h-4" />
                           Import from Marketplace
                        </button>
                     </motion.div>
                  )}

                  {activeTab !== "memory" && activeTab !== "skill" && (
                    <motion.div variants={itemVariants} className="p-4 rounded-xl bg-amber-500/10 border border-amber-500/20 text-amber-200/80 text-sm flex items-start gap-3 text-left">
                      <ShieldAlert className="w-5 h-5 shrink-0 mt-0.5 text-amber-400" />
                      <p>
                        Safety First: Assets created here are saved as drafts in <code>~/.local/share/goat/studio/</code>. Explicit review is required before they are injected into your active brain.
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
