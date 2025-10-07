use std::fmt;

use serde::{Deserialize, Serialize};
#[derive(Default,Debug, Clone, Serialize, Deserialize,PartialEq)]
pub enum QuestStatuses {
    #[default]
    Open,
    InJourney,
    Complete,
    Failed,
}
impl fmt::Display for QuestStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestStatuses::Open => write!(f, "Open"),
            QuestStatuses::InJourney => write!(f, "InJourney"),
            QuestStatuses::Complete => write!(f, "Complete"),
            QuestStatuses::Failed => write!(f, "Failed"),
            
        }
    }
}