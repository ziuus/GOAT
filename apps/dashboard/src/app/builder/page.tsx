'use client';

import { useState, useEffect } from 'react';
import { 
  TerminalSquare, FolderTree, AlertTriangle, ShieldCheck, 
  Play, RotateCcw, FileText, CheckCircle2, Cpu, FileCode2
} from 'lucide-react';

interface FileSummary {
  relative_path: string;
  size_bytes: number;
  is_risk_file: boolean;
}

interface InspectionResult {
  snapshot?: {
    root_path: string;
    file_count: number;
    tech_stack: {
      main_language: string;
      frameworks: string[];
      build_system: string;
    };
    files: FileSummary[];
  };
}

interface PatchPlan {
  id: string;
  goal: string;
  risk_level: string;
  affected_files: Array<{ path: string; change_description: string }>;
  patch_steps: Array<{ order: number; action: string; target_file: string; step_risk: string }>;
}

export default function BuilderPage() {
  const [goal, setGoal] = useState('');
  const [inspecting, setInspecting] = useState(false);
  const [planning, setPlanning] = useState(false);
  const [inspectData, setInspectData] = useState<InspectionResult | null>(null);
  const [planData, setPlanData] = useState<PatchPlan | null>(null);
  const [authToken, setAuthToken] = useState('');

  useEffect(() => {
    const token = localStorage.getItem('goat_api_token') || '';
    setAuthToken(token);
  }, []);

  const runInspect = async () => {
    setInspecting(true);
    try {
      const res = await fetch('/v1/builder/inspect', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`
        },
        body: JSON.stringify({ scope: { max_depth: 3, include_tests: true } })
      });
      const data = await res.json();
      setInspectData(data);
    } catch (e) {
      console.error(e);
    }
    setInspecting(false);
  };

  const generatePlan = async () => {
    if (!goal) return;
    setPlanning(true);
    try {
      const res = await fetch('/v1/builder/plan', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${authToken}`
        },
        body: JSON.stringify({ goal })
      });
      const data = await res.json();
      setPlanData(data);
    } catch (e) {
      console.error(e);
    }
    setPlanning(false);
  };

  return (
    <div className="space-y-8 p-8 max-w-7xl mx-auto text-slate-200">
      {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-white/5 pb-6">
        <div>
          <h1 className="text-3xl font-bold tracking-tight text-white flex items-center gap-3">
            <TerminalSquare className="w-8 h-8 text-indigo-500 animate-pulse" />
            Builder Agent Workspace
          </h1>
          <p className="text-sm text-slate-500 mt-2">
            Safe coding companion: plan patches, review diffs, compile and validate changes safely.
          </p>
        </div>
        <div className="flex items-center gap-3">
          <button 
            onClick={runInspect}
            disabled={inspecting}
            className="px-4 py-2 text-xs font-semibold bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 transition-all text-white flex items-center gap-2"
          >
            <FolderTree className="w-4 h-4 text-indigo-400" />
            {inspecting ? 'Inspecting...' : 'Inspect Repo'}
          </button>
        </div>
      </div>

      {/* Safety Notice */}
      <div className="bg-amber-500/5 border border-amber-500/10 rounded-xl p-4 flex gap-3">
        <ShieldCheck className="w-5 h-5 text-emerald-500 shrink-0 mt-0.5" />
        <div>
          <h4 className="text-sm font-semibold text-white">Safe Execution Boundary Notice</h4>
          <p className="text-xs text-slate-400 mt-1">
            Builder Agent will never apply file mutations or run build tasks directly without explicit ApprovalGate authorization. All patch execution requires manual validation.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Planning Composer & Goals */}
        <div className="lg:col-span-2 space-y-6">
          <div className="bg-[#0b0b0c] border border-white/5 rounded-xl p-6 space-y-4">
            <h3 className="text-base font-semibold text-white flex items-center gap-2">
              <Cpu className="w-4 h-4 text-indigo-400" />
              Goal Composer
            </h3>
            <div className="space-y-2">
              <textarea
                value={goal}
                onChange={(e) => setGoal(e.target.value)}
                placeholder="What would you like the Builder to implement or fix? (e.g., 'improve dashboard provider cards')"
                rows={4}
                className="w-full bg-[#121214] border border-white/5 rounded-lg p-3 text-sm focus:outline-none focus:border-indigo-500/50 text-slate-200 placeholder:text-slate-600 resize-none"
              />
            </div>
            <div className="flex items-center justify-between">
              <div className="text-[10px] text-slate-600 flex items-center gap-2">
                <span>Routing: <strong className="text-slate-400">Coding (Balanced)</strong></span>
                <span>•</span>
                <span>Context: <strong className="text-slate-400">Brain Packs (Active)</strong></span>
              </div>
              <button 
                onClick={generatePlan}
                disabled={planning || !goal}
                className="px-4 py-2 text-xs font-semibold bg-indigo-500 text-white rounded-lg hover:bg-indigo-600 transition-all flex items-center gap-2 disabled:opacity-50"
              >
                <Play className="w-3.5 h-3.5" />
                {planning ? 'Planning...' : 'Generate Patch Plan'}
              </button>
            </div>
          </div>

          {/* Active Patch Plan */}
          {planData && (
            <div className="bg-[#0b0b0c] border border-white/5 rounded-xl p-6 space-y-6">
              <div className="flex items-center justify-between border-b border-white/5 pb-4">
                <h3 className="text-base font-semibold text-white flex items-center gap-2">
                  <FileCode2 className="w-4 h-4 text-indigo-400" />
                  Active Patch Plan
                </h3>
                <span className="text-[10px] bg-red-500/10 text-red-400 border border-red-500/20 px-2 py-0.5 rounded font-mono uppercase">
                  Risk: {planData.risk_level}
                </span>
              </div>

              {/* Affected Files */}
              <div className="space-y-2">
                <h4 className="text-xs font-semibold text-slate-400">Affected Files</h4>
                <div className="space-y-1">
                  {planData.affected_files.map((f, i) => (
                    <div key={i} className="flex items-center justify-between bg-white/[0.02] border border-white/5 rounded-lg p-2 text-xs">
                      <span className="font-mono text-indigo-400">{f.path}</span>
                      <span className="text-slate-500">{f.change_description}</span>
                    </div>
                  ))}
                </div>
              </div>

              {/* Implementation Steps */}
              <div className="space-y-2">
                <h4 className="text-xs font-semibold text-slate-400">Implementation Steps</h4>
                <div className="space-y-2">
                  {planData.patch_steps.map((step, i) => (
                    <div key={i} className="flex items-start gap-3 bg-white/[0.01] border border-white/5 rounded-lg p-3 text-xs">
                      <span className="w-5 h-5 rounded-full bg-white/5 flex items-center justify-center font-bold shrink-0">{step.order}</span>
                      <div>
                        <p className="font-medium text-slate-300">{step.action}</p>
                        <p className="text-[10px] text-slate-500 mt-0.5">Target: {step.target_file} | Step Risk: {step.step_risk}</p>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Repository Snapshot Sidebar */}
        <div className="space-y-6">
          <div className="bg-[#0b0b0c] border border-white/5 rounded-xl p-6 space-y-4">
            <h3 className="text-sm font-semibold text-white flex items-center gap-2">
              <FolderTree className="w-4 h-4 text-indigo-400" />
              Workspace Snapshot
            </h3>

            {inspectData?.snapshot ? (
              <div className="space-y-4 text-xs">
                <div className="grid grid-cols-2 gap-2 bg-white/[0.02] border border-white/5 rounded-lg p-3">
                  <div>
                    <span className="text-slate-500 block text-[10px]">MAIN LANGUAGE</span>
                    <strong className="text-white">{inspectData.snapshot.tech_stack.main_language}</strong>
                  </div>
                  <div>
                    <span className="text-slate-500 block text-[10px]">BUILD SYSTEM</span>
                    <strong className="text-white">{inspectData.snapshot.tech_stack.build_system}</strong>
                  </div>
                </div>

                <div className="space-y-2">
                  <span className="text-slate-500 text-[10px] uppercase font-bold tracking-wider">Scanned Files ({inspectData.snapshot.file_count})</span>
                  <div className="max-h-60 overflow-y-auto space-y-1.5 scrollbar-none pr-1">
                    {inspectData.snapshot.files.map((file, i) => (
                      <div key={i} className="flex items-center justify-between p-2 rounded bg-[#121214] border border-white/5">
                        <span className="font-mono text-slate-300 truncate max-w-[150px]">{file.relative_path}</span>
                        <span className="text-[10px] text-slate-500 font-mono">{(file.size_bytes / 1024).toFixed(1)} KB</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            ) : (
              <p className="text-xs text-slate-500">Run repository inspection to scan the workspace and detect tech stack.</p>
            )}
          </div>

          {/* Test & Validation Panel */}
          <div className="bg-[#0b0b0c] border border-white/5 rounded-xl p-6 space-y-4">
            <h3 className="text-sm font-semibold text-white flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-indigo-400" />
              Automated Validation
            </h3>
            <div className="space-y-2 text-xs">
              <p className="text-slate-500">Validation suite command history:</p>
              <div className="bg-[#121214] border border-white/5 rounded-lg p-3 font-mono text-indigo-400">
                <div>cargo check</div>
                <div className="text-slate-600">cargo test</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
