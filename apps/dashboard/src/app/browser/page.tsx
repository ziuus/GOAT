'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Activity, Camera, ExternalLink, ShieldAlert, MonitorPlay, AlertTriangle, Code, Play, CheckCircle2, XCircle } from 'lucide-react';

interface BrowserStatus {
  enabled: boolean;
}

interface BrowserDoctor {
  doctor: string;
}

interface BrowserWorkflow {
  id: string;
  title: string;
  target_url: string;
  workflow_kind: string;
  status: string;
  risk_level: string;
  created_at: number;
}

export default function BrowserPage() {
  const [status, setStatus] = useState<BrowserStatus | null>(null);
  const [doctor, setDoctor] = useState<BrowserDoctor | null>(null);
  const [url, setUrl] = useState('http://localhost:3000');
  const [workflowTitle, setWorkflowTitle] = useState('My Automation Workflow');
  const [workflowKind, setWorkflowKind] = useState('ui-qa');
  const [loading, setLoading] = useState(false);
  const [workflows, setWorkflows] = useState<BrowserWorkflow[]>([]);
  const [selectedWorkflow, setSelectedWorkflow] = useState<any | null>(null);

  useEffect(() => {
    fetchStatus();
    fetchDoctor();
    fetchWorkflows();
  }, []);

  const fetchStatus = async () => {
    try {
      const res = await fetch('/api/v1/browser/status');
      if (res.ok) setStatus(await res.json());
    } catch (e) {
      console.error('Failed to fetch browser status', e);
    }
  };

  const fetchDoctor = async () => {
    try {
      const res = await fetch('/api/v1/browser/doctor');
      if (res.ok) setDoctor(await res.json());
    } catch (e) {
      console.error('Failed to fetch browser doctor', e);
    }
  };

  const fetchWorkflows = async () => {
    try {
      const res = await fetch('/api/v1/browser/workflows');
      if (res.ok) {
        const data = await res.json();
        setWorkflows(data.workflows || []);
      }
    } catch (e) {
      console.error('Failed to fetch workflows', e);
    }
  };

  const handleCreateWorkflow = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      const res = await fetch('/api/v1/browser/workflows', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          title: workflowTitle,
          target_url: url,
          workflow_kind: workflowKind,
        })
      });
      if (res.ok) {
        const data = await res.json();
        setSelectedWorkflow(data);
        fetchWorkflows();
      }
    } catch (e) {
      console.error('Failed to create workflow', e);
    }
    setLoading(false);
  };

  const handleQuickAction = async (kind: string) => {
    setLoading(true);
    try {
      let endpoint = 'qa';
      if (kind === 'screenshot') endpoint = 'screenshot';
      else if (kind === 'inspect') endpoint = 'read';
      else if (kind === 'landing-review') endpoint = 'landing-review';
      else if (kind === 'dashboard-qa') endpoint = 'dashboard-qa';
      else if (kind === 'web-health-check') endpoint = 'health';

      const res = await fetch(`/api/v1/browser/${endpoint}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ url })
      });
      if (res.ok) {
        const data = await res.json();
        setSelectedWorkflow(data);
        fetchWorkflows();
      }
    } catch (e) {
      console.error('Failed quick action', e);
    }
    setLoading(false);
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed': return <CheckCircle2 className="w-4 h-4 text-green-500" />;
      case 'failed': return <XCircle className="w-4 h-4 text-red-500" />;
      default: return <Activity className="w-4 h-4 text-yellow-500 animate-pulse" />;
    }
  };

  return (
    <div className="flex-1 overflow-y-auto p-8 space-y-8 bg-background">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight bg-gradient-to-r from-blue-400 to-indigo-500 bg-clip-text text-transparent">
            Browser QA & Workflows
          </h1>
          <p className="text-muted-foreground mt-2">
            Safe, approval-gated browser automation for local QA and text extraction (Phase 6.9).
          </p>
        </div>
        <div className="flex gap-2">
          {status?.enabled ? (
            <Badge variant="default" className="bg-green-500/10 text-green-500 border-green-500/20">Enabled</Badge>
          ) : (
            <Badge variant="outline" className="text-amber-500 border-amber-500/20">Disabled</Badge>
          )}
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left column: Create and Quick Actions */}
        <div className="lg:col-span-2 space-y-6">
          <Card className="border-border/50 bg-card/50 backdrop-blur">
            <CardHeader>
              <CardTitle className="text-xl flex items-center gap-2">
                <MonitorPlay className="w-5 h-5 text-blue-400" />
                Run Browser Workflow
              </CardTitle>
              <CardDescription>Launch a structured automation sequence.</CardDescription>
            </CardHeader>
            <CardContent>
              <form onSubmit={handleCreateWorkflow} className="space-y-4">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <label className="text-xs uppercase text-zinc-500 font-semibold">Workflow Title</label>
                    <Input
                      value={workflowTitle}
                      onChange={(e) => setWorkflowTitle(e.target.value)}
                      placeholder="e.g. Test Landing Page CTA"
                      className="bg-background/50"
                    />
                  </div>
                  <div className="space-y-2">
                    <label className="text-xs uppercase text-zinc-500 font-semibold">Workflow Kind</label>
                    <select
                      value={workflowKind}
                      onChange={(e) => setWorkflowKind(e.target.value)}
                      className="w-full h-10 px-3 rounded-md border border-input bg-background/50 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                    >
                      <option value="ui-qa">UI QA Workflow</option>
                      <option value="landing-review">Landing Review Workflow</option>
                      <option value="dashboard-qa">Dashboard QA Workflow</option>
                      <option value="web-health-check">Web Health Check</option>
                    </select>
                  </div>
                </div>

                <div className="space-y-2">
                  <label className="text-xs uppercase text-zinc-500 font-semibold">Target URL</label>
                  <Input
                    value={url}
                    onChange={(e) => setUrl(e.target.value)}
                    placeholder="Enter URL (e.g. http://localhost:3000)"
                    className="font-mono bg-background/50"
                  />
                </div>

                <Button type="submit" disabled={loading} className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium">
                  {loading ? 'Executing...' : 'Start Workflow'}
                </Button>
              </form>
            </CardContent>
          </Card>

          <Card className="border-border/50 bg-card/50 backdrop-blur">
            <CardHeader>
              <CardTitle className="text-lg">Quick Actions</CardTitle>
            </CardHeader>
            <CardContent className="flex flex-wrap gap-3">
              <Button variant="outline" onClick={() => handleQuickAction('screenshot')} disabled={loading}>
                <Camera className="w-4 h-4 mr-2" /> Capture Screenshot
              </Button>
              <Button variant="outline" onClick={() => handleQuickAction('inspect')} disabled={loading}>
                <Code className="w-4 h-4 mr-2" /> Inspect DOM
              </Button>
              <Button variant="outline" onClick={() => handleQuickAction('landing-review')} disabled={loading}>
                <ExternalLink className="w-4 h-4 mr-2" /> Landing Review
              </Button>
              <Button variant="outline" onClick={() => handleQuickAction('dashboard-qa')} disabled={loading}>
                <Play className="w-4 h-4 mr-2" /> Dashboard QA
              </Button>
              <Button variant="outline" onClick={() => handleQuickAction('web-health-check')} disabled={loading}>
                <Activity className="w-4 h-4 mr-2" /> Web Health Check
              </Button>
            </CardContent>
          </Card>
        </div>

        {/* Right column: Provider Info and Security */}
        <div className="space-y-6">
          <Card className="border-border/50 bg-card/50 backdrop-blur">
            <CardHeader>
              <CardTitle className="text-xl flex items-center gap-2">
                <Activity className="w-5 h-5 text-blue-400" />
                Browser Provider
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="p-4 rounded-md bg-muted/50 border border-border/50 text-xs font-mono text-muted-foreground whitespace-pre-wrap">
                {doctor?.doctor || 'Checking...'}
              </div>
              
              <div className="p-3 rounded-md bg-amber-500/10 border border-amber-500/20 flex gap-3">
                <ShieldAlert className="w-5 h-5 text-amber-500 shrink-0" />
                <div className="text-xs text-amber-500">
                  <strong>Safety Notice:</strong> No hidden browser sessions. All automation is visible, logged, and cancellable. Medium/High risk actions require ApprovalGate confirmation.
                </div>
              </div>
            </CardContent>
          </Card>

          <Card className="border-border/50 bg-card/50 backdrop-blur">
            <CardHeader>
              <CardTitle className="text-lg">Recent Workflows</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              {workflows.length === 0 ? (
                <p className="text-xs text-muted-foreground">No recent workflows.</p>
              ) : (
                workflows.slice(0, 5).map((w) => (
                  <div key={w.id} className="flex items-center justify-between p-2 rounded-md hover:bg-muted/30 cursor-pointer" onClick={async () => {
                    const res = await fetch(`/api/v1/browser/workflows/${w.id}`);
                    if (res.ok) setSelectedWorkflow(await res.json());
                  }}>
                    <div>
                      <p className="text-sm font-medium text-zinc-200">{w.title}</p>
                      <p className="text-[10px] text-zinc-500 font-mono">{w.id}</p>
                    </div>
                    {getStatusIcon(w.status)}
                  </div>
                ))
              )}
            </CardContent>
          </Card>
        </div>
      </div>

      {selectedWorkflow && (
        <Card className="border-border/50 bg-card/50 backdrop-blur mt-6">
          <CardHeader>
            <CardTitle className="text-xl flex items-center justify-between">
              <span>Workflow Execution Details</span>
              <Badge variant="outline">{selectedWorkflow.status.toUpperCase()}</Badge>
            </CardTitle>
            <CardDescription className="font-mono text-xs">{selectedWorkflow.id}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 p-4 rounded-lg bg-zinc-900/50 border border-zinc-800">
              <div>
                <p className="text-xs text-zinc-500 uppercase">Workflow Kind</p>
                <p className="text-sm font-semibold">{selectedWorkflow.workflow_kind}</p>
              </div>
              <div>
                <p className="text-xs text-zinc-500 uppercase">Target URL</p>
                <p className="text-sm font-semibold font-mono">{selectedWorkflow.target_url}</p>
              </div>
              <div>
                <p className="text-xs text-zinc-500 uppercase">Risk Level</p>
                <p className="text-sm font-semibold text-yellow-500">{selectedWorkflow.risk_level}</p>
              </div>
            </div>

            <div className="space-y-2">
              <h3 className="text-sm font-bold uppercase text-zinc-400">Steps Execution Trace</h3>
              <div className="space-y-2">
                {selectedWorkflow.steps.map((step: any, index: number) => (
                  <div key={index} className="flex items-start gap-3 p-3 rounded bg-zinc-900/30 border border-zinc-800/50">
                    <span className="text-xs text-zinc-500 font-mono">#{index + 1}</span>
                    <div className="flex-1">
                      <div className="flex items-center justify-between">
                        <p className="text-sm font-medium text-zinc-300 uppercase">{step.kind}</p>
                        <Badge variant="outline" className="text-xs">{step.status.toUpperCase()}</Badge>
                      </div>
                      {step.observation && (
                        <pre className="text-[11px] text-zinc-400 font-mono mt-2 bg-black/30 p-2 rounded overflow-x-auto whitespace-pre-wrap">
                          {step.observation}
                        </pre>
                      )}
                      {step.error_message && (
                        <p className="text-xs text-red-500 mt-1">Error: {step.error_message}</p>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
