use crate::client::{bucket_url, object_url, percent_encode};
use crate::google::storage::v1::common_enums::{PredefinedObjectAcl, Projection};
use crate::google::storage::v1::compose_object_request::SourceObjects;
use crate::google::storage::v1::insert_object_request::FirstMessage;
use crate::google::storage::v1::{
    Bucket, CommonObjectRequestParams, ComposeObjectRequest, CopyObjectRequest,
    DeleteObjectRequest, GetObjectMediaRequest, GetObjectRequest, InsertObjectRequest,
    ListObjectsRequest, ListObjectsResponse, RewriteObjectRequest, RewriteResponse,
    StartResumableWriteRequest, UpdateObjectRequest,
};
use crate::join_segment::JoinSegment;
use crate::paginate::Paginate;
use crate::query::Query;
use crate::request::Request;
use crate::storage::v1::{
    Object, PatchObjectRequest, QueryWriteStatusRequest, QueryWriteStatusResponse,
    StartResumableWriteResponse,
};
use crate::Client;
use crate::Result;
use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Method, Url};
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::pin::Pin;
use std::str::FromStr;
use tracing::Instrument;

impl FromStr for Object {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        let url = s.parse::<Url>()?;

        url.try_into()
    }
}

/// Convert gs://bucket/prefix Urls to an Object
impl TryFrom<Url> for Object {
    type Error = crate::Error;

    fn try_from(value: Url) -> Result<Self> {
        if value.scheme() != "gs" {
            return Err(crate::Error::Other {
                source: "Unexpected scheme {}".into(),
                #[cfg(feature = "backtrace")]
                backtrace: std::backtrace::Backtrace::capture(),
            });
        }

        Ok(Object {
            bucket: value.host_str().unwrap_or_default().to_string(),
            name: value.path().to_string(),
            ..Default::default()
        })
    }
}

impl Query for CommonObjectRequestParams {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        vec![
            (
                "x-goog-encryption-algorithm",
                self.encryption_algorithm.clone(),
            ),
            (
                "x-goog-encryption-key",
                base64::encode(self.encryption_key.as_bytes()),
            ),
            (
                "x-goog-encryption-key-sha256",
                base64::encode(self.encryption_key_sha256.as_bytes()),
            ),
        ]
    }
}

impl Query for InsertObjectRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let insert_object_spec = match self.first_message.as_ref().unwrap() {
            FirstMessage::UploadId(_) => panic!(),
            FirstMessage::InsertObjectSpec(spec) => spec,
        };

        let resource = insert_object_spec.resource.as_ref().unwrap();

        let mut query = self.common_request_params.request_query();
        query.extend(self.common_object_request_params.request_query());
        query.push(("uploadType", "media".to_string()));
        query.push(("name", percent_encode(&resource.name)));
        query
    }
}

impl Request for InsertObjectRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = Object;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        let insert_object_spec = match self.first_message.as_ref().unwrap() {
            FirstMessage::UploadId(_) => panic!(),
            FirstMessage::InsertObjectSpec(spec) => spec,
        };

        let resource = insert_object_spec.resource.as_ref().unwrap();
        let base_url = base_url.join("/upload/storage/v1/")?;

        Ok(bucket_url(&base_url, &resource.bucket)?.join_segment("o")?)
    }

    fn request_headers(&self) -> HeaderMap<HeaderValue> {
        unimplemented!()
    }
}

impl Query for GetObjectRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();
        query.extend(self.common_object_request_params.request_query());
        query.extend(Projection::from_i32(self.projection).request_query());
        unimplemented!()
    }
}

impl Request for GetObjectRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = Object;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        object_url(base_url, self.bucket.as_str(), self.object.as_str())
    }
}

impl From<Object> for GetObjectRequest {
    fn from(value: Object) -> Self {
        GetObjectRequest {
            bucket: value.bucket,
            object: value.name,
            ..Default::default()
        }
    }
}

