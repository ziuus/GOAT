"use client";

import React, { useState, useEffect } from "react";
import { Brain, Star, Sparkles, Check, X, Shield, Activity, Globe, Workflow, Library, Search, BrainCircuit } from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";
import { useRouter } from "next/navigation";

export default function MemoryGalaxyPage() {
  const router = useRouter();
  const [candidates, setCandidates] = useState<any[]>([]);
  const [memories, setMemories] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock fetch for candidates and memories until actual API is fully hooked
    setCandidates([
      { id: "1", type: "project_fact", source: "Job #102", summary: "Project uses Next.js app router and TailwindCSS.", confidence: 0.95 },
      { id: "2", type: "workflow_pattern", source: "Chat Session", summary: "Always runs cargo test after modifying Rust code.", confidence: 0.88 },
    ]);
    setMemories([
      { id: "m1", type: "user_preference", summary: "Prefers concise, code-first answers." },
      { id: "m2", type: "skill_candidate", summary: "Auto-lint and format on save." },
    ]);
    setLoading(false);
  }, []);

  const handleAccept = (id: string) => {
    setCandidates((prev) => prev.filter((c) => c.id !== id));
  };

  const handleReject = (id: string) => {
    setCandidates((prev) => prev.filter((c) => c.id !== id));
  };

  if (loading) {
    return (
      <div className="flex h-full items-center justify-center text-zinc-400">
        <Activity className="h-8 w-8 animate-spin" />
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full overflow-hidden bg-[#0A0A0B] text-zinc-200">
      <header className="flex-none px-6 py-4 border-b border-white/10 backdrop-blur-md sticky top-0 z-10 flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <Brain className="w-6 h-6 text-indigo-400" />
          <h1 className="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-indigo-400 to-purple-400">
            Memory Galaxy
          </h1>
        </div>
        <div className="flex items-center space-x-2 text-sm text-zinc-400">
          <Shield className="w-4 h-4 text-green-400" />
          <span>Privacy Protected</span>
        </div>
      </header>

      <div className="flex-1 overflow-y-auto p-6 space-y-8">
        {/* Candidates Section */}
        <section>
          <div className="flex items-center space-x-2 mb-4">
            <Sparkles className="w-5 h-5 text-amber-400" />
            <h2 className="text-lg font-semibold">Pending Candidates</h2>
            <span className="bg-white/10 px-2 py-0.5 rounded-full text-xs">{candidates.length}</span>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <AnimatePresence>
              {candidates.map((c: any) => (
                <motion.div
                  key={c.id}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, scale: 0.95 }}
                  className="group relative rounded-xl border border-white/10 bg-white/5 p-4 hover:bg-white/10 transition-colors backdrop-blur-md overflow-hidden"
                >
                  <div className="absolute inset-0 bg-gradient-to-br from-indigo-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
                  
                  <div className="relative">
                    <div className="flex justify-between items-start mb-3">
                      <span className="text-xs font-medium uppercase tracking-wider text-indigo-300">
                        {c.type.replace("_", " ")}
                      </span>
                      <span className="text-xs text-zinc-500">{c.source}</span>
                    </div>
                    
                    <p className="text-sm text-zinc-300 mb-6">{c.summary}</p>
                    
                    <div className="flex items-center justify-between mt-auto">
                      <div className="text-xs text-zinc-500">
                        Confidence: {(c.confidence * 100).toFixed(0)}%
                      </div>
                      <div className="flex space-x-2">
                        <button
                          onClick={() => handleReject(c.id)}
                          className="p-1.5 rounded-lg hover:bg-red-500/20 text-zinc-400 hover:text-red-400 transition-colors"
                        >
                          <X className="w-4 h-4" />
                        </button>
                        <button
                          onClick={() => handleAccept(c.id)}
                          className="p-1.5 rounded-lg bg-indigo-500/20 text-indigo-400 hover:bg-indigo-500 hover:text-white transition-all"
                        >
                          <Check className="w-4 h-4" />
                        </button>
                      </div>
                    </div>
                  </div>
                </motion.div>
              ))}
            </AnimatePresence>
            {candidates.length === 0 && (
              <div className="col-span-full py-8 text-center text-zinc-500 border border-dashed border-white/10 rounded-xl">
                No pending memories to review.
              </div>
            )}
          </div>
        </section>

        {/* Existing Memories Section */}
        <section>
          <div className="flex items-center justify-between mb-4">
             <div className="flex items-center space-x-2">
                <Star className="w-5 h-5 text-purple-400" />
                <h2 className="text-lg font-semibold">Accepted Memories</h2>
             </div>
             <button
                onClick={() => router.push('/brain')}
                className="px-4 py-2 flex items-center gap-2 bg-fuchsia-500/10 hover:bg-fuchsia-500/20 text-fuchsia-400 rounded-lg text-sm transition-colors border border-fuchsia-500/20"
             >
                <BrainCircuit className="w-4 h-4" /> Search All Brain
             </button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {memories.map((m: any) => (
              <div
                key={m.id}
                className="rounded-xl border border-white/5 bg-white/5 p-4"
              >
                <div className="flex justify-between items-start mb-2">
                  <span className="text-xs font-medium uppercase tracking-wider text-purple-300">
                    {m.type.replace("_", " ")}
                  </span>
                </div>
                <p className="text-sm text-zinc-300">{m.summary}</p>
                {m.type === "skill_candidate" && (
                   <div className="mt-4 flex flex-wrap gap-2">
                     <button className="flex items-center gap-2 text-xs text-indigo-300 hover:text-indigo-200 transition-colors border border-indigo-500/30 px-3 py-1.5 rounded-lg bg-indigo-500/10 hover:bg-indigo-500/20">
                        <Globe className="w-3.5 h-3.5" /> Search Marketplace
                     </button>
                     <button onClick={() => router.push(`/brain?q=${encodeURIComponent(m.summary)}&mode=hybrid`)} className="flex items-center gap-2 text-xs text-fuchsia-300 hover:text-fuchsia-200 transition-colors border border-fuchsia-500/30 px-3 py-1.5 rounded-lg bg-fuchsia-500/10 hover:bg-fuchsia-500/20">
                        <Search className="w-3.5 h-3.5" /> Find Related
                     </button>
                   </div>
                )}
                {m.type === "workflow_candidate" && (
                   <div className="mt-4 flex flex-wrap gap-2">
                     <button className="flex items-center gap-2 text-xs text-violet-300 hover:text-violet-200 transition-colors border border-violet-500/30 px-3 py-1.5 rounded-lg bg-violet-500/10 hover:bg-violet-500/20">
                        <Workflow className="w-3.5 h-3.5" /> Create Recipe Draft
                     </button>
                     <button className="flex items-center gap-2 text-xs text-indigo-300 hover:text-indigo-200 transition-colors border border-indigo-500/30 px-3 py-1.5 rounded-lg bg-indigo-500/10 hover:bg-indigo-500/20">
                        <Globe className="w-3.5 h-3.5" /> Find Matching Recipe
                     </button>
                     <button onClick={() => router.push(`/brain?q=${encodeURIComponent(m.summary)}&mode=hybrid`)} className="flex items-center gap-2 text-xs text-fuchsia-300 hover:text-fuchsia-200 transition-colors border border-fuchsia-500/30 px-3 py-1.5 rounded-lg bg-fuchsia-500/10 hover:bg-fuchsia-500/20">
                        <Search className="w-3.5 h-3.5" /> Find Related
                     </button>
                   </div>
                )}

              </div>
            ))}
          </div>
        </section>
      </div>
    </div>
  );
}
