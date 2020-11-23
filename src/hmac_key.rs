use crate::google::storage::v1::{
    CreateHmacKeyRequest, CreateHmacKeyResponse, DeleteHmacKeyRequest, GetHmacKeyRequest,
    HmacKeyMetadata, ListHmacKeysRequest, ListHmacKeysResponse, UpdateHmacKeyRequest,
};
use crate::paginate::Paginate;
use crate::query::Query;
use crate::request::Request;
use crate::{Client, Result};
use futures::Stream;
use reqwest::Method;
use std::fmt::Debug;
use std::pin::Pin;
use url::Url;

fn hmac_keys_url(base_url: &Url, project_id: &str) -> Result<Url> {
    Ok(base_url
        .join("projects/")?
        .join(project_id)?
        .join("hmacKeys/")?)
}

impl Query for CreateHmacKeyRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        query.push(("serviceAccountEmail", self.service_account_email.clone()));

        query
    }
}

impl Request for CreateHmacKeyRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = CreateHmacKeyResponse;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        hmac_keys_url(base_url, &self.project_id)
    }
}

impl Query for ListHmacKeysRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        if self.max_results != 0 {
            query.push(("maxResults", self.max_results.to_string()));
        }

        if !self.page_token.is_empty() {
            query.push(("pageToken", self.page_token.clone()));
        }

        if !self.service_account_email.is_empty() {
            query.push(("serviceAccountEmail", self.service_account_email.clone()));
        }

        if self.show_deleted_keys {
            query.push(("showDeletedKeys", self.show_deleted_keys.to_string()));
        }

        query
    }
}

impl Request for ListHmacKeysRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = ListHmacKeysResponse;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        hmac_keys_url(base_url, &self.project_id)
    }
}

impl<'a> Paginate<'a> for ListHmacKeysRequest {
    type Item = HmacKeyMetadata;

    fn extract_items(response: ListHmacKeysResponse) -> Vec<Self::Item> {
        response.items
    }

    fn next_request(response: &ListHmacKeysResponse) -> Option<Self> {
        if response.next_page_token.is_empty() {
            None
        } else {
            Some(ListHmacKeysRequest {
                page_token: response.next_page_token.clone(),
                ..Default::default()
            })
        }
    }
}

impl Query for GetHmacKeyRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for GetHmacKeyRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = HmacKeyMetadata;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(hmac_keys_url(base_url, &self.project_id)?.join(&self.access_id)?)
    }
}

impl Query for UpdateHmacKeyRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for UpdateHmacKeyRequest {
    const REQUEST_METHOD: Method = Method::PUT;

    type Response = HmacKeyMetadata;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(hmac_keys_url(base_url, &self.project_id)?.join(&self.access_id)?)
    }
}

impl Query for DeleteHmacKeyRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for DeleteHmacKeyRequest {
    const REQUEST_METHOD: Method = Method::DELETE;

    type Response = ();

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(hmac_keys_url(base_url, &self.project_id)?.join(&self.access_id)?)
    }
}

impl Client {
    #[doc = " Creates a new HMAC key for the given service account."]
    #[tracing::instrument]
    pub async fn create_hmac_key(
        &self,
        request: impl Into<CreateHmacKeyRequest> + Debug,
    ) -> crate::Result<CreateHmacKeyResponse> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Lists HMAC keys under a given project with the additional filters provided."]
    #[tracing::instrument]
    pub async fn list_hmac_keys(
        &self,
        request: impl Into<ListHmacKeysRequest> + Debug,
    ) -> crate::Result<ListHmacKeysResponse> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Lists HMAC keys under a given project with the additional filters provided."]
    #[tracing::instrument]
    pub async fn list_hmac_keys_stream<'a>(
        &'a self,
        request: impl Into<ListHmacKeysRequest> + Debug,
    ) -> Pin<Box<dyn Stream<Item = Result<HmacKeyMetadata>> + 'a>> {
        self.paginate(request.into())
    }

    #[doc = " Gets an existing HMAC key metadata for the given id."]
    #[tracing::instrument]
    pub async fn get_hmac_key(
        &self,
        request: impl Into<GetHmacKeyRequest> + Debug,
    ) -> crate::Result<HmacKeyMetadata> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Updates a given HMAC key state between ACTIVE and INACTIVE."]
    #[tracing::instrument]
    pub async fn update_hmac_key(
        &self,
        request: impl Into<UpdateHmacKeyRequest> + Debug,
    ) -> crate::Result<HmacKeyMetadata> {
        let request = request.into();

        self.invoke_json(&request, &request.metadata).await
    }

    #[doc = " Deletes a given HMAC key.  Key must be in an INACTIVE state."]
    #[tracing::instrument]
    pub async fn delete_hmac_key(
        &self,
        request: impl Into<DeleteHmacKeyRequest> + Debug,
    ) -> crate::Result<()> {
        let request = request.into();

        self.invoke(&request).await
    }
}
