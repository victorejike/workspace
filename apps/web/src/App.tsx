import React, { useEffect, useState } from 'react';
import { Navbar } from './components/Navbar';
import { DashboardOverview } from './components/DashboardOverview';
import { CurriculumView } from './components/CurriculumView';
import { AiMentorView } from './components/AiMentorView';
import { TaskModal } from './components/TaskModal';
import { DailyTask, FocusDashboardMetrics, TaskPriority, TaskType, UserProfile } from './types';

export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'overview' | 'planner' | 'projects' | 'goals'>('overview');
  const [isTaskModalOpen, setIsTaskModalOpen] = useState(false);
  const [isTimerActive, setIsTimerActive] = useState(true);
  const [activeTopic, setActiveTopic] = useState('Rust Memory Safety & Lifetimes');

  // User Profile matching mockup
  const [profile] = useState<UserProfile>({
    user_id: 'user-001',
    display_name: 'Kate Schowalter',
    timezone: 'UTC+1',
    experience_level: 'INTERMEDIATE',
    career_goal: 'Staff Systems & AI Engineer',
    target_roles: ['Systems Engineer', 'AI Platform Engineer'],
  });

  // Focus Metrics matching mockup (1h 25m = 5100s, 88% attention, 64 stability)
  const [metrics, setMetrics] = useState<FocusDashboardMetrics>({
    deep_work_seconds_today: 5100,
    attention_quality_pct: 88,
    focus_stability_score: 64,
    peak_focus_hour: '11 am',
    completed_goals_today: 2,
    total_goals_today: 5,
  });

  // Tasks matching mockup screenshot
  const [tasks, setTasks] = useState<DailyTask[]>([
    {
      id: 'task-1',
      user_id: 'user-001',
      title: 'Plan Weekly Routine',
      description: 'Review curriculum pacing and allocate deep work windows',
      task_type: 'STUDY',
      priority: 'LOW',
      status: 'COMPLETED',
      scheduled_date: '2026-11-09',
      scheduled_time_start: '08:30',
      estimated_minutes: 45,
      actual_minutes: 45,
    },
    {
      id: 'task-2',
      user_id: 'user-001',
      title: 'Review Project Materials',
      description: 'Analyze lock-free queue benchmarks and documentation',
      task_type: 'CODING',
      priority: 'IMPORTANT',
      status: 'IN_PROGRESS',
      scheduled_date: '2026-11-09',
      scheduled_time_start: '13:00',
      estimated_minutes: 90,
      actual_minutes: 40,
    },
    {
      id: 'task-3',
      user_id: 'user-001',
      title: 'Call With Client',
      description: 'Discuss technical requirements for AI inference pipeline',
      task_type: 'PROJECT',
      priority: 'HIGH',
      status: 'PENDING',
      scheduled_date: '2026-11-09',
      scheduled_time_start: '17:00',
      estimated_minutes: 60,
      actual_minutes: 0,
    },
  ]);

  // Deep work live ticking timer effect
  useEffect(() => {
    let interval: any = null;
    if (isTimerActive) {
      interval = setInterval(() => {
        setMetrics((prev) => ({
          ...prev,
          deep_work_seconds_today: prev.deep_work_seconds_today + 1,
        }));
      }, 1000);
    }
    return () => clearInterval(interval);
  }, [isTimerActive]);

  const handleToggleTimer = () => {
    setIsTimerActive((prev) => !prev);
  };

  const handleToggleTaskStatus = (taskId: string) => {
    setTasks((prev) =>
      prev.map((t) => {
        if (t.id === taskId) {
          const nextStatus = t.status === 'COMPLETED' ? 'PENDING' : 'COMPLETED';
          return { ...t, status: nextStatus };
        }
        return t;
      })
    );

    // Update completed goals count dynamically
    setMetrics((prev) => {
      const completedCount = tasks.filter((t) =>
        t.id === taskId ? t.status !== 'COMPLETED' : t.status === 'COMPLETED'
      ).length;
      return { ...prev, completed_goals_today: completedCount };
    });
  };

  const handleAddTask = (newTaskData: {
    title: string;
    description: string;
    priority: TaskPriority;
    task_type: TaskType;
    scheduled_time_start: string;
    estimated_minutes: number;
  }) => {
    const newTask: DailyTask = {
      id: `task-${Date.now()}`,
      user_id: profile.user_id,
      title: newTaskData.title,
      description: newTaskData.description,
      task_type: newTaskData.task_type,
      priority: newTaskData.priority,
      status: 'PENDING',
      scheduled_date: '2026-11-09',
      scheduled_time_start: newTaskData.scheduled_time_start,
      estimated_minutes: newTaskData.estimated_minutes,
      actual_minutes: 0,
    };
    setTasks((prev) => [...prev, newTask]);
    setMetrics((prev) => ({ ...prev, total_goals_today: prev.total_goals_today + 1 }));
  };

  const handleContinueLearning = () => {
    setActiveTab('planner');
  };

  const handleSelectTopicFromRoadmap = (topicTitle: string) => {
    setActiveTopic(topicTitle);
    setActiveTab('goals'); // Navigate to AI Mentor Checkpoint
  };

  return (
    <div className="min-h-screen bg-[#0b0d11] text-slate-100 flex flex-col font-sans">
      {/* Top Navigation */}
      <Navbar
        activeTab={activeTab}
        onSelectTab={setActiveTab}
        profile={profile}
      />

      {/* Main Content Area */}
      <main className="flex-1 max-w-7xl w-full mx-auto px-6 pt-7">
        {activeTab === 'overview' && (
          <DashboardOverview
            metrics={metrics}
            tasks={tasks}
            isTimerActive={isTimerActive}
            onToggleTimer={handleToggleTimer}
            onAddTask={() => setIsTaskModalOpen(true)}
            onToggleTaskStatus={handleToggleTaskStatus}
            onContinueLearning={handleContinueLearning}
          />
        )}

        {activeTab === 'planner' && (
          <CurriculumView
            curricula={[]}
            onSelectTopic={handleSelectTopicFromRoadmap}
          />
        )}

        {activeTab === 'goals' && (
          <AiMentorView currentTopic={activeTopic} />
        )}

        {activeTab === 'projects' && (
          <div className="space-y-6">
            <h2 className="text-2xl font-bold text-white tracking-tight">Practical Projects & Repositories</h2>
            <p className="text-sm text-slate-400">
              Verifiable engineering projects proving skill mastery and practical competence.
            </p>
            <div className="rounded-3xl bg-[#12151b] border border-white/[0.07] p-8 text-center space-y-3">
              <span className="text-xs font-semibold px-3 py-1 rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
                GitHub Repository Linked
              </span>
              <h3 className="text-lg font-bold text-white">victorejike/workspace-learning</h3>
              <p className="text-xs text-slate-400 max-w-md mx-auto">
                Daily learning journals, checkpoint records, and practical code implementations are compiled and pushed automatically upon user approval.
              </p>
            </div>
          </div>
        )}
      </main>

      {/* Task Creation Modal */}
      <TaskModal
        isOpen={isTaskModalOpen}
        onClose={() => setIsTaskModalOpen(false)}
        onSave={handleAddTask}
      />
    </div>
  );
};
