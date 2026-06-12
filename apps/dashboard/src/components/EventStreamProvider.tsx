'use client';

import { useEffect, useState } from 'react';
import { getGoatConfig } from '@/lib/goat-api';
import { Info, AlertTriangle, XCircle, X } from 'lucide-react';
import { useRouter } from 'next/navigation';

interface Toast {
  id: string;
  title: string;
  message: string;
  type: 'info' | 'warning' | 'error';
  link?: string;
}

export default function EventStreamProvider({ children }: { children: React.ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([]);
  const [connected, setConnected] = useState(true);
  const router = useRouter();

  useEffect(() => {
    const config = getGoatConfig();
    if (!config) return;

    const eventSourceUrl = `${config.baseUrl}/v1/events/stream`;
    const urlWithToken = `${eventSourceUrl}?token=${config.token}`;
    const sse = new EventSource(urlWithToken);
    
    sse.onopen = () => setConnected(true);

    sse.onmessage = async (event) => {
      try {
        const data = JSON.parse(event.data);
        const type = data.severity.toLowerCase();
        
        let title = data.kind;
        let message = data.message;
        let link: string | undefined;

        // Redact secrets / format natively
        if (title === 'ApprovalRequested') {
          title = 'GOAT needs approval';
          message = 'A pending command or action requires your explicit approval.';
          link = '/approvals';
        }

        setToasts(prev => [...prev, {
          id: data.id,
          title,
          message,
          type: type === 'info' || type === 'warning' || type === 'error' ? type as any : 'info',
          link
        }]);

        // Attempt Native Notification
        if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
          try {
            const { isPermissionGranted, requestPermission, sendNotification } = await import('@tauri-apps/plugin-notification');
            let permissionGranted = await isPermissionGranted();
            if (!permissionGranted) {
              const permission = await requestPermission();
              permissionGranted = permission === 'granted';
            }
            if (permissionGranted) {
              sendNotification({ title, body: message });
            }
          } catch (e) {
            console.error('Failed to send native notification:', e);
          }
        }

        setTimeout(() => {
          setToasts(prev => prev.filter(t => t.id !== data.id));
        }, 5000);
      } catch (err) {
        console.error('Failed to parse event', err);
      }
    };

    sse.onerror = (err) => {
      console.error('EventSource failed:', err);
      setConnected(false);
    };

    return () => {
      sse.close();
    };
  }, []);

  const removeToast = (id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  };

  const getIcon = (type: string) => {
    switch (type) {
      case 'error': return <XCircle className="w-5 h-5 text-red-500" />;
      case 'warning': return <AlertTriangle className="w-5 h-5 text-yellow-500" />;
      default: return <Info className="w-5 h-5 text-blue-500" />;
    }
  };

  return (
    <>
      {!connected && (
        <div className="bg-red-500/10 border-b border-red-500/20 text-red-400 px-4 py-2 flex items-center justify-center gap-2 text-sm z-[100] relative">
          <AlertTriangle className="w-4 h-4" />
          <span><strong>Daemon Disconnected:</strong> GOAT requires the local daemon to be running. Run <code className="bg-black/20 px-1 rounded">cargo run --release -- daemon start</code> in your terminal, then refresh this page.</span>
          <button onClick={() => window.location.reload()} className="ml-4 px-2 py-0.5 bg-red-500/20 hover:bg-red-500/30 rounded text-xs">Retry</button>
        </div>
      )}
      {children}
      <div className="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
        {toasts.map(toast => (
          <div key={toast.id} className="bg-slate-900 border border-slate-800 rounded-lg shadow-xl p-4 w-80 flex items-start gap-3 transform transition-all animate-in slide-in-from-right-4">
            {getIcon(toast.type)}
            <div className="flex-1">
              <h4 className="text-sm font-medium text-white">{toast.title}</h4>
              <p className="text-sm text-slate-400 mt-1">{toast.message}</p>
              {toast.link && (
                <button 
                  onClick={() => router.push(toast.link!)}
                  className="mt-2 text-xs font-medium text-indigo-400 hover:text-indigo-300 transition-colors"
                >
                  View Details &rarr;
                </button>
              )}
            </div>
            <button 
              onClick={() => removeToast(toast.id)}
              className="text-slate-500 hover:text-white transition-colors"
            >
              <X className="w-4 h-4" />
            </button>
          </div>
        ))}
      </div>
    </>
  );
}
