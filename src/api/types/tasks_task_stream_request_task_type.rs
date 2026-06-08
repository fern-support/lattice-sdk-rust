pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TaskStreamRequestTaskType {
    TaskStreamRequestTaskTypeTaskTypeURLs(TaskStreamRequestTaskTypeTaskTypeURLs),

    TaskStreamRequestTaskTypeTaskTypePrefix(TaskStreamRequestTaskTypeTaskTypePrefix),
}

impl TaskStreamRequestTaskType {
    pub fn is_task_stream_request_task_type_task_type_urls(&self) -> bool {
        matches!(self, Self::TaskStreamRequestTaskTypeTaskTypeURLs(_))
    }

    pub fn is_task_stream_request_task_type_task_type_prefix(&self) -> bool {
        matches!(self, Self::TaskStreamRequestTaskTypeTaskTypePrefix(_))
    }

    pub fn as_task_stream_request_task_type_task_type_urls(
        &self,
    ) -> Option<&TaskStreamRequestTaskTypeTaskTypeURLs> {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypeURLs(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_task_stream_request_task_type_task_type_urls(
        self,
    ) -> Option<TaskStreamRequestTaskTypeTaskTypeURLs> {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypeURLs(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_task_stream_request_task_type_task_type_prefix(
        &self,
    ) -> Option<&TaskStreamRequestTaskTypeTaskTypePrefix> {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypePrefix(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_task_stream_request_task_type_task_type_prefix(
        self,
    ) -> Option<TaskStreamRequestTaskTypeTaskTypePrefix> {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypePrefix(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for TaskStreamRequestTaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypeURLs(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TaskStreamRequestTaskTypeTaskTypePrefix(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
