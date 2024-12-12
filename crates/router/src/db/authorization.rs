

#[async_trait::async_trait]
pub trait AuthorizationInterface {

    async fn insert_authorization(
        &self,
        authorization: storage::AuthorizationNew,
    ) -> CustomResult<storage::Authorization, error::StorageError>;


}