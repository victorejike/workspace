import React from 'react';
import { CheckCircle2, Edit3, FileText, PhoneCall, Plus } from 'lucide-react';
import { DailyTask } from '../types';

interface DailyTimelineProps {
  tasks: DailyTask[];
  onAddTask: () => void;
  onToggleTaskStatus: (taskId: string) => void;
}

export const DailyTimeline: React.FC<DailyTimelineProps> = ({
  tasks,
  onAddTask,
  onToggleTaskStatus,
}) => {
  const hours = [
    '8 AM', '9 AM', '10 AM', '11 AM', '12 PM',
    '1 PM', '2 PM', '3 PM', '4 PM', '5 PM',
    '6 PM', '7 PM', '8 PM'
  ];

  // Helper to choose priority chip styling matching the mockup
  const renderPriorityBadge = (task: DailyTask) => {
    switch (task.priority) {
      case 'LOW':
        return (
          <span className="flex items-center space-x-1.5 text-xs text-slate-300">
            <span className="w-2 h-2 rounded-full bg-blue-500" />
            <span>Low Priority</span>
          </span>
        );
      case 'HIGH':
        return (
          <span className="flex items-center space-x-1.5 text-xs text-slate-300">
            <span className="w-2 h-2 rounded-full bg-pink-500" />
            <span>High priority</span>
          </span>
        );
      case 'IMPORTANT':
      default:
        return (
          <span className="flex items-center space-x-1.5 text-xs text-slate-300">
            <span className="w-2 h-2 rounded-full bg-emerald-400" />
            <span>Important</span>
          </span>
        );
    }
  };

  // Helper for task icon
  const getTaskIcon = (task: DailyTask) => {
    if (task.title.toLowerCase().includes('call')) {
      return <PhoneCall className="w-3.5 h-3.5 text-slate-400" />;
    } else if (task.title.toLowerCase().includes('review') || task.title.toLowerCase().includes('project')) {
      return <FileText className="w-3.5 h-3.5 text-slate-400" />;
    }
    return <Edit3 className="w-3.5 h-3.5 text-slate-400" />;
  };

  return (
    <div className="w-full rounded-3xl bg-[#12151b] border border-white/[0.07] p-6 shadow-lg shadow-black/40">
      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <div className="flex items-center space-x-3">
          <h3 className="text-base font-medium text-slate-200">Daily Timeline</h3>
          <span className="text-xs text-slate-500">|</span>
          <span className="text-xs text-slate-400">
            {tasks.length} {tasks.length === 1 ? 'Task' : 'Tasks'} for today
          </span>
        </div>

        <button
          onClick={onAddTask}
          className="flex items-center space-x-1 text-xs font-medium text-slate-300 hover:text-white transition-colors"
        >
          <Plus className="w-3.5 h-3.5" />
          <span>Add an Event</span>
        </button>
      </div>

      {/* Horizontal Axis Guides */}
      <div className="relative w-full overflow-x-auto pb-4">
        {/* Hour markers row */}
        <div className="flex items-center justify-between min-w-[760px] text-[11px] font-medium text-slate-400 px-2 mb-8">
          {hours.map((h) => (
            <div key={h} className="text-center w-12 flex-shrink-0">
              {h}
            </div>
          ))}
        </div>

        {/* Vertical subtle timeline lines */}
        <div className="absolute top-8 left-0 right-0 bottom-6 flex justify-between min-w-[760px] px-8 pointer-events-none opacity-20">
          {hours.map((_, i) => (
            <div key={i} className="w-[1px] h-full bg-slate-500/40" />
          ))}
        </div>

        {/* Task Cards plotted along the timeline */}
        <div className="min-w-[760px] space-y-4 relative z-10 px-4">
          {tasks.map((task) => {
            const isCompleted = task.status === 'COMPLETED';

            return (
              <div
                key={task.id}
                onClick={() => onToggleTaskStatus(task.id)}
                className={`group flex items-center justify-between max-w-sm rounded-full py-2 px-4 transition-all duration-200 cursor-pointer border ${
                  isCompleted
                    ? 'bg-emerald-950/20 border-emerald-500/20 opacity-60'
                    : 'bg-[#1a1e27]/90 hover:bg-[#202531] border-white/[0.08] shadow-md shadow-black/30'
                }`}
                style={{
                  // Position card horizontally across the timeline
                  marginLeft:
                    task.scheduled_time_start === '08:30' || task.priority === 'LOW'
                      ? '2%'
                      : task.scheduled_time_start === '13:00' || task.priority === 'IMPORTANT'
                      ? '35%'
                      : '62%',
                }}
              >
                <div className="flex items-center space-x-2.5">
                  <div className="w-6 h-6 rounded-full bg-white/[0.05] flex items-center justify-center">
                    {isCompleted ? (
                      <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" />
                    ) : (
                      getTaskIcon(task)
                    )}
                  </div>
                  <span
                    className={`text-xs font-medium ${
                      isCompleted ? 'line-through text-slate-500' : 'text-slate-200'
                    }`}
                  >
                    {task.title}
                  </span>
                </div>

                <div className="flex items-center space-x-3 ml-4">
                  {renderPriorityBadge(task)}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
