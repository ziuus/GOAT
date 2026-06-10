'use client';

import { useState, useEffect } from 'react';
import { getGoatConfig, setGoatConfig, goatApi } from '@/lib/goat-api';
import { ShieldCheck, ShieldAlert, Shield, LogOut } from 'lucide-react';
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

  const clearToken = () => {
    setToken('');
    setGoatConfig(url, '');
    setStatus('idle');
    router.refresh();
  };

  const isLocalhost = url.includes('127.0.0.1') || url.includes('localhost');

  return (
    <div className="max-w-2xl mx-auto mt-12">
      <div className="mb-8 flex justify-between items-start">
        <div>
          <h1 className="text-3xl font-bold tracking-tight mb-2">Connection Settings</h1>
          <p className="text-muted-foreground">Connect dashboard to your local GOAT Daemon.</p>
        </div>
        <div className="flex flex-col items-end gap-2">
          <span className="flex items-center gap-1.5 px-3 py-1 bg-green-500/10 text-green-500 border border-green-500/20 rounded-full text-xs font-medium">
            <Shield className="w-3.5 h-3.5" />
            Local Daemon Only
          </span>
        </div>
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
          {!isLocalhost && (
            <p className="text-xs text-amber-500 flex items-center gap-1 mt-1">
              <ShieldAlert className="w-3 h-3" />
              Warning: Connecting to non-localhost URLs is not recommended for security.
            </p>
          )}
        </div>

        <div className="space-y-2">
          <div className="flex justify-between items-end">
            <label className="text-sm font-medium">Daemon Token</label>
            {token && (
              <button 
                type="button" 
                onClick={clearToken}
                className="text-xs text-destructive hover:text-destructive/80 flex items-center gap-1"
              >
                <LogOut className="w-3 h-3" /> Clear Token
              </button>
            )}
          </div>
          <input 
            type="password" 
            value={token}
            onChange={e => setToken(e.target.value)}
            placeholder="Paste token from ~/.local/share/goat/daemon.token"
            className="w-full bg-input border border-border rounded-md px-4 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary"
            required
          />
          <p className="text-xs text-muted-foreground mt-2">
            Run <code>cat ~/.local/share/goat/daemon.token</code> in your terminal to view your secure token. Token is never displayed after entry.
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