/// Convert gs://bucket/prefix Urls to a GetObjectRequest
impl TryFrom<Url> for GetObjectRequest {
    type Error = crate::Error;

    fn try_from(value: Url) -> Result<Self> {
        let object: Object = value.try_into()?;
        Ok(object.into())
    }
}

impl FromStr for GetObjectRequest {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self> {
        Ok(value.parse::<Object>()?.into())
    }
}

impl Query for ComposeObjectRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        if let Some(if_metageneration_match) = self.if_metageneration_match {
            query.push(("ifMetagenerationMatch", if_metageneration_match.to_string()));
        }

        if let Some(if_generation_match) = self.if_generation_match {
            query.push(("ifGenerationMatch", if_generation_match.to_string()));
        }

        query.extend(
            PredefinedObjectAcl::from_i32(self.destination_predefined_acl)
                .request_query()
                .into_iter()
                .map(|(_, v)| ("destinationPredefinedAcl", v)),
        );

        if !self.kms_key_name.is_empty() {
            query.push(("kmsKeyName", self.kms_key_name.clone()));
        }

        query
    }
}

impl Request for ComposeObjectRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = Object;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        object_url(
            base_url,
            self.destination_bucket.as_str(),
            self.destination_object.as_str(),
        )
    }
}

impl Query for CopyObjectRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();
        query.extend(self.common_object_request_params.request_query());

        if !self.destination_kms_key_name.is_empty() {
            query.push((
                "destinationKmsKeyName",
                self.destination_kms_key_name.clone(),
            ));
        }

        query.extend(
            PredefinedObjectAcl::from_i32(self.destination_predefined_acl)
                .request_query()
                .into_iter()
                .map(|(_, v)| ("destinationPredefinedAcl", v)),
        );

        if let Some(if_generation_match) = self.if_generation_match {
            query.push(("ifGenerationMatch", if_generation_match.to_string()));
        }

        if let Some(if_generation_not_match) = self.if_generation_not_match {
            query.push(("ifGenerationNotMatch", if_generation_not_match.to_string()));
        }

        if let Some(if_metageneration_match) = self.if_metageneration_match {
            query.push(("ifMetagenerationMatch", if_metageneration_match.to_string()));
        }

        if let Some(if_metageneration_not_match) = self.if_metageneration_not_match {
            query.push((
                "ifMetagenerationNotMatch",
                if_metageneration_not_match.to_string(),
            ));
        }

        if let Some(if_source_generation_match) = self.if_source_generation_match {
            query.push((
                "ifSourceGenerationMatch",
                if_source_generation_match.to_string(),
            ));
        }

        if let Some(if_source_generation_not_match) = self.if_source_generation_not_match {
            query.push((
                "ifSourceGenerationNotMatch",
                if_source_generation_not_match.to_string(),
            ));
        }

        if let Some(if_source_metageneration_match) = self.if_source_metageneration_match {
            query.push((
                "ifSourceMetagenerationMatch",
                if_source_metageneration_match.to_string(),
            ));
        }

        if let Some(if_source_metageneration_not_match) = self.if_source_metageneration_not_match {
            query.push((
                "ifSourceMetagenerationNotMatch",
                if_source_metageneration_not_match.to_string(),
            ));
        }

        query.extend(Projection::from_i32(self.projection).request_query());

        if self.source_generation != 0 {
            query.push(("sourceGeneration", self.source_generation.to_string()));
        }

        query
    }
}

impl Request for CopyObjectRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = Object;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        let url = object_url(base_url, &self.source_bucket, &self.source_object)?
            .join_segment("copyTo")?;

        Ok(object_url(
            &url,
            &self.destination_bucket,
            &self.destination_object,
        )?)
    }
}

