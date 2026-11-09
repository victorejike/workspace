use workspace_types::{MasteryLevel, MasteryState};

pub struct MasteryEvaluator;

impl MasteryEvaluator {
    /// Evaluates multi-factor evidence to compute composite score and mastery level.
    /// Never marks a topic mastered solely because time was spent.
    pub fn evaluate(
        conceptual_score: u8,
        practical_code_score: u8,
        retention_score: u8,
        has_practical_evidence: bool,
    ) -> (u8, MasteryLevel, MasteryState) {
        // Weighted formula:
        // Conceptual: 40%
        // Practical Implementation: 40%
        // Retention: 20%
        let composite = ((conceptual_score as f32 * 0.40)
            + (practical_code_score as f32 * 0.40)
            + (retention_score as f32 * 0.20))
            .round() as u8;

        // Strict criteria for MASTERED:
        // Composite must be >= 80 AND practical code score >= 70 AND practical evidence verified
        if composite >= 80 && practical_code_score >= 70 && has_practical_evidence {
            (composite, MasteryLevel::Mastered, MasteryState::Mastered)
        } else if composite >= 50 {
            (composite, MasteryLevel::NeedsRevision, MasteryState::NeedsRevision)
        } else {
            (composite, MasteryLevel::NotMastered, MasteryState::InProgress)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mastered_evaluation() {
        let (composite, level, state) = MasteryEvaluator::evaluate(90, 85, 80, true);
        // (90 * 0.4) + (85 * 0.4) + (80 * 0.2) = 36 + 34 + 16 = 86
        assert_eq!(composite, 86);
        assert_eq!(level, MasteryLevel::Mastered);
        assert_eq!(state, MasteryState::Mastered);
    }

    #[test]
    fn test_needs_revision_due_to_weak_practical() {
        // High conceptual (95), but practical is 60 (below 70 threshold for mastered)
        let (composite, level, state) = MasteryEvaluator::evaluate(95, 60, 70, true);
        // (95 * 0.4) + (60 * 0.4) + (70 * 0.2) = 38 + 24 + 14 = 76
        assert_eq!(composite, 76);
        assert_eq!(level, MasteryLevel::NeedsRevision);
        assert_eq!(state, MasteryState::NeedsRevision);
    }

    #[test]
    fn test_not_mastered_without_practical_evidence() {
        // 100 on everything, but no practical project/code committed
        let (composite, level, state) = MasteryEvaluator::evaluate(100, 100, 100, false);
        // Cannot be MASTERED without practical evidence!
        assert_eq!(composite, 100);
        assert_eq!(level, MasteryLevel::NeedsRevision);
        assert_eq!(state, MasteryState::NeedsRevision);
    }

    #[test]
    fn test_low_score_in_progress() {
        let (composite, level, state) = MasteryEvaluator::evaluate(40, 30, 40, true);
        // (40 * 0.4) + (30 * 0.4) + (40 * 0.2) = 16 + 12 + 8 = 36
        assert_eq!(composite, 36);
        assert_eq!(level, MasteryLevel::NotMastered);
        assert_eq!(state, MasteryState::InProgress);
    }
}
