use reqwest::Client as ReqClient;
use reqwest::ClientBuilder;
use reqwest::RequestBuilder;
use reqwest::Response;

use crate::api::POKE_API_BASE_URL;
use crate::api::POKE_API_CLIENT_NAME;
use crate::Result;

const POKEMON_API_V2: &'static str = "v2";

#[derive(Clone)]
pub struct ApiClient {
    client: ReqClient,
    url: String,
}

impl ApiClient {
    pub fn new() -> Result<ApiClient> {
        let builder =  ClientBuilder::new()
            .user_agent(POKE_API_CLIENT_NAME);

        Ok(ApiClient {
            client: builder.build()?,
            url: POKE_API_BASE_URL.to_string() + "/".into() + POKEMON_API_V2.into()
        })
    }
}

impl ApiClient {
    pub(crate) async fn request_api<T: Into<String>>(&self, req: T) -> Result<Response> {
        self.client.get((self.url.clone() + "/" + req.into().as_str()).as_str())
            .send()
            .await
            .map_err(|err| crate::Error::from(err))
    }

    pub(crate) async fn request<T: Into<String>>(&self, req: T) -> Result<Response> {
        self.client.get(req.into().as_str())
            .send()
            .await
            .map_err(|err| crate::Error::from(err))
    }
}