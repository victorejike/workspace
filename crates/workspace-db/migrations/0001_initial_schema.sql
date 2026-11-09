-- ============================================================================
-- 0001_initial_schema.sql
-- Portable schema for SQLite and PostgreSQL
-- ============================================================================

-- 1. Users
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 2. User Profiles
CREATE TABLE IF NOT EXISTS user_profiles (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    display_name TEXT NOT NULL,
    avatar_url TEXT,
    timezone TEXT NOT NULL DEFAULT 'UTC',
    experience_level TEXT NOT NULL DEFAULT 'BEGINNER',
    career_goal TEXT NOT NULL DEFAULT '',
    target_roles TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 3. User Preferences
CREATE TABLE IF NOT EXISTS user_preferences (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    theme TEXT NOT NULL DEFAULT 'DARK',
    preferred_daily_hours REAL NOT NULL DEFAULT 2.0,
    available_days_per_week INTEGER NOT NULL DEFAULT 5,
    quiet_hours_start TEXT,
    quiet_hours_end TEXT,
    idle_timeout_minutes INTEGER NOT NULL DEFAULT 5,
    notifications_enabled INTEGER NOT NULL DEFAULT 1,
    updated_at TEXT NOT NULL
);

-- 4. User Consents (Transparent Privacy Audit)
CREATE TABLE IF NOT EXISTS user_consents (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    scope TEXT NOT NULL,
    granted INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (user_id, scope)
);

-- 5. Curricula
CREATE TABLE IF NOT EXISTS curricula (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    source_type TEXT NOT NULL DEFAULT 'MANUAL',
    status TEXT NOT NULL DEFAULT 'DRAFT',
    target_career_role TEXT,
    estimated_total_hours REAL NOT NULL DEFAULT 0.0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_curricula_user ON curricula(user_id);

-- 6. Curriculum Modules
CREATE TABLE IF NOT EXISTS curriculum_modules (
    id TEXT PRIMARY KEY,
    curriculum_id TEXT NOT NULL REFERENCES curricula(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    order_index INTEGER NOT NULL DEFAULT 0,
    estimated_hours REAL NOT NULL DEFAULT 0.0,
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_modules_curriculum ON curriculum_modules(curriculum_id);

-- 7. Curriculum Topics
CREATE TABLE IF NOT EXISTS curriculum_topics (
    id TEXT PRIMARY KEY,
    module_id TEXT NOT NULL REFERENCES curriculum_modules(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    order_index INTEGER NOT NULL DEFAULT 0,
    estimated_minutes INTEGER NOT NULL DEFAULT 60,
    difficulty TEXT NOT NULL DEFAULT 'BEGINNER',
    key_concepts TEXT NOT NULL DEFAULT '[]',
    practical_exercises TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_topics_module ON curriculum_topics(module_id);

-- 8. Topic Prerequisites
CREATE TABLE IF NOT EXISTS topic_prerequisites (
    topic_id TEXT NOT NULL REFERENCES curriculum_topics(id) ON DELETE CASCADE,
    prerequisite_topic_id TEXT NOT NULL REFERENCES curriculum_topics(id) ON DELETE CASCADE,
    PRIMARY KEY (topic_id, prerequisite_topic_id)
);

-- 9. Learning Goals
CREATE TABLE IF NOT EXISTS learning_goals (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    curriculum_id TEXT REFERENCES curricula(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    target_date TEXT,
    target_hours REAL NOT NULL DEFAULT 0.0,
    current_hours REAL NOT NULL DEFAULT 0.0,
    is_completed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_learning_goals_user ON learning_goals(user_id);

-- 10. Learning Schedules
CREATE TABLE IF NOT EXISTS learning_schedules (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    daily_committed_hours REAL NOT NULL DEFAULT 2.0,
    available_days_per_week INTEGER NOT NULL DEFAULT 5,
    preferred_start_hour INTEGER NOT NULL DEFAULT 8,
    preferred_end_hour INTEGER NOT NULL DEFAULT 20,
    buffer_revision_percentage REAL NOT NULL DEFAULT 0.25
);

-- 11. Daily Tasks
CREATE TABLE IF NOT EXISTS daily_tasks (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic_id TEXT REFERENCES curriculum_topics(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    task_type TEXT NOT NULL DEFAULT 'STUDY',
    priority TEXT NOT NULL DEFAULT 'IMPORTANT',
    status TEXT NOT NULL DEFAULT 'PENDING',
    scheduled_date TEXT NOT NULL,
    scheduled_time_start TEXT,
    scheduled_time_end TEXT,
    estimated_minutes INTEGER NOT NULL DEFAULT 60,
    actual_minutes INTEGER NOT NULL DEFAULT 0,
    skip_reason TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_daily_tasks_user_date ON daily_tasks(user_id, scheduled_date);

-- 12. Learning Sessions & Task Sessions
CREATE TABLE IF NOT EXISTS learning_sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    task_id TEXT REFERENCES daily_tasks(id) ON DELETE SET NULL,
    start_time TEXT NOT NULL,
    end_time TEXT,
    active_seconds INTEGER NOT NULL DEFAULT 0,
    idle_seconds INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    attention_quality_pct INTEGER NOT NULL DEFAULT 100,
    focus_stability_score INTEGER NOT NULL DEFAULT 70,
    notes TEXT
);
CREATE INDEX IF NOT EXISTS idx_sessions_user ON learning_sessions(user_id);

CREATE TABLE IF NOT EXISTS activity_events (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL REFERENCES learning_sessions(id) ON DELETE CASCADE,
    source TEXT NOT NULL DEFAULT 'MANUAL',
    event_type TEXT NOT NULL,
    metadata_json TEXT,
    timestamp TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_events_session ON activity_events(session_id);

-- 13. Assessments & Question Checkpoints
CREATE TABLE IF NOT EXISTS assessments (
    id TEXT PRIMARY KEY,
    topic_id TEXT NOT NULL REFERENCES curriculum_topics(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS assessment_questions (
    id TEXT PRIMARY KEY,
    assessment_id TEXT NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    question_text TEXT NOT NULL,
    question_type TEXT NOT NULL,
    expected_concepts TEXT NOT NULL DEFAULT '[]'
);

CREATE TABLE IF NOT EXISTS assessment_attempts (
    id TEXT PRIMARY KEY,
    assessment_id TEXT NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    score_percentage INTEGER NOT NULL,
    mastery_level TEXT NOT NULL,
    conceptual_accuracy_score INTEGER NOT NULL,
    practical_code_score INTEGER NOT NULL,
    feedback_markdown TEXT NOT NULL DEFAULT '',
    identified_knowledge_gaps TEXT NOT NULL DEFAULT '[]',
    attempted_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_attempts_user ON assessment_attempts(user_id);

-- 14. Skill Mastery State
CREATE TABLE IF NOT EXISTS skill_mastery (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic_id TEXT NOT NULL REFERENCES curriculum_topics(id) ON DELETE CASCADE,
    mastery_state TEXT NOT NULL DEFAULT 'INTRODUCED',
    composite_score INTEGER NOT NULL DEFAULT 0,
    assessments_count INTEGER NOT NULL DEFAULT 0,
    practical_projects_count INTEGER NOT NULL DEFAULT 0,
    last_assessed_at TEXT,
    next_spaced_revision_date TEXT,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (user_id, topic_id)
);

-- 15. Learning Notes & Daily Reports
CREATE TABLE IF NOT EXISTS learning_notes (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic_id TEXT REFERENCES curriculum_topics(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    content_markdown TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS daily_reports (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    report_date TEXT NOT NULL,
    title TEXT NOT NULL,
    content_markdown TEXT NOT NULL,
    deep_work_minutes INTEGER NOT NULL DEFAULT 0,
    tasks_completed_count INTEGER NOT NULL DEFAULT 0,
    published_to_github INTEGER NOT NULL DEFAULT 0,
    github_commit_sha TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (user_id, report_date)
);

-- 16. Projects & Project Tasks
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    github_repo_url TEXT,
    status TEXT NOT NULL DEFAULT 'PLANNING',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS project_tasks (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    is_completed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

-- 17. Resources & Sources
CREATE TABLE IF NOT EXISTS resource_sources (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    base_url TEXT NOT NULL,
    reliability_tier INTEGER NOT NULL DEFAULT 100,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS resources (
    id TEXT PRIMARY KEY,
    topic_id TEXT REFERENCES curriculum_topics(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    url TEXT NOT NULL,
    publisher TEXT NOT NULL,
    resource_type TEXT NOT NULL DEFAULT 'OFFICIAL_DOCS',
    technology TEXT NOT NULL,
    difficulty_level TEXT NOT NULL DEFAULT 'BEGINNER',
    published_date TEXT,
    last_updated_date TEXT,
    verified_by_workspace_at TEXT NOT NULL,
    reliability_score INTEGER NOT NULL DEFAULT 95,
    is_official INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS user_resources (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    resource_id TEXT NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    is_bookmarked INTEGER NOT NULL DEFAULT 0,
    is_completed INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, resource_id)
);

-- 18. Integrations & External Repositories
CREATE TABLE IF NOT EXISTS integrations (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    encrypted_token TEXT,
    is_enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (user_id, provider)
);

CREATE TABLE IF NOT EXISTS github_repositories (
    id TEXT PRIMARY KEY,
    integration_id TEXT NOT NULL REFERENCES integrations(id) ON DELETE CASCADE,
    repo_full_name TEXT NOT NULL,
    branch TEXT NOT NULL DEFAULT 'main',
    documentation_root TEXT NOT NULL DEFAULT 'workspace-learning/',
    auto_sync_on_completion INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT
);

-- 19. Notifications
CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    is_read INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_notifications_user ON notifications(user_id, is_read);

-- 20. AI Conversations & Mentorship Context
CREATE TABLE IF NOT EXISTS ai_conversations (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic_id TEXT REFERENCES curriculum_topics(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ai_messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL REFERENCES ai_conversations(id) ON DELETE CASCADE,
    role TEXT NOT NULL, -- 'user', 'assistant', 'system'
    content TEXT NOT NULL,
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_messages_conversation ON ai_messages(conversation_id);