impl Query for RewriteObjectRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();
        query.extend(self.common_object_request_params.request_query());

        if !self.destination_kms_key_name.is_empty() {
            query.push((
                "destinationKmsKeyName",
                self.destination_kms_key_name.clone(),
            ));
        }

        query.extend(
            PredefinedObjectAcl::from_i32(self.destination_predefined_acl)
                .request_query()
                .into_iter()
                .map(|(_, v)| ("destinationPredefinedAcl", v)),
        );

        if let Some(if_generation_match) = self.if_generation_match {
            query.push(("ifGenerationMatch", if_generation_match.to_string()));
        }

        if let Some(if_generation_not_match) = self.if_generation_not_match {
            query.push(("ifGenerationNotMatch", if_generation_not_match.to_string()));
        }

        if let Some(if_metageneration_match) = self.if_metageneration_match {
            query.push(("ifMetagenerationMatch", if_metageneration_match.to_string()));
        }

        if let Some(if_metageneration_not_match) = self.if_metageneration_not_match {
            query.push((
                "ifMetagenerationNotMatch",
                if_metageneration_not_match.to_string(),
            ));
        }

        if let Some(if_source_generation_match) = self.if_source_generation_match {
            query.push((
                "ifSourceGenerationMatch",
                if_source_generation_match.to_string(),
            ));
        }

        if let Some(if_source_generation_not_match) = self.if_source_generation_not_match {
            query.push((
                "ifSourceGenerationNotMatch",
                if_source_generation_not_match.to_string(),
            ));
        }

        if let Some(if_source_metageneration_match) = self.if_source_metageneration_match {
            query.push((
                "ifSourceMetagenerationMatch",
                if_source_metageneration_match.to_string(),
            ));
        }

        if let Some(if_source_metageneration_not_match) = self.if_source_metageneration_not_match {
            query.push((
                "ifSourceMetagenerationNotMatch",
                if_source_metageneration_not_match.to_string(),
            ));
        }

        if self.max_bytes_rewritten_per_call != 0 {
            query.push((
                "maxBytesRewrittenPerCall",
                self.max_bytes_rewritten_per_call.to_string(),
            ));
        }

        query.extend(Projection::from_i32(self.projection).request_query());

        if !self.rewrite_token.is_empty() {
            query.push(("rewriteToken", self.rewrite_token.clone()));
        }

        if self.source_generation != 0 {
            query.push(("sourceGeneration", self.source_generation.to_string()));
        }

        query
    }
}

impl Request for RewriteObjectRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = RewriteResponse;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        let url = object_url(base_url, &self.source_bucket, &self.source_object)?
            .join_segment("rewriteTo")?;

        Ok(object_url(
            &url,
            &self.destination_bucket,
            &self.destination_object,
        )?)
    }
}

impl Query for GetObjectMediaRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();
        query.extend(self.common_object_request_params.request_query());

        query.push(("alt", "media".to_string()));

        if self.generation != 0 {
            query.push(("generation", self.generation.to_string()));
        }

        if let Some(if_generation_match) = self.if_generation_match {
            query.push(("ifGenerationMatch", if_generation_match.to_string()));
        }

        if let Some(if_generation_not_match) = self.if_generation_not_match {
            query.push(("ifGenerationNotMatch", if_generation_not_match.to_string()));
        }

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

#[derive(serde::Deserialize)]
pub struct Void;

impl Request for GetObjectMediaRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = Void;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        object_url(base_url, &self.bucket, &self.object)
    }
}

impl From<Object> for GetObjectMediaRequest {
    fn from(value: Object) -> Self {
        GetObjectMediaRequest {
            bucket: value.bucket,
            object: value.name,
            ..Default::default()
        }
    }
}

/// Convert gs://bucket/prefix Urls to a GetObjectMediaRequest
impl TryFrom<Url> for GetObjectMediaRequest {
    type Error = crate::Error;

    fn try_from(value: Url) -> Result<GetObjectMediaRequest> {
        let object: Object = value.try_into()?;
        Ok(object.into())
    }
}

impl FromStr for GetObjectMediaRequest {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self> {
        Ok(value.parse::<Object>()?.into())
    }
}

