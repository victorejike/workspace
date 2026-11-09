use std::collections::{HashMap, HashSet};
use workspace_types::{CurriculumTopic, TopicId, TopicPrerequisite};

use crate::error::CoreError;

pub struct DependencyGraph;

impl DependencyGraph {
    /// Validates that topic prerequisites form a valid Directed Acyclic Graph (DAG) with no cycles.
    pub fn validate_no_cycles(
        topics: &[CurriculumTopic],
        prerequisites: &[TopicPrerequisite],
    ) -> Result<(), CoreError> {
        let topic_map: HashMap<TopicId, &CurriculumTopic> =
            topics.iter().map(|t| (t.id, t)).collect();

        // Build adjacency list: topic -> list of prerequisites
        let mut adj: HashMap<TopicId, Vec<TopicId>> = HashMap::new();
        for prereq in prerequisites {
            if !topic_map.contains_key(&prereq.topic_id) {
                return Err(CoreError::TopicNotFound(prereq.topic_id.to_string()));
            }
            if !topic_map.contains_key(&prereq.prerequisite_topic_id) {
                return Err(CoreError::TopicNotFound(
                    prereq.prerequisite_topic_id.to_string(),
                ));
            }
            adj.entry(prereq.topic_id)
                .or_default()
                .push(prereq.prerequisite_topic_id);
        }

        // Cycle detection using 3-color DFS (0 = unvisited, 1 = visiting, 2 = visited)
        let mut state: HashMap<TopicId, u8> = HashMap::new();

        fn dfs(
            node: TopicId,
            adj: &HashMap<TopicId, Vec<TopicId>>,
            state: &mut HashMap<TopicId, u8>,
            topic_map: &HashMap<TopicId, &CurriculumTopic>,
        ) -> Result<(), CoreError> {
            state.insert(node, 1); // visiting

            if let Some(neighbors) = adj.get(&node) {
                for &next in neighbors {
                    let st = state.get(&next).copied().unwrap_or(0);
                    if st == 1 {
                        let name = topic_map
                            .get(&next)
                            .map(|t| t.title.as_str())
                            .unwrap_or("unknown");
                        return Err(CoreError::DependencyCycle(name.to_string()));
                    }
                    if st == 0 {
                        dfs(next, adj, state, topic_map)?;
                    }
                }
            }

            state.insert(node, 2); // visited
            Ok(())
        }

        for topic in topics {
            if state.get(&topic.id).copied().unwrap_or(0) == 0 {
                dfs(topic.id, &adj, &mut state, &topic_map)?;
            }
        }

        Ok(())
    }

    /// Determines which topics are currently unlocked based on completed topics.
    pub fn get_unlocked_topics(
        topics: &[CurriculumTopic],
        prerequisites: &[TopicPrerequisite],
        completed_topics: &HashSet<TopicId>,
    ) -> Vec<TopicId> {
        let mut prereq_map: HashMap<TopicId, Vec<TopicId>> = HashMap::new();
        for prereq in prerequisites {
            prereq_map
                .entry(prereq.topic_id)
                .or_default()
                .push(prereq.prerequisite_topic_id);
        }

        topics
            .iter()
            .filter(|t| !completed_topics.contains(&t.id))
            .filter(|t| {
                if let Some(reqs) = prereq_map.get(&t.id) {
                    reqs.iter().all(|r| completed_topics.contains(r))
                } else {
                    true // No prerequisites, unlocked by default
                }
            })
            .map(|t| t.id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use workspace_types::{DifficultyLevel, ModuleId};

    fn make_topic(title: &str) -> CurriculumTopic {
        CurriculumTopic {
            id: TopicId::new(),
            module_id: ModuleId::new(),
            title: title.to_string(),
            description: format!("Description of {}", title),
            order_index: 0,
            estimated_minutes: 60,
            difficulty: DifficultyLevel::Beginner,
            key_concepts: vec![],
            practical_exercises: vec![],
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_valid_dag() {
        let t1 = make_topic("Topic A");
        let t2 = make_topic("Topic B");
        let t3 = make_topic("Topic C");

        let prerequisites = vec![
            TopicPrerequisite {
                topic_id: t2.id,
                prerequisite_topic_id: t1.id,
            },
            TopicPrerequisite {
                topic_id: t3.id,
                prerequisite_topic_id: t2.id,
            },
        ];

        let result = DependencyGraph::validate_no_cycles(&[t1, t2, t3], &prerequisites);
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_cycle() {
        let t1 = make_topic("Topic A");
        let t2 = make_topic("Topic B");

        let prerequisites = vec![
            TopicPrerequisite {
                topic_id: t2.id,
                prerequisite_topic_id: t1.id,
            },
            TopicPrerequisite {
                topic_id: t1.id,
                prerequisite_topic_id: t2.id,
            },
        ];

        let result = DependencyGraph::validate_no_cycles(&[t1, t2], &prerequisites);
        assert!(result.is_err());
        match result.unwrap_err() {
            CoreError::DependencyCycle(name) => {
                assert!(name == "Topic A" || name == "Topic B");
            }
            other => panic!("Expected DependencyCycle, got {:?}", other),
        }
    }

    #[test]
    fn test_unlocked_topics() {
        let t1 = make_topic("Basics");
        let t2 = make_topic("Intermediate");
        let t3 = make_topic("Independent Elective");

        let prerequisites = vec![TopicPrerequisite {
            topic_id: t2.id,
            prerequisite_topic_id: t1.id,
        }];

        let mut completed = HashSet::new();

        // Initially: Basics and Independent Elective are unlocked, Intermediate is locked
        let unlocked =
            DependencyGraph::get_unlocked_topics(&[t1.clone(), t2.clone(), t3.clone()], &prerequisites, &completed);
        assert_eq!(unlocked.len(), 2);
        assert!(unlocked.contains(&t1.id));
        assert!(unlocked.contains(&t3.id));
        assert!(!unlocked.contains(&t2.id));

        // After completing Basics: Intermediate unlocks
        completed.insert(t1.id);
        let unlocked_after =
            DependencyGraph::get_unlocked_topics(&[t1.clone(), t2.clone(), t3.clone()], &prerequisites, &completed);
        assert_eq!(unlocked_after.len(), 2);
        assert!(unlocked_after.contains(&t2.id));
        assert!(unlocked_after.contains(&t3.id));
    }
}
