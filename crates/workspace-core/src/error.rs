use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CoreError {
    #[error("Invalid schedule parameters: {0}")]
    InvalidSchedule(String),

    #[error("Curriculum dependency cycle detected involving topic: {0}")]
    DependencyCycle(String),

    #[error("Topic not found: {0}")]
    TopicNotFound(String),

    #[error("Prerequisite not satisfied: topic '{0}' requires '{1}'")]
    PrerequisiteNotSatisfied(String, String),

    #[error("Invalid calculation data: {0}")]
    InvalidData(String),
}
