'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Activity, Camera, ExternalLink, ShieldAlert, MonitorPlay, AlertTriangle, Code, Play } from 'lucide-react';

interface BrowserStatus {
  enabled: boolean;
}

interface BrowserDoctor {
  doctor: String;
}

export default function BrowserPage() {
  const [status, setStatus] = useState<BrowserStatus | null>(null);
  const [doctor, setDoctor] = useState<BrowserDoctor | null>(null);
  const [url, setUrl] = useState('http://localhost:3000');
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState<any[]>([]);

  useEffect(() => {
    fetchStatus();
    fetchDoctor();
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

  const handleAction = async (endpoint: string) => {
    setLoading(true);
    try {
      const res = await fetch(`/api/v1/browser/${endpoint}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ url })
      });
      const data = await res.json();
      setResults(prev => [{ time: new Date().toISOString(), action: endpoint, data }, ...prev]);
    } catch (e) {
      console.error(`Failed to execute ${endpoint}`, e);
    }
    setLoading(false);
  };

  return (
    <div className="flex-1 overflow-y-auto p-8 space-y-8 bg-background">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight bg-gradient-to-r from-blue-400 to-indigo-500 bg-clip-text text-transparent">
            Browser QA & Automation
          </h1>
          <p className="text-muted-foreground mt-2">
            Safe, approval-gated browser automation for local QA and text extraction.
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

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card className="md:col-span-2 border-border/50 bg-card/50 backdrop-blur">
          <CardHeader>
            <CardTitle className="text-xl flex items-center gap-2">
              <MonitorPlay className="w-5 h-5 text-blue-400" />
              Live Actions
            </CardTitle>
            <CardDescription>Target a URL to run QA actions.</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex gap-4">
              <Input
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                placeholder="Enter URL (e.g., http://localhost:3000)"
                className="font-mono bg-background/50"
              />
            </div>
            
            <div className="flex flex-wrap gap-3">
              <Button onClick={() => handleAction('qa')} disabled={loading} className="gap-2 bg-indigo-500 hover:bg-indigo-600">
                <Play className="w-4 h-4" /> Run Full QA
              </Button>
              <Button variant="outline" onClick={() => handleAction('open')} disabled={loading} className="gap-2">
                <ExternalLink className="w-4 h-4" /> Open URL
              </Button>
              <Button variant="outline" onClick={() => handleAction('screenshot')} disabled={loading} className="gap-2">
                <Camera className="w-4 h-4" /> Screenshot
              </Button>
              <Button variant="outline" onClick={() => handleAction('read')} disabled={loading} className="gap-2">
                <Code className="w-4 h-4" /> Read DOM
              </Button>
            </div>
          </CardContent>
        </Card>

        <Card className="border-border/50 bg-card/50 backdrop-blur">
          <CardHeader>
            <CardTitle className="text-xl flex items-center gap-2">
              <Activity className="w-5 h-5 text-blue-400" />
              Provider Status
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="p-4 rounded-md bg-muted/50 border border-border/50 text-sm font-mono text-muted-foreground whitespace-pre-wrap">
              {doctor?.doctor || 'Checking...'}
            </div>
            <div className="p-3 rounded-md bg-amber-500/10 border border-amber-500/20 flex gap-3">
              <ShieldAlert className="w-5 h-5 text-amber-500 shrink-0" />
              <div className="text-xs text-amber-500">
                <strong>Safety Policy Active:</strong> Medium/High risk actions require ApprovalGate confirmation.
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      <div className="space-y-4">
        <h2 className="text-xl font-semibold">Observation Timeline</h2>
        {results.length === 0 ? (
          <div className="p-12 border border-dashed border-border/50 rounded-xl flex flex-col items-center justify-center text-muted-foreground">
            <Activity className="w-8 h-8 mb-4 opacity-50" />
            <p>No actions run in this session.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {results.map((res, i) => (
              <Card key={i} className="border-border/50">
                <CardHeader className="py-3 px-4 bg-muted/30 border-b border-border/50">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Badge variant="outline" className="uppercase text-[10px]">{res.action}</Badge>
                      <span className="text-xs text-muted-foreground">{new Date(res.time).toLocaleTimeString()}</span>
                    </div>
                  </div>
                </CardHeader>
                <CardContent className="p-4">
                  <pre className="text-xs font-mono text-muted-foreground bg-background/50 p-3 rounded overflow-x-auto">
                    {JSON.stringify(res.data, null, 2)}
                  </pre>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
