pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum StreamTasksResponse {
    #[serde(rename = "heartbeat")]
    #[non_exhaustive]
    Heartbeat {
        #[serde(flatten)]
        data: StreamHeartbeat,
    },

    #[serde(rename = "task_event")]
    #[non_exhaustive]
    TaskEvent {},
}

impl StreamTasksResponse {
    pub fn heartbeat(data: StreamHeartbeat) -> Self {
        Self::Heartbeat { data }
    }

    pub fn task_event() -> Self {
        Self::TaskEvent {}
    }
}
