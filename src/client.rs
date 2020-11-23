use crate::request::Request;
use crate::{GoogleResponse, Result};
use gouth::Token;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use reqwest::{Body, RequestBuilder, Response};
use serde::Serialize;
use std::fmt::{self, Debug, Formatter};
use tracing::Instrument;
use url::Url;

const ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'*')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_');

pub(crate) fn percent_encode(input: &str) -> String {
    utf8_percent_encode(input, ENCODE_SET).to_string()
}

pub(crate) fn bucket_url(base_url: &Url, bucket: &str) -> Result<Url> {
    Ok(base_url.join("b/")?.join(&percent_encode(bucket))?)
}

pub(crate) fn object_url(base_url: &Url, bucket: &str, object: &str) -> Result<Url> {
    Ok(bucket_url(base_url, bucket)?
        .join("o/")?
        .join(&percent_encode(object))?)
}

pub struct Client {
    token: Token,

    client: reqwest::Client,

    base_url: Url,
}

#[derive(Default)]
pub struct ClientBuilder {
    token: Option<Token>,
    scopes: Vec<String>,
    client: Option<reqwest::Client>,
    base_url: Option<Url>,
}

fn scopes_or_default(scopes: Vec<String>) -> Vec<String> {
    if scopes.is_empty() {
        vec!["https://www.googleapis.com/auth/devstorage.full_control".to_string()]
    } else {
        scopes
    }
}

impl ClientBuilder {
    pub fn token(mut self, token: impl Into<Token>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn scopes<T: AsRef<str>>(mut self, scopes: impl Iterator<Item = T>) -> Self {
        self.scopes = scopes.map(|e| e.as_ref().to_string()).collect();
        self
    }

    pub fn client(mut self, client: impl Into<reqwest::Client>) -> Self {
        self.client = Some(client.into());
        self
    }

    pub fn base_url(mut self, base_url: impl Into<Url>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn build(self) -> Result<Client> {
        let token = match self.token {
            Some(token) if self.scopes.is_empty() => token,
            None => gouth::Builder::new()
                .scopes(&scopes_or_default(self.scopes))
                .build()?,
            _ => panic!(),
        };

        let client = self.client.unwrap_or_default();

        let base_url = self
            .base_url
            .unwrap_or_else(|| Url::parse("https://www.googleapis.com/storage/v1/").unwrap());

        Ok(Client {
            token,
            client,
            base_url,
        })
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("token", &"...")
            .field("client", &self.client)
            .field("base_url", &self.base_url.to_string())
            .finish()
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        Default::default()
    }

    /// Create a new storage client with the default authentication scope
    #[tracing::instrument]
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }
}

impl Client {
    fn authorization_headers(&self) -> Result<reqwest::header::HeaderMap> {
        let token = self.token.header_value()?;
        let mut result = reqwest::header::HeaderMap::with_capacity(1);
        result.insert(reqwest::header::AUTHORIZATION, token.parse()?);
        Ok(result)
    }

    fn request_builder<R: Request>(&self, request: &R) -> Result<RequestBuilder> {
        let path = request.request_path(&self.base_url)?;

        tracing::debug!(request_path = %path);

        Ok(self
            .client
            .request(R::REQUEST_METHOD, path)
            .headers(self.authorization_headers()?)
            .headers(request.request_headers())
            .query(&request.request_query()))
    }

    async fn request<R: Request>(&self, request: &R) -> Result<Response> {
        self.request_body(request, vec![]).await
    }

    async fn request_body<R: Request>(
        &self,
        request: &R,
        body: impl Into<Body>,
    ) -> Result<Response> {
        Ok(self
            .request_builder(request)?
            .body(body)
            .send()
            .instrument(tracing::trace_span!("sending"))
            .await?
            .into_google_response()
            .instrument(tracing::trace_span!("error test"))
            .await?)
    }

    async fn request_json<R: Request, T: Serialize>(
        &self,
        request: &R,
        body: &T,
    ) -> Result<Response> {
        let body = serde_json::to_vec(body)?;
        self.request_body(request, body).await
    }

    pub(crate) async fn invoke<R: Request>(&self, request: &R) -> Result<R::Response> {
        Ok(self
            .request(request)
            .instrument(tracing::trace_span!("sending"))
            .await?
            .into_google_response()
            .instrument(tracing::trace_span!("error test"))
            .await?
            .json::<R::Response>()
            .instrument(tracing::trace_span!("parsing"))
            .await?)
    }

    pub(crate) async fn invoke_body<R: Request>(
        &self,
        request: &R,
        body: impl Into<Body>,
    ) -> Result<R::Response> {
        Ok(self
            .request_body(request, body)
            .instrument(tracing::trace_span!("sending"))
            .await?
            .into_google_response()
            .instrument(tracing::trace_span!("error test"))
            .await?
            .json::<R::Response>()
            .instrument(tracing::trace_span!("parsing"))
            .await?)
    }

    pub(crate) async fn invoke_json<R: Request, T: Serialize>(
        &self,
        request: &R,
        body: &T,
    ) -> Result<R::Response> {
        Ok(self
            .request_json(request, body)
            .instrument(tracing::trace_span!("sending"))
            .await?
            .into_google_response()
            .instrument(tracing::trace_span!("error test"))
            .await?
            .json::<R::Response>()
            .instrument(tracing::trace_span!("parsing"))
            .await?)
    }

    pub(crate) async fn get<R: Request>(&self, request: &R) -> Result<Response> {
        Ok(self
            .request(request)
            .instrument(tracing::trace_span!("sending"))
            .await?
            .into_google_response()
            .instrument(tracing::trace_span!("error test"))
            .await?)
    }
}
