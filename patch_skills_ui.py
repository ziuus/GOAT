import re

with open("apps/dashboard/src/app/skills/page.tsx", "r") as f:
    content = f.read()

# Add a hook to fetch installed skills
if "const [installedSkills, setInstalledSkills] = useState" not in content:
    content = content.replace(
        'const [selectedSkill, setSelectedSkill] = useState<any>(null);',
        """const [selectedSkill, setSelectedSkill] = useState<any>(null);
  const [installedSkills, setInstalledSkills] = useState<any[]>([]);

  useEffect(() => {
    fetch("http://127.0.0.1:3000/v1/skills/installed", {
      headers: { "x-goat-token": localStorage.getItem("goat_token") || "goat_dev_token" }
    })
      .then(r => r.json())
      .then(d => {
        if (d.installed) {
          setInstalledSkills(d.installed);
        }
      })
      .catch(console.error);
  }, []);"""
    )

# Replace the placeholder for installed tab
installed_jsx = """              {activeTab === "installed" && (
                <>
                  <div className="flex items-center justify-between mb-6">
                    <h2 className="text-xl font-semibold flex items-center gap-2 text-white">
                      <Download className="text-emerald-400" />
                      Installed Skills
                    </h2>
                  </div>
                  <div className="grid gap-4 mt-4">
                    {installedSkills.length === 0 ? (
                      <div className="text-slate-400 text-sm">No skills installed yet. Use the CLI to create one: `goat skill new --name myskill`</div>
                    ) : (
                      installedSkills.map((skill: any) => (
                        <motion.div variants={itemVariants} key={skill.name} className="bg-white/5 border border-white/10 rounded-2xl p-5 hover:bg-white/[0.07] hover:border-white/20 transition-all flex flex-col gap-4 group">
                          <div className="flex justify-between items-start">
                            <div>
                              <h3 className="text-lg font-bold text-white flex items-center gap-2">
                                {skill.name}
                                {skill.is_suspicious ? (
                                  <span className="px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-red-500/20 text-red-400 border border-red-500/30">Suspicious</span>
                                ) : (
                                  <span className="px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">Safe</span>
                                )}
                              </h3>
                              <div className="text-xs text-slate-400 mt-1">Version {skill.version} • Source: {skill.source}</div>
                            </div>
                            <div className="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                              <button className="px-3 py-1.5 rounded-lg bg-black/40 hover:bg-black/60 border border-white/10 text-xs font-medium flex items-center gap-2">
                                <Info className="w-3.5 h-3.5" /> View Content
                              </button>
                            </div>
                          </div>
                          <p className="text-sm text-slate-300 leading-relaxed">
                            {skill.description}
                          </p>
                        </motion.div>
                      ))
                    )}
                  </div>
                </>
              )}"""

if "activeTab === \"installed\"" not in content.replace('activeTab === "installed"', 'activeTab === "INSTALLED_MOCK"'):
    content = content.replace(
        '{activeTab !== "marketplace" && (',
        installed_jsx + '\n\n              {activeTab !== "marketplace" && activeTab !== "installed" && ('
    )
    # remove the old installed check inside the placeholder
    content = re.sub(r'\{activeTab === "installed" && \(\s*<motion\.div.*?</motion\.div>\s*\)\}', '', content, flags=re.DOTALL)

with open("apps/dashboard/src/app/skills/page.tsx", "w") as f:
    f.write(content)

