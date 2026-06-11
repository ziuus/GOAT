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

interface ExecutionSession {
  id: string;
  goal: string;
  status: string;
  affected_files: string[];
  checkpoint_id: string | null;
  validation_results: any[];
}

export default function BuilderPage() {
  const [goal, setGoal] = useState('');
  const [inspecting, setInspecting] = useState(false);
  const [planning, setPlanning] = useState(false);
  const [executing, setExecuting] = useState(false);
  const [inspectData, setInspectData] = useState<InspectionResult | null>(null);
  const [planData, setPlanData] = useState<PatchPlan | null>(null);
  const [executionSession, setExecutionSession] = useState<ExecutionSession | null>(null);
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
      if (data.inspection) setInspectData(data.inspection);
    } catch (e) {
      console.error(e);
    } finally {
      setInspecting(false);
    }
  };

  const generatePlan = async () => {
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
      if (data.plan) setPlanData(data.plan);
    } catch (e) {
      console.error(e);
    } finally {
      setPlanning(false);
    }
  };

  const executePlan = async () => {
    if (!planData) return;
    setExecuting(true);
    try {
      setExecutionSession({
        id: 'exec-' + planData.id,
        goal: planData.goal,
        status: 'waiting_for_approval',
        affected_files: planData.affected_files.map(f => f.path),
        checkpoint_id: null,
        validation_results: []
      });
    } catch (e) {
      console.error(e);
    } finally {
      setExecuting(false);
    }
  };

  const mockApprove = () => {
    if (executionSession) {
      setExecutionSession({ ...executionSession, status: 'approved' });
    }
  };

  const mockApply = () => {
    if (executionSession) {
      setExecutionSession({ ...executionSession, status: 'completed', checkpoint_id: 'cp-' + Date.now() });
    }
  };

  const mockRollback = () => {
    if (executionSession) {
      setExecutionSession({ ...executionSession, status: 'rolled_back' });
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#030303] text-slate-200">
      <header className="px-6 py-4 border-b border-white/5 shrink-0 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-indigo-500/10 rounded-lg">
            <TerminalSquare className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h1 className="text-sm font-semibold text-white">Builder / Code Execution</h1>
            <p className="text-xs text-slate-500">Plan, validate, and execute safe code patches.</p>
          </div>
        </div>
        <div className="flex items-center gap-2 text-xs">
          <span className="px-2 py-1 bg-white/5 rounded-full text-slate-400 flex items-center gap-1">
            <ShieldCheck className="w-3 h-3 text-emerald-400" /> Safe Execution Mode Active
          </span>
        </div>
      </header>

      <div className="flex-1 overflow-auto p-6 grid grid-cols-3 gap-6">
        <div className="col-span-2 space-y-6">
          <div className="bg-[#0b0b0c] border border-white/5 rounded-xl p-6 space-y-4">
            <h2 className="text-sm font-semibold text-white">Execution Goal</h2>
            <div className="flex gap-2">
              <input 
                value={goal}
                onChange={e => setGoal(e.target.value)}
                placeholder="Describe the feature or fix you want Builder to implement..."
                className="flex-1 bg-black/50 border border-white/5 rounded-lg px-4 py-2 text-sm text-white focus:outline-none focus:border-indigo-500/50"
              />
              <button 
                onClick={generatePlan}
                disabled={planning || !goal}
                className="px-4 py-2 bg-indigo-500 text-white text-sm font-medium rounded-lg hover:bg-indigo-600 disabled:opacity-50 transition-colors flex items-center gap-2"
              >
                {planning ? <RotateCcw className="w-4 h-4 animate-spin" /> : <Play className="w-4 h-4" />}
                Plan Patch
              </button>
            </div>
            <p className="text-xs text-amber-500/80 flex items-center gap-1 mt-2">
              <AlertTriangle className="w-3 h-3" />
              GOAT never mutates files without approval and checkpointing.
            </p>
          </div>

          {planData && !executionSession && (
            <div className="bg-[#0b0b0c] border border-white/5 rounded-xl p-6 space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
              <div className="flex items-center justify-between">
                <div>
                  <h3 className="text-sm font-semibold text-white">Patch Plan Preview</h3>
                  <p className="text-xs text-slate-500 mt-1">Goal: {planData.goal}</p>
                </div>
                <div className="flex items-center gap-3">
                  <span className={`px-2 py-1 rounded text-[10px] font-bold uppercase tracking-wider ${
                    planData.risk_level === 'High' ? 'bg-red-500/10 text-red-400' : 'bg-emerald-500/10 text-emerald-400'
                  }`}>
                    Risk: {planData.risk_level}
                  </span>
                  <button onClick={executePlan} disabled={executing} className="px-3 py-1.5 bg-white/10 hover:bg-white/15 text-white text-xs font-medium rounded-lg transition-colors">
                    Create Execution Session
                  </button>
                </div>
              </div>

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

          {executionSession && (
             <div className="bg-[#0b0b0c] border border-indigo-500/20 rounded-xl p-6 space-y-6 animate-in fade-in zoom-in duration-500 relative overflow-hidden">
               <div className="absolute top-0 left-0 w-1 h-full bg-indigo-500"></div>
               <div className="flex items-center justify-between">
                 <div>
                   <h3 className="text-sm font-semibold text-white">Active Execution Session</h3>
                   <p className="text-xs text-slate-500 mt-1">Status: <span className="text-indigo-400 font-mono">{executionSession.status}</span></p>
                 </div>
                 <div className="flex gap-2">
                   {executionSession.status === 'waiting_for_approval' && (
                     <button onClick={mockApprove} className="px-3 py-1.5 bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 text-xs font-medium rounded-lg">
                       Approve Execution
                     </button>
                   )}
                   {executionSession.status === 'approved' && (
                     <button onClick={mockApply} className="px-3 py-1.5 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-400 text-xs font-medium rounded-lg">
                       Apply & Checkpoint
                     </button>
                   )}
                   {executionSession.status === 'completed' && (
                     <button onClick={mockRollback} className="px-3 py-1.5 bg-red-500/20 hover:bg-red-500/30 text-red-400 text-xs font-medium rounded-lg">
                       Rollback Changes
                     </button>
                   )}
                 </div>
               </div>

               <div className="space-y-4">
                 <div className="bg-black/30 rounded p-3 text-xs font-mono text-slate-300">
                    <p>Affected Files:</p>
                    <ul className="list-disc pl-5 mt-2 space-y-1 text-slate-400">
                      {executionSession.affected_files.map((f, i) => <li key={i}>{f}</li>)}
                    </ul>
                 </div>
                 {executionSession.checkpoint_id && (
                   <p className="text-xs text-emerald-400 flex items-center gap-1">
                     <CheckCircle2 className="w-3 h-3" /> Checkpoint saved: {executionSession.checkpoint_id}
                   </p>
                 )}
               </div>
             </div>
          )}
        </div>

        <div className="space-y-6">
          <div className="bg-[#0b0b0c] border border-white/5 rounded-xl p-6 space-y-4">
            <h3 className="text-sm font-semibold text-white flex items-center gap-2">
              <FolderTree className="w-4 h-4 text-indigo-400" />
              Workspace Snapshot
            </h3>

            <button 
              onClick={runInspect}
              disabled={inspecting}
              className="w-full px-4 py-2 bg-white/5 text-white text-xs font-medium rounded-lg hover:bg-white/10 disabled:opacity-50 transition-colors flex items-center justify-center gap-2"
            >
              {inspecting ? <RotateCcw className="w-3 h-3 animate-spin" /> : <FileCode2 className="w-3 h-3" />}
              Inspect Local Repo
            </button>

            {inspectData ? (
              <div className="space-y-4 mt-4 animate-in fade-in duration-500">
                <div className="p-3 bg-black/40 rounded-lg space-y-2 border border-white/5">
                  <div className="flex justify-between text-xs">
                    <span className="text-slate-500">Language</span>
                    <span className="text-slate-300 font-medium">{inspectData.snapshot?.tech_stack.main_language}</span>
                  </div>
                  <div className="flex justify-between text-xs">
                    <span className="text-slate-500">Build System</span>
                    <span className="text-slate-300 font-medium">{inspectData.snapshot?.tech_stack.build_system}</span>
                  </div>
                  <div className="flex justify-between text-xs">
                    <span className="text-slate-500">Tracked Files</span>
                    <span className="text-slate-300 font-medium">{inspectData.snapshot?.file_count}</span>
                  </div>
                </div>
                
                <div className="space-y-2">
                  <h4 className="text-[10px] font-bold text-slate-500 uppercase tracking-wider">Top Risk Files</h4>
                  <div className="space-y-1">
                    {inspectData.snapshot?.files.filter((f: any) => f.is_risk_file).map((f: any, i: number) => (
                      <div key={i} className="flex items-center justify-between p-2 rounded bg-red-500/5 text-[10px]">
                        <span className="font-mono text-red-400/80 truncate pr-2">{f.relative_path}</span>
                        <ShieldCheck className="w-3 h-3 text-red-400/50 shrink-0" />
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
