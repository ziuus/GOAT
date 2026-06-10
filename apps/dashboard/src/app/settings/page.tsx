'use client';

import { useState, useEffect } from 'react';
import { getGoatConfig, setGoatConfig, goatApi } from '@/lib/goat-api';
import { ShieldCheck, ShieldAlert } from 'lucide-react';
import { useRouter } from 'next/navigation';

export default function SettingsPage() {
  const [url, setUrl] = useState('http://127.0.0.1:47647');
  const [token, setToken] = useState('');
  const [status, setStatus] = useState<'idle' | 'checking' | 'success' | 'error'>('idle');
  const [errorMsg, setErrorMsg] = useState('');
  const router = useRouter();

  useEffect(() => {
    const conf = getGoatConfig();
    if (conf) {
      setUrl(conf.baseUrl);
      setToken(conf.token);
    }
  }, []);

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    setStatus('checking');
    setGoatConfig(url, token);
    
    try {
      await goatApi.getHealth();
      setStatus('success');
      setTimeout(() => router.push('/'), 1000);
    } catch (e: any) {
      setStatus('error');
      setErrorMsg(e.message);
    }
  };

  return (
    <div className="max-w-2xl mx-auto mt-12">
      <div className="mb-8">
        <h1 className="text-3xl font-bold tracking-tight mb-2">Connection Settings</h1>
        <p className="text-muted-foreground">Connect dashboard to your local GOAT Daemon.</p>
      </div>

      <form onSubmit={handleSave} className="space-y-6 bg-card border border-border p-6 rounded-xl shadow-sm">
        <div className="space-y-2">
          <label className="text-sm font-medium">Daemon API URL</label>
          <input 
            type="url" 
            value={url}
            onChange={e => setUrl(e.target.value)}
            className="w-full bg-input border border-border rounded-md px-4 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary"
            required
          />
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium">Daemon Token</label>
          <input 
            type="password" 
            value={token}
            onChange={e => setToken(e.target.value)}
            placeholder="Paste token from ~/.local/share/goat/daemon.token"
            className="w-full bg-input border border-border rounded-md px-4 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary"
            required
          />
          <p className="text-xs text-muted-foreground">
            Run <code>cat ~/.local/share/goat/daemon.token</code> in your terminal to view your secure token.
          </p>
        </div>

        <button 
          type="submit" 
          disabled={status === 'checking'}
          className="bg-primary text-primary-foreground px-4 py-2 rounded-md font-medium text-sm w-full hover:bg-primary/90 transition-colors disabled:opacity-50"
        >
          {status === 'checking' ? 'Connecting...' : 'Connect & Save'}
        </button>

        {status === 'success' && (
          <div className="p-3 bg-emerald-500/10 border border-emerald-500/20 text-emerald-500 rounded-md flex items-center gap-2 text-sm">
            <ShieldCheck className="w-4 h-4" /> Connected successfully!
          </div>
        )}

        {status === 'error' && (
          <div className="p-3 bg-destructive/10 border border-destructive text-destructive rounded-md flex items-center gap-2 text-sm">
            <ShieldAlert className="w-4 h-4" /> Connection failed: {errorMsg}
          </div>
        )}
      </form>
    </div>
  );
}