impl Query for DeleteObjectRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();
        query.extend(self.common_object_request_params.request_query());

        if self.generation != 0 {
            query.push(("generation", self.generation.to_string()));
        }

        if let Some(if_generation_match) = self.if_generation_match {
            query.push(("ifGenerationMatch", if_generation_match.to_string()));
        }

        if let Some(if_generation_not_match) = self.if_generation_not_match {
            query.push(("ifGenerationNotMatch", if_generation_not_match.to_string()));
        }

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

impl Request for DeleteObjectRequest {
    const REQUEST_METHOD: Method = Method::DELETE;

    type Response = ();

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        object_url(base_url, &self.bucket, &self.object)
    }
}

impl From<Object> for DeleteObjectRequest {
    fn from(value: Object) -> Self {
        DeleteObjectRequest {
            bucket: value.bucket,
            object: value.name,
            ..Default::default()
        }
    }
}

/// Convert gs://bucket/prefix Urls to a DeleteObjectRequest
impl TryFrom<Url> for DeleteObjectRequest {
    type Error = crate::Error;

    fn try_from(value: Url) -> Result<DeleteObjectRequest> {
        let object: Object = value.try_into()?;
        Ok(object.into())
    }
}

impl FromStr for DeleteObjectRequest {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(s.parse::<Object>()?.into())
    }
}

impl Query for UpdateObjectRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        unimplemented!()
    }
}

impl Request for UpdateObjectRequest {
    const REQUEST_METHOD: Method = Method::PUT;

    type Response = Object;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        object_url(base_url, &self.bucket, &self.object)
    }
}

impl Query for ListObjectsRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        if !self.delimiter.is_empty() {
            query.push(("delimiter", self.delimiter.clone()));
        }

        if self.include_trailing_delimiter {
            query.push((
                "includeTrailingDelimiter",
                self.include_trailing_delimiter.to_string(),
            ));
        }

        if self.max_results != 0 {
            query.push(("maxResults", self.max_results.to_string()));
        }

        if !self.page_token.is_empty() {
            query.push(("pageToken", self.page_token.clone()));
        }

        if !self.prefix.is_empty() {
            query.push(("prefix", self.prefix.clone()));
        }

        query.extend(Projection::from_i32(self.projection).request_query());

        if self.versions {
            query.push(("versions", self.versions.to_string()));
        }

        query
    }
}

impl Request for ListObjectsRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = ListObjectsResponse;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(bucket_url(base_url, &self.bucket)?.join_segment("o")?)
    }
}

impl<'a> Paginate<'a> for ListObjectsRequest {
    type Item = Object;

    fn extract_items(response: ListObjectsResponse) -> Vec<Self::Item> {
        response.items
    }

    fn into_request(self, response: &ListObjectsResponse) -> Option<Self> {
        if response.next_page_token.is_empty() {
            None
        } else {
            Some(ListObjectsRequest {
                page_token: response.next_page_token.clone(),
                ..self
            })
        }
    }
}

impl Into<ListObjectsRequest> for Object {
    fn into(self) -> ListObjectsRequest {
        ListObjectsRequest {
            bucket: self.bucket,
            prefix: self.name,
            ..Default::default()
        }
    }
}

/// Convert gs://bucket/prefix Urls to a ListObjectsRequest
impl TryInto<ListObjectsRequest> for Url {
    type Error = crate::Error;

    fn try_into(self) -> Result<ListObjectsRequest> {
        let object: Object = self.try_into()?;
        Ok(object.into())
    }
}

impl FromStr for ListObjectsRequest {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        let url = s.parse::<Url>()?;

        url.try_into()
    }
}

impl Into<ListObjectsRequest> for Bucket {
    fn into(self) -> ListObjectsRequest {
        ListObjectsRequest {
            bucket: self.name,
            ..Default::default()
        }
    }
}

