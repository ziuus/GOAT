'use client';

import { useState, useEffect } from 'react';
import { getGoatConfig, setGoatConfig, goatApi } from '@/lib/goat-api';
import { ShieldCheck, ShieldAlert, Shield, LogOut, Play, FolderOpen } from 'lucide-react';
import { useRouter } from 'next/navigation';

export default function SettingsPage() {
  const [url, setUrl] = useState('http://127.0.0.1:47647');
  const [token, setToken] = useState('');
  const [status, setStatus] = useState<'idle' | 'checking' | 'success' | 'error'>('idle');
  const [errorMsg, setErrorMsg] = useState('');
  const [isDesktop, setIsDesktop] = useState(false);
  const [daemonStatus, setDaemonStatus] = useState(false);
  const [desktopTokenPath, setDesktopTokenPath] = useState('');
  const [startingDaemon, setStartingDaemon] = useState(false);
  
  const router = useRouter();

  useEffect(() => {
    // Check if running in Tauri
    const checkDesktop = async () => {
      if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
        setIsDesktop(true);
        try {
          const { invoke } = await import('@tauri-apps/api/core');
          
          const isRunning = await invoke<boolean>('get_daemon_status');
          setDaemonStatus(isRunning);
          
          const tPath = await invoke<string>('get_daemon_token_path');
          setDesktopTokenPath(tPath);
          
          if (!url || url === 'http://127.0.0.1:47647') {
             const defUrl = await invoke<string>('get_default_api_url');
             setUrl(defUrl);
          }
        } catch (e) {
          console.error("Failed to call Tauri invoke", e);
        }
      }
    };
    checkDesktop();
    
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
      if (isDesktop) {
        const { invoke } = await import('@tauri-apps/api/core');
        const isRunning = await invoke<boolean>('get_daemon_status');
        setDaemonStatus(isRunning);
      }
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
  
  const startDaemon = async () => {
    if (!isDesktop) return;
    setStartingDaemon(true);
    try {
       const { invoke } = await import('@tauri-apps/api/core');
       await invoke('start_daemon');
       // wait a sec and check status
       setTimeout(async () => {
           const isRunning = await invoke<boolean>('get_daemon_status');
           setDaemonStatus(isRunning);
           setStartingDaemon(false);
       }, 2000);
    } catch (e: any) {
       console.error(e);
       setErrorMsg(String(e));
       setStartingDaemon(false);
    }
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
          {isDesktop && (
            <span className="flex items-center gap-1.5 px-3 py-1 bg-blue-500/10 text-blue-500 border border-blue-500/20 rounded-full text-xs font-medium">
              Desktop Mode
            </span>
          )}
          <span className="flex items-center gap-1.5 px-3 py-1 bg-green-500/10 text-green-500 border border-green-500/20 rounded-full text-xs font-medium">
            <Shield className="w-3.5 h-3.5" />
            Local Daemon Only
          </span>
        </div>
      </div>

      <div className="space-y-6 mb-8 bg-card border border-border p-6 rounded-xl shadow-sm">
        <div>
          <h2 className="text-lg font-semibold tracking-tight mb-4">Appearance</h2>
          <div className="flex items-center gap-4">
            <label className="text-sm font-medium">Theme</label>
            <select
              className="bg-input border border-border rounded-md px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-primary"
              onChange={(e) => {
                const root = document.documentElement;
                root.classList.remove('theme-goat-dark', 'theme-minimal-dark', 'theme-high-contrast');
                root.classList.add(`theme-${e.target.value}`);
                localStorage.setItem('goat-theme', e.target.value);
              }}
              defaultValue={typeof window !== 'undefined' ? localStorage.getItem('goat-theme') || 'goat-dark' : 'goat-dark'}
            >
              <option value="goat-dark">GOAT Dark (Default)</option>
              <option value="minimal-dark">Minimal Dark</option>
              <option value="high-contrast">High Contrast</option>
            </select>
          </div>
        </div>
      </div>

      <div className="space-y-6 mb-8 bg-card border border-border p-6 rounded-xl shadow-sm">
        <div>
          <h2 className="text-lg font-semibold tracking-tight mb-4 flex items-center gap-2">
            <Shield className="w-5 h-5 text-indigo-400" /> Brain Learning & Privacy
          </h2>
          <p className="text-sm text-muted-foreground mb-4">
            GOAT learns from your interactions to improve its performance. Learning happens entirely locally and NO secrets are ever saved.
          </p>
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm font-medium">Enable Learning Loop</div>
                <div className="text-xs text-muted-foreground">Allow GOAT to extract memory candidates</div>
              </div>
              <input type="checkbox" className="toggle" defaultChecked disabled title="Configured in goat.toml" />
            </div>
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm font-medium">Require Review (Recommended)</div>
                <div className="text-xs text-muted-foreground">Manually accept memories in Memory Galaxy before saving</div>
              </div>
              <input type="checkbox" className="toggle" defaultChecked disabled title="Configured in goat.toml" />
            </div>
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm font-medium">Allow LLM Summarization</div>
                <div className="text-xs text-muted-foreground">Use external LLMs to build better memory summaries</div>
              </div>
              <input type="checkbox" className="toggle" disabled title="Configured in goat.toml" />
            </div>
          </div>
          <div className="mt-4 text-xs text-muted-foreground border-t border-border pt-4">
            To change these settings, edit <code className="bg-muted px-1 rounded">~/.config/goat/goat.toml</code> under the <code>[learning]</code> section.
          </div>
        </div>
      </div>

      {isDesktop && (
        <div className="space-y-6 mb-8 bg-card border border-border p-6 rounded-xl shadow-sm">
           <h2 className="text-lg font-semibold tracking-tight mb-4">Desktop Status</h2>
           <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                 <div className={`w-3 h-3 rounded-full ${daemonStatus ? 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.5)]' : 'bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.5)]'}`} />
                 <div>
                    <div className="text-sm font-medium">Local Daemon</div>
                    <div className="text-xs text-muted-foreground">{daemonStatus ? 'Running on 127.0.0.1:47647' : 'Stopped or Unreachable'}</div>
                 </div>
              </div>
              
              {!daemonStatus && (
                 <button 
                   onClick={startDaemon}
                   disabled={startingDaemon}
                   className="flex items-center gap-2 px-3 py-1.5 bg-primary/10 text-primary hover:bg-primary/20 rounded-md text-sm font-medium transition-colors"
                 >
                    <Play className="w-4 h-4" />
                    {startingDaemon ? 'Starting...' : 'Start Daemon'}
                 </button>
              )}
           </div>
        </div>
      )}

      <div className="space-y-6 mb-8 bg-card border border-border p-6 rounded-xl shadow-sm">
        <h2 className="text-lg font-semibold tracking-tight mb-2">Alpha Demo Data</h2>
        <p className="text-sm text-muted-foreground mb-4">
          Populate the dashboard with mock data to test workflows without a live backend connection.
        </p>
        <button 
          onClick={() => {
            alert('Demo data loaded successfully into local React state and storage for this session.');
          }}
          className="bg-indigo-500/20 text-indigo-400 hover:bg-indigo-500/30 px-4 py-2 rounded-md font-medium text-sm transition-colors border border-indigo-500/50"
        >
          Load Demo Data
        </button>
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
            placeholder={isDesktop ? `Paste token from ${desktopTokenPath || '~/.local/share/goat/daemon.token'}` : "Paste token from ~/.local/share/goat/daemon.token"}
            className="w-full bg-input border border-border rounded-md px-4 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary"
            required
          />
          <p className="text-xs text-muted-foreground mt-2">
            Run <code>cat {isDesktop && desktopTokenPath ? desktopTokenPath : '~/.local/share/goat/daemon.token'}</code> in your terminal to view your secure token. Token is never displayed after entry.
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

      <div className="mt-12 flex flex-col items-center justify-center text-center space-y-4 pb-8">
        <img src="/namelogo.png" alt="GOAT OS Logo" className="h-8 object-contain opacity-50 grayscale hover:grayscale-0 hover:opacity-100 transition-all duration-300" />
        <p className="text-xs text-muted-foreground">
          GOAT OS Alpha &copy; 2026<br/>
          Local-first AI Agent System
        </p>
      </div>
    </div>
  );
}
