"use client";

import React, { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  Sparkles, Plus, Search, FileText, Zap, BrainCircuit, Activity,
  Target, Rocket, CheckCircle2, ChevronRight, BarChart3, Presentation, Users, Briefcase
} from "lucide-react";
import { cofounderApi } from "@/lib/goat-api";

// Mock data
const MOCK_IDEAS = [
  {
    id: "idea-1",
    title: "AI-Powered Code Reviewer",
    description: "An autonomous agent that reviews pull requests and suggests fixes.",
    status: "Validating",
    score: 85,
    scorecard: { marketSize: 8, technicalFeasibility: 9, competition: 6, monetization: 8 },
    validationPlan: ["Interview 10 CTOs", "Analyze GitHub app usage", "Create landing page"],
    mvpScope: "Basic GitHub integration to comment on PRs with linting errors.",
    competitors: ["SonarQube", "CodeRabbit", "ReviewPad"],
    outreachDrafts: ["Draft 1: CTO Cold Email", "Draft 2: Developer Discord Message"]
  },
  {
    id: "idea-2",
    title: "Web3 CRM",
    description: "CRM tool for tracking NFT holders and token distributions.",
    status: "Draft",
    score: 0,
    scorecard: null,
    validationPlan: [],
    mvpScope: "",
    competitors: [],
    outreachDrafts: []
  }
];

