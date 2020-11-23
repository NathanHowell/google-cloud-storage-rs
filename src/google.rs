pub mod iam {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/google.iam.v1.rs"));
    }
}

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/google.api.rs"));
}

pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
}

pub mod r#type {
    include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));
}

pub mod storage {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/google.storage.v1.rs"));
    }
}

pub mod error {
    #[derive(Debug, serde::Deserialize, thiserror::Error)]
    #[error("{error:?}")]
    #[serde(rename = "camelCase")]
    pub struct ErrorResponse {
        error: Errors,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename = "camelCase")]
    pub struct Errors {
        code: u16,
        errors: Vec<Error>,
        message: String,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename = "camelCase")]
    pub struct Error {
        domain: Option<String>,
        location: Option<String>,
        location_type: Option<String>,
        message: Option<String>,
        reason: Option<String>,
    }
}

#[async_trait::async_trait]
pub(crate) trait GoogleResponse
where
    Self: Sized,
{
    async fn into_google_response(self) -> Result<Self, crate::Error>;
}

#[async_trait::async_trait]
impl GoogleResponse for reqwest::Response {
    async fn into_google_response(self) -> Result<Self, crate::Error> {
        match self.error_for_status_ref() {
            Ok(_) => Ok(self),
            Err(err) => {
                // attempt to parse the error json; fall back to the status error if parsing fails
                self.json::<crate::google::error::ErrorResponse>()
                    .await
                    .map::<crate::Error, _>(|e| e.into())
                    .map_err(|_| err.into())
                    .and_then(|err| Err(err))
            }
        }
    }
}
