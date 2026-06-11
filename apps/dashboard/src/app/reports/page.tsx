"use client";

import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  FileText, Search, Download, Clock, 
  Bot, Filter, CheckCircle2, AlertTriangle, Eye
} from "lucide-react";

// Mock data for reports
const MOCK_REPORTS = [
  {
    id: "rep-1",
    title: "Phase 5.16 Audit Report",
    agent: "Architect Prime",
    date: "2026-06-11T10:00:00Z",
    status: "completed",
    content: "# Phase 5.16 Audit Report\n\n## Overview\nThis report summarizes the findings from the Phase 5.16 audit. \n\n## Key Findings\n- **Security**: Passed all checks.\n- **Performance**: 15% improvement in dashboard loading times.\n\n## Action Items\n- Implement new Cofounder Agent endpoints.\n- Update Next.js to 15.2.0.\n"
  },
  {
    id: "rep-2",
    title: "Nightly Security Scan",
    agent: "Security Prime",
    date: "2026-06-10T23:59:00Z",
    status: "warning",
    content: "# Nightly Security Scan\n\n## Warning\nFound 2 vulnerable dependencies in `apps/desktop`.\n\n- `lodash` < 4.17.21\n- `axios` < 1.6.0\n\n## Recommendations\nRun `npm audit fix`."
  },
  {
    id: "rep-3",
    title: "Frontend Build Summary",
    agent: "Frontend Specialist",
    date: "2026-06-11T08:30:00Z",
    status: "completed",
    content: "# Build Summary\n\n- **Target**: `apps/dashboard`\n- **Time**: 45s\n- **Size**: 1.2MB\n- **Status**: SUCCESS\n\nAll components successfully compiled with zero warnings."
  }
];

