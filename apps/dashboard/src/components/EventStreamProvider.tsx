'use client';

import { useEffect, useState } from 'react';
import { getGoatConfig } from '@/lib/goat-api';
import { Info, AlertTriangle, XCircle, X } from 'lucide-react';

interface Toast {
  id: string;
  title: string;
  message: string;
  type: 'info' | 'warning' | 'error';
}

export default function EventStreamProvider({ children }: { children: React.ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([]);

  useEffect(() => {
    const config = getGoatConfig();
    if (!config) return;

    const eventSourceUrl = `${config.baseUrl}/v1/events/stream`;
    
    // We cannot easily pass Authorization header to native EventSource.
    // Wait, by default daemon uses token. If we pass token in URL or we bypass it for SSE?
    // Let's assume the token is in URL for EventSource.
    const urlWithToken = `${eventSourceUrl}?token=${config.token}`;
    
    const sse = new EventSource(urlWithToken);

    sse.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        const type = data.severity.toLowerCase();
        
        setToasts(prev => [...prev, {
          id: data.id,
          title: data.kind,
          message: data.message,
          type: type === 'info' || type === 'warning' || type === 'error' ? type as any : 'info'
        }]);

        setTimeout(() => {
          setToasts(prev => prev.filter(t => t.id !== data.id));
        }, 5000);
      } catch (err) {
        console.error('Failed to parse event', err);
      }
    };

    sse.onerror = (err) => {
      console.error('EventSource failed:', err);
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
      {children}
      <div className="fixed bottom-4 right-4 z-50 flex flex-col gap-2 pointer-events-none w-80">
        {toasts.map(toast => (
          <div key={toast.id} className="pointer-events-auto flex items-start gap-3 bg-card border border-border p-4 rounded-lg shadow-lg animate-in slide-in-from-right">
            {getIcon(toast.type)}
            <div className="flex-1 min-w-0">
              <h4 className="text-sm font-semibold truncate">{toast.title}</h4>
              <p className="text-xs text-muted-foreground line-clamp-2 mt-1">{toast.message}</p>
            </div>
            <button onClick={() => removeToast(toast.id)} className="text-muted-foreground hover:text-foreground">
              <X className="w-4 h-4" />
            </button>
          </div>
        ))}
      </div>
    </>
  );
}
