'use client';

import { useState, useEffect } from 'react';
import { promptforgeApi, PromptForgeHistoryEntry } from '@/lib/goat-api';

export default function PromptForgePage() {
  const [status, setStatus] = useState<any>(null);
  const [config, setConfig] = useState<any>(null);
  const [history, setHistory] = useState<PromptForgeHistoryEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [prompt, setPrompt] = useState('');
  const [refinedPrompt, setRefinedPrompt] = useState('');
  const [refining, setRefining] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [sRes, cRes, hRes] = await Promise.all([
        promptforgeApi.getStatus(),
        promptforgeApi.getConfig(),
        promptforgeApi.getHistory()
      ]);
      setStatus(sRes);
      setConfig(cRes.config);
      setHistory(hRes.history || []);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleRefine = async () => {
    try {
      setRefining(true);
      setError(null);
      const res = await promptforgeApi.refine({
        original_prompt: prompt,
        target_agent: 'user',
        target_format: config?.default_target || 'goat',
        domain: 'general',
        complexity: 'medium',
        safe_context: '',
        constraints: [],
        mode: config?.mode || 'mock',
      });
      if (res.result?.refined_prompt) {
        setRefinedPrompt(res.result.refined_prompt);
      } else if (res.error) {
        setError(res.error);
      }
      await loadData(); // refresh history
    } catch (err: any) {
      setError(err.message);
    } finally {
      setRefining(false);
    }
  };

  if (loading && !status) return <div className="p-8">Loading PromptForge...</div>;

  return (
    <div className="p-8 max-w-6xl mx-auto space-y-8">
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">PromptForge</h1>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          Optional prompt refinement layer for compiling human intent into agent-ready instructions.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white dark:bg-gray-800 p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700">
          <h3 className="text-sm font-medium text-gray-500">Status</h3>
          <p className="mt-2 flex items-center">
            {status?.enabled ? (
              <span className="flex items-center text-green-600"><span className="w-2 h-2 rounded-full bg-green-500 mr-2"></span>Enabled</span>
            ) : (
              <span className="flex items-center text-red-600"><span className="w-2 h-2 rounded-full bg-red-500 mr-2"></span>Disabled</span>
            )}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700">
          <h3 className="text-sm font-medium text-gray-500">Refinement Mode</h3>
          <p className="mt-2 text-lg font-semibold uppercase">{status?.mode || 'Unknown'}</p>
        </div>
        <div className="bg-white dark:bg-gray-800 p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700">
          <h3 className="text-sm font-medium text-gray-500">Auto-Refine</h3>
          <p className="mt-2 text-lg font-semibold">{status?.auto_refine ? 'Enabled' : 'Disabled'}</p>
        </div>
      </div>

      {error && (
        <div className="bg-red-50 dark:bg-red-900/50 text-red-600 dark:text-red-400 p-4 rounded-lg">
          {error}
        </div>
      )}

      {!status?.enabled && (
        <div className="bg-yellow-50 dark:bg-yellow-900/30 border border-yellow-200 dark:border-yellow-800 p-6 rounded-xl">
          <h3 className="text-lg font-semibold text-yellow-800 dark:text-yellow-400">PromptForge is disabled</h3>
          <p className="mt-2 text-yellow-700 dark:text-yellow-500">
            PromptForge is disabled by default. It acts as an optional compiler to refine prompts before they reach prime agents.
            To use it, enable it in your <code>~/.config/goat/goat.toml</code> config.
          </p>
        </div>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div className="space-y-4">
          <h2 className="text-xl font-bold">Manual Refinement</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-1">Rough Prompt</label>
              <textarea
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
                className="w-full h-32 px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
                placeholder="e.g. build me a dashboard for tracking user growth..."
                disabled={!status?.enabled}
              />
            </div>
            <button
              onClick={handleRefine}
              disabled={refining || !prompt.trim() || !status?.enabled}
              className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors disabled:opacity-50"
            >
              {refining ? 'Refining...' : 'Refine Prompt'}
            </button>
          </div>

          {refinedPrompt && (
            <div className="mt-6 space-y-2">
              <label className="block text-sm font-medium">Refined Prompt</label>
              <div className="p-4 bg-gray-50 dark:bg-gray-900 border rounded-lg whitespace-pre-wrap font-mono text-sm">
                {refinedPrompt}
              </div>
            </div>
          )}
        </div>

        <div>
          <h2 className="text-xl font-bold mb-4">Refinement History</h2>
          <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700 overflow-hidden">
            {history.length > 0 ? (
              <ul className="divide-y divide-gray-100 dark:divide-gray-700">
                {history.slice().reverse().map((entry) => (
                  <li key={entry.id} className="p-4 hover:bg-gray-50 dark:hover:bg-gray-700/50">
                    <div className="flex justify-between items-start mb-2">
                      <span className={`px-2 py-1 text-xs font-medium rounded-full ${entry.status === 'success' ? 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400' : 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400'}`}>
                        {entry.status}
                      </span>
                      <span className="text-xs text-gray-500 font-mono text-right">{new Date(entry.timestamp * 1000).toLocaleString()}</span>
                    </div>
                    <p className="text-sm text-gray-800 dark:text-gray-200 truncate mb-1">
                      <span className="font-semibold">Original:</span> {entry.original_prompt}
                    </p>
                    {entry.status === 'success' && (
                      <p className="text-sm text-gray-500 truncate">
                        <span className="font-semibold">Refined:</span> {entry.refined_prompt}
                      </p>
                    )}
                  </li>
                ))}
              </ul>
            ) : (
              <div className="p-8 text-center text-gray-500">
                No history found.
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
