export interface GoatConfig {
  baseUrl: string;
  token: string;
}

export const getGoatConfig = (): GoatConfig | null => {
  if (typeof window === 'undefined') return null;
  const baseUrl = localStorage.getItem('goat_api_url') || 'http://127.0.0.1:47647';
  const token = localStorage.getItem('goat_api_token');
  if (!token) return null;
  return { baseUrl, token };
};

export const setGoatConfig = (baseUrl: string, token: string) => {
  if (typeof window === 'undefined') return;
  localStorage.setItem('goat_api_url', baseUrl);
  localStorage.setItem('goat_api_token', token);
};

export class GoatApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
    this.name = 'GoatApiError';
  }
}

async function fetchGoat<T>(path: string): Promise<T> {
  const config = getGoatConfig();
  if (!config) throw new Error('Not configured');

  const res = await fetch(`${config.baseUrl}${path}`, {
    headers: {
      Authorization: `Bearer ${config.token}`,
    },
  });

  if (!res.ok) {
    throw new GoatApiError(res.status, `API Error: ${res.statusText}`);
  }

  return res.json();
}

export const goatApi = {
  getHealth: () => fetchGoat<any>('/health'),
  getStatus: () => fetchGoat<any>('/v1/status'),
  getJobs: () => fetchGoat<any>('/v1/jobs'),
  getHooks: () => fetchGoat<any>('/v1/hooks'),
  getSchedule: () => fetchGoat<any>('/v1/schedule'),
  getMcpStatus: () => fetchGoat<any>('/v1/mcp/status').catch(() => ({ servers: [] })),
  getLogs: () => fetchGoat<any>('/v1/logs/recent').catch(() => ({ logs: [] })),
  getApprovals: () => fetchGoat<any>('/v1/approvals').catch(() => ({ approvals: [] })),
  approveRequest: async (id: string) => {
    const config = getGoatConfig();
    if (!config) throw new Error('Not configured');
    const res = await fetch(`${config.baseUrl}/v1/approvals/${id}/approve`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${config.token}` },
    });
    return res.json();
  },
  denyRequest: async (id: string) => {
    const config = getGoatConfig();
    if (!config) throw new Error('Not configured');
    const res = await fetch(`${config.baseUrl}/v1/approvals/${id}/deny`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${config.token}` },
    });
    return res.json();
  },
  getBrainStatus: () => fetchGoat<any>('/v1/brain/status').catch(() => ({ total_documents: 0 })),
  searchBrain: (q: string) => fetchGoat<any>(`/v1/brain/search?q=${encodeURIComponent(q)}`).catch(() => ({ results: [] })),
  recallBrain: (q: string) => fetchGoat<any>(`/v1/brain/recall?q=${encodeURIComponent(q)}`).catch(() => ({ recall: {} })),
  reindexBrain: async () => {
    const config = getGoatConfig();
    if (!config) throw new Error('Not configured');
    const res = await fetch(`${config.baseUrl}/v1/brain/reindex`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${config.token}` },
    });
    return res.json();
  },
};

export async function daemonFetch(path: string, options: RequestInit = {}): Promise<Response> {
  const config = getGoatConfig();
  if (!config) throw new Error('Not configured');

  return fetch(`${config.baseUrl}${path}`, {
    ...options,
    headers: {
      Authorization: `Bearer ${config.token}`,
      'Content-Type': 'application/json',
      ...options.headers,
    },
  });
}

// Chat API Additions
export const chatApi = {
  getSessions: () => fetchGoat<any>('/v1/chat/sessions').catch(() => ({ sessions: [] })),
  createSession: (title?: string) => 
    daemonFetch('/v1/chat/sessions', { 
      method: 'POST', 
      body: JSON.stringify({ title }) 
    }).then(res => res.json()),
  getSessionMessages: (id: string) => fetchGoat<any>(`/v1/chat/sessions/${id}/messages`).catch(() => ({ messages: [] })),
  sendMessage: (sessionId: string, message: string, mode: string, contextFiles: string[]) => 
    daemonFetch(`/v1/chat/sessions/${sessionId}/messages`, {
      method: 'POST',
      body: JSON.stringify({ message, mode, context_files: contextFiles })
    }).then(res => res.json()),
};
