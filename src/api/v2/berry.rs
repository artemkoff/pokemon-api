use super::client::ApiClient;
use super::endpoint::ApiEndpoint;
use super::resource::*;
use crate::models::v2::berry::{Berry, BerryFirmness};
use crate::models::v2::resource::NamedResourceList;
use crate::Result;

/// Berries API endpoint. For details see [pokeapi/berries](https://pokeapi.co/docs/v2.html#berries-section)
/// Represents the endpoint `https://pokeapi.co/api/v2/berry`
#[derive(Clone)]
pub struct BerryEndpoint {
    client: ApiClient,
}

impl ApiEndpoint for BerryEndpoint {
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

impl BerryEndpoint {
    /// Creates API Endpoint object
    pub(crate) fn new(client: ApiClient) -> Self {
        Self { client }
    }
}

decl_resource!(BerryResource for Berry);
decl_named_resource!(BerryNamedResource for Berry);
decl_resource_list!(BerryResourceList for Berry with BerryResource);
decl_named_resource_list!(BerryNamedResourceList for Berry with BerryNamedResource);

/// Berry Firmness API endpoint. For details see [pokeapi/berry-firmnesses](https://pokeapi.co/docs/v2.html/#berry-firmnesses)
/// Represents the endpoint `https://pokeapi.co/api/v2/berry-firmnesses`
#[derive(Clone)]
pub struct BerryFirmnessEndpoint {
    client: ApiClient,
}

impl ApiEndpoint for BerryFirmnessEndpoint {
    type Model = BerryFirmness;
    type NamedResourceList = BerryFirmnessNamedResourceList;

    fn client(&self) -> &ApiClient {
        &self.client
    }

    fn name() -> &'static str {
        "berry-firmness"
    }

    fn create_named_resource_list(&self, model: NamedResourceList) -> Self::NamedResourceList {
        BerryFirmnessNamedResourceList::new(self.client().clone(), model)
    }
}

impl BerryFirmnessEndpoint {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self { client }
    }
}

