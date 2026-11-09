use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use workspace_types::{
    Curriculum, CurriculumDetails, CurriculumId, CurriculumModule, CurriculumStatus,
    CurriculumTopic, DifficultyLevel, ModuleId, ModuleWithTopics, SourceType, TopicId,
    TopicPrerequisite, UserId,
};

use crate::error::DbError;

#[async_trait]
pub trait CurriculumRepository: Send + Sync {
    async fn create_curriculum(
        &self,
        user_id: &UserId,
        title: &str,
        description: &str,
        source_type: SourceType,
        target_career_role: Option<&str>,
    ) -> Result<Curriculum, DbError>;

    async fn add_module(
        &self,
        curriculum_id: &CurriculumId,
        title: &str,
        description: &str,
        order_index: u32,
        estimated_hours: f32,
    ) -> Result<CurriculumModule, DbError>;

    async fn add_topic(
        &self,
        module_id: &ModuleId,
        title: &str,
        description: &str,
        order_index: u32,
        estimated_minutes: u32,
        difficulty: DifficultyLevel,
        key_concepts: &[String],
        practical_exercises: &[String],
    ) -> Result<CurriculumTopic, DbError>;

    async fn add_prerequisite(
        &self,
        topic_id: &TopicId,
        prerequisite_topic_id: &TopicId,
    ) -> Result<(), DbError>;

    async fn list_user_curricula(&self, user_id: &UserId) -> Result<Vec<Curriculum>, DbError>;

    async fn get_curriculum_details(
        &self,
        curriculum_id: &CurriculumId,
    ) -> Result<Option<CurriculumDetails>, DbError>;
}

pub struct SqliteCurriculumRepository {
    pool: SqlitePool,
}

