pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityIDsSelector {
    /// Receive tasks as an assignee for one or more of the supplied entity ids.
    #[serde(rename = "entityIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_ids: Option<Vec<String>>,
}

impl EntityIDsSelector {
    pub fn builder() -> EntityIDsSelectorBuilder {
        <EntityIDsSelectorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityIDsSelectorBuilder {
    entity_ids: Option<Vec<String>>,
}

impl EntityIDsSelectorBuilder {
    pub fn entity_ids(mut self, value: Vec<String>) -> Self {
        self.entity_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityIDsSelector`].
    pub fn build(self) -> Result<EntityIDsSelector, BuildError> {
        Ok(EntityIDsSelector {
            entity_ids: self.entity_ids,
        })
    }
}
