'use client';

import React, { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { Activity, Mic, MessageSquare, Send, Settings, Radio } from 'lucide-react';

export default function TransportsPage() {
  const [transportStatus, setTransportStatus] = useState<string>('');
  const [voiceStatus, setVoiceStatus] = useState<string>('');
  const [sessions, setSessions] = useState<any[]>([]);
  const [voiceProviders, setVoiceProviders] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);

  // Form states
  const [ttsText, setTtsText] = useState('');
  const [ttsResult, setTtsResult] = useState('');
  const [sttText, setSttText] = useState('');
  const [sttResult, setSttResult] = useState('');

  const fetchStatus = async () => {
    try {
      const [tStatus, vStatus, sess, vProv] = await Promise.all([
        fetch('http://127.0.0.1:3000/v1/transports/status').then(r => r.json()),
        fetch('http://127.0.0.1:3000/v1/voice/status').then(r => r.json()),
        fetch('http://127.0.0.1:3000/v1/transports/sessions').then(r => r.json()),
        fetch('http://127.0.0.1:3000/v1/voice/providers').then(r => r.json()),
      ]);

      setTransportStatus(tStatus.status || 'Offline');
      setVoiceStatus(vStatus.status || 'Offline');
      setSessions(sess.sessions || []);
      setVoiceProviders(vProv.providers || []);
    } catch (e) {
      console.error('Failed to fetch transport data', e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchStatus();
    const interval = setInterval(fetchStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  const handleTTS = async () => {
    if (!ttsText) return;
    try {
      const res = await fetch('http://127.0.0.1:3000/v1/voice/speak', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: ttsText })
      }).then(r => r.json());
      setTtsResult(res.output?.text || res.error || 'Failed to synthesize');
    } catch (e) {
      setTtsResult('Error communicating with API');
    }
  };

  const handleSTT = async () => {
    if (!sttText) return;
    try {
      const res = await fetch('http://127.0.0.1:3000/v1/voice/transcribe', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text_override: sttText })
      }).then(r => r.json());
      setSttResult(`Transcript: ${res.transcript?.text} (Conf: ${res.transcript?.confidence})` || res.error);
    } catch (e) {
      setSttResult('Error communicating with API');
    }
  };

  return (
    <div className="min-h-screen bg-[#0A0A0A] text-white p-8 overflow-y-auto">
      <div className="max-w-7xl mx-auto space-y-8">
        
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex items-center justify-between"
        >
          <div>
            <h1 className="text-4xl font-light tracking-tight flex items-center gap-3">
              <Radio className="w-8 h-8 text-indigo-400" />
              Transports & Voice
            </h1>
            <p className="text-gray-400 mt-2 text-lg">Manage messaging endpoints and voice interactions</p>
          </div>
          <div className="flex gap-4">
            <div className="bg-white/5 border border-white/10 rounded-full px-4 py-2 flex items-center gap-2">
              <Activity className={`w-4 h-4 ${transportStatus.includes('Offline') ? 'text-red-400' : 'text-green-400'}`} />
              <span className="text-sm font-medium">Transports: {transportStatus.includes('Offline') ? 'Disabled' : 'Active'}</span>
            </div>
            <div className="bg-white/5 border border-white/10 rounded-full px-4 py-2 flex items-center gap-2">
              <Mic className={`w-4 h-4 ${voiceStatus.includes('Offline') ? 'text-red-400' : 'text-green-400'}`} />
              <span className="text-sm font-medium">Voice: {voiceStatus.includes('Offline') ? 'Disabled' : 'Active'}</span>
            </div>
          </div>
        </motion.div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Messaging Transports */}
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.1 }}
            className="bg-white/5 border border-white/10 rounded-2xl p-6 backdrop-blur-xl"
          >
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-2xl font-medium flex items-center gap-2">
                <MessageSquare className="w-6 h-6 text-blue-400" />
                Messaging Transports
              </h2>
              <Settings className="w-5 h-5 text-gray-500 cursor-pointer hover:text-white transition-colors" />
            </div>

            <div className="space-y-6">
              <div className="bg-black/30 rounded-xl p-4 border border-white/5 font-mono text-sm text-gray-300 whitespace-pre-wrap">
                {transportStatus || 'Loading status...'}
              </div>

              <div>
                <h3 className="text-sm text-gray-400 font-medium mb-3 uppercase tracking-wider">Active Sessions</h3>
                {sessions.length === 0 ? (
                  <div className="text-gray-500 text-sm italic p-4 bg-white/5 rounded-lg text-center">No active sessions.</div>
                ) : (
                  <div className="space-y-2">
                    {sessions.map((s, idx) => (
                      <div key={idx} className="flex items-center justify-between bg-white/5 p-3 rounded-lg border border-white/5">
                        <span className="font-mono text-sm">{s.id}</span>
                        <span className="text-xs bg-blue-500/20 text-blue-300 px-2 py-1 rounded-full">{s.provider}</span>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          </motion.div>

          {/* Voice Engine */}
          <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.2 }}
            className="bg-white/5 border border-white/10 rounded-2xl p-6 backdrop-blur-xl flex flex-col"
          >
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-2xl font-medium flex items-center gap-2">
                <Mic className="w-6 h-6 text-purple-400" />
                Voice Engine
              </h2>
            </div>

            <div className="bg-black/30 rounded-xl p-4 border border-white/5 font-mono text-sm text-gray-300 whitespace-pre-wrap mb-6">
              {voiceStatus || 'Loading status...'}
            </div>

            <div className="grid grid-cols-1 gap-6 flex-1">
              {/* TTS Tester */}
              <div className="space-y-3">
                <h3 className="text-sm text-gray-400 font-medium uppercase tracking-wider flex items-center gap-2">
                  <Activity className="w-4 h-4" /> Text-to-Speech Test
                </h3>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={ttsText}
                    onChange={e => setTtsText(e.target.value)}
                    placeholder="Enter text to speak..."
                    className="flex-1 bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-purple-500 transition-colors"
                  />
                  <button
                    onClick={handleTTS}
                    className="bg-purple-600 hover:bg-purple-500 text-white px-4 py-2 rounded-lg font-medium transition-colors flex items-center gap-2"
                  >
                    <Send className="w-4 h-4" /> Speak
                  </button>
                </div>
                {ttsResult && <div className="text-sm text-purple-300 bg-purple-500/10 p-3 rounded-lg border border-purple-500/20 font-mono">{ttsResult}</div>}
              </div>

              {/* STT Tester */}
              <div className="space-y-3">
                <h3 className="text-sm text-gray-400 font-medium uppercase tracking-wider flex items-center gap-2">
                  <MessageSquare className="w-4 h-4" /> Speech-to-Text Test (Simulated)
                </h3>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={sttText}
                    onChange={e => setSttText(e.target.value)}
                    placeholder="Enter text to simulate transcript..."
                    className="flex-1 bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 transition-colors"
                  />
                  <button
                    onClick={handleSTT}
                    className="bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-lg font-medium transition-colors flex items-center gap-2"
                  >
                    <Mic className="w-4 h-4" /> Transcribe
                  </button>
                </div>
                {sttResult && <div className="text-sm text-indigo-300 bg-indigo-500/10 p-3 rounded-lg border border-indigo-500/20 font-mono">{sttResult}</div>}
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </div>
  );
}
