'use client';

import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { CheckCircle, ArrowRight, Settings, Code, Sparkles } from 'lucide-react';
import { goatApi } from '@/lib/goat-api';

const steps = [
  { id: 'welcome', title: 'Welcome to GOAT' },
  { id: 'mode', title: 'Choose Mode' },
  { id: 'project', title: 'Project Setup' },
  { id: 'ready', title: 'Ready' }
];

export default function OnboardingPage() {
  const [currentStep, setCurrentStep] = useState(0);
  const [modes, setModes] = useState<any[]>([]);
  const [projectProfile, setProjectProfile] = useState<any>(null);
  const [selectedMode, setSelectedMode] = useState<string>('Coding Assistant');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const modesRes = await goatApi.get('/v1/profiles/modes');
        if (modesRes.modes) {
          setModes(modesRes.modes);
        }
        
        const projRes = await goatApi.post('/v1/project-profile/detect', {});
        if (projRes.detected) {
          setProjectProfile(projRes.detected);
        }
      } catch (e) {
        console.error('Failed to fetch onboarding data:', e);
      }
    };
    fetchData();
  }, []);

  const handleNext = async () => {
    if (currentStep === 1) {
      setLoading(true);
      await goatApi.post('/v1/profiles/modes/use', { mode: selectedMode });
      setLoading(false);
    } else if (currentStep === 3) {
      setLoading(true);
      await goatApi.post('/v1/onboarding/complete', {});
      window.location.href = '/';
      return;
    }
    setCurrentStep(prev => Math.min(prev + 1, steps.length - 1));
  };

  const handleSkip = async () => {
    await goatApi.post('/v1/onboarding/skip', {});
    window.location.href = '/';
  };

  return (
    <div className="flex-1 flex flex-col items-center justify-center min-h-full p-8 bg-gradient-to-br from-background to-muted/20 relative overflow-hidden">
      {/* Background decoration */}
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-primary/10 rounded-full blur-[100px] pointer-events-none" />
      <div className="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-accent/10 rounded-full blur-[100px] pointer-events-none" />

      <div className="w-full max-w-4xl relative z-10 flex flex-col md:flex-row gap-8">
        
        {/* Left Side: Progress Steps */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
          <h1 className="text-3xl font-bold tracking-tight mb-4 flex items-center gap-2">
            <Sparkles className="w-8 h-8 text-primary" />
            GOAT Setup
          </h1>
          <div className="flex flex-col gap-4">
            {steps.map((step, idx) => (
              <div 
                key={step.id} 
                className={`flex items-center gap-3 transition-all duration-300 ${idx === currentStep ? 'text-primary font-medium scale-105 origin-left' : idx < currentStep ? 'text-muted-foreground' : 'text-muted-foreground/50'}`}
              >
                <div className={`w-8 h-8 rounded-full flex items-center justify-center border-2 transition-colors ${idx === currentStep ? 'border-primary bg-primary/10' : idx < currentStep ? 'border-primary bg-primary text-primary-foreground' : 'border-muted-foreground/30'}`}>
                  {idx < currentStep ? <CheckCircle className="w-5 h-5" /> : <span>{idx + 1}</span>}
                </div>
                <span>{step.title}</span>
              </div>
            ))}
          </div>
        </div>

        {/* Right Side: Step Content */}
        <div className="w-full md:w-2/3 min-h-[400px] bg-card/60 backdrop-blur-xl border border-border/50 rounded-2xl shadow-2xl overflow-hidden flex flex-col">
          <div className="flex-1 p-8 relative">
            <AnimatePresence mode="wait">
              {currentStep === 0 && (
                <motion.div
                  key="welcome"
                  initial={{ opacity: 0, x: 20 }}
                  animate={{ opacity: 1, x: 0 }}
                  exit={{ opacity: 0, x: -20 }}
                  className="flex flex-col h-full justify-center gap-6 text-center"
                >
                  <div className="w-20 h-20 bg-primary/20 rounded-full mx-auto flex items-center justify-center mb-4">
                    <Sparkles className="w-10 h-10 text-primary" />
                  </div>
                  <h2 className="text-4xl font-bold">Welcome to GOAT</h2>
                  <p className="text-xl text-muted-foreground max-w-md mx-auto">
                    Let's set up your General Omniscient Agentic Tool environment for maximum productivity.
                  </p>
                </motion.div>
              )}

              {currentStep === 1 && (
                <motion.div
                  key="mode"
                  initial={{ opacity: 0, x: 20 }}
                  animate={{ opacity: 1, x: 0 }}
                  exit={{ opacity: 0, x: -20 }}
                  className="flex flex-col h-full gap-4"
                >
                  <h2 className="text-2xl font-bold mb-2">Select Agent Mode</h2>
                  <p className="text-muted-foreground mb-4">Choose how you want GOAT to operate initially. You can always change this later.</p>
                  
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 overflow-y-auto max-h-[300px] pr-2 custom-scrollbar">
                    {modes.map(m => (
                      <div 
                        key={m.name}
                        onClick={() => setSelectedMode(m.name)}
                        className={`p-4 rounded-xl border cursor-pointer transition-all duration-300 hover:shadow-lg ${selectedMode === m.name ? 'border-primary bg-primary/5 shadow-md scale-[1.02]' : 'border-border/50 hover:border-primary/50'}`}
                      >
                        <div className="font-semibold text-lg flex items-center justify-between">
                          {m.name}
                          {selectedMode === m.name && <CheckCircle className="w-5 h-5 text-primary" />}
                        </div>
                        <div className="text-xs text-primary/80 mt-1 uppercase tracking-wider">{m.kind}</div>
                        <p className="text-sm text-muted-foreground mt-2 line-clamp-2">{m.description}</p>
                      </div>
                    ))}
                  </div>
                </motion.div>
              )}

              {currentStep === 2 && (
                <motion.div
                  key="project"
                  initial={{ opacity: 0, x: 20 }}
                  animate={{ opacity: 1, x: 0 }}
                  exit={{ opacity: 0, x: -20 }}
                  className="flex flex-col h-full gap-4"
                >
                  <h2 className="text-2xl font-bold mb-2">Project Profile</h2>
                  <p className="text-muted-foreground mb-4">We've scanned your current workspace to recommend settings.</p>
                  
                  {projectProfile ? (
                    <div className="bg-muted/30 p-6 rounded-xl border border-border/50">
                      <div className="flex items-center gap-3 mb-6">
                        <Code className="w-8 h-8 text-primary" />
                        <div>
                          <div className="font-semibold text-xl">{projectProfile.kind}</div>
                          <div className="text-sm text-muted-foreground font-mono">{projectProfile.project_root}</div>
                        </div>
                      </div>
                      
                      <div className="space-y-4">
                        <div>
                          <div className="text-sm font-medium text-muted-foreground uppercase tracking-wider mb-2">Recommended Modes</div>
                          <div className="flex gap-2 flex-wrap">
                            {projectProfile.preferred_mode_profiles.map((p: string) => (
                              <span key={p} className="px-3 py-1 bg-primary/10 text-primary rounded-full text-sm font-medium">{p}</span>
                            ))}
                          </div>
                        </div>
                        <div>
                          <div className="text-sm font-medium text-muted-foreground uppercase tracking-wider mb-2">Build Command</div>
                          <div className="font-mono text-sm bg-background p-2 rounded border">{projectProfile.build_command || 'None detected'}</div>
                        </div>
                      </div>
                    </div>
                  ) : (
                    <div className="flex items-center justify-center h-40 text-muted-foreground">
                      Scanning project...
                    </div>
                  )}
                </motion.div>
              )}

              {currentStep === 3 && (
                <motion.div
                  key="ready"
                  initial={{ opacity: 0, x: 20 }}
                  animate={{ opacity: 1, x: 0 }}
                  exit={{ opacity: 0, x: -20 }}
                  className="flex flex-col h-full justify-center items-center text-center gap-6"
                >
                  <div className="w-24 h-24 bg-green-500/20 rounded-full flex items-center justify-center mb-4 text-green-500">
                    <CheckCircle className="w-12 h-12" />
                  </div>
                  <h2 className="text-4xl font-bold">You're All Set!</h2>
                  <p className="text-xl text-muted-foreground max-w-md mx-auto">
                    GOAT is configured and ready to assist you.
                  </p>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
          
          <div className="p-6 border-t border-border/50 bg-muted/10 flex justify-between items-center">
            <button 
              onClick={handleSkip}
              className="text-muted-foreground hover:text-foreground text-sm font-medium px-4 py-2 rounded-md hover:bg-muted/50 transition-colors"
            >
              Skip Setup
            </button>
            <div className="flex gap-3">
              {currentStep > 0 && (
                <button 
                  onClick={() => setCurrentStep(p => p - 1)}
                  className="px-6 py-2 rounded-md border border-border font-medium hover:bg-muted transition-colors"
                >
                  Back
                </button>
              )}
              <button 
                onClick={handleNext}
                disabled={loading}
                className="px-6 py-2 bg-primary text-primary-foreground rounded-md font-medium hover:bg-primary/90 transition-colors flex items-center gap-2 shadow-lg shadow-primary/20"
              >
                {currentStep === steps.length - 1 ? 'Finish' : 'Next'}
                {currentStep < steps.length - 1 && <ArrowRight className="w-4 h-4" />}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
