'use client';

import { useState, useEffect } from 'react';
import { designerApi } from '@/lib/goat-api'; // using generic fetch wrapper

export default function DesignerPage() {
  const [reviews, setReviews] = useState<any[]>([]);
  const [selectedReview, setSelectedReview] = useState<any | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // New review form state
  const [targetType, setTargetType] = useState('general');
  const [pathOrUrl, setPathOrUrl] = useState('');
  const [description, setDescription] = useState('');

  useEffect(() => {
    loadReviews();
  }, []);

  const loadReviews = async () => {
    try {
      setLoading(true);
      const res = await designerApi.listReviews();
      if (res.reviews) {
        setReviews(res.reviews);
      }
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async () => {
    try {
      const res = await designerApi.createReview({
        target_type: targetType,
        path_or_url: pathOrUrl,
        description: description || null,
      });
      if (res.review) {
        setReviews([...reviews, res.review]);
        setSelectedReview(res.review);
      }
    } catch (err: any) {
      setError(err.message);
    }
  };

  const handleAction = async (id: string, action: string) => {
    try {
            let res: any;
      if (action === 'score') res = await designerApi.scoreReview(id);
      else if (action === 'accessibility') res = await designerApi.checkAccessibility(id);
      else if (action === 'responsive') res = await designerApi.checkResponsive(id);
      else if (action === 'plan') res = await designerApi.createPlan(id);
      else if (action === 'handoff') res = await designerApi.createHandoff(id);
      else if (action === 'report') res = await designerApi.generateReport(id);
      if (res.review) {
        setReviews(reviews.map(r => r.id === id ? res.review : r));
        setSelectedReview(res.review);
      }
      if (action === 'report' && res.report_id) {
        alert('Report generated: ' + res.report_id);
      }
    } catch (err: any) {
      setError(err.message);
    }
  };

  if (loading && reviews.length === 0) return <div className="p-8">Loading Designer...</div>;

  return (
    <div className="p-8 max-w-6xl mx-auto space-y-8">
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Designer Agent</h1>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          UI/UX design reviewer and improvement planner.
        </p>
      </div>

      <div className="bg-yellow-50 dark:bg-yellow-900/30 border border-yellow-200 dark:border-yellow-800 p-4 rounded-xl text-yellow-800 dark:text-yellow-400 text-sm">
        <strong>Safety Disclaimer:</strong> The Designer agent performs design review, inspection, and planning. It does not automatically rewrite UI code or break existing logic. To apply changes, use the Builder Handoff.
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <div className="lg:col-span-1 space-y-6">
          <div className="bg-white dark:bg-gray-800 p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700">
            <h2 className="text-lg font-bold mb-4">New Review</h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm mb-1">Target Type</label>
                <select value={targetType} onChange={e => setTargetType(e.target.value)} className="w-full p-2 border rounded dark:bg-gray-700 dark:border-gray-600">
                  <option value="dashboard">Dashboard</option>
                  <option value="landing">Landing Page</option>
                  <option value="onboarding">Onboarding</option>
                  <option value="form">Form</option>
                  <option value="mobile">Mobile Layout</option>
                  <option value="general">General UI</option>
                </select>
              </div>
              <div>
                <label className="block text-sm mb-1">Path or URL</label>
                <input value={pathOrUrl} onChange={e => setPathOrUrl(e.target.value)} placeholder="/dashboard or https://..." className="w-full p-2 border rounded dark:bg-gray-700 dark:border-gray-600" />
              </div>
              <div>
                <label className="block text-sm mb-1">Description (optional)</label>
                <textarea value={description} onChange={e => setDescription(e.target.value)} className="w-full p-2 border rounded dark:bg-gray-700 dark:border-gray-600 h-20" />
              </div>
              <button onClick={handleCreate} disabled={!pathOrUrl} className="w-full bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 rounded">
                Start Review
              </button>
            </div>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700 overflow-hidden">
            <h2 className="text-lg font-bold p-4 border-b dark:border-gray-700">Recent Reviews</h2>
            <ul className="divide-y divide-gray-100 dark:divide-gray-700 max-h-96 overflow-y-auto">
              {reviews.map(r => (
                <li key={r.id} onClick={() => setSelectedReview(r)} className={`p-4 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50 ${selectedReview?.id === r.id ? 'bg-indigo-50 dark:bg-indigo-900/20' : ''}`}>
                  <div className="font-semibold truncate">{r.title}</div>
                  <div className="text-xs text-gray-500 mt-1">{r.state}</div>
                </li>
              ))}
              {reviews.length === 0 && <li className="p-4 text-center text-gray-500 text-sm">No reviews yet.</li>}
            </ul>
          </div>
        </div>

        <div className="lg:col-span-2 space-y-6">
          {selectedReview ? (
            <div className="bg-white dark:bg-gray-800 p-6 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700 space-y-6">
              <div className="flex justify-between items-start">
                <div>
                  <h2 className="text-2xl font-bold">{selectedReview.title}</h2>
                  <p className="text-sm text-gray-500 mt-1">Status: <span className="font-semibold uppercase text-indigo-600 dark:text-indigo-400">{selectedReview.state}</span></p>
                </div>
              </div>

              <div className="flex flex-wrap gap-2">
                <button onClick={() => handleAction(selectedReview.id, 'score')} className="px-3 py-1 bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 rounded text-sm font-medium">Generate Scorecard</button>
                <button onClick={() => handleAction(selectedReview.id, 'accessibility')} className="px-3 py-1 bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 rounded text-sm font-medium">Accessibility Check</button>
                <button onClick={() => handleAction(selectedReview.id, 'responsive')} className="px-3 py-1 bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 rounded text-sm font-medium">Responsive Check</button>
                <button onClick={() => handleAction(selectedReview.id, 'plan')} className="px-3 py-1 bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 rounded text-sm font-medium">Improvement Plan</button>
                <button onClick={() => handleAction(selectedReview.id, 'handoff')} className="px-3 py-1 bg-indigo-100 text-indigo-800 hover:bg-indigo-200 dark:bg-indigo-900/50 dark:text-indigo-300 rounded text-sm font-medium">Builder Handoff</button>
                <button onClick={() => handleAction(selectedReview.id, 'report')} className="px-3 py-1 bg-gray-800 text-white hover:bg-gray-900 dark:bg-gray-200 dark:text-gray-900 rounded text-sm font-medium">Generate Report</button>
              </div>

              {selectedReview.scorecard && (
                <div className="p-4 bg-gray-50 dark:bg-gray-900 rounded-lg">
                  <h3 className="font-bold text-lg mb-2">Scorecard: {selectedReview.scorecard.total_score}/5.0</h3>
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                    <div>Clarity: {selectedReview.scorecard.clarity}</div>
                    <div>Hierarchy: {selectedReview.scorecard.visual_hierarchy}</div>
                    <div>Typography: {selectedReview.scorecard.typography}</div>
                    <div>Accessibility: {selectedReview.scorecard.accessibility}</div>
                  </div>
                </div>
              )}

              {selectedReview.issues.length > 0 && (
                <div>
                  <h3 className="font-bold text-lg mb-2">Issues Identified</h3>
                  <ul className="space-y-2">
                    {selectedReview.issues.map((issue: any) => (
                      <li key={issue.id} className="p-3 border rounded-lg dark:border-gray-700">
                        <div className="flex items-center gap-2 mb-1">
                          <span className="px-2 py-0.5 text-xs font-bold rounded bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400">{issue.severity}</span>
                          <span className="text-sm font-semibold">{issue.category.toUpperCase()}</span>
                        </div>
                        <p className="text-sm">{issue.description}</p>
                        <p className="text-sm text-gray-500 mt-1">Suggestion: {issue.suggestion}</p>
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              {selectedReview.improvement_plan && (
                <div className="p-4 bg-gray-50 dark:bg-gray-900 rounded-lg">
                  <h3 className="font-bold text-lg mb-2">Improvement Plan</h3>
                  <div className="text-sm space-y-2">
                    <p><strong>Quick Wins:</strong> {selectedReview.improvement_plan.quick_wins.join(', ')}</p>
                    <p><strong>Medium:</strong> {selectedReview.improvement_plan.medium_improvements.join(', ')}</p>
                  </div>
                </div>
              )}

              {selectedReview.handoff_brief && (
                <div className="p-4 bg-indigo-50 dark:bg-indigo-900/20 rounded-lg border border-indigo-100 dark:border-indigo-800">
                  <h3 className="font-bold text-lg mb-2 text-indigo-900 dark:text-indigo-300">Builder Handoff Brief</h3>
                  <div className="text-sm space-y-2 text-indigo-800 dark:text-indigo-200">
                    <p><strong>Goal:</strong> {selectedReview.handoff_brief.goal}</p>
                    <p><strong>Files:</strong> {selectedReview.handoff_brief.target_files.join(', ')}</p>
                    <p><strong>UI Changes:</strong></p>
                    <ul className="list-disc pl-5">
                      {selectedReview.handoff_brief.exact_ui_changes.map((c: string, i: number) => <li key={i}>{c}</li>)}
                    </ul>
                  </div>
                </div>
              )}
            </div>
          ) : (
            <div className="bg-white dark:bg-gray-800 p-12 rounded-xl shadow-sm border border-gray-100 dark:border-gray-700 flex items-center justify-center text-gray-500">
              Select a review from the sidebar or create a new one.
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