impl SqliteCurriculumRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CurriculumRepository for SqliteCurriculumRepository {
    async fn create_curriculum(
        &self,
        user_id: &UserId,
        title: &str,
        description: &str,
        source_type: SourceType,
        target_career_role: Option<&str>,
    ) -> Result<Curriculum, DbError> {
        let id = CurriculumId::new();
        let now = Utc::now().to_rfc3339();
        let source_str = match source_type {
            SourceType::Manual => "MANUAL",
            SourceType::Pdf => "PDF",
            SourceType::Docx => "DOCX",
            SourceType::Markdown => "MARKDOWN",
            SourceType::Txt => "TXT",
            SourceType::Csv => "CSV",
            SourceType::Repository => "REPOSITORY",
        };

        sqlx::query(
            "INSERT INTO curricula (id, user_id, title, description, source_type, status, target_career_role, estimated_total_hours, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'ACTIVE', ?, 0.0, ?, ?)"
        )
        .bind(id.to_string())
        .bind(user_id.to_string())
        .bind(title)
        .bind(description)
        .bind(source_str)
        .bind(target_career_role)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&now)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(Curriculum {
            id,
            user_id: *user_id,
            title: title.to_string(),
            description: description.to_string(),
            source_type,
            status: CurriculumStatus::Active,
            target_career_role: target_career_role.map(|s| s.to_string()),
            estimated_total_hours: 0.0,
            created_at,
            updated_at: created_at,
        })
    }

    async fn add_module(
        &self,
        curriculum_id: &CurriculumId,
        title: &str,
        description: &str,
        order_index: u32,
        estimated_hours: f32,
    ) -> Result<CurriculumModule, DbError> {
        let id = ModuleId::new();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO curriculum_modules (id, curriculum_id, title, description, order_index, estimated_hours, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(curriculum_id.to_string())
        .bind(title)
        .bind(description)
        .bind(order_index as i32)
        .bind(estimated_hours as f64)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&now)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(CurriculumModule {
            id,
            curriculum_id: *curriculum_id,
            title: title.to_string(),
            description: description.to_string(),
            order_index,
            estimated_hours,
            created_at,
        })
    }

    async fn add_topic(
        &self,
        module_id: &ModuleId,
        title: &str,
        description: &str,
        order_index: u32,
        estimated_minutes: u32,
        difficulty: DifficultyLevel,
        key_concepts: &[String],
        practical_exercises: &[String],
    ) -> Result<CurriculumTopic, DbError> {
        let id = TopicId::new();
        let now = Utc::now().to_rfc3339();
        let diff_str = match difficulty {
            DifficultyLevel::Beginner => "BEGINNER",
            DifficultyLevel::Intermediate => "INTERMEDIATE",
            DifficultyLevel::Advanced => "ADVANCED",
        };

        let concepts_json = serde_json::to_string(key_concepts)?;
        let exercises_json = serde_json::to_string(practical_exercises)?;

        sqlx::query(
            "INSERT INTO curriculum_topics (id, module_id, title, description, order_index, estimated_minutes, difficulty, key_concepts, practical_exercises, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(module_id.to_string())
        .bind(title)
        .bind(description)
        .bind(order_index as i32)
        .bind(estimated_minutes as i32)
        .bind(diff_str)
        .bind(&concepts_json)
        .bind(&exercises_json)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&now)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(CurriculumTopic {
            id,
            module_id: *module_id,
            title: title.to_string(),
            description: description.to_string(),
            order_index,
            estimated_minutes,
            difficulty,
            key_concepts: key_concepts.to_vec(),
            practical_exercises: practical_exercises.to_vec(),
            created_at,
        })
    }

    async fn add_prerequisite(
        &self,
        topic_id: &TopicId,
        prerequisite_topic_id: &TopicId,
    ) -> Result<(), DbError> {
        sqlx::query(
            "INSERT OR IGNORE INTO topic_prerequisites (topic_id, prerequisite_topic_id) VALUES (?, ?)"
        )
        .bind(topic_id.to_string())
        .bind(prerequisite_topic_id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list_user_curricula(&self, user_id: &UserId) -> Result<Vec<Curriculum>, DbError> {
        let rows = sqlx::query(
            "SELECT id, user_id, title, description, source_type, status, target_career_role, estimated_total_hours, created_at, updated_at FROM curricula WHERE user_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        let mut curricula = Vec::new();
        for r in rows {
            let id_str: String = r.get("id");
            let uuid = Uuid::parse_str(&id_str).map_err(|e| DbError::NotFound(e.to_string()))?;
            let created_str: String = r.get("created_at");
            let updated_str: String = r.get("updated_at");

            curricula.push(Curriculum {
                id: CurriculumId(uuid),
                user_id: *user_id,
                title: r.get("title"),
                description: r.get("description"),
                source_type: SourceType::Manual,
                status: CurriculumStatus::Active,
                target_career_role: r.get("target_career_role"),
                estimated_total_hours: r.get::<f64, _>("estimated_total_hours") as f32,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            });
        }

        Ok(curricula)
    }

    async fn get_curriculum_details(
        &self,
        curriculum_id: &CurriculumId,
    ) -> Result<Option<CurriculumDetails>, DbError> {
        let curr_row = sqlx::query(
            "SELECT id, user_id, title, description, source_type, status, target_career_role, estimated_total_hours, created_at, updated_at FROM curricula WHERE id = ?"
        )
        .bind(curriculum_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        let curr_r = match curr_row {
            Some(r) => r,
            None => return Ok(None),
        };

        let user_id_str: String = curr_r.get("user_id");
        let user_uuid = Uuid::parse_str(&user_id_str).map_err(|e| DbError::NotFound(e.to_string()))?;
        let created_str: String = curr_r.get("created_at");
        let updated_str: String = curr_r.get("updated_at");

        let curriculum = Curriculum {
            id: *curriculum_id,
            user_id: UserId(user_uuid),
            title: curr_r.get("title"),
            description: curr_r.get("description"),
            source_type: SourceType::Manual,
            status: CurriculumStatus::Active,
            target_career_role: curr_r.get("target_career_role"),
            estimated_total_hours: curr_r.get::<f64, _>("estimated_total_hours") as f32,
            created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        };

        // Fetch modules
        let module_rows = sqlx::query(
            "SELECT id, curriculum_id, title, description, order_index, estimated_hours, created_at FROM curriculum_modules WHERE curriculum_id = ? ORDER BY order_index ASC"
        )
        .bind(curriculum_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        let mut modules_with_topics = Vec::new();

        for m_row in module_rows {
            let mod_id_str: String = m_row.get("id");
            let mod_uuid = Uuid::parse_str(&mod_id_str).map_err(|e| DbError::NotFound(e.to_string()))?;
            let mod_id = ModuleId(mod_uuid);
            let mod_created: String = m_row.get("created_at");

            let module = CurriculumModule {
                id: mod_id,
                curriculum_id: *curriculum_id,
                title: m_row.get("title"),
                description: m_row.get("description"),
                order_index: m_row.get::<i32, _>("order_index") as u32,
                estimated_hours: m_row.get::<f64, _>("estimated_hours") as f32,
                created_at: chrono::DateTime::parse_from_rfc3339(&mod_created)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            };

            // Fetch topics for this module
            let topic_rows = sqlx::query(
                "SELECT id, module_id, title, description, order_index, estimated_minutes, difficulty, key_concepts, practical_exercises, created_at FROM curriculum_topics WHERE module_id = ? ORDER BY order_index ASC"
            )
            .bind(mod_id.to_string())
            .fetch_all(&self.pool)
            .await?;

            let mut topics = Vec::new();
            for t_row in topic_rows {
                let t_id_str: String = t_row.get("id");
                let t_uuid = Uuid::parse_str(&t_id_str).map_err(|e| DbError::NotFound(e.to_string()))?;
                let t_created: String = t_row.get("created_at");

                let concepts_str: String = t_row.get("key_concepts");
                let exercises_str: String = t_row.get("practical_exercises");

                topics.push(CurriculumTopic {
                    id: TopicId(t_uuid),
                    module_id: mod_id,
                    title: t_row.get("title"),
                    description: t_row.get("description"),
                    order_index: t_row.get::<i32, _>("order_index") as u32,
                    estimated_minutes: t_row.get::<i32, _>("estimated_minutes") as u32,
                    difficulty: DifficultyLevel::Beginner,
                    key_concepts: serde_json::from_str(&concepts_str).unwrap_or_default(),
                    practical_exercises: serde_json::from_str(&exercises_str).unwrap_or_default(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&t_created)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                });
            }

            modules_with_topics.push(ModuleWithTopics { module, topics });
        }

        // Fetch prerequisites
        let prereq_rows = sqlx::query(
            "SELECT tp.topic_id, tp.prerequisite_topic_id FROM topic_prerequisites tp
             JOIN curriculum_topics t ON t.id = tp.topic_id
             JOIN curriculum_modules m ON m.id = t.module_id
             WHERE m.curriculum_id = ?"
        )
        .bind(curriculum_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        let mut prerequisites = Vec::new();
        for pr in prereq_rows {
            let t_str: String = pr.get("topic_id");
            let p_str: String = pr.get("prerequisite_topic_id");
            if let (Ok(t_u), Ok(p_u)) = (Uuid::parse_str(&t_str), Uuid::parse_str(&p_str)) {
                prerequisites.push(TopicPrerequisite {
                    topic_id: TopicId(t_u),
                    prerequisite_topic_id: TopicId(p_u),
                });
            }
        }

        Ok(Some(CurriculumDetails {
            curriculum,
            modules: modules_with_topics,
            prerequisites,
        }))
    }
}
