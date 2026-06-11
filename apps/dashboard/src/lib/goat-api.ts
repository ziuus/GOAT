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
  searchBrain: (q: string, mode: string = 'keyword') => fetchGoat<any>(`/v1/brain/search?q=${encodeURIComponent(q)}&mode=${encodeURIComponent(mode)}`).catch(() => ({ results: [] })),
  recallBrain: (q: string, mode: string = 'keyword') => fetchGoat<any>(`/v1/brain/recall?q=${encodeURIComponent(q)}&mode=${encodeURIComponent(mode)}`).catch(() => ({ recall: {} })),
  reindexBrain: async () => {
    const config = getGoatConfig();
    if (!config) throw new Error('Not configured');
    const res = await fetch(`${config.baseUrl}/v1/brain/reindex`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${config.token}` },
    });
    return res.json();
  },
  getEmbeddingsStatus: () => fetchGoat<any>('/v1/brain/embeddings/status').catch(() => ({ enabled: false, total_vectors: 0, provider: 'none' })),
  rebuildEmbeddings: async () => {
    const config = getGoatConfig();
    if (!config) throw new Error('Not configured');
    const res = await fetch(`${config.baseUrl}/v1/brain/embeddings/rebuild`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${config.token}` },
    });
    return res.json();
  },
  get: (path: string) => fetchGoat<any>(path),
  post: async (path: string, body: any) => {
    const config = getGoatConfig();
    if (!config) throw new Error('Not configured');
    const res = await fetch(`${config.baseUrl}${path}`, {
      method: 'POST',
      headers: { 
        Authorization: `Bearer ${config.token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(body)
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

// Cofounder API Additions
export const cofounderApi = {
  getIdeas: () => fetchGoat<any>('/v1/cofounder/ideas').catch(() => ({ ideas: [] })),
  createIdea: (idea: any) => 
    daemonFetch('/v1/cofounder/ideas', { 
      method: 'POST', 
      body: JSON.stringify(idea) 
    }).then(res => res.json()),
  getIdea: (id: string) => fetchGoat<any>(`/v1/cofounder/ideas/${id}`).catch(() => null),
  validateIdea: (id: string) => 
    daemonFetch(`/v1/cofounder/ideas/${id}/validate`, { method: 'POST' }).then(res => res.json()),
  scoreIdea: (id: string) => 
    daemonFetch(`/v1/cofounder/ideas/${id}/score`, { method: 'POST' }).then(res => res.json()),
  generateReport: (id: string) => 
    daemonFetch(`/v1/cofounder/ideas/${id}/report`, { method: 'POST' }).then(res => res.json()),
  generateMvp: (id: string) => 
    daemonFetch(`/v1/cofounder/ideas/${id}/mvp`, { method: 'POST' }).then(res => res.json()),
  handoffToBuilder: (id: string) => 
    daemonFetch(`/v1/cofounder/ideas/${id}/handoff`, { method: 'POST' }).then(res => res.json()),
};

export interface SocializerCampaign {
  id: string;
  title: string;
  project_or_idea_ref: string | null;
  target_audience: string;
  value_proposition: string;
  state: string;
  created_at: number;
  updated_at: number;
}

export const socializerApi = {
  getStatus: () => fetchGoat<any>('/v1/socializer/status'),
  listCampaigns: () => fetchGoat<any>('/v1/socializer/campaigns'),
  createCampaign: (data: any) => daemonFetch('/v1/socializer/campaigns', { method: 'POST', body: JSON.stringify(data) }).then(r => r.json()),
  getCampaign: (id: string) => fetchGoat<any>(`/v1/socializer/campaigns/${id}`),
  generateAudience: (id: string) => daemonFetch(`/v1/socializer/campaigns/${id}/audience`, { method: 'POST' }).then(r => r.json()),
  generateChannels: (id: string) => daemonFetch(`/v1/socializer/campaigns/${id}/channels`, { method: 'POST' }).then(r => r.json()),
  generateAngles: (id: string) => daemonFetch(`/v1/socializer/campaigns/${id}/angles`, { method: 'POST' }).then(r => r.json()),
  generateDraft: (id: string, platform: string) => daemonFetch(`/v1/socializer/campaigns/${id}/${platform.toLowerCase()}`, { method: 'POST' }).then(r => r.json()),
  generateLaunch: (id: string) => daemonFetch(`/v1/socializer/campaigns/${id}/launch`, { method: 'POST' }).then(r => r.json()),
  generateCalendar: (id: string) => daemonFetch(`/v1/socializer/campaigns/${id}/calendar`, { method: 'POST' }).then(r => r.json()),
  generateOutreach: (id: string) => daemonFetch(`/v1/socializer/campaigns/${id}/outreach`, { method: 'POST' }).then(r => r.json()),
  generateFeedback: (id: string) => daemonFetch(`/v1/socializer/campaigns/${id}/feedback`, { method: 'POST' }).then(r => r.json()),
  generateReport: (id: string) => daemonFetch(`/v1/socializer/campaigns/${id}/report`, { method: 'POST' }).then(r => r.json()),
};

export interface PromptForgeHistoryEntry {
  id: string;
  timestamp: number;
  original_prompt: string;
  refined_prompt: string;
  status: string;
  mode: string;
}

export const promptforgeApi = {
  getStatus: () => daemonFetch('/v1/promptforge/status').then(r => r.json()),
  getDoctor: () => daemonFetch('/v1/promptforge/doctor').then(r => r.json()),
  getConfig: () => daemonFetch('/v1/promptforge/config').then(r => r.json()),
  getHistory: () => daemonFetch('/v1/promptforge/history').then(r => r.json()),
  refine: (data: any) => daemonFetch('/v1/promptforge/refine', { method: 'POST', body: JSON.stringify(data) }).then(r => r.json()),
};

export interface DesignerReview {
  id: string;
  target_type: string;
  path_or_url: string;
  state: string;
  created_at: number;
  updated_at: number;
}

export const designerApi = {
  getStatus: () => daemonFetch('/v1/designer/status').then(r => r.json()),
  listReviews: () => daemonFetch('/v1/designer/reviews').then(r => r.json()),
  createReview: (data: any) => daemonFetch('/v1/designer/reviews', { method: 'POST', body: JSON.stringify(data) }).then(r => r.json()),
  getReview: (id: string) => daemonFetch(`/v1/designer/reviews/${id}`).then(r => r.json()),
  scoreReview: (id: string) => daemonFetch(`/v1/designer/reviews/${id}/score`, { method: 'POST' }).then(r => r.json()),
  checkAccessibility: (id: string) => daemonFetch(`/v1/designer/reviews/${id}/accessibility`, { method: 'POST' }).then(r => r.json()),
  checkResponsive: (id: string) => daemonFetch(`/v1/designer/reviews/${id}/responsive`, { method: 'POST' }).then(r => r.json()),
  createPlan: (id: string) => daemonFetch(`/v1/designer/reviews/${id}/plan`, { method: 'POST' }).then(r => r.json()),
  createHandoff: (id: string) => daemonFetch(`/v1/designer/reviews/${id}/handoff`, { method: 'POST' }).then(r => r.json()),
  generateReport: (id: string) => daemonFetch(`/v1/designer/reviews/${id}/report`, { method: 'POST' }).then(r => r.json()),
};

export const researcherApi = {
  getStatus: () => daemonFetch("/v1/researcher/status").then(r => r.json()),
  listTopics: () => daemonFetch("/v1/researcher/topics").then(r => r.json()),
  createTopic: (data: any) => daemonFetch("/v1/researcher/topics", { method: "POST", body: JSON.stringify(data) }).then(r => r.json()),
  getTopic: (id: string) => daemonFetch(`/v1/researcher/topics/${id}`).then(r => r.json()),
  createPlan: (id: string) => daemonFetch(`/v1/researcher/topics/${id}/plan`, { method: "POST" }).then(r => r.json()),
  listSources: (id: string) => daemonFetch(`/v1/researcher/topics/${id}/sources`).then(r => r.json()),
  addSource: (id: string, data: any) => daemonFetch(`/v1/researcher/topics/${id}/sources`, { method: "POST", body: JSON.stringify(data) }).then(r => r.json()),
  listNotes: (id: string) => daemonFetch(`/v1/researcher/topics/${id}/notes`).then(r => r.json()),
  addNote: (id: string, data: any) => daemonFetch(`/v1/researcher/topics/${id}/notes`, { method: "POST", body: JSON.stringify(data) }).then(r => r.json()),
  generateCompetitors: (id: string) => daemonFetch(`/v1/researcher/topics/${id}/competitors`, { method: "POST" }).then(r => r.json()),
  generateCompare: (id: string) => daemonFetch(`/v1/researcher/topics/${id}/compare`, { method: "POST" }).then(r => r.json()),
  generateMarket: (id: string) => daemonFetch(`/v1/researcher/topics/${id}/market`, { method: "POST" }).then(r => r.json()),
  generateBrief: (id: string) => daemonFetch(`/v1/researcher/topics/${id}/brief`, { method: "POST" }).then(r => r.json()),
  generateReport: (id: string) => daemonFetch(`/v1/researcher/topics/${id}/report`, { method: "POST" }).then(r => r.json()),
};

export const operatorApi = {
  getStatus: () => daemonFetch("/v1/operator/status").then(r => r.json()),
  listSystems: () => daemonFetch("/v1/operator/systems").then(r => r.json()),
  createSystem: (data: any) => daemonFetch("/v1/operator/systems", { method: "POST", body: JSON.stringify(data) }).then(r => r.json()),
  getSystem: (id: string) => daemonFetch(`/v1/operator/systems/${id}`).then(r => r.json()),
  healthCheck: (id: string) => daemonFetch(`/v1/operator/systems/${id}/health`, { method: "POST" }).then(r => r.json()),
  logsCheck: (id: string) => daemonFetch(`/v1/operator/systems/${id}/logs`, { method: "POST" }).then(r => r.json()),
  incidentCheck: (id: string) => daemonFetch(`/v1/operator/systems/${id}/incident`, { method: "POST" }).then(r => r.json()),
  deployPlan: (id: string) => daemonFetch(`/v1/operator/systems/${id}/deploy-plan`, { method: "POST" }).then(r => r.json()),
  ciReview: (id: string) => daemonFetch(`/v1/operator/systems/${id}/ci`, { method: "POST" }).then(r => r.json()),
  rollbackPlan: (id: string) => daemonFetch(`/v1/operator/systems/${id}/rollback`, { method: "POST" }).then(r => r.json()),
  runbook: (id: string) => daemonFetch(`/v1/operator/systems/${id}/runbook`, { method: "POST" }).then(r => r.json()),
  reliability: (id: string) => daemonFetch(`/v1/operator/systems/${id}/reliability`, { method: "POST" }).then(r => r.json()),
  report: (id: string) => daemonFetch(`/v1/operator/systems/${id}/report`, { method: "POST" }).then(r => r.json()),
};
