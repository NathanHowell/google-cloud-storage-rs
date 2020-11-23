use crate::query::Query;
use crate::Result;
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde::de::DeserializeOwned;
use url::Url;

pub(crate) trait Request: Query {
    const REQUEST_METHOD: Method;

    type Response: DeserializeOwned;

    fn scope(&self) -> &'static str {
        if Self::REQUEST_METHOD == Method::GET {
            "https://www.googleapis.com/auth/devstorage.read_only"
        } else {
            "https://www.googleapis.com/auth/devstorage.read_write"
        }
    }

    fn request_path(&self, base_url: &Url) -> Result<Url>;

    fn request_headers(&self) -> HeaderMap {
        HeaderMap::new()
    }
}
