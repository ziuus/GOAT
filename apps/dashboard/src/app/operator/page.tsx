'use client';

import { useState, useEffect } from 'react';
import { operatorApi } from '@/lib/goat-api';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  Server, ShieldCheck, Activity, TerminalSquare, AlertTriangle, Play, FileText, 
  Settings, Clock, CheckCircle2, XCircle, AlertCircle, RefreshCw, Layers
} from 'lucide-react';

export default function OperatorPage() {
  const [systems, setSystems] = useState<any[]>([]);
  const [selectedSystem, setSelectedSystem] = useState<any | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [newName, setNewName] = useState('');
  const [newType, setNewType] = useState('');
  const [newEnv, setNewEnv] = useState('');

  useEffect(() => {
    loadSystems();
  }, []);

  const loadSystems = async () => {
    try {
      setLoading(true);
      const res = await operatorApi.listSystems();
      if (res.systems) setSystems(res.systems);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async () => {
    try {
      const res = await operatorApi.createSystem({
        name: newName || 'Unnamed System',
        system_type: newType || 'Web Service',
        environment: newEnv || 'Production',
      });
      if (res.system) {
        setSystems([...systems, res.system]);
        setSelectedSystem(res.system);
        setNewName('');
        setNewType('');
        setNewEnv('');
      }
    } catch (err: any) {
      setError(err.message);
    }
  };

  const handleAction = async (id: string, action: string) => {
    try {
      let res: any;
      if (action === 'health') res = await operatorApi.healthCheck(id);
      else if (action === 'logs') res = await operatorApi.logsCheck(id);
      else if (action === 'incident') res = await operatorApi.incidentCheck(id);
      else if (action === 'deploy-plan') res = await operatorApi.deployPlan(id);
      else if (action === 'ci') res = await operatorApi.ciReview(id);
      else if (action === 'rollback') res = await operatorApi.rollbackPlan(id);
      else if (action === 'runbook') res = await operatorApi.runbook(id);
      else if (action === 'report') res = await operatorApi.report(id);

      if (res) alert(`Action ${action} completed successfully. (Plans generated, no destructive actions executed)`);
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <div className="min-h-screen bg-[#0A0A0A] text-slate-300 p-8">
      <div className="max-w-6xl mx-auto space-y-8">
        
        <header className="flex justify-between items-end border-b border-white/10 pb-6">
          <div>
            <h1 className="text-3xl font-bold text-white flex items-center gap-3">
              <TerminalSquare className="w-8 h-8 text-emerald-400" />
              Operator Prime
            </h1>
            <p className="text-sm text-slate-400 mt-2">
              System Health, Safe Deployments, Rollbacks, and Incident Analysis.
            </p>
          </div>
          <div className="flex items-center gap-2">
            <span className="flex items-center gap-1.5 px-3 py-1 bg-amber-500/10 text-amber-400 rounded-full text-xs font-medium border border-amber-500/20">
              <ShieldCheck className="w-3.5 h-3.5" /> ApprovalGate Active
            </span>
            <span className="flex items-center gap-1.5 px-3 py-1 bg-emerald-500/10 text-emerald-400 rounded-full text-xs font-medium border border-emerald-500/20">
              <Server className="w-3.5 h-3.5" /> Safety First
            </span>
          </div>
        </header>

        {error && (
          <div className="p-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm flex items-center gap-2">
            <AlertCircle className="w-4 h-4" /> {error}
          </div>
        )}

        <div className="grid grid-cols-12 gap-8">
          
          <div className="col-span-4 space-y-4">
            <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-5">
              <h2 className="text-sm font-semibold text-white mb-4 flex items-center gap-2">
                <Play className="w-4 h-4" /> Register System
              </h2>
              <div className="space-y-3">
                <input 
                  type="text" 
                  placeholder="System Name (e.g. GOAT API)" 
                  value={newName}
                  onChange={e => setNewName(e.target.value)}
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-emerald-500 transition-colors"
                />
                <input 
                  type="text" 
                  placeholder="System Type (e.g. Node Service)" 
                  value={newType}
                  onChange={e => setNewType(e.target.value)}
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-emerald-500 transition-colors"
                />
                <select 
                  value={newEnv}
                  onChange={e => setNewEnv(e.target.value)}
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-emerald-500 transition-colors"
                >
                  <option value="">Select Environment...</option>
                  <option value="Production">Production</option>
                  <option value="Staging">Staging</option>
                  <option value="Dev">Dev</option>
                </select>
                <button 
                  onClick={handleCreate}
                  className="w-full bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 font-medium text-sm py-2 rounded-lg transition-colors border border-emerald-500/50"
                >
                  Register System
                </button>
              </div>
            </div>

            <div className="space-y-2">
              <h3 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-3">Monitored Systems</h3>
              {systems.map(s => (
                <button
                  key={s.id}
                  onClick={() => setSelectedSystem(s)}
                  className={`w-full text-left p-4 rounded-xl border transition-all ${
                    selectedSystem?.id === s.id 
                      ? 'bg-emerald-500/10 border-emerald-500/50' 
                      : 'bg-white/[0.02] border-white/5 hover:border-white/20'
                  }`}
                >
                  <div className="flex items-center justify-between mb-1">
                    <h4 className="font-medium text-white truncate">{s.name}</h4>
                    <span className="text-[10px] uppercase bg-black px-2 py-0.5 rounded text-slate-400 border border-white/10">{s.environment}</span>
                  </div>
                  <div className="flex items-center gap-1.5 text-xs text-slate-500 mt-2">
                    <Activity className="w-3.5 h-3.5" />
                    <span>{s.status}</span>
                  </div>
                </button>
              ))}
            </div>
          </div>

          <div className="col-span-8">
            {selectedSystem ? (
              <div className="bg-white/[0.02] border border-white/5 rounded-2xl p-6">
                <div className="mb-6 pb-6 border-b border-white/10">
                  <h2 className="text-2xl font-bold text-white mb-2 flex items-center gap-3">
                    {selectedSystem.name}
                  </h2>
                  <p className="text-slate-400 text-sm">Type: {selectedSystem.system_type}</p>
                </div>

                <div className="bg-amber-500/5 border border-amber-500/20 p-4 rounded-xl mb-6 flex gap-3 text-amber-200 text-sm">
                  <AlertTriangle className="w-5 h-5 shrink-0 text-amber-400" />
                  <p><strong>Safety Notice:</strong> Operator does not execute destructive actions or production deployments automatically. All actions generate plans requiring explicit approval via ApprovalGate.</p>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <button onClick={() => handleAction(selectedSystem.id, 'health')} className="p-4 border border-white/10 hover:border-emerald-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <Activity className="w-5 h-5 text-emerald-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Health Check</h3>
                    <p className="text-xs text-slate-500 mt-1">Diagnose system health and connectivity.</p>
                  </button>
                  <button onClick={() => handleAction(selectedSystem.id, 'logs')} className="p-4 border border-white/10 hover:border-emerald-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <FileText className="w-5 h-5 text-emerald-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Review Logs</h3>
                    <p className="text-xs text-slate-500 mt-1">Find anomalies, with secrets redacted.</p>
                  </button>
                  <button onClick={() => handleAction(selectedSystem.id, 'incident')} className="p-4 border border-white/10 hover:border-emerald-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <AlertCircle className="w-5 h-5 text-emerald-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Incident Analysis</h3>
                    <p className="text-xs text-slate-500 mt-1">Determine root cause and mitigation.</p>
                  </button>
                  <button onClick={() => handleAction(selectedSystem.id, 'deploy-plan')} className="p-4 border border-white/10 hover:border-emerald-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <Layers className="w-5 h-5 text-emerald-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Deploy Plan</h3>
                    <p className="text-xs text-slate-500 mt-1">Generate pre-flight and release steps.</p>
                  </button>
                  <button onClick={() => handleAction(selectedSystem.id, 'rollback')} className="p-4 border border-white/10 hover:border-emerald-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <RefreshCw className="w-5 h-5 text-emerald-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Rollback Plan</h3>
                    <p className="text-xs text-slate-500 mt-1">Prepare safe revert procedures.</p>
                  </button>
                  <button onClick={() => handleAction(selectedSystem.id, 'runbook')} className="p-4 border border-white/10 hover:border-emerald-500/50 rounded-xl bg-black/40 text-left group transition-all">
                    <ShieldCheck className="w-5 h-5 text-emerald-400 mb-2" />
                    <h3 className="font-medium text-white text-sm">Create Runbook</h3>
                    <p className="text-xs text-slate-500 mt-1">Generate operational runbooks.</p>
                  </button>
                </div>
              </div>
            ) : (
              <div className="h-full min-h-[400px] border border-white/5 border-dashed rounded-2xl flex items-center justify-center text-slate-500 flex-col gap-4">
                <TerminalSquare className="w-12 h-12 opacity-20" />
                <p>Select a system or register a new one.</p>
              </div>
            )}
          </div>

        </div>
      </div>
    </div>
  );
}
