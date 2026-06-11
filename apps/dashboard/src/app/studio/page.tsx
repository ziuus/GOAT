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
  Globe,
  LayoutTemplate,
  Library,
  UserCheck,
  X
} from "lucide-react";
import { goatApi } from "@/lib/goat-api";

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

  // Brain Context Modal State
  const [isBrainModalOpen, setIsBrainModalOpen] = useState(false);
  const [brainQuery, setBrainQuery] = useState("");
  const [brainSearchMode, setBrainSearchMode] = useState("hybrid");
  const [brainResults, setBrainResults] = useState<any[]>([]);
  const [isBrainSearching, setIsBrainSearching] = useState(false);

  const handleBrainSearch = async () => {
    if (!brainQuery) return;
    setIsBrainSearching(true);
    try {
      const res = await goatApi.searchBrain(brainQuery, brainSearchMode);
      setBrainResults(res.results || []);
    } catch (e) {
      console.error("Search failed", e);
    } finally {
      setIsBrainSearching(false);
    }
  };

  const insertContext = (body: string, kind: string) => {
    setPromptText(prev => `${prev}\n\n[Context: ${kind}]\n${body}`);
    setIsBrainModalOpen(false);
  };

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
                      <div className="flex justify-between items-center relative">
                        <button onClick={() => setIsBrainModalOpen(true)} className="text-xs flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-fuchsia-500/10 text-fuchsia-400 hover:bg-fuchsia-500/20 border border-fuchsia-500/20 transition-colors">
                          <BrainCircuit className="w-3.5 h-3.5" /> Attach Brain Context
                        </button>
                        
                        {/* Brain Context Modal Popup */}
                        <AnimatePresence>
                          {isBrainModalOpen && (
                            <motion.div
                              initial={{ opacity: 0, y: 10 }}
                              animate={{ opacity: 1, y: 0 }}
                              exit={{ opacity: 0, y: 10 }}
                              className="absolute top-10 left-0 w-96 bg-black/90 border border-white/20 rounded-xl shadow-2xl z-50 p-4 backdrop-blur-xl"
                            >
                              <div className="flex justify-between items-center mb-3">
                                <h4 className="text-sm font-semibold text-white">Search Brain</h4>
                                <button onClick={() => setIsBrainModalOpen(false)} className="text-slate-400 hover:text-white">
                                  <X className="w-4 h-4" />
                                </button>
                              </div>
                              <div className="flex gap-2 mb-3">
                                <input
                                  type="text"
                                  placeholder="Query..."
                                  value={brainQuery}
                                  onChange={(e) => setBrainQuery(e.target.value)}
                                  onKeyDown={(e) => e.key === 'Enter' && handleBrainSearch()}
                                  className="flex-1 bg-white/5 border border-white/10 rounded-lg px-3 py-1.5 text-xs text-white placeholder:text-slate-500 focus:outline-none focus:border-indigo-500/50"
                                />
                                <select 
                                  value={brainSearchMode}
                                  onChange={(e) => setBrainSearchMode(e.target.value)}
                                  className="bg-white/5 border border-white/10 rounded-lg px-2 text-xs text-white"
                                >
                                  <option value="hybrid">Hybrid</option>
                                  <option value="semantic">Semantic</option>
                                  <option value="keyword">Keyword</option>
                                </select>
                                <button onClick={handleBrainSearch} disabled={isBrainSearching} className="px-3 bg-indigo-500 text-white rounded-lg text-xs hover:bg-indigo-400 disabled:opacity-50">
                                  {isBrainSearching ? "..." : <Search className="w-3 h-3" />}
                                </button>
                              </div>
                              <div className="max-h-64 overflow-y-auto space-y-2 custom-scrollbar">
                                {brainResults.map((res: any) => (
                                  <div key={res.document.id} className="bg-white/5 p-2 rounded-lg border border-white/5">
                                    <div className="flex justify-between items-start mb-1">
                                      <div className="text-xs font-medium text-indigo-300 truncate max-w-[200px]">{res.document.title}</div>
                                      <button onClick={() => insertContext(res.document.body, res.document.kind)} className="text-[10px] px-2 py-0.5 bg-fuchsia-500/20 text-fuchsia-300 rounded hover:bg-fuchsia-500/40">
                                        Inject
                                      </button>
                                    </div>
                                    <div className="text-[10px] text-slate-400 line-clamp-2">{res.document.summary}</div>
                                  </div>
                                ))}
                                {brainResults.length === 0 && !isBrainSearching && brainQuery && (
                                  <div className="text-xs text-slate-500 text-center py-4">No context found.</div>
                                )}
                              </div>
                            </motion.div>
                          )}
                        </AnimatePresence><button 
                          onClick={runPrompt}
                          disabled={isProcessing || !promptText}
                          className="px-6 py-2.5 rounded-xl bg-gradient-to-r from-indigo-500 to-violet-600 hover:from-indigo-400 hover:to-violet-500 text-white font-medium shadow-lg shadow-indigo-500/25 flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all hover:scale-105 active:scale-95"
                        >
                          {isProcessing ? (
                            <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                          ) : (
                            <Play className="w-4 h-4 fill-current" />
                          )}
                          <span>{isProcessing ? "Running..." : "Run Prompt"}</span>
                        </button>
                      </div>
                      <div className="flex gap-2 mt-2">
                        <button 
                          onClick={async () => {
                            if (!promptText) return;
                            setIsProcessing(true);
                            try {
                               const { promptforgeApi } = await import('@/lib/goat-api');
                               const res = await promptforgeApi.refine({
                                  original_prompt: promptText,
                                  target_agent: 'user',
                                  target_format: 'goat',
                                  domain: 'general',
                                  complexity: 'medium',
                                  safe_context: '',
                                  constraints: [],
                                  mode: 'mock',
                               });
                               if (res.result?.refined_prompt) {
                                  setPromptText(res.result.refined_prompt);
                               }
                            } catch (e) {
                               console.error(e);
                            } finally {
                               setIsProcessing(false);
                            }
                          }}
                          disabled={isProcessing || !promptText}
                          className="text-xs flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 border border-indigo-500/20 transition-colors"
                        >
                          <Wand2 className="w-3.5 h-3.5" /> Refine with PromptForge
                        </button>
                        <button 
                          onClick={() => {
                            if (!promptText) return;
                            const blob = new Blob([promptText], { type: 'text/plain' });
                            const url = URL.createObjectURL(blob);
                            const a = document.createElement('a');
                            a.href = url;
                            a.download = 'prompt_draft.txt';
                            document.body.appendChild(a);
                            a.click();
                            document.body.removeChild(a);
                            URL.revokeObjectURL(url);
                          }}
                          disabled={!promptText}
                          className="text-xs flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-green-500/10 text-green-400 hover:bg-green-500/20 border border-green-500/20 transition-colors"
                        >
                          <Save className="w-3.5 h-3.5" /> Save Draft
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
                        <div key={c.id} className="p-4 rounded-xl bg-white/5 border border-white/10 hover:border-indigo-500/50 transition-colors flex flex-col group cursor-pointer relative overflow-hidden">
                          <div className="flex items-center justify-between z-10">
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
                          
                          {/* Hover Actions */}
                          <div className="mt-4 pt-4 border-t border-white/10 hidden group-hover:flex items-center gap-2 z-10">
                            <button className="px-3 py-1.5 rounded-lg bg-indigo-500/20 hover:bg-indigo-500/30 border border-indigo-500/30 text-indigo-300 transition-colors flex items-center gap-2 text-xs shadow-lg shadow-indigo-500/10">
                              <Workflow className="w-3.5 h-3.5" />
                              Create Recipe Draft
                            </button>
                            <button className="px-3 py-1.5 rounded-lg bg-fuchsia-500/20 hover:bg-fuchsia-500/30 border border-fuchsia-500/30 text-fuchsia-300 transition-colors flex items-center gap-2 text-xs shadow-lg shadow-fuchsia-500/10">
                              <Workflow className="w-3.5 h-3.5" />
                              Convert to Hook Template
                            </button>
                          </div>
                        </div>
                      ))}
                    </motion.div>
                  )}

                  {activeTab === "skill" && (
                     <motion.div variants={itemVariants} className="w-full flex justify-center gap-3 mt-4">
                        <button className="px-6 py-2 rounded-xl bg-indigo-500/20 hover:bg-indigo-500/30 border border-indigo-500/30 text-indigo-300 transition-colors flex items-center gap-2 text-sm shadow-lg shadow-indigo-500/10">
                           <Globe className="w-4 h-4" />
                           Import from Marketplace
                        </button>
                        <button className="px-6 py-2 rounded-xl bg-violet-500/20 hover:bg-violet-500/30 border border-violet-500/30 text-violet-300 transition-colors flex items-center gap-2 text-sm shadow-lg shadow-violet-500/10">
                           <Workflow className="w-4 h-4" />
                           Chain Skills into Recipe
                        </button>
                     </motion.div>
                  )}
                  {activeTab === "workflow" && (
                     <motion.div variants={itemVariants} className="w-full flex justify-center mt-4">
                        <button className="px-6 py-2 rounded-xl bg-violet-500/20 hover:bg-violet-500/30 border border-violet-500/30 text-violet-300 transition-colors flex items-center gap-2 text-sm shadow-lg shadow-violet-500/10">
                           <Library className="w-4 h-4" />
                           Browse Built-in Recipes
                        </button>
                     </motion.div>
                  )}
                  {activeTab === "agent" && (
                     <motion.div variants={itemVariants} className="w-full flex flex-col items-center gap-4 mt-4">
                        <div className="flex gap-3">
                           <button className="px-6 py-2 rounded-xl bg-indigo-500/20 hover:bg-indigo-500/30 border border-indigo-500/30 text-indigo-300 transition-colors flex items-center gap-2 text-sm shadow-lg shadow-indigo-500/10">
                              <Bot className="w-4 h-4" />
                              Create Custom Prime Agent Draft
                           </button>
                           <button className="px-6 py-2 rounded-xl bg-violet-500/20 hover:bg-violet-500/30 border border-violet-500/30 text-violet-300 transition-colors flex items-center gap-2 text-sm shadow-lg shadow-violet-500/10">
                              <TerminalSquare className="w-4 h-4" />
                              Convert Prompt to Agent Draft
                           </button>
                           <button className="px-6 py-2 rounded-xl bg-slate-500/20 hover:bg-slate-500/30 border border-slate-500/30 text-slate-300 transition-colors flex items-center gap-2 text-sm shadow-lg">
                              <UserCheck className="w-4 h-4" />
                              Use Agent Templates
                           </button>
                        </div>
                        <div className="mt-4 p-4 rounded-xl bg-fuchsia-500/10 border border-fuchsia-500/20 text-fuchsia-200/80 text-sm flex items-start gap-3 w-full text-left">
                           <Sparkles className="w-5 h-5 shrink-0 mt-0.5 text-fuchsia-400" />
                           <p>
                              <strong>Coming Soon:</strong> The "Cofounder Agent" integration is planned for a future phase. It will allow you to generate full agent architectures dynamically.
                           </p>
                        </div>
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
