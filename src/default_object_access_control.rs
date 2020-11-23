use crate::client::bucket_url;
use crate::google::storage::v1::{
    DeleteDefaultObjectAccessControlRequest, GetDefaultObjectAccessControlRequest,
    InsertDefaultObjectAccessControlRequest, ListDefaultObjectAccessControlsRequest,
    ListObjectAccessControlsResponse, ObjectAccessControl, UpdateDefaultObjectAccessControlRequest,
};
use crate::query::Query;
use crate::request::Request;
use crate::storage::v1::PatchDefaultObjectAccessControlRequest;
use crate::{Client, Result};
use reqwest::Method;
use std::fmt::Debug;
use url::Url;

fn default_object_acl_url(base_url: &Url, bucket: &str) -> Result<Url> {
    Ok(bucket_url(base_url, bucket)?.join("defaultObjectAcl")?)
}

impl Query for InsertDefaultObjectAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for InsertDefaultObjectAccessControlRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = ObjectAccessControl;

    fn scope(&self) -> &'static str {
        crate::iam::FULL_CONTROL
    }

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        default_object_acl_url(base_url, &self.bucket)
    }
}

impl Query for ListDefaultObjectAccessControlsRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        if let Some(if_metageneration_match) = self.if_metageneration_match {
            query.push(("ifMetagenerationMatch", if_metageneration_match.to_string()));
        }

        if let Some(if_metageneration_not_match) = self.if_metageneration_not_match {
            query.push((
                "ifMetagenerationNotMatch",
                if_metageneration_not_match.to_string(),
            ));
        }

        query
    }
}

impl Request for ListDefaultObjectAccessControlsRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = ListObjectAccessControlsResponse;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        default_object_acl_url(base_url, &self.bucket)
    }
}

impl Query for GetDefaultObjectAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for GetDefaultObjectAccessControlRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = ObjectAccessControl;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(default_object_acl_url(base_url, &self.bucket)?.join(&self.entity)?)
    }
}

impl Query for UpdateDefaultObjectAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for UpdateDefaultObjectAccessControlRequest {
    const REQUEST_METHOD: Method = Method::PUT;

    type Response = ObjectAccessControl;

    fn scope(&self) -> &'static str {
        crate::iam::FULL_CONTROL
    }

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(default_object_acl_url(base_url, &self.bucket)?.join(&self.entity)?)
    }
}

impl Query for DeleteDefaultObjectAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for DeleteDefaultObjectAccessControlRequest {
    const REQUEST_METHOD: Method = Method::DELETE;

    type Response = ();

    fn scope(&self) -> &'static str {
        crate::iam::FULL_CONTROL
    }

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(default_object_acl_url(base_url, &self.bucket)?.join(&self.entity)?)
    }
}

impl Client {
    #[doc = " Creates a new default object ACL entry on the specified bucket."]
    #[tracing::instrument]
    pub async fn insert_default_object_access_control(
        &self,
        request: impl Into<InsertDefaultObjectAccessControlRequest> + Debug,
    ) -> crate::Result<ObjectAccessControl> {
        let request = request.into();

        self.invoke_json(&request, &request.object_access_control)
            .await
    }

    #[doc = " Retrieves default object ACL entries on the specified bucket."]
    #[tracing::instrument]
    pub async fn list_default_object_access_controls(
        &self,
        request: impl Into<ListDefaultObjectAccessControlsRequest> + Debug,
    ) -> crate::Result<ListObjectAccessControlsResponse> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Returns the default object ACL entry for the specified entity on the"]
    #[doc = " specified bucket."]
    #[tracing::instrument]
    pub async fn get_default_object_access_control(
        &self,
        request: impl Into<GetDefaultObjectAccessControlRequest> + Debug,
    ) -> crate::Result<ObjectAccessControl> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Updates a default object ACL entry on the specified bucket."]
    pub async fn patch_default_object_access_control(
        &mut self,
        _request: impl Into<PatchDefaultObjectAccessControlRequest>,
    ) -> Result<ObjectAccessControl> {
        unimplemented!()
    }

    #[doc = " Updates a default object ACL entry on the specified bucket. Equivalent to"]
    #[doc = " PatchDefaultObjectAccessControl, but modifies all unspecified fields to"]
    #[doc = " their default values."]
    #[tracing::instrument]
    pub async fn update_default_object_access_control(
        &self,
        request: impl Into<UpdateDefaultObjectAccessControlRequest> + Debug,
    ) -> crate::Result<ObjectAccessControl> {
        let request = request.into();

        self.invoke_json(&request, &request.object_access_control)
            .await
    }

    #[doc = " Permanently deletes the default object ACL entry for the specified entity"]
    #[doc = " on the specified bucket."]
    #[tracing::instrument]
    pub async fn delete_default_object_access_control(
        &self,
        request: impl Into<DeleteDefaultObjectAccessControlRequest> + Debug,
    ) -> Result<()> {
        let request = request.into();

        self.invoke(&request).await
    }
}
