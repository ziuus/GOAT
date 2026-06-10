'use client';

import { useState, useEffect, useRef } from 'react';
import { chatApi, getGoatConfig } from '@/lib/goat-api';
import { goatEvents, GoatEvent } from '@/lib/goat-events';
import { Send, Loader2, MessageSquare, Plus, FileCode2, ChevronLeft, ChevronRight, Activity, ShieldAlert, CheckCircle2, XCircle, Bot, User, Clock, Terminal } from 'lucide-react';
import Link from 'next/link';
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

type Message = {
  id?: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  mode?: 'chat' | 'plan' | 'act';
  job_id?: string;
  status?: 'queued' | 'running' | 'approval_required' | 'completed' | 'failed';
  context_files?: string[];
  created_at?: string;
};

type Session = {
  id: string;
  title: string;
  created_at?: string;
};

export default function ChatPage() {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [activeSessionId, setActiveSessionId] = useState<string | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [mode, setMode] = useState<'chat' | 'plan' | 'act'>('chat');
  const [loading, setLoading] = useState(false);
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Load sessions
  const fetchSessions = async () => {
    try {
      const res = await chatApi.getSessions();
      if (res.sessions && res.sessions.length > 0) {
        setSessions(res.sessions);
        if (!activeSessionId) setActiveSessionId(res.sessions[0].id);
      } else {
        // Create default session if none exist
        const newSess = await chatApi.createSession('New Conversation');
        if (newSess.id) {
          setSessions([{ id: newSess.id, title: 'New Conversation' }]);
          setActiveSessionId(newSess.id);
        }
      }
    } catch (err) {
      console.error('Failed to load sessions', err);
    }
  };

  useEffect(() => {
    fetchSessions();
    goatEvents.connect();
    
    const handleJobEvent = (event: GoatEvent) => {
      // Update message statuses based on job events
      if (event.metadata?.job_id) {
        setMessages(prev => prev.map(msg => {
          if (msg.job_id === event.metadata.job_id) {
            let newStatus = msg.status;
            if (event.kind === 'job_started') newStatus = 'running';
            else if (event.kind === 'approval_required') newStatus = 'approval_required';
            else if (event.kind === 'job_completed') newStatus = 'completed';
            else if (event.kind === 'job_failed') newStatus = 'failed';
            
            return { ...msg, status: newStatus };
          }
          return msg;
        }));
      }
      
      // If it's a new message coming in via SSE
      if (event.kind === 'chat_message' && activeSessionId && event.metadata?.session_id === activeSessionId) {
        // Instead of trying to append smartly, maybe just refresh messages or append if not echo
        loadMessages(activeSessionId);
      }
    };

    const unsubAll = goatEvents.on('*', handleJobEvent);

    return () => {
      unsubAll();
      goatEvents.disconnect();
    };
  }, [activeSessionId]);

  const loadMessages = async (sid: string) => {
    try {
      setLoading(true);
      const res = await chatApi.getSessionMessages(sid);
      if (res.messages) {
        setMessages(res.messages);
      }
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
      scrollToBottom();
    }
  };

  useEffect(() => {
    if (activeSessionId) {
      loadMessages(activeSessionId);
    }
  }, [activeSessionId]);

  const scrollToBottom = () => {
    setTimeout(() => {
      messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, 100);
  };

  const handleCreateSession = async () => {
    try {
      const newSess = await chatApi.createSession('New Conversation');
      setSessions(prev => [{ id: newSess.id, title: 'New Conversation' }, ...prev]);
      setActiveSessionId(newSess.id);
    } catch (err) {
      console.error('Failed to create session', err);
    }
  };

  const handleSend = async () => {
    if (!input.trim() || loading || !activeSessionId) return;

    const userMsg = input.trim();
    const tempId = Date.now().toString();
    const tempMsg: Message = { id: tempId, role: 'user', content: userMsg, mode };
    setMessages(prev => [...prev, tempMsg]);
    setInput('');
    setLoading(true);
    scrollToBottom();

    try {
      const res = await chatApi.sendMessage(activeSessionId, userMsg, mode, []);
      if (res.error) {
        setMessages(prev => [...prev, { role: 'assistant', content: `[Error]: ${res.error}`, status: 'failed' }]);
      } else {
        // Server might return the actual job_id or response message immediately
        if (res.job_id) {
          setMessages(prev => [...prev, { 
            role: 'assistant', 
            content: `Started ${mode} job...`,
            job_id: res.job_id,
            status: 'queued'
          }]);
        } else if (res.message) {
          setMessages(prev => [...prev, { role: 'assistant', content: res.message }]);
        }
      }
    } catch (err: any) {
      setMessages(prev => [...prev, { role: 'assistant', content: `[Error]: Failed to send message. ${err.message}`, status: 'failed' }]);
    } finally {
      setLoading(false);
      scrollToBottom();
    }
  };

  const getStatusBadge = (status?: string) => {
    switch (status) {
      case 'queued':
        return <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium bg-zinc-500/10 text-zinc-400 border border-zinc-500/20"><Clock className="w-3 h-3"/> Queued</span>;
      case 'running':
        return <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium bg-blue-500/10 text-blue-400 border border-blue-500/20"><Loader2 className="w-3 h-3 animate-spin"/> Running</span>;
      case 'approval_required':
        return (
          <Link href="/approvals" className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium bg-amber-500/10 text-amber-400 border border-amber-500/20 hover:bg-amber-500/20 transition-colors">
            <ShieldAlert className="w-3 h-3"/> Approval Required
          </Link>
        );
      case 'completed':
        return <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"><CheckCircle2 className="w-3 h-3"/> Completed</span>;
      case 'failed':
        return <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium bg-red-500/10 text-red-400 border border-red-500/20"><XCircle className="w-3 h-3"/> Failed</span>;
      default:
        return null;
    }
  };

  return (
    <div className="flex h-[calc(100vh-4rem)] overflow-hidden bg-background">
      {/* Sidebar */}
      <div className={cn(
        "flex flex-col border-r border-border bg-card/30 backdrop-blur-sm transition-all duration-300 ease-in-out",
        sidebarOpen ? "w-64 opacity-100" : "w-0 opacity-0 overflow-hidden"
      )}>
        <div className="p-4 flex items-center justify-between border-b border-border/50">
          <h2 className="font-semibold text-sm tracking-tight text-foreground/80 flex items-center gap-2">
            <MessageSquare className="w-4 h-4" /> Sessions
          </h2>
          <button 
            onClick={handleCreateSession}
            className="p-1.5 hover:bg-white/5 rounded-md text-muted-foreground hover:text-foreground transition-colors"
          >
            <Plus className="w-4 h-4" />
          </button>
        </div>
        <div className="flex-1 overflow-y-auto p-2 space-y-1 custom-scrollbar">
          {sessions.map(s => (
            <button
              key={s.id}
              onClick={() => setActiveSessionId(s.id)}
              className={cn(
                "w-full text-left px-3 py-2.5 rounded-lg text-sm transition-all duration-200 flex items-center gap-3",
                activeSessionId === s.id 
                  ? "bg-primary/10 text-primary font-medium" 
                  : "text-muted-foreground hover:bg-white/5 hover:text-foreground"
              )}
            >
              <Terminal className={cn("w-4 h-4", activeSessionId === s.id ? "text-primary" : "text-muted-foreground/50")} />
              <span className="truncate">{s.title || 'Untitled Session'}</span>
            </button>
          ))}
        </div>
      </div>

      {/* Main Chat Area */}
      <div className="flex-1 flex flex-col min-w-0 relative bg-black/20 relative z-0">
        <button 
          onClick={() => setSidebarOpen(!sidebarOpen)}
          className="absolute left-0 top-1/2 -translate-y-1/2 -translate-x-1/2 z-10 p-1.5 bg-card border border-border rounded-full text-muted-foreground hover:text-foreground shadow-lg transition-transform hover:scale-110"
        >
          {sidebarOpen ? <ChevronLeft className="w-4 h-4" /> : <ChevronRight className="w-4 h-4" />}
        </button>

        {/* Header */}
        <header className="px-6 py-4 border-b border-border/50 bg-card/30 backdrop-blur-md flex items-center justify-between z-10">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center">
              <Bot className="w-4 h-4 text-primary" />
            </div>
            <div>
              <h1 className="text-lg font-semibold tracking-tight leading-tight">GOAT Agent</h1>
              <p className="text-xs text-muted-foreground flex items-center gap-1.5">
                <span className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                System Online
              </p>
            </div>
          </div>
          
          <div className="flex items-center gap-2 bg-black/40 p-1 rounded-lg border border-white/5">
            {(['chat', 'plan', 'act'] as const).map(m => (
              <button
                key={m}
                onClick={() => setMode(m)}
                className={cn(
                  "px-3 py-1.5 text-xs font-medium rounded-md capitalize transition-all duration-200",
                  mode === m 
                    ? "bg-primary text-primary-foreground shadow-md" 
                    : "text-muted-foreground hover:text-foreground hover:bg-white/5"
                )}
              >
                {m}
              </button>
            ))}
          </div>
        </header>

        {/* Messages */}
        <div className="flex-1 overflow-y-auto p-6 space-y-6 custom-scrollbar z-0 relative">
          {messages.length === 0 && (
            <div className="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground/50 gap-4">
              <div className="w-16 h-16 rounded-2xl bg-white/5 flex items-center justify-center border border-white/10 shadow-2xl">
                <Bot className="w-8 h-8 text-primary/50" />
              </div>
              <p className="text-sm font-medium">How can I assist you today?</p>
            </div>
          )}

          {messages.map((msg, i) => {
            const isUser = msg.role === 'user';
            return (
              <div key={msg.id || i} className={cn("flex gap-4 max-w-4xl mx-auto group animate-in slide-in-from-bottom-2 duration-300", isUser ? "flex-row-reverse" : "flex-row")}>
                <div className={cn(
                  "w-8 h-8 rounded-full flex-shrink-0 flex items-center justify-center border shadow-sm",
                  isUser 
                    ? "bg-card border-border" 
                    : "bg-primary/10 border-primary/20 text-primary"
                )}>
                  {isUser ? <User className="w-4 h-4 text-foreground/70" /> : <Bot className="w-4 h-4" />}
                </div>
                
                <div className={cn(
                  "flex flex-col gap-2 max-w-[80%]",
                  isUser ? "items-end" : "items-start"
                )}>
                  {/* Metadata line */}
                  {!isUser && msg.status && (
                    <div className="flex items-center gap-2 mb-1">
                      {getStatusBadge(msg.status)}
                      {msg.job_id && <span className="text-[10px] text-muted-foreground font-mono">ID: {msg.job_id.slice(0, 8)}</span>}
                    </div>
                  )}

                  <div className={cn(
                    "px-4 py-3 text-sm leading-relaxed whitespace-pre-wrap shadow-sm",
                    isUser 
                      ? "bg-primary text-primary-foreground rounded-2xl rounded-tr-sm" 
                      : "bg-card border border-border text-foreground rounded-2xl rounded-tl-sm"
                  )}>
                    {msg.content}
                  </div>

                  {msg.context_files && msg.context_files.length > 0 && (
                    <div className="flex flex-wrap gap-2 mt-2">
                      {msg.context_files.map(file => (
                        <div key={file} className="flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-white/5 border border-white/10 text-xs text-muted-foreground font-mono">
                          <FileCode2 className="w-3 h-3" />
                          {file.split('/').pop()}
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              </div>
            );
          })}
          
          {loading && (
            <div className="flex gap-4 max-w-4xl mx-auto animate-in fade-in duration-300">
              <div className="w-8 h-8 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center flex-shrink-0 text-primary">
                <Bot className="w-4 h-4" />
              </div>
              <div className="bg-card border border-border rounded-2xl rounded-tl-sm px-4 py-3 flex items-center gap-3 text-sm text-muted-foreground shadow-sm">
                <Loader2 className="w-4 h-4 animate-spin text-primary" />
                Processing request...
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* Input Area */}
        <div className="p-4 bg-background/50 backdrop-blur-xl border-t border-border z-10">
          <div className="max-w-4xl mx-auto relative group">
            {/* Ambient shadow */}
            <div className="absolute -inset-1 bg-gradient-to-r from-primary/20 via-primary/5 to-primary/20 rounded-xl blur opacity-20 group-hover:opacity-40 transition duration-500"></div>
            
            <div className="relative bg-card border border-border rounded-xl shadow-lg flex flex-col focus-within:ring-1 focus-within:ring-primary/50 focus-within:border-primary/50 transition-all duration-300">
              
              <textarea
                value={input}
                onChange={e => setInput(e.target.value)}
                onKeyDown={e => {
                  if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
                    e.preventDefault();
                    handleSend();
                  }
                }}
                placeholder="Message GOAT..."
                className="w-full bg-transparent px-4 py-4 text-sm resize-none focus:outline-none min-h-[56px] max-h-[200px] custom-scrollbar"
                rows={1}
              />
              
              <div className="flex items-center justify-between px-3 py-2 border-t border-border/50 bg-black/20 rounded-b-xl">
                <div className="flex items-center gap-2">
                  <button className="p-1.5 rounded-md hover:bg-white/10 text-muted-foreground hover:text-foreground transition-colors group/btn relative">
                    <Plus className="w-4 h-4" />
                    <span className="absolute -top-8 left-1/2 -translate-x-1/2 px-2 py-1 bg-foreground text-background text-[10px] rounded opacity-0 group-hover/btn:opacity-100 transition-opacity whitespace-nowrap pointer-events-none">Add Context</span>
                  </button>
                </div>
                
                <button
                  onClick={handleSend}
                  disabled={loading || !input.trim()}
                  className="px-3 py-1.5 bg-primary text-primary-foreground rounded-lg text-sm font-medium hover:bg-primary/90 hover:shadow-md hover:shadow-primary/20 disabled:opacity-50 disabled:hover:shadow-none transition-all duration-200 flex items-center gap-2"
                >
                  <Send className="w-3.5 h-3.5" />
                  Send
                </button>
              </div>
            </div>
            <div className="text-center mt-2">
              <span className="text-[10px] text-muted-foreground/60">Press Ctrl+Enter or Cmd+Enter to send, Enter for new line. Mode: {mode}.</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
