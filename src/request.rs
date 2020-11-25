use crate::query::Query;
use crate::Result;
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde::de::DeserializeOwned;
use url::Url;

pub(crate) struct Scope;

impl Scope {
    pub(crate) const READ_ONLY: &'static str =
        "https://www.googleapis.com/auth/devstorage.read_only";

    pub(crate) const READ_WRITE: &'static str =
        "https://www.googleapis.com/auth/devstorage.read_write";

    pub(crate) const FULL_CONTROL: &'static str =
        "https://www.googleapis.com/auth/devstorage.full_control";
}

pub(crate) trait Request: Query {
    const REQUEST_METHOD: Method;

    type Response: DeserializeOwned;

    fn scope(&self) -> &'static str {
        if Self::REQUEST_METHOD == Method::GET {
            Scope::READ_ONLY
        } else {
            Scope::READ_WRITE
        }
    }

    fn request_path(&self, base_url: Url) -> Result<Url>;

    fn request_headers(&self) -> HeaderMap {
        HeaderMap::new()
    }
}
