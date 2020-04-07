use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ApiResource {
    type ResourceType : Sized;

    fn url(&self) -> String;

    async fn get(&self) -> Result<Self::ResourceType>;
}

#[async_trait]
pub trait ApiNamedResource : ApiResource {
    fn name(&self) -> String;
}

#[async_trait]
pub trait ApiResourceList {
    type ResourceType : Sized;

    fn count(&self) -> usize;

    async fn next_list(&self) -> Result<Option<Box<dyn ApiResourceList<ResourceType = Self::ResourceType>>>>;
    async fn previous_list(&self) -> Result<Option<Box<dyn ApiResourceList<ResourceType = Self::ResourceType>>>>;

    fn resources(&self) -> Vec<Box<dyn ApiResource<ResourceType = Self::ResourceType>>>;
}

#[async_trait]
pub trait ApiNamedResourceList {
    type ResourceType : Sized;

    fn count(&self) -> usize;

    async fn next_list(&self) -> Result<Option<Box<dyn ApiNamedResourceList<ResourceType = Self::ResourceType>>>>;
    async fn previous_list(&self) -> Result<Option<Box<dyn ApiNamedResourceList<ResourceType = Self::ResourceType>>>>;

    fn resources(&self) -> Vec<Box<dyn ApiNamedResource<ResourceType = Self::ResourceType>>>;
}

macro_rules! decl_resource {
    ($res:tt for $model:tt) => {

        #[derive(Clone)]
        pub struct $res {
            client: $crate::api::v2::client::ApiClient,
            resource: $crate::models::v2::resource::Resource,
        }

        impl $res {
            pub(crate) fn new(
                client: $crate::api::v2::client::ApiClient,
                resource: $crate::models::v2::resource::Resource
            ) -> Self {
                Self {
                    client,
                    resource,
                }
            }
        }

        #[async_trait::async_trait]
        impl $crate::api::v2::resource::ApiResource for $res {
            type ResourceType = $model;

            fn url(&self) -> String {
                self.resource.url.clone()
            }

            async fn get(&self) -> Result<Self::ResourceType> {
                Ok(self.client.request(self.url())
                    .await?
                    .json::<Self::ResourceType>()
                    .await?)
            }
        }
    };
}

macro_rules! decl_named_resource {
    ($res:tt for $model:tt) => {

        #[derive(Clone)]
        pub struct $res {
            client: $crate::api::v2::client::ApiClient,
            resource: $crate::models::v2::resource::NamedResource,
        }

        impl $res {
            pub(crate) fn new(
                client: $crate::api::v2::client::ApiClient,
                resource: $crate::models::v2::resource::NamedResource
            ) -> Self {
                Self {
                    client,
                    resource,
                }
            }
        }

        #[async_trait::async_trait]
        impl $crate::api::v2::resource::ApiResource for $res {
            type ResourceType = $model;

            fn url(&self) -> String {
                self.resource.url.clone()
            }

            async fn get(&self) -> Result<Self::ResourceType> {
                Ok(self.client.request(self.url())
                    .await?
                    .json::<Self::ResourceType>()
                    .await?)
            }
        }

        impl $crate::api::v2::resource::ApiNamedResource for $res {
            fn name(&self) -> String { self.resource.name.clone() }
        }
    };
}

macro_rules! decl_resource_list {
    ($list:tt for $model:tt with $res:tt) => {
        pub struct $list {
            client: $crate::api::v2::client::ApiClient,
            resource_list: $crate::models::v2::resource::ResourceList,
        }
        
        impl $list {
            pub(crate) fn new(
                client: $crate::api::v2::client::ApiClient,
                resource_list: $crate::models::v2::resource::ResourceList
            ) -> Self {
                Self {
                    client,
                    resource_list,
                }
            }
        }

        #[async_trait::async_trait]
        impl $crate::api::v2::resource::ApiResourceList for $list {
            type ResourceType = $model;

            fn count(&self) -> usize { self.resource_list.count }

            async fn next_list(&self) ->
            Result<Option<Box<dyn $crate::api::v2::resource::ApiResourceList<ResourceType = Self::ResourceType>>>> {
                match self.resource_list.next {
                    None => Ok(None),
                    Some(ref url) => {
                        Ok(Some(Box::new(Self::new(
                            self.client.clone(),
                            self.client
                                .request(url)
                                .await?
                                .json::<$crate::models::v2::resource::ResourceList>()
                                .await?
                        ))))
                    }
                }
            }

            async fn previous_list(&self) ->
            Result<Option<Box<dyn $crate::api::v2::resource::ApiResourceList<ResourceType = Self::ResourceType>>>> {
                match self.resource_list.previous {
                    None => Ok(None),
                    Some(ref url) => {
                        Ok(Some(Box::new(Self::new(
                            self.client.clone(),
                            self.client
                                .request(url)
                                .await?
                                .json::<$crate::models::v2::resource::ResourceList>()
                                .await?
                        ))))
                    }
                }
            }
            
            fn resources(&self) -> Vec<Box<dyn ApiResource<ResourceType = Self::ResourceType>>> {
                let mut resources = Vec::<Box<dyn ApiResource<ResourceType = Self::ResourceType>>>::new();
                for res in self.resource_list.results.iter() {
                    resources.push(Box::new($res::new(self.client.clone(),res.clone())));
                }

                resources
            }    
        }
    };
}

macro_rules! decl_named_resource_list {
    ($list:tt for $model:tt with $res:tt) => {
        pub struct $list {
            client: $crate::api::v2::client::ApiClient,
            resource_list: $crate::models::v2::resource::NamedResourceList,
        }
        
        impl $list {
            pub(crate) fn new(
                client: $crate::api::v2::client::ApiClient,
                resource_list: $crate::models::v2::resource::NamedResourceList
            ) -> Self {
                Self {
                    client,
                    resource_list,
                }
            }
        }

        #[async_trait::async_trait]
        impl $crate::api::v2::resource::ApiNamedResourceList for $list {
            type ResourceType = $model;

            fn count(&self) -> usize { self.resource_list.count }

            async fn next_list(&self) ->
            Result<Option<Box<dyn $crate::api::v2::resource::ApiNamedResourceList<ResourceType = Self::ResourceType>>>> {
                match self.resource_list.next {
                    None => Ok(None),
                    Some(ref url) => {
                        Ok(Some(Box::new(Self::new(
                            self.client.clone(),
                            self.client
                                .request(url)
                                .await?
                                .json::<$crate::models::v2::resource::NamedResourceList>()
                                .await?
                        ))))
                    }
                }
            }

            async fn previous_list(&self) ->
            Result<Option<Box<dyn $crate::api::v2::resource::ApiNamedResourceList<ResourceType = Self::ResourceType>>>> {
                match self.resource_list.previous {
                    None => Ok(None),
                    Some(ref url) => {
                        Ok(Some(Box::new(Self::new(
                            self.client.clone(),
                            self.client
                                .request(url)
                                .await?
                                .json::<$crate::models::v2::resource::NamedResourceList>()
                                .await?
                        ))))
                    }
                }
            }
            
            fn resources(&self) -> Vec<Box<dyn ApiNamedResource<ResourceType = Self::ResourceType>>> {
                let mut resources = Vec::<Box<dyn ApiNamedResource<ResourceType = Self::ResourceType>>>::new();
                for res in self.resource_list.results.iter() {
                    resources.push(Box::new($res::new(self.client.clone(),res.clone())));
                }

                resources
            }    
        }
    };
}

