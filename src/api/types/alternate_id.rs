pub use crate::prelude::*;

/// An alternate id for an Entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlternateID {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AlternateIDType>,
}

impl AlternateID {
    pub fn builder() -> AlternateIDBuilder {
        <AlternateIDBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlternateIDBuilder {
    id: Option<String>,
    r#type: Option<AlternateIDType>,
}

impl AlternateIDBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AlternateIDType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AlternateID`].
    pub fn build(self) -> Result<AlternateID, BuildError> {
        Ok(AlternateID {
            id: self.id,
            r#type: self.r#type,
        })
    }
}
