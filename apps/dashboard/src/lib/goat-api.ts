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
};
