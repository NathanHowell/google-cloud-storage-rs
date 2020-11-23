use crate::client::object_url;
use crate::google::storage::v1::{
    DeleteObjectAccessControlRequest, GetObjectAccessControlRequest,
    InsertObjectAccessControlRequest, ListObjectAccessControlsRequest,
    ListObjectAccessControlsResponse, ObjectAccessControl, UpdateObjectAccessControlRequest,
};
use crate::query::Query;
use crate::request::Request;
use crate::{Client, Result};
use reqwest::{Method, Url};
use std::fmt::Debug;

fn acl_url(base_url: &Url, bucket: &str, object: &str) -> Result<Url> {
    Ok(object_url(base_url, bucket, object)?.join("acl")?)
}

impl Query for InsertObjectAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        if self.generation != 0 {
            query.push(("generation", self.generation.to_string()));
        }

        query
    }
}

impl Request for InsertObjectAccessControlRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = ObjectAccessControl;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        acl_url(base_url, &self.bucket, &self.object)
    }
}

impl Query for ListObjectAccessControlsRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        unimplemented!()
    }
}

impl Request for ListObjectAccessControlsRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = ListObjectAccessControlsResponse;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        acl_url(base_url, &self.bucket, &self.object)
    }
}

impl Query for GetObjectAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        if self.generation != 0 {
            query.push(("generation", self.generation.to_string()));
        }

        query
    }
}

impl Request for GetObjectAccessControlRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = ObjectAccessControl;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(acl_url(base_url, &self.bucket, &self.object)?.join(&self.entity)?)
    }
}

impl Query for UpdateObjectAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        if self.generation != 0 {
            query.push(("generation", self.generation.to_string()));
        }

        query
    }
}

impl Request for UpdateObjectAccessControlRequest {
    const REQUEST_METHOD: Method = Method::PUT;

    type Response = ObjectAccessControl;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(acl_url(base_url, &self.bucket, &self.object)?.join(&self.entity)?)
    }
}

impl Query for DeleteObjectAccessControlRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        if self.generation != 0 {
            query.push(("generation", self.generation.to_string()));
        }

        query
    }
}

impl Request for DeleteObjectAccessControlRequest {
    const REQUEST_METHOD: Method = Method::DELETE;

    type Response = ();

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(acl_url(base_url, &self.bucket, &self.object)?.join(&self.entity)?)
    }
}

impl Client {
    #[doc = " Creates a new ACL entry on the specified object."]
    #[tracing::instrument]
    pub async fn insert_object_access_control(
        &self,
        request: impl Into<InsertObjectAccessControlRequest> + Debug,
    ) -> crate::Result<ObjectAccessControl> {
        let request = request.into();

        self.invoke_json(&request, &request.object_access_control)
            .await
    }

    #[doc = " Retrieves ACL entries on the specified object."]
    #[tracing::instrument]
    pub async fn list_object_access_controls(
        &self,
        request: impl Into<ListObjectAccessControlsRequest> + Debug,
    ) -> crate::Result<ListObjectAccessControlsResponse> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Returns the ACL entry for the specified entity on the specified object."]
    #[tracing::instrument]
    pub async fn get_object_access_control(
        &self,
        request: impl Into<GetObjectAccessControlRequest> + Debug,
    ) -> crate::Result<ObjectAccessControl> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Updates an ACL entry on the specified object."]
    #[tracing::instrument]
    pub async fn update_object_access_control(
        &self,
        request: impl Into<UpdateObjectAccessControlRequest> + Debug,
    ) -> crate::Result<ObjectAccessControl> {
        let request = request.into();

        self.invoke_json(&request, &request.object_access_control)
            .await
    }

    #[doc = " Permanently deletes the ACL entry for the specified entity on the specified"]
    #[doc = " object."]
    #[tracing::instrument]
    pub async fn delete_object_access_control(
        &self,
        request: impl Into<DeleteObjectAccessControlRequest> + Debug,
    ) -> crate::Result<()> {
        let request = request.into();

        self.invoke(&request).await
    }
}
