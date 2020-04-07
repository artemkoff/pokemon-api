use super::resource::*;
use crate::models::v2::berries::Berry;
use crate::Result;
use super::client::ApiClient;

#[derive(Clone)]
pub struct BerryApi {
    client: ApiClient,
}

decl_resource!(BerryResource for Berry);
decl_named_resource!(BerryNamedResource for Berry);
decl_resource_list!(BerryResourceList for Berry with BerryResource);
decl_named_resource_list!(BerryNamedResourceList for Berry with BerryNamedResource);

#[cfg(test)]
mod test {
    use crate::api::v2::client::ApiClient;
    use crate::api::v2::resource::ApiResource;
    use crate::models::v2::resource::{Resource, NamedResource};
    use crate::api::v2::berries::{BerryResource, BerryNamedResource};

    #[tokio::test]
    async fn test_berry_resource() {
        let client = ApiClient::new().unwrap();
        let resource = Resource { url: "https://pokeapi.co/api/v2/berry/1".into() };

        let berry_resource = BerryResource::new(client, resource);

        let berry = berry_resource.get().await;

        assert_eq!(berry.is_ok(), true);

        let berry = berry.unwrap();
        assert_eq!(berry.id, 1);
        assert_eq!(berry.name, "cheri");
    }

    #[tokio::test]
    async fn test_berry_named_resource() {
        let client = ApiClient::new().unwrap();
        let named_resource = NamedResource {
            name: "cheri".into(),
            url: "https://pokeapi.co/api/v2/berry/1".into(),
        };

        let named_berry_resource = BerryNamedResource::new(client.clone(), named_resource);

        let berry = named_berry_resource.get().await;

        assert_eq!(berry.is_ok(), true);
    }
}