impl Client {
    #[doc = " Stores a new object and metadata."]
    #[doc = ""]
    #[doc = " An object can be written either in a single message stream or in a"]
    #[doc = " resumable sequence of message streams. To write using a single stream,"]
    #[doc = " the client should include in the first message of the stream an"]
    #[doc = " `InsertObjectSpec` describing the destination bucket, object, and any"]
    #[doc = " preconditions. Additionally, the final message must set 'finish_write' to"]
    #[doc = " true, or else it is an error."]
    #[doc = ""]
    #[doc = " For a resumable write, the client should instead call"]
    #[doc = " `StartResumableWrite()` and provide that method an `InsertObjectSpec.`"]
    #[doc = " They should then attach the returned `upload_id` to the first message of"]
    #[doc = " each following call to `Insert`. If there is an error or the connection is"]
    #[doc = " broken during the resumable `Insert()`, the client should check the status"]
    #[doc = " of the `Insert()` by calling `QueryWriteStatus()` and continue writing from"]
    #[doc = " the returned `committed_size`. This may be less than the amount of data the"]
    #[doc = " client previously sent."]
    #[doc = ""]
    #[doc = " The service will not view the object as complete until the client has"]
    #[doc = " sent an `Insert` with `finish_write` set to `true`. Sending any"]
    #[doc = " requests on a stream after sending a request with `finish_write` set to"]
    #[doc = " `true` will cause an error. The client **should** check the"]
    #[doc = " `Object` it receives to determine how much data the service was"]
    #[doc = " able to commit and whether the service views the object as complete."]
    #[tracing::instrument]
    pub async fn insert_object(
        &self,
        _request: impl Into<InsertObjectRequest> + Debug,
    ) -> crate::Result<Object> {
        unimplemented!()
    }

    #[tracing::instrument]
    pub async fn insert_object_stream<S: Stream<Item = Bytes> + Debug>(
        &self,
        first_message: FirstMessage,
        request: impl Into<S> + Debug,
    ) -> crate::Result<Object> {
        unimplemented!();
    }

