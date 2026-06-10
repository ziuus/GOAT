"use client";

import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  Github, ShieldAlert, GitBranch, GitPullRequest, GitCommit, 
  TerminalSquare, CheckCircle, ExternalLink, ShieldCheck, Link2
} from "lucide-react";

export default function GitHubPage() {
  const [status, setStatus] = useState<any>(null);
  const [issueId, setIssueId] = useState("");
  const [loading, setLoading] = useState(true);

  const fetchStatus = async () => {
    try {
      const res = await fetch("http://localhost:3000/v1/github/status");
      const data = await res.json();
      setStatus(data);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchStatus();
  }, []);

  const handleLinkIssue = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!issueId) return;
    try {
      await fetch("http://localhost:3000/v1/github/issue/link", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: issueId })
      });
      fetchStatus();
      setIssueId("");
    } catch (e) {
      console.error(e);
    }
  };

  const handleBranchPlan = async () => {
    try {
      await fetch("http://localhost:3000/v1/github/branch/plan", { method: "POST" });
      fetchStatus();
    } catch (e) { console.error(e); }
  };

  const handleDraftPr = async () => {
    try {
      await fetch("http://localhost:3000/v1/github/pr/draft", { method: "POST" });
      fetchStatus();
    } catch (e) { console.error(e); }
  };

  if (loading) return <div className="p-8 text-white font-mono animate-pulse">Loading GitHub Workflow...</div>;

  return (
    <div className="flex-1 p-8 text-white min-h-screen bg-[#050505]">
      <div className="max-w-6xl mx-auto space-y-8">
        
        {/* Header */}
        <div className="flex items-end justify-between border-b border-white/10 pb-6">
          <div>
            <h1 className="text-3xl font-light tracking-tight flex items-center gap-3">
              <Github className="text-indigo-400" />
              GitHub Workflow
            </h1>
            <p className="text-gray-400 mt-2 font-mono text-sm">
              Safe, approval-gated GitHub interaction for issues, branches, and PRs.
            </p>
          </div>
          <div className="flex items-center gap-3">
             <div className="px-4 py-2 bg-green-500/10 border border-green-500/20 text-green-400 rounded-xl flex items-center gap-2 text-sm font-mono">
              <ShieldCheck className="w-4 h-4" /> ApprovalGate Active
            </div>
            <div className="px-4 py-2 bg-white/5 border border-white/10 rounded-xl flex items-center gap-2 text-sm font-mono">
              State: {status?.state || "Unknown"}
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          
          {/* Left Column: Context & Branch */}
          <div className="lg:col-span-1 space-y-6">
            
            {/* Issue Linking */}
            <div className="p-6 rounded-2xl bg-white/[0.02] border border-white/[0.05] backdrop-blur-md">
              <h2 className="text-lg font-medium text-gray-200 flex items-center gap-2 mb-4">
                <Link2 className="text-indigo-400 w-5 h-5" />
                Context Linking
              </h2>
              {status?.linked_issue ? (
                <div className="p-4 bg-indigo-500/10 border border-indigo-500/20 rounded-xl space-y-2">
                  <div className="flex justify-between items-start">
                    <span className="text-indigo-400 font-mono text-sm">#{status.linked_issue.number}</span>
                    <button className="text-xs text-gray-500 hover:text-white transition-colors">Unlink</button>
                  </div>
                  <h3 className="font-medium text-gray-200">{status.linked_issue.title}</h3>
                  <a href={status.linked_issue.url} target="_blank" rel="noreferrer" className="text-xs font-mono text-gray-400 flex items-center gap-1 hover:text-indigo-300">
                    Open in GitHub <ExternalLink className="w-3 h-3" />
                  </a>
                </div>
              ) : (
                <form onSubmit={handleLinkIssue} className="space-y-3">
                  <input 
                    type="text" 
                    placeholder="Issue URL or Number..."
                    value={issueId}
                    onChange={(e) => setIssueId(e.target.value)}
                    className="w-full bg-black/50 border border-white/10 rounded-xl px-4 py-2 text-sm font-mono focus:outline-none focus:border-indigo-500/50"
                  />
                  <button type="submit" className="w-full py-2 bg-white/10 hover:bg-white/15 rounded-xl font-mono text-sm transition-colors">
                    Link Issue
                  </button>
                </form>
              )}
            </div>

            {/* Branch Management */}
            <div className="p-6 rounded-2xl bg-white/[0.02] border border-white/[0.05] backdrop-blur-md">
              <h2 className="text-lg font-medium text-gray-200 flex items-center gap-2 mb-4">
                <GitBranch className="text-indigo-400 w-5 h-5" />
                Branch
              </h2>
              {status?.branch_plan ? (
                <div className="space-y-3">
                  <div className="p-3 bg-black/30 border border-white/5 rounded-xl font-mono text-sm text-gray-300">
                    {status.branch_plan.suggested_name}
                  </div>
                  <button className="w-full py-2 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-300 rounded-xl font-mono text-sm transition-colors border border-indigo-500/20">
                    Create & Checkout
                  </button>
                </div>
              ) : (
                <button onClick={handleBranchPlan} className="w-full py-2 bg-white/10 hover:bg-white/15 rounded-xl font-mono text-sm transition-colors">
                  Plan Branch Name
                </button>
              )}
            </div>
          </div>

          {/* Right Column: PR & Review */}
          <div className="lg:col-span-2 space-y-6">
            
            {/* PR Draft Preview */}
            <div className="p-6 rounded-2xl bg-white/[0.02] border border-white/[0.05] backdrop-blur-md flex flex-col h-full">
              <div className="flex justify-between items-start mb-6">
                <h2 className="text-lg font-medium text-gray-200 flex items-center gap-2">
                  <GitPullRequest className="text-indigo-400 w-5 h-5" />
                  Pull Request Draft
                </h2>
                <div className="flex gap-2">
                  <button onClick={handleDraftPr} className="px-3 py-1 bg-white/10 hover:bg-white/15 rounded-lg text-sm font-mono transition-colors">
                    Generate Draft
                  </button>
                  {status?.pr_draft && (
                    <button className="px-3 py-1 bg-indigo-500/80 hover:bg-indigo-500 text-white rounded-lg text-sm font-mono transition-colors shadow-[0_0_15px_rgba(99,102,241,0.3)]">
                      Request PR Approval
                    </button>
                  )}
                </div>
              </div>

              {status?.pr_draft ? (
                <div className="flex-1 bg-black/40 border border-white/10 rounded-xl p-6 font-mono text-sm overflow-auto">
                  <h3 className="text-xl font-bold text-gray-200 mb-4 pb-4 border-b border-white/10">
                    {status.pr_draft.title}
                  </h3>
                  <pre className="text-gray-400 whitespace-pre-wrap font-sans">
                    {status.pr_draft.body}
                  </pre>
                  
                  <div className="mt-8 pt-4 border-t border-white/10 flex gap-4 text-xs text-gray-500">
                    <span className="flex items-center gap-1"><GitBranch className="w-3 h-3" /> Base: {status.pr_draft.base}</span>
                    <span className="flex items-center gap-1"><GitBranch className="w-3 h-3" /> Head: {status.pr_draft.head}</span>
                    {status.pr_draft.is_draft && <span className="bg-yellow-500/10 text-yellow-500 px-2 rounded">Draft PR</span>}
                  </div>
                </div>
              ) : (
                <div className="flex-1 flex flex-col items-center justify-center text-gray-500 font-mono border-2 border-dashed border-white/10 rounded-xl p-8">
                  <GitPullRequest className="w-8 h-8 mb-4 opacity-50" />
                  <p>No PR drafted yet.</p>
                  <p className="text-xs mt-2">Drafting aggregates timeline, jobs, and repo diffs.</p>
                </div>
              )}
            </div>

          </div>
        </div>

        {/* Security / Review Bar */}
        <div className="p-4 rounded-xl bg-orange-500/10 border border-orange-500/20 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <ShieldAlert className="text-orange-400 w-5 h-5" />
            <div>
              <h4 className="text-sm font-medium text-orange-200">Security & Review Checks</h4>
              <p className="text-xs font-mono text-orange-400/80 mt-1">
                Pushing code directly requires explicit user confirmation. Tokens are never exposed.
              </p>
            </div>
          </div>
          <button className="px-4 py-2 bg-orange-500/20 hover:bg-orange-500/30 text-orange-300 rounded-lg text-sm font-mono transition-colors">
            Run Pre-PR Review
          </button>
        </div>

      </div>
    </div>
  );
}