export default function CofounderPage() {
  const [ideas, setIdeas] = useState(MOCK_IDEAS);
  const [activeIdeaId, setActiveIdeaId] = useState("idea-1");
  const [isCreating, setIsCreating] = useState(false);
  const [newIdeaTitle, setNewIdeaTitle] = useState("");
  const [newIdeaDesc, setNewIdeaDesc] = useState("");

  const activeIdea = ideas.find(i => i.id === activeIdeaId);

  const handleCreate = () => {
    if (!newIdeaTitle) return;
    const newIdea = {
      id: `idea-${Date.now()}`,
      title: newIdeaTitle,
      description: newIdeaDesc,
      status: "Draft",
      score: 0,
      scorecard: null,
      validationPlan: [],
      mvpScope: "",
      competitors: [],
      outreachDrafts: []
    };
    setIdeas([newIdea, ...ideas]);
    setActiveIdeaId(newIdea.id);
    setIsCreating(false);
    setNewIdeaTitle("");
    setNewIdeaDesc("");
  };

  const runAction = (action: string) => {
    alert(`Running: ${action} on ${activeIdea?.title}`);
  };

  return (
    <div className="flex-1 h-full flex overflow-hidden bg-[#0A0A0B] text-slate-200">
      {/* Background Orbs */}
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] rounded-full bg-indigo-600/10 blur-[120px] pointer-events-none" />
      <div className="absolute bottom-[-10%] right-[-10%] w-[30%] h-[30%] rounded-full bg-violet-600/10 blur-[100px] pointer-events-none" />

      {/* Left Sidebar: Idea List */}
      <div className="w-80 border-r border-white/5 bg-white/[0.02] backdrop-blur-md flex flex-col z-10">
        <div className="p-4 border-b border-white/5 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-lg bg-indigo-500/20 border border-indigo-500/30 flex items-center justify-center">
              <Sparkles className="w-4 h-4 text-indigo-400" />
            </div>
            <span className="font-semibold text-white">Cofounder</span>
          </div>
          <button 
            onClick={() => setIsCreating(true)}
            className="w-8 h-8 rounded-lg bg-white/5 hover:bg-white/10 flex items-center justify-center transition-colors border border-white/10"
          >
            <Plus className="w-4 h-4 text-slate-300" />
          </button>
        </div>

        <div className="p-4">
          <div className="relative mb-4">
            <Search className="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
            <input 
              type="text" 
              placeholder="Search ideas..." 
              className="w-full bg-black/40 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-white placeholder:text-slate-500 focus:outline-none focus:border-indigo-500/50"
            />
          </div>
        </div>

        <div className="flex-1 overflow-y-auto px-4 pb-4 space-y-2 custom-scrollbar">
          {ideas.map(idea => (
            <button
              key={idea.id}
              onClick={() => { setActiveIdeaId(idea.id); setIsCreating(false); }}
              className={`w-full text-left p-3 rounded-xl border transition-all ${
                activeIdeaId === idea.id && !isCreating
                  ? "bg-indigo-500/10 border-indigo-500/30"
                  : "bg-white/[0.02] border-white/5 hover:border-white/10"
              }`}
            >
              <h4 className={`font-medium text-sm truncate ${activeIdeaId === idea.id && !isCreating ? "text-indigo-300" : "text-slate-300"}`}>
                {idea.title}
              </h4>
              <p className="text-xs text-slate-500 truncate mt-1">{idea.description}</p>
              <div className="flex items-center gap-2 mt-2">
                <span className={`text-[10px] px-1.5 py-0.5 rounded-sm border ${
                  idea.status === "Validating" ? "bg-amber-400/10 border-amber-400/20 text-amber-400" : "bg-slate-400/10 border-slate-400/20 text-slate-400"
                }`}>
                  {idea.status}
                </span>
                {idea.score > 0 && (
                  <span className="text-[10px] text-emerald-400 font-medium">
                    Score: {idea.score}
                  </span>
                )}
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto custom-scrollbar relative z-10 p-8">
        <AnimatePresence mode="wait">
          {isCreating ? (
            <motion.div
              key="create"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              className="max-w-2xl mx-auto bg-white/[0.03] border border-white/10 rounded-2xl p-8"
            >
              <div className="flex items-center gap-4 mb-8">
                <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-indigo-500/20 to-purple-500/20 border border-white/10 flex items-center justify-center">
                  <BrainCircuit className="w-6 h-6 text-indigo-400" />
                </div>
                <div>
                  <h2 className="text-2xl font-bold text-white">Pitch a New Idea</h2>
                  <p className="text-sm text-slate-400">Let Cofounder Prime evaluate and flesh out your next big thing.</p>
                </div>
              </div>

              <div className="space-y-6">
                <div>
                  <label className="block text-sm font-medium text-slate-300 mb-2">Project Name</label>
                  <input
                    value={newIdeaTitle}
                    onChange={(e) => setNewIdeaTitle(e.target.value)}
                    type="text"
                    placeholder="e.g. NextGen AI DevTool"
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-3 text-white placeholder:text-slate-600 focus:outline-none focus:border-indigo-500/50 transition-colors"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-slate-300 mb-2">Elevator Pitch</label>
                  <textarea
                    value={newIdeaDesc}
                    onChange={(e) => setNewIdeaDesc(e.target.value)}
                    rows={4}
                    placeholder="Describe the problem and your proposed solution..."
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-3 text-white placeholder:text-slate-600 focus:outline-none focus:border-indigo-500/50 transition-colors resize-none"
                  />
                </div>
                <div className="flex justify-end pt-4">
                  <button 
                    onClick={handleCreate}
                    className="px-6 py-3 bg-indigo-500 hover:bg-indigo-600 text-white rounded-lg font-medium transition-colors flex items-center gap-2 shadow-lg shadow-indigo-500/20"
                  >
                    <Rocket className="w-4 h-4" /> Initialize
                  </button>
                </div>
              </div>
            </motion.div>
          ) : activeIdea ? (
            <motion.div
              key="detail"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              className="max-w-4xl mx-auto space-y-6"
            >
              {/* Header */}
              <div className="flex items-start justify-between bg-white/[0.03] border border-white/10 rounded-2xl p-6">
                <div>
                  <div className="flex items-center gap-3 mb-2">
                    <h2 className="text-2xl font-bold text-white">{activeIdea.title}</h2>
                    <span className="px-2 py-1 rounded border bg-white/5 border-white/10 text-xs text-slate-300 font-medium">
                      {activeIdea.status}
                    </span>
                  </div>
                  <p className="text-slate-400">{activeIdea.description}</p>
                </div>
                {activeIdea.score > 0 && (
                  <div className="text-center">
                    <div className="text-3xl font-black text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-cyan-400">
                      {activeIdea.score}
                    </div>
                    <div className="text-[10px] text-slate-500 uppercase tracking-widest font-bold mt-1">Overall Score</div>
                  </div>
                )}
              </div>

              {/* Actions */}
              <div className="flex flex-wrap gap-3">
                <button onClick={() => runAction("Validate")} className="flex items-center gap-2 px-4 py-2 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-300 border border-indigo-500/30 rounded-lg text-sm font-medium transition-colors">
                  <Activity className="w-4 h-4" /> Validate
                </button>
                <button onClick={() => runAction("Score")} className="flex items-center gap-2 px-4 py-2 bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 rounded-lg text-sm font-medium transition-colors">
                  <Target className="w-4 h-4" /> Score
                </button>
                <button onClick={() => runAction("Generate MVP")} className="flex items-center gap-2 px-4 py-2 bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 rounded-lg text-sm font-medium transition-colors">
                  <Briefcase className="w-4 h-4" /> Generate MVP
                </button>
                <button onClick={() => runAction("Generate Report")} className="flex items-center gap-2 px-4 py-2 bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 rounded-lg text-sm font-medium transition-colors">
                  <FileText className="w-4 h-4" /> Generate Report
                </button>
                <button onClick={() => runAction("Handoff to Builder")} className="ml-auto flex items-center gap-2 px-4 py-2 bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 border border-emerald-500/30 rounded-lg text-sm font-medium transition-colors">
                  <Rocket className="w-4 h-4" /> Handoff to Builder
                </button>
              </div>

              {/* Detail Grid */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                
                {/* Scorecard */}
                <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6">
                  <h3 className="flex items-center gap-2 font-medium text-white mb-4">
                    <BarChart3 className="w-4 h-4 text-indigo-400" /> Scorecard
                  </h3>
                  {activeIdea.scorecard ? (
                    <div className="space-y-4">
                      {Object.entries(activeIdea.scorecard).map(([key, value]) => (
                        <div key={key}>
                          <div className="flex justify-between text-xs text-slate-400 mb-1">
                            <span className="capitalize">{key.replace(/([A-Z])/g, ' $1').trim()}</span>
                            <span>{value}/10</span>
                          </div>
                          <div className="h-1.5 w-full bg-black/40 rounded-full overflow-hidden">
                            <div 
                              className="h-full bg-gradient-to-r from-indigo-500 to-violet-500 rounded-full" 
                              style={{ width: `${(value as number) * 10}%` }}
                            />
                          </div>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <p className="text-sm text-slate-500">Not scored yet. Run a scoring analysis to generate the scorecard.</p>
                  )}
                </div>

                {/* MVP Scope */}
                <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6">
                  <h3 className="flex items-center gap-2 font-medium text-white mb-4">
                    <Target className="w-4 h-4 text-pink-400" /> MVP Scope
                  </h3>
                  {activeIdea.mvpScope ? (
                    <p className="text-sm text-slate-300 leading-relaxed">{activeIdea.mvpScope}</p>
                  ) : (
                    <p className="text-sm text-slate-500">No MVP defined. Generate MVP specifications first.</p>
                  )}
                </div>

                {/* Validation Plan */}
                <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6">
                  <h3 className="flex items-center gap-2 font-medium text-white mb-4">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400" /> Validation Plan
                  </h3>
                  {activeIdea.validationPlan.length > 0 ? (
                    <ul className="space-y-2">
                      {activeIdea.validationPlan.map((step, idx) => (
                        <li key={idx} className="flex items-start gap-2 text-sm text-slate-300">
                          <ChevronRight className="w-4 h-4 text-slate-500 shrink-0 mt-0.5" />
                          <span>{step}</span>
                        </li>
                      ))}
                    </ul>
                  ) : (
                    <p className="text-sm text-slate-500">No validation steps defined.</p>
                  )}
                </div>

                {/* Competitors & Outreach */}
                <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6 flex flex-col gap-6">
                  <div>
                    <h3 className="flex items-center gap-2 font-medium text-white mb-4">
                      <Users className="w-4 h-4 text-amber-400" /> Competitors
                    </h3>
                    {activeIdea.competitors.length > 0 ? (
                      <div className="flex flex-wrap gap-2">
                        {activeIdea.competitors.map(comp => (
                          <span key={comp} className="px-2.5 py-1 rounded bg-black/40 border border-white/5 text-xs text-slate-300">
                            {comp}
                          </span>
                        ))}
                      </div>
                    ) : (
                      <p className="text-sm text-slate-500">No competitors identified.</p>
                    )}
                  </div>

                  <div>
                    <h3 className="flex items-center gap-2 font-medium text-white mb-4">
                      <Presentation className="w-4 h-4 text-cyan-400" /> Outreach Drafts
                    </h3>
                    {activeIdea.outreachDrafts.length > 0 ? (
                      <ul className="space-y-2">
                        {activeIdea.outreachDrafts.map((draft, idx) => (
                          <li key={idx} className="flex items-start gap-2 text-sm text-slate-300">
                            <FileText className="w-4 h-4 text-slate-500 shrink-0 mt-0.5" />
                            <span>{draft}</span>
                          </li>
                        ))}
                      </ul>
                    ) : (
                      <p className="text-sm text-slate-500">No drafts available.</p>
                    )}
                  </div>
                </div>

              </div>
            </motion.div>
          ) : null}
        </AnimatePresence>
      </div>
    </div>
  );
}
