pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
pub enum StreamManualControlFramesResponse {
    #[serde(rename = "heartbeat")]
    #[non_exhaustive]
    Heartbeat {
        #[serde(flatten)]
        data: StreamHeartbeat,
    },

    #[serde(rename = "manual_control_frame")]
    #[non_exhaustive]
    ManualControlFrame {},
}

impl StreamManualControlFramesResponse {
    pub fn heartbeat(data: StreamHeartbeat) -> Self {
        Self::Heartbeat { data }
    }

    pub fn manual_control_frame() -> Self {
        Self::ManualControlFrame {}
    }
}
