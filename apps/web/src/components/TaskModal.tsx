import React, { useState } from 'react';
import { X } from 'lucide-react';
import { TaskPriority, TaskType } from '../types';

interface TaskModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (task: {
    title: string;
    description: string;
    priority: TaskPriority;
    task_type: TaskType;
    scheduled_time_start: string;
    estimated_minutes: number;
  }) => void;
}

export const TaskModal: React.FC<TaskModalProps> = ({ isOpen, onClose, onSave }) => {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [priority, setPriority] = useState<TaskPriority>('IMPORTANT');
  const [taskType, setTaskType] = useState<TaskType>('STUDY');
  const [timeStart, setTimeStart] = useState('09:00');
  const [estimatedMinutes, setEstimatedMinutes] = useState(60);

  if (!isOpen) return null;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim()) return;

    onSave({
      title: title.trim(),
      description: description.trim(),
      priority,
      task_type: taskType,
      scheduled_time_start: timeStart,
      estimated_minutes: estimatedMinutes,
    });
    onClose();
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm">
      <div className="w-full max-w-md rounded-3xl bg-[#14171f] border border-white/[0.1] p-6 shadow-2xl relative">
        <div className="flex items-center justify-between pb-4 border-b border-white/[0.06]">
          <h3 className="text-base font-semibold text-white">Add Timeline Event / Task</h3>
          <button
            onClick={onClose}
            className="w-7 h-7 rounded-full bg-white/[0.05] hover:bg-white/[0.1] flex items-center justify-center text-slate-400 hover:text-white"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="mt-5 space-y-4">
          <div>
            <label className="block text-xs font-medium text-slate-300 mb-1">Title</label>
            <input
              type="text"
              required
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="e.g. Implement Lock-free Ring Buffer"
              className="w-full bg-[#1b202a] border border-white/[0.08] rounded-xl px-3.5 py-2 text-xs text-white placeholder-slate-500 focus:outline-none focus:border-amber-400"
            />
          </div>

          <div>
            <label className="block text-xs font-medium text-slate-300 mb-1">Description</label>
            <textarea
              rows={2}
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Key concepts or associated practical exercise..."
              className="w-full bg-[#1b202a] border border-white/[0.08] rounded-xl px-3.5 py-2 text-xs text-white placeholder-slate-500 focus:outline-none focus:border-amber-400"
            />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs font-medium text-slate-300 mb-1">Priority</label>
              <select
                value={priority}
                onChange={(e) => setPriority(e.target.value as TaskPriority)}
                className="w-full bg-[#1b202a] border border-white/[0.08] rounded-xl px-3 py-2 text-xs text-white focus:outline-none focus:border-amber-400"
              >
                <option value="LOW">Low Priority</option>
                <option value="IMPORTANT">Important</option>
                <option value="HIGH">High Priority</option>
              </select>
            </div>

            <div>
              <label className="block text-xs font-medium text-slate-300 mb-1">Task Type</label>
              <select
                value={taskType}
                onChange={(e) => setTaskType(e.target.value as TaskType)}
                className="w-full bg-[#1b202a] border border-white/[0.08] rounded-xl px-3 py-2 text-xs text-white focus:outline-none focus:border-amber-400"
              >
                <option value="STUDY">Study</option>
                <option value="CODING">Coding</option>
                <option value="PROJECT">Project</option>
                <option value="ASSESSMENT">Assessment</option>
                <option value="REVISION">Revision</option>
              </select>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs font-medium text-slate-300 mb-1">Scheduled Time</label>
              <input
                type="text"
                value={timeStart}
                onChange={(e) => setTimeStart(e.target.value)}
                placeholder="09:00"
                className="w-full bg-[#1b202a] border border-white/[0.08] rounded-xl px-3.5 py-2 text-xs text-white focus:outline-none focus:border-amber-400"
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-slate-300 mb-1">Estimated Minutes</label>
              <input
                type="number"
                min={15}
                step={15}
                value={estimatedMinutes}
                onChange={(e) => setEstimatedMinutes(Number(e.target.value))}
                className="w-full bg-[#1b202a] border border-white/[0.08] rounded-xl px-3.5 py-2 text-xs text-white focus:outline-none focus:border-amber-400"
              />
            </div>
          </div>

          <div className="pt-3 flex justify-end space-x-2.5">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 text-xs font-medium text-slate-400 hover:text-white rounded-full transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-5 py-2 text-xs font-semibold text-black bg-amber-400 hover:bg-amber-300 rounded-full shadow-lg shadow-amber-400/20 transition-transform active:scale-95"
            >
              Add to Timeline
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
