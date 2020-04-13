use super::client::ApiClient;
use super::endpoint::ApiEndpoint;
use super::resource::*;
use crate::models::v2::berries::Berry;
use crate::models::v2::resource::NamedResourceList;
use crate::Result;

/// Berries API endpoint. For details see [pokeapi/berries](https://pokeapi.co/docs/v2.html#berries-section)
/// Represents the endpoint `https://pokeapi.co/api/v2/berry`
#[derive(Clone)]
pub struct BerryApiEndpoint {
    client: ApiClient,
}

impl ApiEndpoint for BerryApiEndpoint {
    type Model = Berry;
    type NamedResourceList = BerryNamedResourceList;

    fn client(&self) -> &ApiClient {
        &self.client
    }

    fn name() -> &'static str {
        "berry"
    }

    fn create_named_resource_list(&self, model: NamedResourceList) -> Self::NamedResourceList {
        BerryNamedResourceList::new(self.client.clone(), model)
    }
}

impl BerryApiEndpoint {
    /// Creates API Endpoint object
    pub(crate) fn new(client: ApiClient) -> Self {
        Self { client }
    }
}

decl_resource!(BerryResource for Berry);
decl_named_resource!(BerryNamedResource for Berry);
decl_resource_list!(BerryResourceList for Berry with BerryResource);
decl_named_resource_list!(BerryNamedResourceList for Berry with BerryNamedResource);

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use crate::api::v2::berries::{
        BerryApiEndpoint, BerryNamedResource, BerryNamedResourceList, BerryResource,
    };
    use crate::api::v2::client::ApiClient;
    use crate::api::v2::endpoint::ApiEndpoint;
    use crate::api::v2::resource::{ApiNamedResourceList, ApiResource};
    use crate::models::v2::resource::{NamedResource, NamedResourceList, Resource};

    #[tokio::test]
    async fn berry_resource() {
        let client = ApiClient::new().unwrap();
        let resource = Resource {
            url: "https://pokeapi.co/api/v2/berry/1".into(),
        };

        let berry_resource = BerryResource::new(client, resource);

        let berry = berry_resource.get().await;

        assert_eq!(berry.is_ok(), true);

        let berry = berry.unwrap();
        assert_eq!(berry.id, 1);
        assert_eq!(berry.name, "cheri");
    }

    #[tokio::test]
    async fn berry_named_resource() {
        let client = ApiClient::new().unwrap();
        let named_resource = NamedResource {
            name: "cheri".into(),
            url: "https://pokeapi.co/api/v2/berry/1".into(),
        };

        let named_berry_resource = BerryNamedResource::new(client.clone(), named_resource);

        let berry = named_berry_resource.get().await;

        assert_eq!(berry.is_ok(), true);
    }

    #[tokio::test]
    async fn berry_named_resource_list() {
        let client = ApiClient::new().unwrap();

        let resource_list = NamedResourceList {
            count: 64,
            next: Some("https://pokeapi.co/api/v2/berry?offset=5&limit=5".into()),
            previous: None,
            results: vec![
                NamedResource {
                    name: "cheri".into(),
                    url: "https://pokeapi.co/api/v2/berry/1/".into(),
                },
                NamedResource {
                    name: "chesto".into(),
                    url: "https://pokeapi.co/api/v2/berry/2/".into(),
                },
                NamedResource {
                    name: "pecha".into(),
                    url: "https://pokeapi.co/api/v2/berry/3/".into(),
                },
                NamedResource {
                    name: "rawst".into(),
                    url: "https://pokeapi.co/api/v2/berry/4/".into(),
                },
                NamedResource {
                    name: "aspear".into(),
                    url: "https://pokeapi.co/api/v2/berry/5/".into(),
                },
            ],
        };

        let berry_named_resource_list = BerryNamedResourceList::new(client, resource_list.clone());

        assert_eq!(berry_named_resource_list.count(), 64);

        let previous_list = berry_named_resource_list.previous_list().await;
        assert_eq!(previous_list.is_ok(), true);
        assert_eq!(previous_list.unwrap().is_none(), true);

        let next_list = berry_named_resource_list.next_list().await;
        assert_eq!(next_list.is_ok(), true);
        assert_eq!(next_list.unwrap().is_some(), true);

        let resources = berry_named_resource_list.resources();

        for (idx, resource) in resources.iter().enumerate() {
            assert_eq!(resource.name(), resource_list.results[idx].name);
            assert_eq!(resource.url(), resource_list.results[idx].url);

            let berry_resource = resource.get().await.unwrap();
            assert_eq!(berry_resource.id as usize, idx + 1);
            assert_eq!(berry_resource.name, resource.name());
        }
    }

    #[tokio::test]
    async fn berry_api_get_by_id() {
        let berry_api = BerryApiEndpoint::new(ApiClient::new().unwrap());

        assert_eq!(BerryApiEndpoint::name(), "berry");

        {
            let cheri = {
                let res = berry_api.get_by_id(1).await;
                assert_eq!(res.is_ok(), true);
                res.unwrap()
            };

            assert_eq!(cheri.name, "cheri");
            assert_eq!(cheri.id, 1);
        }

        {
            let aspear = {
                let res = berry_api.get_by_id(5).await;
                assert_eq!(res.is_ok(), true);
                res.unwrap()
            };

            assert_eq!(aspear.id, 5);
            assert_eq!(aspear.name, "aspear");
        }
    }

    #[tokio::test]
    async fn berry_api_get_by_name() {
        let berry_api = BerryApiEndpoint::new(ApiClient::new().unwrap());

        assert_eq!(BerryApiEndpoint::name(), "berry");

        {
            let cheri = {
                let res = berry_api.get_by_name("cheri").await;
                assert_eq!(res.is_ok(), true);
                res.unwrap()
            };

            assert_eq!(cheri.name, "cheri");
            assert_eq!(cheri.id, 1);
        }

        {
            let pecha = {
                let res = berry_api.get_by_name("pecha").await;
                assert_eq!(res.is_ok(), true);
                res.unwrap()
            };

            assert_eq!(pecha.name, "pecha");
            assert_eq!(pecha.id, 3);
        }
    }
}
