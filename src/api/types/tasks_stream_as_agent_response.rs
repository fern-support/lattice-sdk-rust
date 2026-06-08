pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum StreamAsAgentResponse {
    #[serde(rename = "heartbeat")]
    #[non_exhaustive]
    Heartbeat {
        #[serde(flatten)]
        data: StreamHeartbeat,
    },

    #[serde(rename = "agent_request")]
    #[non_exhaustive]
    AgentRequest {},
}

impl StreamAsAgentResponse {
    pub fn heartbeat(data: StreamHeartbeat) -> Self {
        Self::Heartbeat { data }
    }

    pub fn agent_request() -> Self {
        Self::AgentRequest {}
    }
}