export default function ReportsPage() {
  const [searchQuery, setSearchQuery] = useState("");
  const [reports, setReports] = useState(MOCK_REPORTS);
  const [isLoading, setIsLoading] = useState(true);
  const [selectedReport, setSelectedReport] = useState<any>(null);

  useEffect(() => {
    // Simulate API fetch: GET /v1/reports
    const timer = setTimeout(() => {
      setReports(MOCK_REPORTS);
      setIsLoading(false);
    }, 600);
    return () => clearTimeout(timer);
  }, []);

  const filteredReports = reports.filter(report => 
    report.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
    report.agent.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const containerVariants = {
    hidden: { opacity: 0 },
    visible: { 
      opacity: 1, 
      transition: { staggerChildren: 0.05 }
    }
  };

  const itemVariants = {
    hidden: { opacity: 0, x: -20 },
    visible: { opacity: 1, x: 0 }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "completed": return <CheckCircle2 className="w-4 h-4 text-emerald-400" />;
      case "warning": return <AlertTriangle className="w-4 h-4 text-amber-400" />;
      default: return <Clock className="w-4 h-4 text-slate-400" />;
    }
  };

  return (
    <div className="flex-1 h-full flex flex-col relative overflow-hidden bg-[#0A0A0B] text-slate-200">
      {/* Background Orbs */}
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] rounded-full bg-emerald-600/10 blur-[120px] pointer-events-none" />
      
      {/* Header */}
      <header className="px-8 py-6 border-b border-white/5 bg-white/[0.02] backdrop-blur-md z-10 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-emerald-500/20 to-teal-500/20 border border-white/10 flex items-center justify-center shadow-lg shadow-emerald-500/10">
            <FileText className="w-6 h-6 text-emerald-400" />
          </div>
          <div>
            <h1 className="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-white/60">Reports</h1>
            <p className="text-sm text-slate-400">View and analyze agent generated outputs</p>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <div className="relative">
            <Search className="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
            <input 
              type="text" 
              placeholder="Search reports..." 
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="bg-black/40 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-white placeholder:text-slate-500 focus:outline-none focus:border-emerald-500/50 w-64"
            />
          </div>
          <button className="px-4 py-2 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white transition-colors flex items-center gap-2 text-sm font-medium">
            <Filter className="w-4 h-4" />
            <span>Filter</span>
          </button>
        </div>
      </header>

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden z-10">
        
        {/* Reports List */}
        <div className="w-1/3 border-r border-white/5 bg-black/20 overflow-y-auto custom-scrollbar flex flex-col">
          {isLoading ? (
            <div className="flex items-center justify-center h-32">
              <span className="w-6 h-6 border-2 border-emerald-500/30 border-t-emerald-500 rounded-full animate-spin" />
            </div>
          ) : (
            <motion.div variants={containerVariants} initial="hidden" animate="visible" className="flex flex-col">
              {filteredReports.map(report => (
                <motion.button
                  key={report.id}
                  variants={itemVariants}
                  onClick={() => setSelectedReport(report)}
                  className={`p-4 text-left border-b border-white/5 transition-all hover:bg-white/5 ${
                    selectedReport?.id === report.id ? 'bg-emerald-500/10 border-l-4 border-l-emerald-500' : 'border-l-4 border-l-transparent'
                  }`}
                >
                  <div className="flex items-start justify-between mb-2">
                    <h3 className="font-medium text-white line-clamp-1">{report.title}</h3>
                    {getStatusIcon(report.status)}
                  </div>
                  <div className="flex items-center gap-2 text-xs text-slate-400 mb-2">
                    <Bot className="w-3.5 h-3.5" />
                    <span className="truncate">{report.agent}</span>
                  </div>
                  <div className="flex items-center gap-2 text-[10px] text-slate-500">
                    <Clock className="w-3 h-3" />
                    <span>{new Date(report.date).toLocaleString()}</span>
                  </div>
                </motion.button>
              ))}
              {filteredReports.length === 0 && (
                <div className="p-8 text-center text-slate-500">
                  <FileText className="w-8 h-8 mx-auto mb-2 opacity-20" />
                  <p className="text-sm">No reports found.</p>
                </div>
              )}
            </motion.div>
          )}
        </div>

        {/* Report Content */}
        <div className="flex-1 bg-black/40 overflow-y-auto custom-scrollbar relative">
          <AnimatePresence mode="wait">
            {selectedReport ? (
              <motion.div
                key={selectedReport.id}
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -10 }}
                transition={{ duration: 0.2 }}
                className="p-8 max-w-4xl mx-auto"
              >
                <div className="flex justify-between items-start mb-8 pb-6 border-b border-white/10">
                  <div>
                    <h2 className="text-3xl font-bold text-white mb-2">{selectedReport.title}</h2>
                    <div className="flex items-center gap-4 text-sm text-slate-400">
                      <span className="flex items-center gap-1.5"><Bot className="w-4 h-4" /> {selectedReport.agent}</span>
                      <span className="flex items-center gap-1.5"><Clock className="w-4 h-4" /> {new Date(selectedReport.date).toLocaleString()}</span>
                    </div>
                  </div>
                  <button className="p-2 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-white transition-colors" title="Download Markdown">
                    <Download className="w-5 h-5" />
                  </button>
                </div>

                {/* Markdown Render Wrapper (Mock) */}
                <div className="prose prose-invert prose-emerald max-w-none prose-headings:text-slate-100 prose-p:text-slate-300 prose-li:text-slate-300">
                  {/* Basic parsing for mock markdown display */}
                  {selectedReport.content.split('\n').map((line: string, i: number) => {
                    if (line.startsWith('# ')) return <h1 key={i} className="text-3xl font-bold mt-8 mb-4">{line.slice(2)}</h1>;
                    if (line.startsWith('## ')) return <h2 key={i} className="text-2xl font-semibold mt-6 mb-3">{line.slice(3)}</h2>;
                    if (line.startsWith('- ')) {
                      // bold text parsing
                      const content = line.slice(2).replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>').replace(/`(.*?)`/g, '<code class="bg-white/10 px-1 py-0.5 rounded text-emerald-300">$1</code>');
                      return <li key={i} className="ml-4 list-disc" dangerouslySetInnerHTML={{ __html: content }} />;
                    }
                    if (line.trim() === '') return <br key={i} />;
                    return <p key={i} className="mb-4">{line}</p>;
                  })}
                </div>
              </motion.div>
            ) : (
              <motion.div
                key="empty"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full flex flex-col items-center justify-center text-slate-500"
              >
                <Eye className="w-16 h-16 mb-4 opacity-20" />
                <p>Select a report from the list to view its contents.</p>
              </motion.div>
            )}
          </AnimatePresence>
        </div>

      </div>
    </div>
  );
}
