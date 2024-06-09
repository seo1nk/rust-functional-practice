use crate::{
    repository_impl::RepositoryImpl, something_model::base_something_model::BaseSomethingModel,
};
use domain::{
    id::Id,
    something::base_something::{self, BaseSomething},
    something_repository::base_something_repository::{self, BaseSomethingRepository},
};

use sqlx::{MySql, MySqlPool, Pool};

// impl BaseSomethingRepository for RepositoryImpl<BaseSomething> {
//     async fn create(&self, base: &BaseSomething) -> Result<(), RepositoryError> {}
// }
