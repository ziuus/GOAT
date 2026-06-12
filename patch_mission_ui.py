import re

with open("apps/dashboard/src/app/mission-control/page.tsx", "r") as f:
    content = f.read()

replacement = """                    <div className="flex items-center gap-4 text-[10px] text-gray-600 font-bold uppercase tracking-wider">
                      <span>{new Date(m.created_at).toLocaleDateString()}</span>
                      <span>{m.mission_type}</span>
                      <span>{m.progress}% Done</span>
                      {m.status === "completed" && (
                        <div className="ml-auto">
                           <button className="px-3 py-1 bg-indigo-500/20 text-indigo-300 hover:bg-indigo-500/40 rounded border border-indigo-500/30 transition-colors pointer-events-none">
                             To save as skill: run `goat skill create-from-mission {m.mission_id}`
                           </button>
                        </div>
                      )}
                    </div>"""

if "To save as skill" not in content:
    content = content.replace(
"""                    <div className="flex items-center gap-4 text-[10px] text-gray-600 font-bold uppercase tracking-wider">
                      <span>{new Date(m.created_at).toLocaleDateString()}</span>
                      <span>{m.mission_type}</span>
                      <span>{m.progress}% Done</span>
                    </div>""",
        replacement
    )

with open("apps/dashboard/src/app/mission-control/page.tsx", "w") as f:
    f.write(content)

