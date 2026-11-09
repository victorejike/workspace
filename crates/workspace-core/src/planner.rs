use chrono::{Duration, NaiveDate};
use workspace_types::ScheduleProjection;

use crate::error::CoreError;

pub struct TimelineCalculator;

impl TimelineCalculator {
    /// Calculates realistic timeline projections.
    /// Never promises an exact completion date, but provides realistic, buffered projections.
    pub fn calculate(
        total_curriculum_hours: f32,
        daily_committed_hours: f32,
        available_days_per_week: u8,
        buffer_percentage: f32,
        start_date: NaiveDate,
    ) -> Result<ScheduleProjection, CoreError> {
        if total_curriculum_hours <= 0.0 {
            return Err(CoreError::InvalidSchedule(
                "Total curriculum hours must be greater than zero".to_string(),
            ));
        }

        if daily_committed_hours <= 0.0 {
            return Err(CoreError::InvalidSchedule(
                "Daily committed hours must be greater than zero".to_string(),
            ));
        }

        if available_days_per_week == 0 || available_days_per_week > 7 {
            return Err(CoreError::InvalidSchedule(
                "Available days per week must be between 1 and 7".to_string(),
            ));
        }

        let buffer = if buffer_percentage < 0.0 { 0.25 } else { buffer_percentage };

        // Theoretical study days (assuming 100% efficiency and every available day used)
        let total_study_days_needed = (total_curriculum_hours / daily_committed_hours).ceil() as u32;

        // Calendar days needed to accumulate those study days:
        let calendar_days_multiplier = 7.0 / (available_days_per_week as f32);
        let theoretical_calendar_days = ((total_study_days_needed as f32) * calendar_days_multiplier).ceil() as u32;

        // Buffered study days (adding revision, difficulty spikes, checkpoint failures)
        let buffered_study_days = ((total_study_days_needed as f32) * (1.0 + buffer)).ceil() as u32;
        let realistic_calendar_days = ((buffered_study_days as f32) * calendar_days_multiplier).ceil() as u32;
        let buffer_days_allocated = realistic_calendar_days.saturating_sub(theoretical_calendar_days);

        let estimated_completion_date = start_date + Duration::days(realistic_calendar_days as i64);

        let pace_warning = if daily_committed_hours > 6.0 {
            Some("Committing more than 6 hours daily can lead to cognitive burnout. We recommend 2-4 hours of focused deep work.".to_string())
        } else if daily_committed_hours < 0.5 {
            Some("Less than 30 minutes daily may significantly slow down retention and momentum.".to_string())
        } else {
            None
        };

        Ok(ScheduleProjection {
            total_curriculum_hours,
            daily_committed_hours,
            theoretical_minimum_days: theoretical_calendar_days,
            realistic_projected_days: realistic_calendar_days,
            estimated_completion_date,
            buffer_days_allocated,
            pace_warning,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_schedule_calculation() {
        let start = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
        // 600 hours curriculum, 3 hours daily, 5 days per week, 25% buffer
        let proj = TimelineCalculator::calculate(600.0, 3.0, 5, 0.25, start).unwrap();

        // 600h / 3h/day = 200 study days.
        // Calendar multiplier: 7/5 = 1.4 -> 200 * 1.4 = 280 calendar days theoretical.
        assert_eq!(proj.theoretical_minimum_days, 280);

        // Buffered: 200 * 1.25 = 250 study days -> 250 * 1.4 = 350 calendar days.
        assert_eq!(proj.realistic_projected_days, 350);
        assert_eq!(proj.buffer_days_allocated, 70);
        assert!(proj.pace_warning.is_none());
    }

    #[test]
    fn test_unrealistic_hours_warning() {
        let start = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
        let proj = TimelineCalculator::calculate(100.0, 8.0, 7, 0.25, start).unwrap();
        assert!(proj.pace_warning.is_some());
        assert!(proj.pace_warning.unwrap().contains("cognitive burnout"));
    }

    #[test]
    fn test_zero_daily_hours_error() {
        let start = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
        let result = TimelineCalculator::calculate(100.0, 0.0, 5, 0.25, start);
        assert!(result.is_err());
    }
}
