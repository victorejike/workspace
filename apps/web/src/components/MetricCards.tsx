import React from 'react';
import { Pause, Play } from 'lucide-react';
import { FocusDashboardMetrics } from '../types';

interface MetricCardsProps {
  metrics: FocusDashboardMetrics;
  isTimerActive: boolean;
  onToggleTimer: () => void;
}

export const MetricCards: React.FC<MetricCardsProps> = ({
  metrics,
  isTimerActive,
  onToggleTimer,
}) => {
  const hours = Math.floor(metrics.deep_work_seconds_today / 3600);
  const minutes = Math.floor((metrics.deep_work_seconds_today % 3600) / 60);
  const timeFormatted = `${hours}h ${minutes}m`;

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-5 w-full">
      {/* 1. Deep Work Time Card */}
      <div className="relative overflow-hidden rounded-3xl bg-[#12151b] border border-white/[0.07] p-6 flex flex-col justify-between min-h-[175px] shadow-lg shadow-black/40">
        {/* Ambient spotlight glow effect */}
        <div className="absolute -top-12 -right-12 w-44 h-44 bg-gradient-to-br from-amber-400/30 via-amber-500/10 to-transparent rounded-full blur-2xl pointer-events-none" />
        
        <div className="flex items-center justify-between z-10">
          <span className="text-sm font-medium text-slate-300">Deep Work Time</span>
          <button
            onClick={onToggleTimer}
            title={isTimerActive ? "Pause Timer" : "Resume Timer"}
            className="w-8 h-8 rounded-full bg-white/[0.08] hover:bg-white/[0.15] border border-white/[0.1] flex items-center justify-center text-slate-200 transition-transform active:scale-95"
          >
            {isTimerActive ? (
              <Pause className="w-3.5 h-3.5 fill-current" />
            ) : (
              <Play className="w-3.5 h-3.5 fill-current ml-0.5" />
            )}
          </button>
        </div>

        <div className="z-10 mt-6">
          <div className="text-4xl font-semibold tracking-tight text-white font-sans">
            {timeFormatted}
          </div>
        </div>
      </div>

      {/* 2. Attention Quality Card */}
      <div className="relative overflow-hidden rounded-3xl bg-[#12151b] border border-white/[0.07] p-6 flex flex-col justify-between min-h-[175px] shadow-lg shadow-black/40">
        {/* Warm visual graphic overlay */}
        <div className="absolute inset-0 bg-gradient-to-br from-amber-500/15 via-transparent to-black/60 pointer-events-none" />
        <div className="absolute right-0 top-0 bottom-0 w-1/2 bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-amber-500/25 via-amber-600/10 to-transparent pointer-events-none" />

        <div className="z-10">
          <span className="text-sm font-medium text-slate-300">Attention Quality</span>
        </div>

        <div className="z-10 mt-6 flex items-baseline justify-between">
          <div className="text-4xl font-semibold tracking-tight text-white">
            {metrics.attention_quality_pct}%
          </div>
          <span className="text-xs font-medium text-amber-400/90 bg-amber-500/10 px-2.5 py-1 rounded-full border border-amber-500/20">
            Last 60 min
          </span>
        </div>
      </div>

      {/* 3. Focus Stability Card */}
      <div className="relative overflow-hidden rounded-3xl bg-[#12151b] border border-white/[0.07] p-6 flex flex-col justify-between min-h-[175px] shadow-lg shadow-black/40">
        <div className="flex items-center justify-between z-10">
          <span className="text-sm font-medium text-slate-300">Focus Stability</span>
          <span className="text-xs font-medium text-slate-400 bg-white/[0.05] border border-white/[0.08] px-2.5 py-0.5 rounded-full">
            Current Score: {metrics.focus_stability_score}
          </span>
        </div>

        {/* Visual Bar Graph with Peak pill */}
        <div className="relative z-10 mt-4 pt-2">
          {/* Dashed threshold guide */}
          <div className="absolute top-7 left-0 right-0 border-b border-dashed border-slate-600/40 z-0" />

          {/* Peak badge positioned above bar 3 */}
          <div className="absolute -top-3 left-[28%] bg-white text-slate-900 text-[10px] font-semibold px-2 py-0.5 rounded-full shadow-md z-20">
            Peak: {metrics.peak_focus_hour}
          </div>

          {/* Vertical Bar Chart Pillars */}
          <div className="flex items-end justify-between h-14 z-10 relative pt-3 px-1">
            <div className="w-5 h-8 bg-slate-700/60 rounded-full" />
            <div className="w-5 h-11 bg-slate-700/60 rounded-full" />
            <div className="w-5 h-14 bg-gradient-to-t from-slate-600 to-slate-400 rounded-full" />
            <div className="w-5 h-6 bg-amber-500/80 shadow-glow-amber rounded-full" />
            <div className="w-5 h-9 bg-amber-400/90 shadow-glow-amber rounded-full" />
            <div className="w-5 h-11 bg-slate-700/60 rounded-full" />
            <div className="w-5 h-12 bg-slate-600/80 rounded-full" />
            <div className="w-5 h-8 bg-amber-500/70 rounded-full" />
            <div className="w-5 h-11 bg-slate-700/60 rounded-full" />
          </div>
        </div>
      </div>
    </div>
  );
};
