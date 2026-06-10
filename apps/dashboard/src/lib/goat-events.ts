import { getGoatConfig } from './goat-api';

export type GoatEvent = {
  id: string;
  kind: string;
  message: string;
  severity: string;
  timestamp: string;
  metadata?: any;
};

export class GoatEventManager {
  private sse: EventSource | null = null;
  private listeners: Map<string, Array<(event: GoatEvent) => void>> = new Map();

  connect() {
    if (this.sse) return;
    const config = getGoatConfig();
    if (!config) return;

    const url = `${config.baseUrl}/v1/events/stream?token=${config.token}`;
    this.sse = new EventSource(url);

    this.sse.onmessage = (event) => {
      try {
        const data: GoatEvent = JSON.parse(event.data);
        const eventKind = data.kind; // e.g. chat_job_started, chat_job_completed
        
        // Call specific listeners
        if (this.listeners.has(eventKind)) {
          this.listeners.get(eventKind)?.forEach(fn => fn(data));
        }
        
        // Call wildcard listeners
        if (this.listeners.has('*')) {
          this.listeners.get('*')?.forEach(fn => fn(data));
        }
      } catch (err) {
        console.error('Failed to parse SSE event', err);
      }
    };

    this.sse.onerror = (err) => {
      console.error('SSE Error:', err);
    };
  }

  on(eventKind: string, callback: (event: GoatEvent) => void) {
    if (!this.listeners.has(eventKind)) {
      this.listeners.set(eventKind, []);
    }
    this.listeners.get(eventKind)?.push(callback);
    return () => this.off(eventKind, callback);
  }

  off(eventKind: string, callback: (event: GoatEvent) => void) {
    if (!this.listeners.has(eventKind)) return;
    const filtered = this.listeners.get(eventKind)?.filter(cb => cb !== callback) || [];
    this.listeners.set(eventKind, filtered);
  }

  disconnect() {
    if (this.sse) {
      this.sse.close();
      this.sse = null;
    }
  }
}

export const goatEvents = new GoatEventManager();
