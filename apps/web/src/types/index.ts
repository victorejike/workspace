export type TaskPriority = 'LOW' | 'IMPORTANT' | 'HIGH';
export type TaskStatus = 'PENDING' | 'IN_PROGRESS' | 'PAUSED' | 'COMPLETED' | 'SKIPPED' | 'RESCHEDULED';
export type TaskType = 'STUDY' | 'CODING' | 'PROJECT' | 'ASSESSMENT' | 'REVISION';

export interface DailyTask {
  id: string;
  user_id: string;
  topic_id?: string;
  title: string;
  description: string;
  task_type: TaskType;
  priority: TaskPriority;
  status: TaskStatus;
  scheduled_date: string;
  scheduled_time_start?: string;
  scheduled_time_end?: string;
  estimated_minutes: number;
  actual_minutes: number;
  skip_reason?: string;
  completed_at?: string;
}

export interface FocusDashboardMetrics {
  deep_work_seconds_today: number;
  attention_quality_pct: number;
  focus_stability_score: number;
  peak_focus_hour: string;
  completed_goals_today: number;
  total_goals_today: number;
}

export interface FocusSession {
  id: string;
  user_id: string;
  task_id?: string;
  start_time: string;
  end_time?: string;
  active_seconds: number;
  idle_seconds: number;
  is_active: boolean;
  attention_quality_pct: number;
  focus_stability_score: number;
  notes?: string;
}

export interface UserProfile {
  user_id: string;
  display_name: string;
  avatar_url?: string;
  timezone: string;
  experience_level: 'BEGINNER' | 'INTERMEDIATE' | 'ADVANCED';
  career_goal: string;
  target_roles: string[];
}

export interface Curriculum {
  id: string;
  title: string;
  description: string;
  status: string;
  target_career_role?: string;
  estimated_total_hours: number;
}

export interface CurriculumTopic {
  id: string;
  module_id: string;
  title: string;
  description: string;
  order_index: number;
  estimated_minutes: number;
  difficulty: 'BEGINNER' | 'INTERMEDIATE' | 'ADVANCED';
  key_concepts: string[];
  practical_exercises: string[];
}
