'use client';

import { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { FolderTree, Settings, Play, CheckCircle, PackageSearch, Save, Activity } from 'lucide-react';
import { goatApi } from '@/lib/goat-api';

export default function ProjectProfilePage() {
  const [profile, setProfile] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    fetchProfile();
  }, []);

  const fetchProfile = async () => {
    try {
      const res = await goatApi.get('/v1/project-profile');
      if (res.profile) setProfile(res.profile);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const handleDetect = async () => {
    setLoading(true);
    try {
      const res = await goatApi.post('/v1/project-profile/detect', {});
      if (res.detected) setProfile(res.detected);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await goatApi.post('/v1/project-profile/save', { profile });
      // Show toast
    } catch (e) {
      console.error(e);
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return <div className="p-8 flex justify-center"><Activity className="w-8 h-8 animate-spin text-primary" /></div>;
  }

  return (
    <div className="p-8 max-w-5xl mx-auto">
      <div className="flex justify-between items-center mb-8">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Project Profile</h1>
          <p className="text-muted-foreground mt-1">Configure environment specifics and agent context.</p>
        </div>
        <div className="flex gap-3">
          <button 
            onClick={handleDetect}
            className="px-4 py-2 flex items-center gap-2 rounded-md bg-muted text-foreground hover:bg-muted/80 transition-colors"
          >
            <PackageSearch className="w-4 h-4" />
            Redetect
          </button>
          <button 
            onClick={handleSave}
            disabled={saving}
            className="px-4 py-2 flex items-center gap-2 rounded-md bg-primary text-primary-foreground hover:bg-primary/90 transition-colors shadow-sm"
          >
            {saving ? <Activity className="w-4 h-4 animate-spin" /> : <Save className="w-4 h-4" />}
            Save Profile
          </button>
        </div>
      </div>

      {profile ? (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div className="col-span-2 space-y-6">
            <motion.div 
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="p-6 rounded-2xl border bg-card shadow-sm"
            >
              <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
                <FolderTree className="w-5 h-5 text-primary" />
                Workspace Metadata
              </h2>
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-1">
                  <div className="text-sm text-muted-foreground">Type</div>
                  <div className="font-medium text-lg">{profile.kind}</div>
                </div>
                <div className="space-y-1">
                  <div className="text-sm text-muted-foreground">Root Directory</div>
                  <div className="font-mono text-sm bg-muted/50 p-1.5 rounded">{profile.project_root}</div>
                </div>
              </div>
            </motion.div>

            <motion.div 
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.1 }}
              className="p-6 rounded-2xl border bg-card shadow-sm"
            >
              <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
                <Play className="w-5 h-5 text-primary" />
                Execution Commands
              </h2>
              <div className="space-y-4">
                <div>
                  <label className="text-sm font-medium mb-1.5 block">Build Command</label>
                  <input 
                    type="text" 
                    value={profile.build_command || ''} 
                    onChange={(e) => setProfile({...profile, build_command: e.target.value})}
                    className="w-full bg-background border rounded-md px-3 py-2 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-primary/50"
                    placeholder="e.g. npm run build"
                  />
                </div>
                <div>
                  <label className="text-sm font-medium mb-1.5 block">Test Command</label>
                  <input 
                    type="text" 
                    value={profile.test_command || ''} 
                    onChange={(e) => setProfile({...profile, test_command: e.target.value})}
                    className="w-full bg-background border rounded-md px-3 py-2 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-primary/50"
                    placeholder="e.g. npm test"
                  />
                </div>
                <div>
                  <label className="text-sm font-medium mb-1.5 block">Lint Command</label>
                  <input 
                    type="text" 
                    value={profile.lint_command || ''} 
                    onChange={(e) => setProfile({...profile, lint_command: e.target.value})}
                    className="w-full bg-background border rounded-md px-3 py-2 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-primary/50"
                    placeholder="e.g. npm run lint"
                  />
                </div>
              </div>
            </motion.div>
          </div>

          <div className="space-y-6">
            <motion.div 
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.2 }}
              className="p-6 rounded-2xl border bg-card shadow-sm"
            >
              <h2 className="text-lg font-semibold mb-4 flex items-center gap-2">
                <Settings className="w-5 h-5 text-primary" />
                Recommendations
              </h2>
              
              <div className="space-y-5">
                <div>
                  <div className="text-sm font-medium mb-2">Agent Modes</div>
                  <div className="flex flex-wrap gap-2">
                    {profile.preferred_mode_profiles?.map((p: string) => (
                      <span key={p} className="px-2.5 py-1 bg-primary/10 text-primary border border-primary/20 rounded-md text-xs font-medium">{p}</span>
                    ))}
                  </div>
                </div>
                
                <div>
                  <div className="text-sm font-medium mb-2">Skill Packs</div>
                  <div className="flex flex-col gap-2">
                    {profile.recommended_skill_packs?.map((s: string) => (
                      <div key={s} className="flex items-center gap-2 text-sm text-muted-foreground bg-muted/50 p-2 rounded border border-border/50">
                        <CheckCircle className="w-4 h-4 text-green-500" />
                        {s}
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </motion.div>

            <motion.div 
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.3 }}
              className="p-6 rounded-2xl border bg-card shadow-sm bg-gradient-to-br from-card to-primary/5"
            >
              <h2 className="text-lg font-semibold mb-4">Setup Checklist</h2>
              <ul className="space-y-3">
                {[
                  { key: 'github_linked', label: 'GitHub Repository Linked' },
                  { key: 'mcp_tools_configured', label: 'MCP Tools Configured' },
                  { key: 'index_built', label: 'Brain Index Built' },
                  { key: 'environment_variables_set', label: 'Env Variables Set' },
                ].map(item => {
                  const isChecked = profile.checklist?.[item.key];
                  return (
                    <li key={item.key} className="flex items-center gap-3">
                      <div className={`w-5 h-5 rounded-full flex items-center justify-center border ${isChecked ? 'bg-primary border-primary text-primary-foreground' : 'border-muted-foreground/30'}`}>
                        {isChecked && <CheckCircle className="w-3.5 h-3.5" />}
                      </div>
                      <span className={`text-sm ${isChecked ? 'text-foreground' : 'text-muted-foreground'}`}>{item.label}</span>
                    </li>
                  )
                })}
              </ul>
            </motion.div>
          </div>
        </div>
      ) : (
        <div className="text-center p-12 bg-muted/20 rounded-2xl border border-dashed">
          No project profile loaded.
        </div>
      )}
    </div>
  );
}
