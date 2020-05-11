use reqwest::Client as ReqClient;
use reqwest::ClientBuilder;
use reqwest::Response;

use crate::api::v2::berry::BerryEndpoint;
use crate::api::POKE_API_BASE_URL;
use crate::api::POKE_API_CLIENT_NAME;
use crate::Result;

/// API version path
const POKEMON_API_V2: &'static str = "v2";

/// Client for version 2 of PokeAPI
#[derive(Clone)]
pub struct ApiClient {
    /// Inner reqwest client
    client: ReqClient,

    /// Base url for requesting API resources
    url: String,
}

impl ApiClient {
    /// Initializes client.
    pub fn new() -> Result<ApiClient> {
        let builder = ClientBuilder::new().user_agent(POKE_API_CLIENT_NAME);

        Ok(ApiClient {
            client: builder.build()?,
            url: POKE_API_BASE_URL.to_string() + "/".into() + POKEMON_API_V2.into(),
        })
    }

    /// Access to berries API enpoint
    pub fn berries(&self) -> BerryEndpoint {
        BerryEndpoint::new(self.clone())
    }
}

impl ApiClient {
    /// Request the API resource given the path.
    /// For example, with path `path`, will request the `https://pokeapi.co/api/v2/path`
    pub(crate) async fn request_api<T: Into<String>>(&self, req: T) -> Result<Response> {
        self.client
            .get(format!("{}/{}", self.url, req.into()).as_str())
            .send()
            .await
            .map_err(|err| crate::Error::from(err))
    }

    /// Request the API resource given the path and casts it to the type `P`.
    /// Type `P` must be deserializable.
    pub(crate) async fn request_api_object<P, T: Into<String>>(&self, req: T) -> Result<P>
    where
        P: Sized + serde::de::DeserializeOwned,
    {
        self.client
            .get(format!("{}/{}", self.url, req.into()).as_str())
            .send()
            .await
            .map_err(|err| crate::Error::from(err))?
            .json::<P>()
            .await
            .map_err(|err| crate::Error::from(err))
    }

    /// Request the API resource give nthe path and pagination parameters.
    /// Casts the result to the model type `P` which has to be deserializable.
    pub(crate) async fn request_api_object_paginated<P, T: Into<String>>(
        &self,
        req: T,
        offset: usize,
        limit: usize,
    ) -> Result<P>
    where
        P: Sized + serde::de::DeserializeOwned,
    {
        self.client
            .get(
                format!(
                    "{}/{}?offset={}&limit={}",
                    self.url,
                    req.into(),
                    offset,
                    limit
                )
                .as_str(),
            )
            .send()
            .await
            .map_err(|err| crate::Error::from(err))?
            .json::<P>()
            .await
            .map_err(|err| crate::Error::from(err))
    }

    /// Request given url.
    pub(crate) async fn request<T: Into<String>>(&self, req: T) -> Result<Response> {
        self.client
            .get(req.into().as_str())
            .send()
            .await
            .map_err(|err| crate::Error::from(err))
    }
}
