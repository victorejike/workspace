// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Welcome to WORKSPACE, {}!", name)
}

#[tauri::command]
fn get_system_status() -> serde_json::Value {
    json!({
        "status": "online",
        "app": "WORKSPACE",
        "version": "0.1.0",
        "mode": "desktop-local-first"
    })
}

fn main() {# MASTER PROJECT INSTRUCTION: WORKSPACE
## An AI-Powered Personal Learning, Focus and Developer Productivity Environment

You are acting as a senior software architect, Rust engineer, AI engineer, product designer, database architect, security engineer and technical lead.

Your responsibility is to design and build WORKSPACE: a multi-user, AI-powered learning and productivity environment that helps developers, computer science students, aspiring engineers and self-taught learners stay focused, follow structured learning paths, monitor their development and achieve their technical goals.

This is not a simple task manager, ordinary learning management system, chatbot or time-tracking application.

It is a personal engineering workspace that understands each user's learning objectives, organizes their daily activities, tracks their actual work, evaluates their understanding and continuously adapts their learning journey.

The product must be useful to its creator and independently usable by other people around the world.

---

# 1. PRODUCT VISION

Product name: WORKSPACE

Repository: workspace

Core philosophy:

"Know what to learn. Know what to do. Understand what you've achieved. Always know what's next."

The problem we are solving:

Many people begin learning programming, artificial intelligence, cybersecurity, software engineering, cloud computing and other technical disciplines without a reliable system for managing their learning.

They frequently:
- Lose track of their curriculum.
- Jump between unrelated technologies.
- Forget what they studied previously.
- Spend time working without knowing whether they are progressing.
- Struggle to balance learning with personal responsibilities.
- Abandon learning plans because they lack accountability.
- Have no reliable record of what they have learned.
- Confuse time spent studying with actual understanding.
- Use different applications without a unified progress record.

WORKSPACE must address these problems.

The application should become an intelligent environment that helps individuals remain consistent, organized, accountable and technically competent.

Do not build features merely because they sound impressive. Every feature must solve a real user problem.

# 2. TARGET USERS

Design WORKSPACE for multiple categories of users:

1. Computer science students.
2. University and college students.
3. Self-taught programmers.
4. Aspiring AI engineers.
5. Backend and frontend developers.
6. Full-stack developers.
7. Cybersecurity learners.
8. Data scientists and machine learning engineers.
9. DevOps and cloud engineers.
10. Professionals learning new technologies.
11. People changing careers into technology.

The application must not assume that every user knows how to program.

Support complete beginners, intermediate learners and experienced engineers.

Every user must have an independent profile, learning history, preferences, integrations and progress records.

# 3. TECHNOLOGY AND ARCHITECTURE

Rust must be the primary engineering language.

Use a modular architecture with clearly separated responsibilities.

## Recommended stack

Desktop application:
- Rust
- Tauri 2
- React
- TypeScript

Backend:
- Rust
- Axum
- Tokio
- SQLx

Databases:
- PostgreSQL for multi-user cloud data.
- SQLite for local-first storage and offline operation.
- pgvector when semantic search becomes necessary.

AI:
- Model-agnostic AI provider architecture.
- Support configurable AI providers.
- Structured outputs for curriculum extraction, assessments and task generation.
- Streaming responses for interactive tutoring.
- Persistent user learning context.

Frontend:
- React
- TypeScript
- Tailwind CSS
- Accessible component architecture.

Integrations:
- GitHub API
- Chrome Extensions Manifest V3
- VS Code Extension API
- WhatsApp Business Platform
- Email notifications
- Desktop notifications

Infrastructure:
- Docker for reproducible development.
- GitHub Actions for CI.
- Environment-based configuration.
- Secure secret management.
- Automated database migrations.

Do not introduce unnecessary technologies.

Do not add Redis, Kafka, Kubernetes or additional databases without a demonstrated engineering requirement.

Do not put all business logic inside the frontend.

Keep the application modular and testable.

# 4. CORE PRODUCT MODULES

## MODULE A: USER ACCOUNTS AND PERSONAL PROFILE

Build a complete account system.

Users should be able to:
- Register.
- Log in and log out.
- Reset passwords.
- Manage their profile.
- Set their timezone.
- Configure their preferred learning hours.
- Choose their experience level.
- Define career objectives.
- Set daily and weekly learning targets.
- Connect external services.
- Export their personal data.
- Delete their account and associated data.

Every user's data must be isolated.

Implement proper authentication, authorization and multi-tenant data boundaries.

Never expose another user's curriculum, schedule, activity or AI conversation.

## MODULE B: CURRICULUM INTELLIGENCE

This is one of the most important modules.

Users should be able to provide their existing curriculum through:

- PDF.
- DOCX.
- Markdown.
- TXT.
- CSV.
- Manually entered topics.
- External learning resources.
- Public curriculum repositories.

The AI must analyze the submitted curriculum.

It should identify:
- Learning modules.
- Individual topics.
- Subtopics.
- Prerequisites.
- Required tools.
- Practical exercises.
- Projects.
- Recommended resources.
- Estimated learning effort.
- Dependencies between topics.

Convert the curriculum into a structured learning roadmap.

Do not simply summarize an uploaded document.

The system must extract actionable learning objectives.

For example:

A user wants to become an AI Engineer.

The system should identify relevant learning areas such as:

1. Python programming.
2. Mathematics and statistics.
3. Data manipulation.
4. Data structures and algorithms.
5. Machine learning.
6. Deep learning.
7. Neural networks.
8. Natural language processing.
9. Large language models.
10. Retrieval-augmented generation.
11. AI agents.
12. Model deployment.
13. AI system evaluation.
14. Production AI engineering.

These topics are illustrative, not a mandatory curriculum for every user.

The system must adapt the roadmap to the user's background and actual goals.

Allow users to edit and approve the generated curriculum before activating it.

## MODULE C: PERSONALIZED LEARNING PLANNER

After analyzing the curriculum, ask the user how much time they can realistically dedicate.

Examples:
- 1 hour daily.
- 2 hours daily.
- 4 hours daily.
- 6 hours daily.
- Custom schedule.

Ask about:
- Available days.
- Preferred learning hours.
- Existing commitments.
- Target completion date, if any.
- Preferred learning style.
- Existing technical experience.

Calculate an estimated learning timeline.

For example:

If a curriculum requires 600 estimated learning hours and someone commits 3 hours daily, the theoretical minimum is 200 learning days.

However, the system must account for:
- Revision.
- Rest days.
- Assessment failures.
- Practical projects.
- Learning difficulties.
- Schedule changes.

Never promise an exact completion date.

Present estimates as adjustable projections.

Allow users to accelerate their learning journey by increasing their available time, but never encourage unrealistic schedules.

## MODULE D: DAILY WORKSPACE

This is the primary interface users should interact with every day.

Create a dashboard containing:

- Today's learning objectives.
- Today's tasks.
- Daily schedule.
- Active learning session.
- Learning progress.
- Upcoming deadlines.
- Pending assessments.
- AI recommendations.
- Recent activity.
- Current streak.
- Weekly performance.
- Next recommended action.

The user should never have to search through multiple screens to discover what they should do next.

Provide a prominent action:

"Continue Learning"

When clicked, the system should open the current learning objective and its associated study environment.

Support:
- Start task.
- Pause task.
- Resume task.
- Complete task.
- Skip task with a reason.
- Reschedule task.
- Add personal tasks.
- Record notes.

## MODULE E: AI LEARNING MENTOR

Build an interactive AI mentor.

It must not behave like a generic chatbot.

It should understand the user's:
- Curriculum.
- Current topic.
- Previous topics.
- Learning history.
- Knowledge gaps.
- Assessment results.
- Learning preferences.
- Current projects.

The mentor should support:

1. Topic explanations.
2. Step-by-step tutorials.
3. Code examples.
4. Practical exercises.
5. Debugging assistance.
6. Concept comparisons.
7. Project suggestions.
8. Revision sessions.
9. Interactive questioning.
10. Technical interview simulations.

The AI should teach according to the user's experience level.

For a beginner, explain concepts clearly.

For an experienced developer, use appropriate technical terminology and deeper explanations.

### Interactive checkpoints

After a topic, the AI should initiate an assessment.

Example:

Topic: Python decorators.

Questions:
- Explain what a decorator is.
- Why would you use one?
- Explain how a wrapper function works.
- Implement a decorator independently.
- Solve a practical programming problem.

Assess the user's answers using:
- Conceptual accuracy.
- Reasoning.
- Practical implementation.
- Ability to explain the solution.
- Ability to solve related problems.

Classify results:

MASTERED:
The user demonstrates adequate understanding.

NEEDS_REVISION:
The user understands some concepts but has identifiable gaps.

NOT_MASTERED:
The user requires additional instruction and practice.

Never mark a topic mastered solely because the user opened a lesson or spent a certain number of hours.

The AI must provide explanations for assessment results.

Store assessment history so the system can measure improvement over time.

## MODULE F: LEARNING ACTIVITY AND FOCUS TRACKER

Develop an activity-tracking engine.

The purpose is to help users understand how they spend their learning time.

Track relevant activities such as:
- Active learning sessions.
- Coding sessions.
- Reading technical documentation.
- Working on practical projects.
- Completing exercises.
- Interacting with the AI mentor.
- Taking assessments.

Record:
- Start time.
- End time.
- Active duration.
- Idle duration.
- Associated learning objective.
- Associated project.
- Application or integration source.
- User-provided activity description.

Build:
- Daily activity reports.
- Weekly summaries.
- Monthly summaries.
- Learning consistency statistics.
- Focus trends.
- Time allocation by technology.
- Time allocation by project.

Important:

Do not equate an open application with productive work.

Implement configurable idle detection.

Allow users to correct inaccurate activity records.

Activity tracking must be transparent and permission-based.

Do not collect passwords, private messages, keystrokes, unrelated browsing content or screenshots by default.

Provide pause tracking and data deletion controls.

## MODULE G: DEVELOPER ENVIRONMENT INTEGRATION

Allow users to connect their preferred development environments.

Initial integration targets:

1. Visual Studio Code.
2. Cursor.
3. JetBrains IDEs where feasible.
4. GitHub.
5. Chrome.

Build integrations progressively.

For VS Code, develop an extension that can communicate with the local WORKSPACE application.

Possible events:
- Project opened.
- Coding session started.
- Coding session ended.
- Workspace project selected.
- Learning task associated with a project.
- Git activity detected.

Do not attempt to monitor every internal editor operation.

Collect only useful, authorized activity metadata.

The system should allow users to associate an editor session with a particular learning task.

## MODULE H: GITHUB INTEGRATION

Build a secure GitHub integration.

Users should be able to:

- Connect GitHub accounts.
- Select repositories.
- Create learning repositories.
- Associate repositories with curriculum modules.
- Track commits.
- Track pull requests.
- Track repository activity.
- View practical projects.
- Generate Markdown progress reports.
- Push approved learning documentation.

Create an automated documentation system.

Example repository structure:

workspace-learning/
├── README.md
├── curriculum/
│   └── learning-roadmap.md
├── daily-progress/
│   ├── day-001.md
│   └── day-002.md
├── assessments/
│   ├── python-basics.md
│   └── machine-learning.md
├── projects/
│   └── project-progress.md
└── OVERVIEW.md

Each daily Markdown report should contain:

- Date.
- Learning objectives.
- Topics studied.
- Practical work completed.
- Active learning time.
- Assessment results.
- Knowledge gaps.
- Achievements.
- Next learning objectives.

Never invent commits, projects, completed tasks or achievements.

Allow users to review generated documents before publishing.

Use minimal GitHub permissions and secure token handling.

## MODULE I: CHROME EXTENSION

Develop a Chrome extension using Manifest V3.

Its purpose is to help users track relevant learning activities.

Examples:
- Reading technical documentation.
- Watching educational lectures.
- Reading research papers.
- Visiting programming tutorials.
- Working with online coding exercises.

Allow users to:
- Connect Chrome to WORKSPACE.
- View their learning timer.
- Associate browsing sessions with learning objectives.
- Exclude websites.
- Pause tracking.
- Review collected activity.

Do not collect unrelated browsing history or private webpage contents.

Use explicit permissions and explain what each permission does.

## MODULE J: UPDATED LEARNING RESOURCE DISCOVERY

This is a major differentiating capability.

Users should be able to discover learning materials relevant to their current curriculum.

The AI must be capable of finding current and trustworthy educational resources.

Do not rely exclusively on the AI model's training knowledge.

Use current sources.

### Recommended resource sources

Official documentation:

Rust:
https://doc.rust-lang.org/book/
https://doc.rust-lang.org/stable/
https://rust-lang.org/learn/

Tauri:
https://v2.tauri.app/

GitHub:
https://docs.github.com/en/rest
https://docs.github.com/en/apps

Chrome:
https://developer.chrome.com/docs/extensions/

VS Code:
https://code.visualstudio.com/api/

SQLite:
https://sqlite.org/docs.html

MDN:
https://developer.mozilla.org/en-US/docs/Learn_web_development

Programming curricula:

https://roadmap.sh/
https://www.freecodecamp.org/learn/
https://www.theodinproject.com/
https://github.com/TheOdinProject/curriculum

AI and machine learning:

https://huggingface.co/learn
https://course.fast.ai/
https://developers.google.com/machine-learning/crash-course
https://pytorch.org/tutorials/
https://scikit-learn.org/stable/user_guide.html

Computer science:

https://cs50.harvard.edu/
https://teachyourselfcs.com/

Documentation and standards:

https://www.rfc-editor.org/
https://www.w3.org/TR/
https://www.nist.gov/

These are starting sources, not an exhaustive list.

### Resource discovery requirements

Build a resource discovery service.

It should:
1. Understand the current topic.
2. Search relevant official documentation.
3. Find current tutorials and courses.
4. Identify relevant open-source projects.
5. Find practical exercises.
6. Identify recently updated documentation.
7. Compare resources by quality and relevance.
8. Return links with source attribution.

For every resource, store:

- Title.
- Description.
- URL.
- Publisher.
- Resource type.
- Technology.
- Difficulty level.
- Publication date, if available.
- Last updated date, if available.
- Date verified by WORKSPACE.
- Source reliability.
- Relevant curriculum topic.

Prefer official documentation, established educational institutions and reputable open-source projects.

Use release notes, official changelogs, GitHub releases, documentation feeds and official course updates where available.

Do not assume that a recently published tutorial is necessarily accurate.

Do not fabricate publication dates or claim that a resource is current without verification.

Respect website terms, robots policies, copyright and API rate limits.

Do not scrape paid courses or reproduce copyrighted course materials.

When appropriate, link to the original material instead of copying it.

### Up-to-date learning

If a user is studying a technology that changes frequently, the AI should be able to identify:

- Current stable versions.
- Deprecated APIs.
- New framework releases.
- Updated installation instructions.
- Breaking changes.
- Security advisories.
- New official documentation.
- Relevant recent tutorials.

Show the date when a resource was last verified.

When no current information is available, say so clearly.

## MODULE K: PROGRESS JOURNAL AND AUTOMATIC MARKDOWN

Build a persistent learning journal.

At the end of each session, the AI should help generate a structured summary.

Example:

# Day 001 - Python Functions

Date: 2026-11-09

## Learning objectives
- Understand functions.
- Understand parameters.
- Understand return values.

## Completed activities
- Studied function definitions.
- Completed practical exercises.
- Built a calculator.

## Assessment
Status: Needs revision.

## Knowledge gaps
- Variable scope.
- Default parameters.

## Time spent
3 hours 20 minutes.

## Next objectives
- Revise variable scope.
- Study decorators.

Generate Markdown automatically.

Allow the user to edit, export and optionally push the documentation to GitHub.

The journal must be searchable.

## MODULE L: REMINDERS AND COMMUNICATION

Build a notification system.

Initial channels:
- Desktop notifications.
- In-app notifications.
- Email.

Future integrations:
- WhatsApp Business Platform.
- Mobile push notifications.
- Supported voice communication providers.

The system should send:
- Daily learning reminders.
- Upcoming task reminders.
- Missed-session notifications.
- Weekly progress summaries.
- Assessment reminders.
- Revision reminders.

WhatsApp integration must use the official Meta WhatsApp Business Platform.

Respect opt-in requirements, approved message templates and applicable messaging policies.

Voice calls must only be implemented through an officially supported provider and with explicit user consent.

Never spam users.

Allow users to configure notification frequency and quiet hours.

# 5. AI PERSONALIZATION ENGINE

Do not implement the AI as one enormous prompt.

Create modular AI capabilities with clear responsibilities:

1. Curriculum Analyzer.
2. Learning Planner.
3. Teaching Assistant.
4. Assessment Evaluator.
5. Resource Researcher.
6. Progress Analyst.
7. Revision Planner.
8. Project Recommendation Assistant.
9. Daily Accountability Assistant.

These components should use a shared, permission-controlled user learning profile.

The profile should contain:
- Learning objectives.
- Experience level.
- Curriculum.
- Topic mastery.
- Assessment history.
- Learning preferences.
- Available hours.
- Schedule.
- Active projects.
- Knowledge gaps.
- Progress history.

The AI should adapt its recommendations as new evidence becomes available.

Do not create a new AI agent for every simple task.

Use ordinary deterministic application logic where AI is unnecessary.

AI should not independently modify user schedules, push to GitHub or send external messages without the relevant authorization.

# 6. SKILL MASTERY AND PROGRESS ANALYTICS

Build a skill mastery system.

Represent skills as connected concepts with prerequisites.

For example:

Python fundamentals
    ↓
Data structures
    ↓
NumPy and Pandas
    ↓
Machine learning
    ↓
Deep learning
    ↓
AI engineering

Users should see:
- Skills introduced.
- Skills being studied.
- Skills requiring revision.
- Skills demonstrated through assessments.
- Skills demonstrated through projects.

Do not use one arbitrary percentage to claim someone is proficient.

Use multiple forms of evidence.

Measure:
- Conceptual understanding.
- Assessment performance.
- Practical implementation.
- Project completion.
- Retention over time.
- Independent problem-solving.

Create visual progress dashboards.

# 7. DESIGN AND USER EXPERIENCE

The interface must look like a professional developer workspace.

Design principles:

- Clean.
- Minimal.
- Modern.
- Technical.
- Responsive.
- Accessible.
- Fast.
- Consistent.

Avoid excessive gradients, unnecessary animations and cluttered dashboards.

Create the following main navigation:

1. Overview
2. My Learning
3. Roadmap
4. Daily Tasks
5. AI Mentor
6. Focus Tracker
7. Projects
8. Progress Journal
9. Resources
10. Analytics
11. Integrations
12. Settings

The user should have one obvious next action.

Support dark and light themes.

Do not sacrifice functionality for visual appearance.

# 8. SECURITY, PRIVACY AND DATA OWNERSHIP

WORKSPACE handles personal learning data and connected accounts.

Implement:

- Secure authentication.
- Password hashing.
- Authorization checks.
- Tenant isolation.
- Encrypted transport.
- Secure token storage.
- Input validation.
- API rate limiting.
- Audit logs for sensitive actions.
- Data export.
- Account deletion.
- Consent management.
- Secure OAuth flows.

Never store provider secrets in frontend code.

Never expose GitHub access tokens to the browser.

Never silently collect activity.

Every integration must explain what it accesses and why.

Make tracking optional and controllable.

Use data minimization.

Design for GDPR principles and other applicable privacy regulations.

# 9. DATABASE DESIGN

Design a normalized database with appropriate relationships.

At minimum, evaluate these entities:

users
user_profiles
user_preferences
curricula
curriculum_modules
curriculum_topics
topic_prerequisites
learning_goals
learning_schedules
daily_tasks
task_sessions
activity_events
learning_sessions
assessments
assessment_questions
assessment_attempts
skill_mastery
learning_notes
daily_reports
projects
project_tasks
resources
resource_sources
user_resources
integrations
github_repositories
notifications
ai_conversations
ai_messages
user_consents

Include:
- Primary keys.
- Foreign keys.
- Tenant ownership.
- Created and updated timestamps.
- Appropriate indexes.
- Database constraints.
- Migration files.

Avoid storing redundant or unnecessary information.

Keep sensitive integration credentials outside ordinary user profile records.

Design the database so the application can evolve without losing historical learning data.

# 10. DEVELOPMENT ROADMAP

Do not attempt to implement every feature in one development operation.

Build incrementally.

## PHASE 0: Repository audit and architecture

Inspect the existing GitHub repository.

Determine:
- Current directory structure.
- Existing code.
- Existing dependencies.
- Development environment.
- Available build tools.

Do not delete existing files without justification.

Create:
- README.md
- ARCHITECTURE.md
- DEVELOPMENT.md
- ROADMAP.md
- SECURITY.md
- CONTRIBUTING.md
- .env.example

Document the architecture and development decisions.

## PHASE 1: Functional foundation

Implement:
- Tauri desktop application.
- Rust application core.
- Authentication.
- User profiles.
- Database migrations.
- Curriculum management.
- Daily tasks.
- Learning schedule.

Acceptance criteria:
A user can create an account, enter learning objectives, create a curriculum and generate daily tasks.

## PHASE 2: Focus and progress

Implement:
- Focus timer.
- Learning sessions.
- Activity records.
- Progress journal.
- Daily and weekly summaries.
- Task completion.

Acceptance criteria:
A user can track a learning session, close the application, reopen it and recover their progress.

## PHASE 3: AI learning intelligence

Implement:
- AI provider abstraction.
- Curriculum analysis.
- Personalized task generation.
- AI tutoring.
- Interactive assessments.
- Learning reports.
- Knowledge gap identification.

Acceptance criteria:
A user can study a topic, complete a checkpoint and receive a meaningful progress update.

## PHASE 4: Current resource discovery

Implement:
- Resource source registry.
- Official documentation discovery.
- Release and changelog monitoring.
- Search and resource ranking.
- Resource verification dates.
- Curriculum-resource associations.

Acceptance criteria:
A user can request current learning materials for a topic and receive relevant, traceable sources with dates.

## PHASE 5: GitHub integration

Implement:
- GitHub authentication.
- Repository selection.
- Activity retrieval.
- Markdown generation.
- User-approved publishing.

Acceptance criteria:
A user can connect GitHub and publish a learning report to an authorized repository.

## PHASE 6: External tracking

Implement:
- Chrome extension.
- VS Code extension.
- Local activity collection.
- Consent management.
- Session association.

Acceptance criteria:
A user can connect an editor or browser and see relevant learning activity.

## PHASE 7: Communication

Implement:
- Desktop notifications.
- Email.
- Scheduled reminders.
- WhatsApp integration.
- Optional voice communication.

Acceptance criteria:
A user can configure reminders and receive them through their selected supported channel.

## PHASE 8: Production readiness

Implement:
- Automated tests.
- Security testing.
- Performance testing.
- Error monitoring.
- Backups.
- Deployment automation.
- Cross-platform packaging.
- Documentation.

Prepare the application for real users.

# 11. TESTING REQUIREMENTS

Testing is mandatory.

Rust:
- Unit tests.
- Integration tests.
- Error handling.
- Database tests.
- Concurrency tests.

Frontend:
- Component tests.
- State management tests.
- User interaction tests.

Integration:
- GitHub authentication.
- Database persistence.
- AI provider failures.
- Resource discovery failures.
- Notification delivery.
- Permission handling.

Security:
- Tenant isolation.
- Unauthorized access.
- Token protection.
- Input validation.
- Data deletion.
- Permission enforcement.

Never report a feature as complete if it has only been implemented visually.

A feature is complete when it works, persists the correct data and passes its relevant tests.

# 12. ENGINEERING RULES

Follow these rules throughout development:

1. Inspect before modifying.
2. Use stable, supported dependencies.
3. Check official documentation before selecting libraries.
4. Avoid unnecessary abstraction.
5. Avoid duplicated business logic.
6. Keep Rust modules small and maintainable.
7. Use proper error types.
8. Avoid panics in normal application execution.
9. Use asynchronous operations appropriately.
10. Keep database operations transactional where needed.
11. Validate external data.
12. Never hardcode API credentials.
13. Never fabricate integration results.
14. Never create fake activity statistics.
15. Never use mock data as if it were real user data.
16. Document architectural decisions.
17. Write tests alongside implementation.
18. Keep the application usable without an AI API connection for its core scheduling and tracking functions.
19. Make cloud services optional where practical.
20. Preserve existing working functionality.

Do not claim to have tested an integration when credentials or external services are unavailable.

Use clear interfaces and report what remains unverified.

# 13. OPEN-SOURCE AND CONTRIBUTION READINESS

Although the initial repository may be private, design the project so it can eventually support public contributions.

Create:
- Contribution guidelines.
- Development setup instructions.
- Issue templates.
- Pull request templates.
- Architecture documentation.
- API documentation.
- Code formatting rules.
- Testing instructions.

Avoid personal hardcoding.

No personal curriculum, name, schedule, GitHub repository or private preferences should be embedded in the source code.

Every user must be able to configure their own workspace.

# 14. EXPECTED INITIAL OUTPUT

Before implementing major functionality, provide:

1. Your assessment of the repository.
2. Proposed architecture.
3. Proposed directory structure.
4. Database design.
5. Core Rust modules.
6. Frontend modules.
7. External integration boundaries.
8. Security considerations.
9. Development phases.
10. Initial acceptance tests.

Identify technical risks and dependencies.

If a major architectural choice is unclear, explain the alternatives and recommend one.

Do not silently make expensive or irreversible decisions.

# 15. HOW YOU SHOULD WORK

Work as an engineering partner, not a code-generation machine.

First understand the product.

Then understand the repository.

Then design the architecture.

Then implement the smallest complete, useful version.

After each phase:
- Explain what was implemented.
- List files created or modified.
- Explain architectural decisions.
- Run the relevant tests.
- Report failures honestly.
- Identify incomplete work.
- Update the documentation.
- Recommend the next implementation milestone.

Do not attempt to generate thousands of lines of code in a single response.

Do not create a beautiful dashboard with non-functional controls.

Do not pretend to connect services that have not been configured.

Do not introduce dependencies just to make the architecture appear sophisticated.

Prioritize working software, reliability, maintainability and user experience.

---

# FINAL PRODUCT OBJECTIVE

Build WORKSPACE into a reliable, intelligent, multi-user learning and productivity environment.

A person should be able to arrive with a goal, provide their curriculum, establish their available learning time, receive a structured roadmap, follow daily learning activities, interact with an AI mentor, access current learning resources, record their progress and continuously improve their technical abilities.

The system should help users remain focused without requiring them to manually organize every part of their learning journey.

The ultimate measure of success is not how many features the application contains.

It is whether people consistently use WORKSPACE, understand what they need to do next, maintain their learning discipline and demonstrate real improvement in their technical abilities.

**Build the foundation correctly. Keep the architecture extensible. Deliver working functionality in verified stages.**
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main");
                if let Some(w) = window {
                    let _ = w.show();
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_system_status])
        .run(tauri::generate_context!())
        .expect("error while running WORKSPACE desktop application");
}
