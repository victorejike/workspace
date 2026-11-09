import React from 'react';
import { ChevronLeft, ChevronRight, Compass, Plus, SlidersHorizontal, Sparkles } from 'lucide-react';
import { DailyTask, FocusDashboardMetrics } from '../types';
import { MetricCards } from './MetricCards';
import { DailyTimeline } from './DailyTimeline';

interface DashboardOverviewProps {
  metrics: FocusDashboardMetrics;
  tasks: DailyTask[];
  isTimerActive: boolean;
  onToggleTimer: () => void;
  onAddTask: () => void;
  onToggleTaskStatus: (taskId: string) => void;
  onContinueLearning: () => void;
}

export const DashboardOverview: React.FC<DashboardOverviewProps> = ({
  metrics,
  tasks,
  isTimerActive,
  onToggleTimer,
  onAddTask,
  onToggleTaskStatus,
  onContinueLearning,
}) => {
  // Format current date badge
  const today = new Date();
  const dayNumber = today.getDate();
  const monthShort = today.toLocaleString('default', { month: 'short' });

  return (
    <div className="space-y-7 pb-12">
      {/* Hero Header & Daily Status */}
      <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
        <div>
          <span className="text-xs font-medium text-slate-400 tracking-wide uppercase">
            Your daily plan
          </span>
          <h1 className="text-3xl font-semibold text-white tracking-tight mt-0.5">
            Stay on Track Today
          </h1>
        </div>

        {/* Motivational Status Pill + Action Buttons */}
        <div className="flex items-center space-x-3.5">
          {/* Keep it going pill */}
          <div className="flex items-center space-x-3 bg-[#161a22] border border-white/[0.08] rounded-2xl px-3.5 py-2">
            <div className="w-7 h-7 rounded-lg bg-white/[0.08] flex items-center justify-center text-slate-300">
              <Sparkles className="w-4 h-4 text-amber-400" />
            </div>
            <div className="flex flex-col">
              <span className="text-xs font-semibold text-white">Keep it going!</span>
              <span className="text-[11px] text-slate-400">You've done all previous tasks!</span>
            </div>
          </div>

          {/* Add Widget Button */}
          <button
            onClick={onAddTask}
            className="flex items-center space-x-1.5 bg-[#161a22] hover:bg-[#1e232e] text-slate-200 text-xs font-medium px-3.5 py-2.5 rounded-full border border-white/[0.08] transition-colors"
          >
            <Plus className="w-3.5 h-3.5" />
            <span>Add Widget</span>
          </button>

          {/* Customize Button */}
          <button
            className="flex items-center space-x-1.5 bg-[#252a36] hover:bg-[#303746] text-white text-xs font-medium px-4 py-2.5 rounded-full transition-colors shadow-sm"
          >
            <SlidersHorizontal className="w-3.5 h-3.5" />
            <span>Customize</span>
          </button>
        </div>
      </div>

      {/* Date & Daily Goals Completion Banner */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-4">
          {/* Date Badge */}
          <div className="w-11 h-11 rounded-2xl bg-[#161a22] border border-white/[0.08] flex flex-col items-center justify-center text-center shadow-sm">
            <span className="text-xs font-bold text-white leading-none">{dayNumber}</span>
            <span className="text-[10px] font-medium text-slate-400 uppercase leading-none mt-0.5">
              {monthShort}
            </span>
          </div>

          {/* Goals Completion Text */}
          <div className="text-sm font-medium text-slate-300">
            Your Daily Goals{' '}
            <span className="text-white font-semibold">
              {metrics.completed_goals_today}/{metrics.total_goals_today} completed
            </span>
          </div>
        </div>

        {/* Date Stepper Controls + Prominent Continue Learning Action */}
        <div className="flex items-center space-x-4">
          <button
            onClick={onContinueLearning}
            className="flex items-center space-x-2 bg-gradient-to-r from-amber-500 to-amber-600 hover:from-amber-400 hover:to-amber-500 text-black font-semibold text-xs px-4 py-2 rounded-full shadow-lg shadow-amber-500/20 transition-all hover:scale-[1.02] active:scale-95"
          >
            <Compass className="w-3.5 h-3.5" />
            <span>Continue Learning</span>
          </button>

          <div className="flex items-center space-x-1.5">
            <button
              title="Previous Day"
              className="w-8 h-8 rounded-full bg-white/[0.04] hover:bg-white/[0.08] border border-white/[0.06] flex items-center justify-center text-slate-400 transition-colors"
            >
              <ChevronLeft className="w-4 h-4" />
            </button>
            <button
              title="Next Day"
              className="w-8 h-8 rounded-full bg-white/[0.04] hover:bg-white/[0.08] border border-white/[0.06] flex items-center justify-center text-slate-400 transition-colors"
            >
              <ChevronRight className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      {/* 3 Metric Cards */}
      <MetricCards
        metrics={metrics}
        isTimerActive={isTimerActive}
        onToggleTimer={onToggleTimer}
      />

      {/* Daily Timeline */}
      <DailyTimeline
        tasks={tasks}
        onAddTask={onAddTask}
        onToggleTaskStatus={onToggleTaskStatus}
      />
    </div>
  );
};
