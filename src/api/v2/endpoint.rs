use super::client::ApiClient;
use crate::api::v2::resource::ApiNamedResourceList;
use crate::models::v2::resource::NamedResourceList;
use crate::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// Trait for accessing API endpoint.
/// Encapsulates common methods.
#[async_trait]
pub trait ApiEndpoint {
    /// Type of the model the API gets access to.
    /// It should be deserializable.
    type Model: Sized + DeserializeOwned;

    /// Type of the named resource.
    type NamedResourceList: Sized + ApiNamedResourceList;

    /// Getter method to access the client object reference.
    fn client(&self) -> &ApiClient;

    /// Method to get the API name.
    fn name() -> &'static str;

    /// Helper method to create an `ApiNamedResourceList` based on `NamedResourceList` model.
    fn create_named_resource_list(&self, model: NamedResourceList) -> Self::NamedResourceList;

    /// Gets the resource list of objets for the API endpoint.
    /// By default returned list will be paginated and contain up to 20 resources.
    /// For more details see ([PokeApi/ResourceList](https://pokeapi.co/docs/v2.html#resource-lists-section))
    async fn all(&self) -> Result<Self::NamedResourceList> {
        let res_list = self
            .client()
            .request_api_object::<NamedResourceList, _>(Self::name())
            .await?;

        Ok(self.create_named_resource_list(res_list))
    }

    /// Gets paginated resource list of objects for the API endpoint.
    /// Two parameters `offset` and `limit` are used for pagination
    async fn all_paginated(&self, offset: usize, limit: usize) -> Result<Self::NamedResourceList> {
        let res_list = self
            .client()
            .request_api_object_paginated::<NamedResourceList, _>(Self::name(), offset, limit)
            .await?;

        Ok(self.create_named_resource_list(res_list))
    }

    /// Gets and object by its id.
    /// For example, given the id `3` will result in the following request
    /// `https://pokeapi.co/api/v2/{endpoint-name}/3`
    async fn get_by_id(&self, id: usize) -> Result<Self::Model> {
        self.client()
            .request_api_object::<Self::Model, _>(format!("{}/{}", Self::name(), id))
            .await
    }

    /// Gets a resource by its name.
    /// For example, given the name `name` will result in the following request
    /// `https://pokeapi.com/api/v2/{endpoint-name}/name`
    async fn get_by_name<T: Into<String> + Send>(&self, name: T) -> Result<Self::Model> {
        self.client()
            .request_api_object::<Self::Model, _>(format!(
                "{}/{}",
                Self::name(),
                name.into().as_str()
            ))
            .await
    }
}
