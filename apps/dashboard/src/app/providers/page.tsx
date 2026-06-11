"use client";

import React, { useEffect, useState } from "react";
import Sidebar from "@/components/Sidebar";

export default function ProvidersPage() {
  const [providers, setProviders] = useState([]);
  const [loading, setLoading] = useState(true);
  const [routingTest, setRoutingTest] = useState({ agentId: "", taskKind: "", result: null });

  useEffect(() => {
    fetchProviders();
  }, []);

  const fetchProviders = async () => {
    try {
      const res = await fetch("/api/v1/providers");
      if (res.ok) {
        const data = await res.json();
        setProviders(data.providers || []);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const testRouting = async () => {
    try {
      const res = await fetch("/api/v1/models/route", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          agent_id: routingTest.agentId || "test_agent",
          task_kind: routingTest.taskKind || "generic",
          required_capabilities: [],
          local_only: false,
          allow_external: true,
          quality_preference: "balanced",
          latency_preference: "balanced",
          cost_preference: "balanced",
          fallback_allowed: true,
        }),
      });
      if (res.ok) {
        const data = await res.json();
        setRoutingTest((prev) => ({ ...prev, result: data.decision }));
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="flex h-screen bg-[#0A0A0B] text-white overflow-hidden">
      <Sidebar />
      <main className="flex-1 overflow-y-auto p-8 pt-12 relative">
        {/* Background glow effects */}
        <div className="absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none">
          <div className="absolute -top-[20%] -left-[10%] w-[50%] h-[50%] rounded-full bg-purple-500/10 blur-[120px]" />
          <div className="absolute top-[40%] right-[10%] w-[30%] h-[40%] rounded-full bg-blue-500/10 blur-[120px]" />
        </div>

        <div className="max-w-6xl mx-auto relative z-10">
          <h1 className="text-4xl font-light mb-2 tracking-tight">Provider & Model Routing</h1>
          <p className="text-gray-400 mb-10">
            Manage your AI providers, local models, and test routing decisions dynamically.
          </p>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            {/* Providers List */}
            <div className="bg-white/5 border border-white/10 rounded-2xl p-6 backdrop-blur-md">
              <div className="flex justify-between items-center mb-6">
                <h2 className="text-xl font-medium">Registered Providers</h2>
                <button
                  onClick={fetchProviders}
                  className="px-4 py-2 bg-white/10 hover:bg-white/20 transition rounded-lg text-sm font-medium"
                >
                  Refresh
                </button>
              </div>

              {loading ? (
                <div className="animate-pulse space-y-4">
                  {[1, 2, 3].map((i) => (
                    <div key={i} className="h-20 bg-white/5 rounded-xl"></div>
                  ))}
                </div>
              ) : providers.length === 0 ? (
                <p className="text-gray-400">No providers found.</p>
              ) : (
                <div className="space-y-4 max-h-[500px] overflow-y-auto pr-2 custom-scrollbar">
                  {providers.map((p: any) => (
                    <div
                      key={p.id}
                      className="p-4 rounded-xl border border-white/5 bg-black/20 hover:bg-black/40 transition group relative overflow-hidden"
                    >
                      <div className={`absolute top-0 left-0 w-1 h-full ${p.is_local ? 'bg-green-500' : 'bg-purple-500'}`} />
                      <div className="flex justify-between items-start pl-2">
                        <div>
                          <h3 className="font-semibold text-lg flex items-center gap-2">
                            {p.name}
                            <span className="text-xs px-2 py-0.5 rounded-full bg-white/10 text-gray-300">
                              {p.kind}
                            </span>
                          </h3>
                          <p className="text-sm text-gray-400 mt-1 truncate max-w-[250px]">{p.base_url}</p>
                        </div>
                        <div className="text-right">
                          <span className={`text-xs px-2 py-1 rounded-md ${p.is_local ? 'bg-green-500/20 text-green-300' : 'bg-blue-500/20 text-blue-300'}`}>
                            {p.is_local ? 'Local' : 'Cloud'}
                          </span>
                        </div>
                      </div>
                      
                      {p.models && p.models.length > 0 && (
                        <div className="mt-4 pl-2">
                          <p className="text-xs text-gray-500 mb-2 uppercase tracking-wider font-semibold">Available Models</p>
                          <div className="flex flex-wrap gap-2">
                            {p.models.map((m: string) => (
                              <span key={m} className="text-xs bg-white/5 border border-white/10 px-2 py-1 rounded-md text-gray-300">
                                {m}
                              </span>
                            ))}
                          </div>
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </div>

            {/* Routing Tester */}
            <div className="bg-white/5 border border-white/10 rounded-2xl p-6 backdrop-blur-md">
              <h2 className="text-xl font-medium mb-6">Test Model Routing</h2>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm text-gray-400 mb-1">Agent ID</label>
                  <input
                    type="text"
                    value={routingTest.agentId}
                    onChange={(e) => setRoutingTest({ ...routingTest, agentId: e.target.value })}
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2 focus:outline-none focus:border-purple-500 transition text-white"
                    placeholder="e.g. promptforge, researcher"
                  />
                </div>
                <div>
                  <label className="block text-sm text-gray-400 mb-1">Task Kind</label>
                  <input
                    type="text"
                    value={routingTest.taskKind}
                    onChange={(e) => setRoutingTest({ ...routingTest, taskKind: e.target.value })}
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2 focus:outline-none focus:border-purple-500 transition text-white"
                    placeholder="e.g. generic, reasoning, simple"
                  />
                </div>
                <button
                  onClick={testRouting}
                  className="w-full py-3 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-500 hover:to-blue-500 rounded-lg font-medium shadow-[0_0_20px_rgba(147,51,234,0.3)] transition-all active:scale-[0.98]"
                >
                  Evaluate Route
                </button>
              </div>

              {routingTest.result && (
                <div className="mt-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
                  <h3 className="text-sm text-gray-400 mb-3 uppercase tracking-wider font-semibold">Routing Decision</h3>
                  <div className="bg-black/40 border border-purple-500/30 rounded-xl p-4 shadow-[0_0_15px_rgba(147,51,234,0.1)] relative overflow-hidden">
                    <div className="absolute -right-10 -top-10 w-32 h-32 bg-purple-500/20 blur-2xl rounded-full" />
                    <pre className="text-sm font-mono text-gray-200 overflow-x-auto relative z-10">
                      {JSON.stringify(routingTest.result, null, 2)}
                    </pre>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
