'use client';

import { useState, useEffect } from 'react';
import { socializerApi, SocializerCampaign } from '@/lib/goat-api';

export default function SocializerPage() {
  const [campaigns, setCampaigns] = useState<SocializerCampaign[]>([]);
  const [selectedCampaign, setSelectedCampaign] = useState<SocializerCampaign | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [newTitle, setNewTitle] = useState('');
  const [newAudience, setNewAudience] = useState('');
  const [newProp, setNewProp] = useState('');

  useEffect(() => {
    loadCampaigns();
  }, []);

  const loadCampaigns = async () => {
    try {
      setLoading(true);
      const res = await socializerApi.listCampaigns();
      setCampaigns(res.campaigns || []);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      await socializerApi.createCampaign({
        title: newTitle,
        target_audience: newAudience,
        value_proposition: newProp,
        project_or_idea_ref: null
      });
      setNewTitle('');
      setNewAudience('');
      setNewProp('');
      await loadCampaigns();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const runWorkflow = async (id: string, action: string) => {
    try {
      setLoading(true);
      switch(action) {
        case 'audience': await socializerApi.generateAudience(id); break;
        case 'channels': await socializerApi.generateChannels(id); break;
        case 'angles': await socializerApi.generateAngles(id); break;
        case 'reddit': await socializerApi.generateDraft(id, 'Reddit'); break;
        case 'linkedin': await socializerApi.generateDraft(id, 'LinkedIn'); break;
        case 'x': await socializerApi.generateDraft(id, 'X'); break;
        case 'launch': await socializerApi.generateLaunch(id); break;
        case 'calendar': await socializerApi.generateCalendar(id); break;
        case 'outreach': await socializerApi.generateOutreach(id); break;
        case 'feedback': await socializerApi.generateFeedback(id); break;
        case 'report': await socializerApi.generateReport(id); break;
      }
      const res = await socializerApi.getCampaign(id);
      setSelectedCampaign(res.campaign);
      await loadCampaigns();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  if (loading && campaigns.length === 0) return <div className="p-8">Loading Socializer...</div>;

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8">
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Socializer Prime Agent</h1>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          Community building, ethical distribution, and relationship management.
        </p>
      </div>

      {error && (
        <div className="bg-red-50 dark:bg-red-900/50 text-red-600 dark:text-red-400 p-4 rounded-lg">
          {error}
        </div>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <div className="space-y-6">
          <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm p-6 border border-gray-100 dark:border-gray-700">
            <h2 className="text-xl font-semibold mb-4">New Campaign</h2>
            <form onSubmit={handleCreate} className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">Title</label>
                <input
                  type="text"
                  required
                  value={newTitle}
                  onChange={(e) => setNewTitle(e.target.value)}
                  className="w-full px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
                  placeholder="e.g. Next.js Boilerplate Launch"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Target Audience</label>
                <input
                  type="text"
                  required
                  value={newAudience}
                  onChange={(e) => setNewAudience(e.target.value)}
                  className="w-full px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
                  placeholder="e.g. React developers"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Value Proposition</label>
                <textarea
                  required
                  value={newProp}
                  onChange={(e) => setNewProp(e.target.value)}
                  className="w-full px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
                  placeholder="e.g. Saves 10 hours of setup time."
                />
              </div>
              <button
                type="submit"
                disabled={loading}
                className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors disabled:opacity-50"
              >
                Create Campaign
              </button>
            </form>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm p-6 border border-gray-100 dark:border-gray-700">
            <h2 className="text-xl font-semibold mb-4">Active Campaigns</h2>
            <div className="space-y-2">
              {campaigns.map((c) => (
                <button
                  key={c.id}
                  onClick={() => setSelectedCampaign(c)}
                  className={`w-full text-left p-3 rounded-lg border transition-colors ${
                    selectedCampaign?.id === c.id
                      ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                      : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700/50'
                  }`}
                >
                  <div className="font-medium truncate">{c.title}</div>
                  <div className="text-sm text-gray-500 truncate">{c.state}</div>
                </button>
              ))}
              {campaigns.length === 0 && (
                <p className="text-gray-500 text-sm">No campaigns found.</p>
              )}
            </div>
          </div>
        </div>

        <div className="lg:col-span-2">
          {selectedCampaign ? (
            <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm p-6 border border-gray-100 dark:border-gray-700 space-y-6">
              <div>
                <div className="flex items-center justify-between">
                  <h2 className="text-2xl font-bold">{selectedCampaign.title}</h2>
                  <span className="px-3 py-1 bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300 rounded-full text-sm font-medium">
                    {selectedCampaign.state}
                  </span>
                </div>
                <div className="mt-4 grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <span className="text-gray-500">Target Audience</span>
                    <p className="font-medium">{selectedCampaign.target_audience}</p>
                  </div>
                  <div>
                    <span className="text-gray-500">Value Proposition</span>
                    <p className="font-medium">{selectedCampaign.value_proposition}</p>
                  </div>
                </div>
              </div>

              <div className="border-t border-gray-200 dark:border-gray-700 pt-6">
                <h3 className="text-lg font-semibold mb-4">Workflow Actions</h3>
                <div className="flex flex-wrap gap-2">
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'audience')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Map Audience</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'channels')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Channel Strategy</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'angles')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Content Angles</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'reddit')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Draft Reddit</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'linkedin')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Draft LinkedIn</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'x')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Draft X</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'launch')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Launch Plan</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'calendar')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Calendar</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'outreach')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Outreach</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'feedback')} className="px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Track Feedback</button>
                  <button onClick={() => runWorkflow(selectedCampaign.id, 'report')} className="px-4 py-2 bg-blue-100 text-blue-700 hover:bg-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:hover:bg-blue-900/50 rounded-lg text-sm font-medium transition-colors disabled:opacity-50" disabled={loading}>Generate Report</button>
                </div>
              </div>

              <div className="border-t border-gray-200 dark:border-gray-700 pt-6">
                <h3 className="text-lg font-semibold mb-2">Ethics & Safety</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  The Socializer Agent strictly follows an ethical distribution policy. It generates content drafts and strategies but never automatically posts or mass-messages on your behalf. All outward actions require explicit user review and ApprovalGate clearance.
                </p>
              </div>
            </div>
          ) : (
            <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm p-12 border border-gray-100 dark:border-gray-700 flex flex-col items-center justify-center text-center">
              <div className="w-16 h-16 bg-gray-100 dark:bg-gray-700 rounded-full flex items-center justify-center mb-4">
                <svg className="w-8 h-8 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                </svg>
              </div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white">No Campaign Selected</h3>
              <p className="mt-1 text-gray-500">Select a campaign from the list or create a new one to start working on distribution.</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
