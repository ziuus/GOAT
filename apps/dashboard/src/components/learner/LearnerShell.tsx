'use client';

import { useState } from 'react';
import { Target, Compass, ListTodo, Code, Brain, Activity, FileText } from 'lucide-react';
import { LearnerSafetyNotice } from './LearnerSafetyNotice';
import { LearnerAgentStatus, AgentStatusType } from './LearnerAgentStatus';
import { LearnerOverview } from './LearnerOverview';
import { LearnerRoadmapTree } from './LearnerRoadmapTree';
import { LearnerTodayPanel } from './LearnerTodayPanel';
import { LearnerPracticePanel } from './LearnerPracticePanel';
import { LearnerRevisionPanel } from './LearnerRevisionPanel';
import { LearnerProgressPanel } from './LearnerProgressPanel';
import { LearnerReportPanel } from './LearnerReportPanel';
import { learnerApi } from '@/lib/goat-api';

interface Props {
  goal: any;
  onUpdate?: () => void;
}

type TabKey = 'overview' | 'roadmap' | 'today' | 'practice' | 'revise' | 'progress' | 'report';

export function LearnerShell({ goal, onUpdate }: Props) {
  const [activeTab, setActiveTab] = useState<TabKey>('overview');
  const [tabData, setTabData] = useState<Record<TabKey, any>>({} as any);
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState<{ type: AgentStatusType; msg: string }>({ type: 'idle', msg: '' });

  const fetchAction = async (action: TabKey) => {
    try {
      setLoading(true);
      setStatus({ type: 'generating', msg: `Generating ${action}...` });
      
      let res: any;
      if (action === 'roadmap') res = await learnerApi.roadmap(goal.id);
      else if (action === 'today') res = await learnerApi.today(goal.id);
      else if (action === 'practice') res = await learnerApi.practice(goal.id);
      else if (action === 'revise') res = await learnerApi.revise(goal.id);
      else if (action === 'progress') res = await learnerApi.progress(goal.id);
      else if (action === 'report') res = await learnerApi.report(goal.id);

      setTabData(prev => ({ ...prev, [action]: res }));
      setStatus({ type: 'success', msg: `${action} completed.` });
      
      setTimeout(() => setStatus({ type: 'idle', msg: '' }), 3000);
    } catch (err: any) {
      setStatus({ type: 'error', msg: `Failed: ${err.message}` });
      setTimeout(() => setStatus({ type: 'idle', msg: '' }), 5000);
    } finally {
      setLoading(false);
    }
  };

  const navigateTo = (tab: TabKey) => {
    setActiveTab(tab);
    if (tab !== 'overview' && !tabData[tab]) {
      fetchAction(tab);
    }
  };

  const tabs: { key: TabKey; label: string; icon: any }[] = [
    { key: 'overview', label: 'Overview', icon: Target },
    { key: 'roadmap', label: 'Roadmap', icon: Compass },
    { key: 'today', label: 'Today', icon: ListTodo },
    { key: 'practice', label: 'Practice', icon: Code },
    { key: 'revise', label: 'Revision', icon: Brain },
    { key: 'progress', label: 'Progress', icon: Activity },
    { key: 'report', label: 'Report', icon: FileText },
  ];

  return (
    <div className="bg-white/[0.02] border border-white/5 rounded-2xl overflow-hidden flex flex-col h-full min-h-[600px]">
      <LearnerAgentStatus status={status.type} message={status.msg} />
      
      {/* Header */}
      <div className="p-6 border-b border-white/10 bg-black/20">
        <h2 className="text-2xl font-bold text-white mb-2">{goal.title}</h2>
        <div className="flex gap-2 text-xs">
          <span className="px-2 py-1 bg-white/5 border border-white/10 rounded-md text-slate-400 uppercase tracking-wider">
            {goal.domain}
          </span>
          <span className="px-2 py-1 bg-white/5 border border-white/10 rounded-md text-slate-400">
            {goal.current_level} → {goal.target_level}
          </span>
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar Nav */}
        <div className="w-48 border-r border-white/5 p-4 space-y-1 overflow-y-auto hidden md:block">
          {tabs.map(t => {
            const active = activeTab === t.key;
            return (
              <button
                key={t.key}
                onClick={() => navigateTo(t.key)}
                className={`w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors ${
                  active 
                    ? 'bg-blue-500/10 text-blue-400' 
                    : 'text-slate-400 hover:text-white hover:bg-white/5'
                }`}
              >
                <t.icon className={`w-4 h-4 ${active ? 'text-blue-400' : 'text-slate-500'}`} />
                {t.label}
              </button>
            );
          })}
        </div>

        {/* Content Area */}
        <div className="flex-1 p-6 overflow-y-auto">
          <LearnerSafetyNotice />
          
          {activeTab === 'overview' && <LearnerOverview goal={goal} onNavigate={navigateTo as any} />}
          {activeTab === 'roadmap' && <LearnerRoadmapTree data={tabData.roadmap} loading={loading && !tabData.roadmap} onGenerate={() => fetchAction('roadmap')} />}
          {activeTab === 'today' && <LearnerTodayPanel data={tabData.today} loading={loading && !tabData.today} onGenerate={() => fetchAction('today')} />}
          {activeTab === 'practice' && <LearnerPracticePanel data={tabData.practice} loading={loading && !tabData.practice} onGenerate={() => fetchAction('practice')} />}
          {activeTab === 'revise' && <LearnerRevisionPanel data={tabData.revise} loading={loading && !tabData.revise} onGenerate={() => fetchAction('revise')} />}
          {activeTab === 'progress' && <LearnerProgressPanel data={tabData.progress} loading={loading && !tabData.progress} onGenerate={() => fetchAction('progress')} />}
          {activeTab === 'report' && <LearnerReportPanel data={tabData.report} loading={loading && !tabData.report} onGenerate={() => fetchAction('report')} />}
        </div>
      </div>
    </div>
  );
}
