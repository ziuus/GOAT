"use client";

import React, { useEffect, useState } from "react";
import { PageHeader } from "@/components/ui/PageHeader";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";

interface ExtensionManifest {
  id: string;
  name: string;
  version: string;
  description: string;
  kind: string;
  author?: string;
  permissions: string[];
}

interface ExtensionRecord {
  manifest: ExtensionManifest;
  status: string;
  trust_level: string;
  source: any;
}

export default function ExtensionsPage() {
  const [extensions, setExtensions] = useState<ExtensionRecord[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchExtensions = async () => {
    try {
      const res = await fetch("http://127.0.0.1:3000/v1/extensions");
      if (res.ok) {
        const data = await res.json();
        setExtensions(data.extensions || []);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchExtensions();
  }, []);

  const handleAction = async (id: string, action: string) => {
    try {
      await fetch(`http://127.0.0.1:3000/v1/extensions/${id}/${action}`, {
        method: "POST"
      });
      fetchExtensions();
    } catch (e) {
      console.error(e);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case "enabled": return "bg-green-500/10 text-green-500 border-green-500/20";
      case "disabled": return "bg-zinc-500/10 text-zinc-400 border-zinc-500/20";
      case "discovered": return "bg-blue-500/10 text-blue-400 border-blue-500/20";
      case "installed": return "bg-yellow-500/10 text-yellow-500 border-yellow-500/20";
      default: return "bg-zinc-800 text-zinc-400";
    }
  };

  return (
    <div className="p-6 max-w-7xl mx-auto space-y-6">
      <div>
        <PageHeader title="Extensions & Plugins" />
        <p className="text-zinc-400 mt-2">Safe, auditable extensions for the GOAT ecosystem (Phase 6.8)</p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
        {loading ? (
          <p className="text-zinc-400">Loading extensions...</p>
        ) : extensions.length === 0 ? (
          <p className="text-zinc-400">No extensions found in the registry.</p>
        ) : (
          extensions.map((ext) => (
            <Card key={ext.manifest.id} className="bg-zinc-900 border-zinc-800 flex flex-col">
              <CardHeader className="pb-3 border-b border-zinc-800/50">
                <div className="flex justify-between items-start">
                  <div>
                    <CardTitle className="text-lg text-zinc-100 flex items-center gap-2">
                      {ext.manifest.name}
                      <span className="text-xs text-zinc-500 font-mono">v{ext.manifest.version}</span>
                    </CardTitle>
                    <CardDescription className="text-zinc-400 font-mono text-xs mt-1">
                      {ext.manifest.id}
                    </CardDescription>
                  </div>
                  <Badge variant="outline" className={getStatusColor(ext.status)}>
                    {ext.status.toUpperCase()}
                  </Badge>
                </div>
              </CardHeader>
              <CardContent className="pt-4 flex-1 flex flex-col">
                <p className="text-sm text-zinc-300 mb-4 flex-1">
                  {ext.manifest.description}
                </p>
                
                <div className="space-y-3 mb-6">
                  <div className="flex flex-wrap gap-2">
                    <Badge variant="secondary" className="bg-zinc-800 text-zinc-300 border-zinc-700">
                      {ext.manifest.kind}
                    </Badge>
                    <Badge variant="secondary" className="bg-zinc-800 text-zinc-300 border-zinc-700">
                      Trust: {ext.trust_level}
                    </Badge>
                  </div>
                  
                  {ext.manifest.permissions && ext.manifest.permissions.length > 0 && (
                    <div>
                      <p className="text-xs text-zinc-500 mb-1 uppercase font-semibold">Permissions</p>
                      <div className="flex flex-wrap gap-1">
                        {ext.manifest.permissions.map(p => (
                          <span key={p} className="text-xs px-1.5 py-0.5 rounded bg-red-500/10 text-red-400 border border-red-500/20 font-mono">
                            {p}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                </div>

                <div className="flex gap-2 mt-auto border-t border-zinc-800/50 pt-4">
                  {(ext.status === "discovered" || ext.status === "disabled") && (
                    <Button 
                      variant="outline" 
                      className="w-full bg-zinc-800 hover:bg-zinc-700 border-zinc-700"
                      onClick={() => handleAction(ext.manifest.id, "install")}
                    >
                      Install
                    </Button>
                  )}
                  {(ext.status === "installed" || ext.status === "disabled") && (
                    <Button 
                      variant="default" 
                      className="w-full bg-blue-600 hover:bg-blue-700 text-white"
                      onClick={() => handleAction(ext.manifest.id, "enable")}
                    >
                      Enable
                    </Button>
                  )}
                  {ext.status === "enabled" && (
                    <Button 
                      variant="outline" 
                      className="w-full bg-zinc-800 hover:bg-zinc-700 border-zinc-700 text-yellow-500 hover:text-yellow-400"
                      onClick={() => handleAction(ext.manifest.id, "disable")}
                    >
                      Disable
                    </Button>
                  )}
                </div>
              </CardContent>
            </Card>
          ))
        )}
      </div>
    </div>
  );
}
