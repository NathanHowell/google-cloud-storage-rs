use crate::headers::Headers;
use crate::join_segment::JoinSegment;
use crate::request::Request;
use crate::{GoogleResponse, Result};
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
    Ok(base_url
        .join("b/")?
        .join(&percent_encode(&(bucket.to_string())))?)
}

pub(crate) fn object_url(base_url: &Url, bucket: &str, object: &str) -> Result<Url> {
    Ok(bucket_url(base_url, bucket)?
        .join_segment("o/")?
        .join(&percent_encode(object))?)
}

pub struct Client {
    headers: Box<dyn Headers>,

    client: reqwest::Client,

    base_url: Url,
}

#[derive(Default)]
pub struct ClientBuilder {
    headers: Option<Box<dyn Headers>>,
    client: Option<reqwest::Client>,
    base_url: Option<Url>,
}

impl ClientBuilder {
    #[cfg(feature = "gouth")]
    pub fn token(self, token: impl Into<gouth::Token>) -> Self {
        let token: Box<dyn Headers> = Box::new(token.into());
        self.headers(token)
    }

    pub fn headers(mut self, headers: impl Into<Box<dyn Headers>>) -> Self {
        self.headers = Some(headers.into());
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
        let client = self.client.unwrap_or_default();

        let headers = self.headers.unwrap_or_else(|| Box::new(()));

        let base_url = self
            .base_url
            .unwrap_or_else(|| Url::parse("https://storage.googleapis.com/storage/v1/").unwrap());

        Ok(Client {
            headers,
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
    fn request_builder<R: Request>(&self, request: &R) -> Result<RequestBuilder> {
        let path = request.request_path(&self.base_url)?;

        tracing::debug!(request_path = %path);

        Ok(self
            .client
            .request(R::REQUEST_METHOD, path)
            .headers(self.headers.headers(request.scope())?)
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
