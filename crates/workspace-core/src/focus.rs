pub struct FocusScorer;

impl FocusScorer {
    /// Calculates Attention Quality percentage: active_time / (active_time + idle_time).
    pub fn calculate_attention_quality(active_seconds: u32, idle_seconds: u32) -> u8 {
        let total = active_seconds + idle_seconds;
        if total == 0 {
            return 100;
        }
        let ratio = (active_seconds as f32 / total as f32) * 100.0;
        ratio.clamp(0.0, 100.0).round() as u8
    }

    /// Calculates Focus Stability score based on uninterrupted deep work streaks.
    pub fn calculate_focus_stability(active_seconds: u32, pause_count: u32) -> u8 {
        if active_seconds == 0 {
            return 0;
        }

        // Higher active duration increases score, frequent pauses diminish stability
        let minutes = active_seconds / 60;
        let base_score = match minutes {
            0..=15 => 40.0,
            16..=30 => 60.0,
            31..=60 => 80.0,
            _ => 95.0,
        };

        let penalty = (pause_count as f32) * 8.0;
        let final_score = (base_score - penalty).clamp(10.0, 100.0);
        final_score.round() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attention_quality_calculation() {
        // 50m active, 10m idle -> 50 / 60 = 83.3% -> 83%
        let quality = FocusScorer::calculate_attention_quality(50 * 60, 10 * 60);
        assert_eq!(quality, 83);

        // 0 idle -> 100%
        let perfect = FocusScorer::calculate_attention_quality(3600, 0);
        assert_eq!(perfect, 100);
    }

    #[test]
    fn test_focus_stability_with_pauses() {
        // 45m active with 1 pause
        let score = FocusScorer::calculate_focus_stability(45 * 60, 1);
        // Base 80 - 8 = 72
        assert_eq!(score, 72);

        // 45m active with 5 pauses
        let score_fragmented = FocusScorer::calculate_focus_stability(45 * 60, 5);
        // Base 80 - 40 = 40
        assert_eq!(score_fragmented, 40);
    }
}
