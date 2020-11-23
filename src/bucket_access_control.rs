use crate::client::bucket_url;
use crate::google::storage::v1::{
    BucketAccessControl, DeleteBucketAccessControlRequest, GetBucketAccessControlRequest,
    InsertBucketAccessControlRequest, ListBucketAccessControlsRequest,
    ListBucketAccessControlsResponse, UpdateBucketAccessControlRequest,
};
use crate::query::Query;
use crate::request::Request;
use crate::storage::v1::PatchBucketAccessControlRequest;
use crate::{Client, Result};
use reqwest::Method;
use std::fmt::Debug;
use url::Url;

fn acl_url(base_url: &Url, bucket: &str) -> Result<Url> {
    Ok(bucket_url(base_url, bucket)?.join("acl/")?)
}

impl Query for InsertBucketAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for InsertBucketAccessControlRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = BucketAccessControl;

    fn scope(&self) -> &'static str {
        crate::iam::FULL_CONTROL
    }

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        acl_url(base_url, &self.bucket)
    }
}

impl Query for GetBucketAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for GetBucketAccessControlRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = BucketAccessControl;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(acl_url(base_url, &self.bucket)?.join(&self.entity)?)
    }
}

impl Query for UpdateBucketAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for UpdateBucketAccessControlRequest {
    const REQUEST_METHOD: Method = Method::PUT;

    type Response = BucketAccessControl;

    fn scope(&self) -> &'static str {
        crate::iam::FULL_CONTROL
    }

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(acl_url(base_url, &self.bucket)?.join(&self.entity)?)
    }
}

impl Query for DeleteBucketAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for DeleteBucketAccessControlRequest {
    const REQUEST_METHOD: Method = Method::DELETE;

    type Response = ();

    fn scope(&self) -> &'static str {
        crate::iam::FULL_CONTROL
    }

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(acl_url(base_url, &self.bucket)?.join(&self.entity)?)
    }
}

impl Query for ListBucketAccessControlsRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for ListBucketAccessControlsRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = ListBucketAccessControlsResponse;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        acl_url(base_url, &self.bucket)
    }
}

impl Client {
    #[doc = " Creates a new ACL entry on the specified bucket."]
    #[tracing::instrument]
    pub async fn insert_bucket_access_control(
        &self,
        request: impl Into<InsertBucketAccessControlRequest> + Debug,
    ) -> crate::Result<BucketAccessControl> {
        let request = request.into();

        self.invoke_json(&request, &request.bucket_access_control)
            .await
    }

    #[doc = " Retrieves ACL entries on the specified bucket."]
    #[tracing::instrument]
    pub async fn list_bucket_access_controls(
        &self,
        request: impl Into<ListBucketAccessControlsRequest> + Debug,
    ) -> crate::Result<ListBucketAccessControlsResponse> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Returns the ACL entry for the specified entity on the specified bucket."]
    #[tracing::instrument]
    pub async fn get_bucket_access_control(
        &self,
        request: impl Into<GetBucketAccessControlRequest> + Debug,
    ) -> crate::Result<BucketAccessControl> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Updates an ACL entry on the specified bucket. Equivalent to"]
    #[doc = " PatchBucketAccessControl, but all unspecified fields will be"]
    #[doc = " reset to their default values."]
    #[tracing::instrument]
    pub async fn update_bucket_access_control(
        &self,
        request: impl Into<UpdateBucketAccessControlRequest> + Debug,
    ) -> crate::Result<BucketAccessControl> {
        let request = request.into();

        self.invoke_json(&request, &request.bucket_access_control)
            .await
    }

    #[doc = " Permanently deletes the ACL entry for the specified entity on the specified"]
    #[doc = " bucket."]
    #[tracing::instrument]
    pub async fn delete_bucket_access_control(
        &self,
        request: impl Into<DeleteBucketAccessControlRequest> + Debug,
    ) -> crate::Result<()> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Updates an ACL entry on the specified bucket."]
    pub async fn patch_bucket_access_control(
        &mut self,
        _request: impl Into<PatchBucketAccessControlRequest>,
    ) -> Result<BucketAccessControl> {
        unimplemented!()
    }
}
