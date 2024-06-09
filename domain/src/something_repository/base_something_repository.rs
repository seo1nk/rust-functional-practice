use crate::something::base_something::BaseSomething;

use super::super::id::Id;
use anyhow;
use async_trait::async_trait;

type RepositoryError = anyhow::Error;

#[async_trait]
pub trait BaseSomethingRepository: Send + Sync + 'static {
    async fn create(&self, base: &BaseSomething) -> Result<(), RepositoryError>;
    async fn find_by_id(
        &self,
        base_id: &Id<BaseSomething>,
    ) -> Result<BaseSomething, RepositoryError>;
}
