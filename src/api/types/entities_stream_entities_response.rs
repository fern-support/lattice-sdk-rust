pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum StreamEntitiesResponse {
    #[serde(rename = "heartbeat")]
    #[non_exhaustive]
    Heartbeat {},

    #[serde(rename = "entity")]
    #[non_exhaustive]
    Entity {},
}

impl StreamEntitiesResponse {
    pub fn heartbeat() -> Self {
        Self::Heartbeat {}
    }

    pub fn entity() -> Self {
        Self::Entity {}
    }
}
