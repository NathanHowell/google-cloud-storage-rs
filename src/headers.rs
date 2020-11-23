use crate::Result;
use reqwest::header::{HeaderMap, HeaderValue};

pub trait Headers {
    fn headers(&self, scope: &str) -> Result<HeaderMap<HeaderValue>>;
}

impl Headers for () {
    fn headers(&self, _scope: &str) -> Result<HeaderMap<HeaderValue>> {
        Ok(HeaderMap::new())
    }
}

#[cfg(feature = "gouth")]
impl Headers for gouth::Token {
    fn headers(&self, _scopes: &str) -> Result<HeaderMap<HeaderValue>> {
        let mut map = HeaderMap::with_capacity(1);
        map.insert(
            reqwest::header::AUTHORIZATION,
            self.header_value()?.parse()?,
        );
        Ok(map)
    }
}

#[cfg(feature = "yup-oauth2")]
impl Headers for yup_oauth2::authenticator::Authenticator<C> {
    fn headers(&self, scope: &str) -> Result<HeaderMap<HeaderValue>> {
        let token = self.token(&[scope]).await?;
        let mut map = HeaderMap::with_capacity(1);
        map.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token.as_str()).parse()?,
        );
        Ok(map)
    }
}