    #[doc = " Retrieves a list of objects matching the criteria."]
    #[tracing::instrument]
    pub async fn list_objects<'a>(
        &self,
        request: impl Into<ListObjectsRequest> + Debug,
    ) -> Result<ListObjectsResponse> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Retrieves a list of objects matching the criteria."]
    #[tracing::instrument]
    pub async fn list_objects_stream<'a>(
        &'a self,
        request: impl Into<ListObjectsRequest> + Debug + 'a,
    ) -> Pin<Box<dyn Stream<Item = Result<Object>> + 'a>> {
        self.paginate(request.into())
    }

    #[doc = " Retrieves a list of objects matching the criteria."]
    #[tracing::instrument]
    pub async fn list_objects_vec(
        &self,
        request: impl Into<ListObjectsRequest> + Debug,
    ) -> Result<Vec<Object>> {
        self.list_objects_stream(request)
            .await
            .try_collect()
            .instrument(tracing::trace_span!("try_collect"))
            .await
    }

    #[doc = " Retrieves an object's metadata."]
    #[tracing::instrument]
    pub async fn get_object(
        &self,
        request: impl Into<GetObjectRequest> + Debug,
    ) -> crate::Result<Object> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Reads an object's data."]
    #[tracing::instrument]
    pub async fn get_object_media_bytes(
        &self,
        request: impl Into<GetObjectMediaRequest> + Debug,
    ) -> Result<Vec<u8>> {
        let request = request.into();

        Ok(self.get(&request).await?.bytes().await?.to_vec())
    }

    #[doc = " Reads an object's data."]
    #[tracing::instrument]
    pub async fn get_object_media_stream(
        &self,
        request: impl Into<GetObjectMediaRequest> + Debug,
    ) -> crate::Result<impl Stream<Item = crate::Result<Bytes>> + Unpin> {
        let request = request.into();

        Ok(self
            .get(&request)
            .await?
            .bytes_stream()
            .map_err(|e| crate::Error::Reqwest {
                source: e,
                #[cfg(feature = "backtrace")]
                backtrace: std::backtrace::Backtrace::capture(),
            }))
    }

    #[doc = " Updates an object's metadata."]
    pub async fn patch_object(
        &mut self,
        _request: impl Into<PatchObjectRequest>,
    ) -> Result<Object> {
        unimplemented!()
    }

    #[doc = " Updates an object's metadata. Equivalent to PatchObject, but always"]
    #[doc = " replaces all mutatable fields of the bucket with new values, reverting all"]
    #[doc = " unspecified fields to their default values."]
    #[tracing::instrument]
    pub async fn update_object(
        &self,
        request: impl Into<UpdateObjectRequest> + Debug,
    ) -> crate::Result<Object> {
        let request = request.into();

        self.invoke_json(&request, &request.metadata).await
    }

    #[doc = " Deletes an object and its metadata. Deletions are permanent if versioning"]
    #[doc = " is not enabled for the bucket, or if the `generation` parameter"]
    #[doc = " is used."]
    #[tracing::instrument]
    pub async fn delete_object(
        &self,
        request: impl Into<DeleteObjectRequest> + Debug,
    ) -> Result<()> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Concatenates a list of existing objects into a new object in the same"]
    #[doc = " bucket."]
    #[tracing::instrument]
    pub async fn compose_object(
        &self,
        request: impl Into<ComposeObjectRequest> + Debug,
    ) -> crate::Result<Object> {
        let request = request.into();

        #[derive(Debug, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ComposeRequest {
            kind: &'static str,
            source_objects: Vec<SourceObjects>,
            destination: Object,
        }

        let body = ComposeRequest {
            kind: "storage#composeRequest",
            source_objects: request.source_objects.clone(),
            destination: Object {
                name: request.destination_object.clone(),
                bucket: request.destination_bucket.clone(),
                ..request.destination.clone().unwrap_or_default()
            },
        };

        self.invoke_json(&request, &body).await
    }

    #[doc = " Copies a source object to a destination object. Optionally overrides"]
    #[doc = " metadata."]
    #[tracing::instrument]
    pub async fn copy_object(
        &self,
        request: impl Into<CopyObjectRequest> + Debug,
    ) -> crate::Result<Object> {
        let request = request.into();

        self.invoke_json(&request, &request.destination).await
    }

    #[doc = " Rewrites a source object to a destination object. Optionally overrides"]
    #[doc = " metadata."]
    #[tracing::instrument]
    pub async fn rewrite_object(
        &self,
        request: impl Into<RewriteObjectRequest> + Debug,
    ) -> crate::Result<RewriteResponse> {
        let request = request.into();

        self.invoke_json(&request, &request.object).await
    }

    #[doc = " Starts a resumable write. How long the write operation remains valid, and"]
    #[doc = " what happens when the write operation becomes invalid, are"]
    #[doc = " service-dependent."]
    #[tracing::instrument]
    pub async fn start_resumable_write(
        &self,
        _request: impl Into<StartResumableWriteRequest> + Debug,
    ) -> Result<StartResumableWriteResponse> {
        unimplemented!()
    }

    #[doc = " Determines the `committed_size` for an object that is being written, which"]
    #[doc = " can then be used as the `write_offset` for the next `Write()` call."]
    #[doc = ""]
    #[doc = " If the object does not exist (i.e., the object has been deleted, or the"]
    #[doc = " first `Write()` has not yet reached the service), this method returns the"]
    #[doc = " error `NOT_FOUND`."]
    #[doc = ""]
    #[doc = " The client **may** call `QueryWriteStatus()` at any time to determine how"]
    #[doc = " much data has been processed for this object. This is useful if the"]
    #[doc = " client is buffering data and needs to know which data can be safely"]
    #[doc = " evicted. For any sequence of `QueryWriteStatus()` calls for a given"]
    #[doc = " object name, the sequence of returned `committed_size` values will be"]
    #[doc = " non-decreasing."]
    #[tracing::instrument]
    pub async fn query_write_status(
        &mut self,
        _request: impl Into<QueryWriteStatusRequest> + Debug,
    ) -> Result<QueryWriteStatusResponse> {
        unimplemented!()
    }
}