decl_resource!(BerryFirmnessResource for BerryFirmness);
decl_named_resource!(BerryFirmnessNamedResource for BerryFirmness);
decl_resource_list!(BerryFirmnessResourceList for BerryFirmness with BerryFirmnessResource);
decl_named_resource_list!(BerryFirmnessNamedResourceList for BerryFirmness with BerryFirmnessNamedResource);

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use crate::api::v2::berry::{
        BerryEndpoint, BerryFirmnessEndpoint, BerryFirmnessNamedResource,
        BerryFirmnessNamedResourceList, BerryFirmnessResource, BerryNamedResource,
        BerryNamedResourceList, BerryResource,
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
        let berry = berry.unwrap();

        assert_eq!(berry.id, 1);
        assert_eq!(berry.name, "cheri");
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
        let berry_api = BerryEndpoint::new(ApiClient::new().unwrap());

        assert_eq!(BerryEndpoint::name(), "berry");

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
        let berry_api = BerryEndpoint::new(ApiClient::new().unwrap());

        assert_eq!(BerryEndpoint::name(), "berry");

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

    #[tokio::test]
    async fn berry_firmness_resource() {
        let client = ApiClient::new().unwrap();
        let resource = Resource {
            url: "https://pokeapi.co/api/v2/berry-firmness/1".into(),
        };

        let berry_firmness_resource = BerryFirmnessResource::new(client, resource);

        let berry_firmness = berry_firmness_resource.get().await;

        assert!(berry_firmness.is_ok(), true);

        let berry_firmness = berry_firmness.unwrap();
        assert_eq!(berry_firmness.id, 1);
    }

    #[tokio::test]
    async fn berry_firmness_named_resource() {
        let client = ApiClient::new().unwrap();
        let resource = NamedResource {
            name: "very-soft".to_string(),
            url: "https://pokeapi.co/api/v2/berry-firmness/1".to_string(),
        };

        let firmness_named_resource = BerryFirmnessNamedResource::new(client.clone(), resource);

        let berry_firmness = firmness_named_resource.get().await;

        assert_eq!(berry_firmness.is_ok(), true);
        let berry_firmness = berry_firmness.unwrap();

        assert_eq!(berry_firmness.id, 1);
        assert_eq!(berry_firmness.name, "very-soft");
    }

    #[tokio::test]
    async fn berry_firmness_named_resource_list() {
        let client = ApiClient::new().unwrap();

        let resource_list = NamedResourceList {
            count: 5,
            next: None,
            previous: None,
            results: vec![
                NamedResource {
                    name: "very-soft".into(),
                    url: "https://pokeapi.co/api/v2/berry-firmness/1/".into(),
                },
                NamedResource {
                    name: "soft".into(),
                    url: "https://pokeapi.co/api/v2/berry-firmness/2/".into(),
                },
                NamedResource {
                    name: "hard".into(),
                    url: "https://pokeapi.co/api/v2/berry-firmness/3/".into(),
                },
                NamedResource {
                    name: "very-hard".into(),
                    url: "https://pokeapi.co/api/v2/berry-firmness/4/".into(),
                },
                NamedResource {
                    name: "super-hard".into(),
                    url: "https://pokeapi.co/api/v2/berry-firmness/5/".into(),
                },
            ],
        };

        let named_resource_list =
            BerryFirmnessNamedResourceList::new(client.clone(), resource_list.clone());

        assert_eq!(named_resource_list.count(), 5);

        let prev_list = named_resource_list.previous_list().await;
        assert_eq!(prev_list.is_ok(), true);
        assert_eq!(prev_list.unwrap().is_none(), true);

        let next_lit = named_resource_list.next_list().await;
        assert_eq!(next_lit.is_ok(), true);
        assert_eq!(next_lit.unwrap().is_none(), true);

        let resources = named_resource_list.resources();
        for (idx, resource) in resources.iter().enumerate() {
            assert_eq!(resource.name(), resource_list.results[idx].name);
            assert_eq!(resource.url(), resource_list.results[idx].url);

            let firmness = resource.get().await;
            assert_eq!(firmness.is_ok(), true);
            let firmness = firmness.unwrap();

            assert_eq!(firmness.id as usize, idx + 1);
            assert_eq!(firmness.name, resource.name());
        }
    }

    #[tokio::test]
    async fn firmness_api_get_by_id() {
        let firmness_api = BerryFirmnessEndpoint::new(ApiClient::new().unwrap());

        assert_eq!(BerryFirmnessEndpoint::name(), "berry-firmness");

        {
            let very_soft = firmness_api.get_by_id(1).await;
            assert_eq!(very_soft.is_ok(), true);
            let very_soft = very_soft.unwrap();

            assert_eq!(very_soft.id, 1);
            assert_eq!(very_soft.name, "very-soft")
        }

        {
            let super_hard = firmness_api.get_by_id(5).await;
            assert_eq!(super_hard.is_ok(), true);
            let super_hard = super_hard.unwrap();

            assert_eq!(super_hard.id, 5);
            assert_eq!(super_hard.name, "super-hard");
        }
    }

    #[tokio::test]
    async fn firmness_api_get_by_name() {
        let firmness_api = BerryFirmnessEndpoint::new(ApiClient::new().unwrap());

        assert_eq!(BerryFirmnessEndpoint::name(), "berry-firmness");

        {
            let very_soft = firmness_api.get_by_name("very-soft").await;
            assert_eq!(very_soft.is_ok(), true);
            let very_soft = very_soft.unwrap();

            assert_eq!(very_soft.id, 1);
            assert_eq!(very_soft.name, "very-soft");
        }

        {
            let hard = firmness_api.get_by_name("hard").await;
            assert_eq!(hard.is_ok(), true);
            let hard = hard.unwrap();

            assert_eq!(hard.id, 3);
            assert_eq!(hard.name, "hard");
        }
    }
}
