'use client';

import React, { useState, useEffect } from 'react';
import { PageShell } from '@/components/ui/PageShell';
import { PageHeader } from '@/components/ui/PageHeader';
import { EmptyState, LoadingState } from '@/components/ui/States';
import { StatusBadge } from '@/components/ui/Status';
import { FileText, Search } from 'lucide-react';
import { reportsApi } from '@/lib/goat-api';

export default function ReportsPage() {
  const [searchQuery, setSearchQuery] = useState("");
  const [reports, setReports] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [selectedReport, setSelectedReport] = useState<any>(null);

  useEffect(() => {
    const fetchReports = async () => {
      try {
        const data = await reportsApi.listReports();
        if (data.reports) {
          // Sort newest first
          const sorted = data.reports.sort((a: any, b: any) => 
            new Date(b.created_at || b.date).getTime() - new Date(a.created_at || a.date).getTime()
          );
          setReports(sorted);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setIsLoading(false);
      }
    };
    fetchReports();
  }, []);

  const filteredReports = reports.filter(report => 
    (report.title || "").toLowerCase().includes(searchQuery.toLowerCase()) || 
    (report.kind || "").toLowerCase().includes(searchQuery.toLowerCase())
  );

  if (isLoading) return <PageShell><LoadingState title="Loading Reports..." /></PageShell>;

  return (
    <PageShell className="!p-0 !max-w-none flex h-full">
      <div className="w-80 border-r border-white/5 bg-[#0A0A0A] flex flex-col shrink-0 h-screen overflow-y-auto">
        <div className="p-6 border-b border-white/5 sticky top-0 bg-[#0A0A0A] z-10 space-y-4">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-emerald-500/10 text-emerald-400 rounded-lg">
              <FileText className="w-5 h-5" />
            </div>
            <span className="font-semibold text-white">Reports</span>
          </div>
          <div className="relative">
            <Search className="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
            <input 
              type="text" 
              placeholder="Search reports..." 
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-white focus:outline-none focus:border-emerald-500 transition-colors"
            />
          </div>
        </div>

        <div className="p-4 space-y-2">
          {filteredReports.map(report => (
            <button
              key={report.id}
              onClick={() => setSelectedReport(report)}
              className={`w-full text-left p-3 rounded-xl border transition-all ${
                selectedReport?.id === report.id ? 'bg-emerald-500/10 border-emerald-500/50' : 'bg-white/[0.02] border-white/5 hover:border-white/20'
              }`}
            >
              <div className="flex justify-between items-start mb-1">
                <span className="font-medium text-white text-sm truncate pr-2">{report.title}</span>
              </div>
              <div className="flex justify-between items-center mt-2">
                <span className="text-xs text-slate-500 capitalize">{report.kind || 'General'}</span>
                <StatusBadge status={report.status || 'Completed'} />
              </div>
            </button>
          ))}
          {filteredReports.length === 0 && (
            <div className="text-center p-4 text-sm text-slate-500">No reports found.</div>
          )}
        </div>
      </div>

      <div className="flex-1 h-screen overflow-y-auto p-8">
        {selectedReport ? (
          <div className="max-w-4xl mx-auto space-y-8">
            <PageHeader 
              title={selectedReport.title}
              subtitle={`Generated ${new Date(selectedReport.created_at).toLocaleString()} • ${selectedReport.kind || 'General'}`}
            />
            <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6">
              <pre className="text-sm text-slate-300 font-mono whitespace-pre-wrap">{selectedReport.markdown}</pre>
            </div>
          </div>
        ) : (
          <EmptyState 
            title="Generate your first report" 
            description="Select a report from the list to view its contents, or use an agent to generate a new one."
            icon={<FileText className="w-12 h-12" />}
          />
        )}
      </div>
    </PageShell>
  );
}
