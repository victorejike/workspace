use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use workspace_types::{
    ExperienceLevel, ThemePreference, User, UserId, UserPreferences, UserProfile,
};

use crate::error::DbError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(
        &self,
        email: &str,
        password_hash: &str,
        display_name: &str,
    ) -> Result<User, DbError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DbError>;
    async fn get_profile(&self, user_id: &UserId) -> Result<Option<UserProfile>, DbError>;
    async fn get_preferences(&self, user_id: &UserId) -> Result<Option<UserPreferences>, DbError>;
    async fn update_profile(
        &self,
        user_id: &UserId,
        display_name: &str,
        experience_level: ExperienceLevel,
        career_goal: &str,
    ) -> Result<(), DbError>;
    async fn update_preferences(
        &self,
        user_id: &UserId,
        theme: ThemePreference,
        preferred_daily_hours: f32,
        available_days: u8,
    ) -> Result<(), DbError>;
}

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(
        &self,
        email: &str,
        password_hash: &str,
        display_name: &str,
    ) -> Result<User, DbError> {
        let user_id = UserId::new();
        let now = Utc::now().to_rfc3339();

        let mut tx = self.pool.begin().await?;

        // 1. Insert user
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, is_active, created_at, updated_at) VALUES (?, ?, ?, 1, ?, ?)"
        )
        .bind(user_id.to_string())
        .bind(email)
        .bind(password_hash)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await?;

        // 2. Insert profile
        sqlx::query(
            "INSERT INTO user_profiles (user_id, display_name, timezone, experience_level, career_goal, target_roles, created_at, updated_at) VALUES (?, ?, 'UTC', 'BEGINNER', '', '[]', ?, ?)"
        )
        .bind(user_id.to_string())
        .bind(display_name)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await?;

        // 3. Insert default preferences
        sqlx::query(
            "INSERT INTO user_preferences (user_id, theme, preferred_daily_hours, available_days_per_week, idle_timeout_minutes, notifications_enabled, updated_at) VALUES (?, 'DARK', 2.0, 5, 5, 1, ?)"
        )
        .bind(user_id.to_string())
        .bind(&now)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&now)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(User {
            id: user_id,
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            is_active: true,
            created_at,
            updated_at: created_at,
        })
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DbError> {
        let row = sqlx::query(
            "SELECT id, email, password_hash, is_active, created_at, updated_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            let id_str: String = r.get("id");
            let uuid = Uuid::parse_str(&id_str).map_err(|e| DbError::NotFound(e.to_string()))?;
            let created_at_str: String = r.get("created_at");
            let updated_at_str: String = r.get("updated_at");

            Ok(Some(User {
                id: UserId(uuid),
                email: r.get("email"),
                password_hash: r.get("password_hash"),
                is_active: r.get::<i32, _>("is_active") == 1,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DbError> {
        let row = sqlx::query(
            "SELECT id, email, password_hash, is_active, created_at, updated_at FROM users WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            let id_str: String = r.get("id");
            let uuid = Uuid::parse_str(&id_str).map_err(|e| DbError::NotFound(e.to_string()))?;
            let created_at_str: String = r.get("created_at");
            let updated_at_str: String = r.get("updated_at");

            Ok(Some(User {
                id: UserId(uuid),
                email: r.get("email"),
                password_hash: r.get("password_hash"),
                is_active: r.get::<i32, _>("is_active") == 1,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_profile(&self, user_id: &UserId) -> Result<Option<UserProfile>, DbError> {
        let row = sqlx::query(
            "SELECT user_id, display_name, avatar_url, timezone, experience_level, career_goal, target_roles, created_at, updated_at FROM user_profiles WHERE user_id = ?"
        )
        .bind(user_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            let exp_str: String = r.get("experience_level");
            let exp_level = match exp_str.as_str() {
                "INTERMEDIATE" => ExperienceLevel::Intermediate,
                "ADVANCED" => ExperienceLevel::Advanced,
                _ => ExperienceLevel::Beginner,
            };

            let roles_json: String = r.get("target_roles");
            let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();
            let created_str: String = r.get("created_at");
            let updated_str: String = r.get("updated_at");

            Ok(Some(UserProfile {
                user_id: *user_id,
                display_name: r.get("display_name"),
                avatar_url: r.get("avatar_url"),
                timezone: r.get("timezone"),
                experience_level: exp_level,
                career_goal: r.get("career_goal"),
                target_roles: roles,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_preferences(&self, user_id: &UserId) -> Result<Option<UserPreferences>, DbError> {
        let row = sqlx::query(
            "SELECT user_id, theme, preferred_daily_hours, available_days_per_week, quiet_hours_start, quiet_hours_end, idle_timeout_minutes, notifications_enabled, updated_at FROM user_preferences WHERE user_id = ?"
        )
        .bind(user_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            let theme_str: String = r.get("theme");
            let theme = match theme_str.as_str() {
                "LIGHT" => ThemePreference::Light,
                "SYSTEM" => ThemePreference::System,
                _ => ThemePreference::Dark,
            };
            let updated_str: String = r.get("updated_at");

            Ok(Some(UserPreferences {
                user_id: *user_id,
                theme,
                preferred_daily_hours: r.get::<f64, _>("preferred_daily_hours") as f32,
                available_days_per_week: r.get::<i32, _>("available_days_per_week") as u8,
                quiet_hours_start: r.get("quiet_hours_start"),
                quiet_hours_end: r.get("quiet_hours_end"),
                idle_timeout_minutes: r.get::<i32, _>("idle_timeout_minutes") as u32,
                notifications_enabled: r.get::<i32, _>("notifications_enabled") == 1,
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_profile(
        &self,
        user_id: &UserId,
        display_name: &str,
        experience_level: ExperienceLevel,
        career_goal: &str,
    ) -> Result<(), DbError> {
        let exp_str = match experience_level {
            ExperienceLevel::Beginner => "BEGINNER",
            ExperienceLevel::Intermediate => "INTERMEDIATE",
            ExperienceLevel::Advanced => "ADVANCED",
        };
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE user_profiles SET display_name = ?, experience_level = ?, career_goal = ?, updated_at = ? WHERE user_id = ?"
        )
        .bind(display_name)
        .bind(exp_str)
        .bind(career_goal)
        .bind(&now)
        .bind(user_id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_preferences(
        &self,
        user_id: &UserId,
        theme: ThemePreference,
        preferred_daily_hours: f32,
        available_days: u8,
    ) -> Result<(), DbError> {
        let theme_str = match theme {
            ThemePreference::Dark => "DARK",
            ThemePreference::Light => "LIGHT",
            ThemePreference::System => "SYSTEM",
        };
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE user_preferences SET theme = ?, preferred_daily_hours = ?, available_days_per_week = ?, updated_at = ? WHERE user_id = ?"
        )
        .bind(theme_str)
        .bind(preferred_daily_hours as f64)
        .bind(available_days as i32)
        .bind(&now)
        .bind(user_id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
