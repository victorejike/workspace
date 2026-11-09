use workspace_types::JournalEntryData;

pub struct MarkdownJournalGenerator;

impl MarkdownJournalGenerator {
    /// Compiles structured journal data into clean, GitHub-compatible Markdown.
    pub fn generate_daily_report(data: &JournalEntryData) -> String {
        let hours = data.time_spent_minutes / 60;
        let minutes = data.time_spent_minutes % 60;

        let mut doc = String::new();
        doc.push_str(&format!(
            "# Day {:03} - {}\n\n",
            data.day_number, data.topic_title
        ));
        doc.push_str(&format!("**Date:** {}\n\n", data.date));

        doc.push_str("## Learning Objectives\n");
        if data.learning_objectives.is_empty() {
            doc.push_str("- None specified\n");
        } else {
            for obj in &data.learning_objectives {
                doc.push_str(&format!("- {}\n", obj));
            }
        }
        doc.push('\n');

        doc.push_str("## Completed Activities\n");
        if data.completed_activities.is_empty() {
            doc.push_str("- No recorded activities\n");
        } else {
            for act in &data.completed_activities {
                doc.push_str(&format!("- {}\n", act));
            }
        }
        doc.push('\n');

        doc.push_str("## Assessment\n");
        doc.push_str(&format!("- **Status:** {}\n\n", data.assessment_status));

        doc.push_str("## Knowledge Gaps\n");
        if data.knowledge_gaps.is_empty() {
            doc.push_str("- No significant knowledge gaps identified\n");
        } else {
            for gap in &data.knowledge_gaps {
                doc.push_str(&format!("- {}\n", gap));
            }
        }
        doc.push('\n');

        doc.push_str("## Time Spent\n");
        if hours > 0 {
            doc.push_str(&format!("{} hours {} minutes\n\n", hours, minutes));
        } else {
            doc.push_str(&format!("{} minutes\n\n", minutes));
        }

        if !data.achievements.is_empty() {
            doc.push_str("## Achievements\n");
            for ach in &data.achievements {
                doc.push_str(&format!("- {}\n", ach));
            }
            doc.push('\n');
        }

        doc.push_str("## Next Objectives\n");
        if data.next_objectives.is_empty() {
            doc.push_str("- Continue with scheduled curriculum\n");
        } else {
            for next in &data.next_objectives {
                doc.push_str(&format!("- {}\n", next));
            }
        }

        doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_markdown_report_formatting() {
        let entry = JournalEntryData {
            day_number: 1,
            topic_title: "Python Functions".to_string(),
            date: NaiveDate::from_ymd_opt(2026, 11, 9).unwrap(),
            learning_objectives: vec![
                "Understand functions".to_string(),
                "Understand parameters".to_string(),
                "Understand return values".to_string(),
            ],
            completed_activities: vec![
                "Studied function definitions".to_string(),
                "Completed practical exercises".to_string(),
                "Built a calculator".to_string(),
            ],
            assessment_status: "Needs revision".to_string(),
            knowledge_gaps: vec![
                "Variable scope".to_string(),
                "Default parameters".to_string(),
            ],
            time_spent_minutes: 200, // 3h 20m
            achievements: vec!["Built modular calculator in Python".to_string()],
            next_objectives: vec![
                "Revise variable scope".to_string(),
                "Study decorators".to_string(),
            ],
        };

        let md = MarkdownJournalGenerator::generate_daily_report(&entry);
        assert!(md.contains("# Day 001 - Python Functions"));
        assert!(md.contains("**Date:** 2026-11-09"));
        assert!(md.contains("- Understand parameters"));
        assert!(md.contains("- Built a calculator"));
        assert!(md.contains("- **Status:** Needs revision"));
        assert!(md.contains("3 hours 20 minutes"));
        assert!(md.contains("- Revise variable scope"));
    }
}
