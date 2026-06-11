import { Activity, CheckCircle2, Loader2 } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

export type AgentStatusType = 'idle' | 'generating' | 'success' | 'error';

interface Props {
  status: AgentStatusType;
  message: string;
}

export function LearnerAgentStatus({ status, message }: Props) {
  return (
    <AnimatePresence>
      {status !== 'idle' && (
        <motion.div 
          initial={{ opacity: 0, y: -10 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0, y: -10 }}
          className="fixed bottom-6 right-6 z-50 flex items-center gap-3 bg-black/80 backdrop-blur-md border border-white/10 px-4 py-3 rounded-full shadow-2xl"
        >
          {status === 'generating' && <Loader2 className="w-4 h-4 text-blue-400 animate-spin" />}
          {status === 'success' && <CheckCircle2 className="w-4 h-4 text-green-400" />}
          {status === 'error' && <Activity className="w-4 h-4 text-red-400" />}
          
          <span className="text-sm font-medium text-white">{message}</span>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
