'use client';

import { useState, useEffect } from 'react';
import { daemonFetch } from '@/lib/goat-api';
import { FolderTree, FileCode2, Loader2, Plus } from 'lucide-react';

export default function RepoPage() {
  const [repoMap, setRepoMap] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [fileContent, setFileContent] = useState<string | null>(null);
  const [fileLoading, setFileLoading] = useState(false);
  const [contextFiles, setContextFiles] = useState<string[]>([]);

  useEffect(() => {
    fetchRepo();
    fetchContext();
  }, []);

  const fetchRepo = async () => {
    try {
      const res = await daemonFetch('/v1/repo/tree');
      if (res.ok) {
        setRepoMap(await res.json());
      }
    } catch (e) {
      console.error('Failed to fetch repo map', e);
    } finally {
      setLoading(false);
    }
  };

  const fetchContext = async () => {
    try {
      const res = await daemonFetch('/v1/context');
      if (res.ok) {
        const data = await res.json();
        setContextFiles(data.selected_files || []);
      }
    } catch (e) {
      console.error('Failed to fetch context', e);
    }
  };

  const handleSelectFile = async (path: string) => {
    setSelectedFile(path);
    setFileLoading(true);
    setFileContent(null);
    try {
      const res = await daemonFetch(`/v1/repo/file?path=${encodeURIComponent(path)}`);
      const data = await res.json();
      if (data.error) {
        setFileContent(`[Error]: ${data.error}`);
      } else {
        setFileContent(data.content);
      }
    } catch (e: any) {
      setFileContent(`[Error]: ${e.message}`);
    } finally {
      setFileLoading(false);
    }
  };

  const handleAddToContext = async (path: string) => {
    try {
      const res = await daemonFetch('/v1/context/add', {
        method: 'POST',
        body: JSON.stringify({ path })
      });
      if (res.ok) {
        const data = await res.json();
        if (data.error) {
          alert(data.error);
        } else {
          setContextFiles(data.selected_files || []);
        }
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="flex flex-col h-[calc(100vh-4rem)] p-6 space-y-4">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Repo Explorer</h1>
        <p className="text-muted-foreground">Browse workspace files safely.</p>
      </div>

      <div className="flex gap-4 flex-1 min-h-0">
        <div className="w-1/3 bg-card border border-border rounded-md flex flex-col">
          <div className="p-3 border-b border-border bg-muted/30 font-medium text-sm flex items-center gap-2">
            <FolderTree className="w-4 h-4" />
            Project Files
          </div>
          <div className="flex-1 overflow-y-auto p-2">
            {loading ? (
              <div className="flex items-center justify-center h-full">
                <Loader2 className="w-5 h-5 animate-spin text-muted-foreground" />
              </div>
            ) : repoMap && repoMap.files ? (
              <ul className="space-y-1">
                {repoMap.files.map((f: any) => (
                  <li key={f.path}>
                    <button
                      onClick={() => handleSelectFile(f.path)}
                      className={`w-full text-left px-2 py-1.5 text-sm rounded-md transition-colors flex items-center justify-between ${
                        selectedFile === f.path ? 'bg-primary/20 text-foreground' : 'text-muted-foreground hover:bg-muted hover:text-foreground'
                      }`}
                    >
                      <div className="flex items-center gap-2 truncate">
                        <FileCode2 className="w-4 h-4 shrink-0" />
                        <span className="truncate">{f.path}</span>
                      </div>
                      <span className="text-xs opacity-50 ml-2">{f.line_count}L</span>
                    </button>
                  </li>
                ))}
              </ul>
            ) : (
              <div className="text-sm text-muted-foreground p-4">No files found.</div>
            )}
          </div>
        </div>

        <div className="w-2/3 bg-card border border-border rounded-md flex flex-col">
          {selectedFile ? (
            <>
              <div className="p-3 border-b border-border bg-muted/30 font-medium text-sm flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <FileCode2 className="w-4 h-4" />
                  {selectedFile}
                </div>
                <button
                  onClick={() => handleAddToContext(selectedFile)}
                  className="flex items-center gap-1.5 text-xs bg-primary text-primary-foreground px-2 py-1 rounded-md hover:bg-primary/90"
                >
                  <Plus className="w-3 h-3" /> Add to Context
                </button>
              </div>
              <div className="flex-1 overflow-auto p-4 text-sm font-mono whitespace-pre text-muted-foreground">
                {fileLoading ? (
                  <div className="flex items-center justify-center h-full">
                    <Loader2 className="w-5 h-5 animate-spin" />
                  </div>
                ) : (
                  fileContent || 'Empty file'
                )}
              </div>
            </>
          ) : (
            <div className="flex-1 flex items-center justify-center text-muted-foreground text-sm">
              Select a file to preview safely
            </div>
          )}
        </div>
      </div>
      
      {/* Context footer */}
      {contextFiles.length > 0 && (
        <div className="bg-card border border-border rounded-md p-3">
          <h3 className="text-sm font-medium mb-2">Context Files ({contextFiles.length})</h3>
          <div className="flex flex-wrap gap-2">
            {contextFiles.map(f => (
              <span key={f} className="text-xs px-2 py-1 bg-muted rounded-md border border-border">
                {f}
              </span>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
