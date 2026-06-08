pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TaskStreamRequestTaskTypeTaskTypeURLs {
    /// List of exact task type URLs to match.
    pub task_type_urls: Vec<String>,
}

impl TaskStreamRequestTaskTypeTaskTypeURLs {
    pub fn builder() -> TaskStreamRequestTaskTypeTaskTypeURLsBuilder {
        <TaskStreamRequestTaskTypeTaskTypeURLsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskStreamRequestTaskTypeTaskTypeURLsBuilder {
    task_type_urls: Option<Vec<String>>,
}

impl TaskStreamRequestTaskTypeTaskTypeURLsBuilder {
    pub fn task_type_urls(mut self, value: Vec<String>) -> Self {
        self.task_type_urls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskStreamRequestTaskTypeTaskTypeURLs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_type_urls`](TaskStreamRequestTaskTypeTaskTypeURLsBuilder::task_type_urls)
    pub fn build(self) -> Result<TaskStreamRequestTaskTypeTaskTypeURLs, BuildError> {
        Ok(TaskStreamRequestTaskTypeTaskTypeURLs {
            task_type_urls: self
                .task_type_urls
                .ok_or_else(|| BuildError::missing_field("task_type_urls"))?,
        })
    }
}
