use crate::google::storage::v1::common_enums::{
    PredefinedBucketAcl, PredefinedObjectAcl, Projection,
};
use crate::google::storage::v1::{
    DeleteBucketRequest, GetBucketRequest, InsertBucketRequest, ListBucketsRequest,
    ListBucketsResponse, UpdateBucketRequest,
};
use crate::paginate::Paginate;
use crate::query::Query;
use crate::request::Request;
use crate::storage::v1::{Bucket, Object, PatchBucketRequest};
use crate::urls::Urls;
use crate::{push_enum, push_if, push_if_opt, Client, Result};
use futures::{Stream, TryStreamExt};
use reqwest::{Method, Url};
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::mem;
use std::pin::Pin;
use std::str::FromStr;
use tracing::Instrument;

impl FromStr for Bucket {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self> {
        let object = value.parse::<Object>()?;
        Ok(Bucket {
            name: object.bucket,
            ..Default::default()
        })
    }
}

impl TryFrom<Url> for Bucket {
    type Error = crate::Error;

    fn try_from(value: Url) -> Result<Self> {
        let object: Object = value.try_into()?;
        Ok(object.into())
    }
}

impl From<Object> for Bucket {
    fn from(value: Object) -> Self {
        Bucket {
            name: value.bucket,
            ..Default::default()
        }
    }
}

impl Query for ListBucketsRequest {
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        push_if!(self, query, project);
        push_if!(self, query, max_results);
        push_if!(self, query, page_token);
        push_if!(self, query, prefix);

        push_enum!(self, query, Projection, projection);

        query
    }
}

impl Request for ListBucketsRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = ListBucketsResponse;

    fn request_path(&self, base_url: Url) -> Result<Url> {
        Ok(base_url.join("b")?)
    }
}

impl<'a> Paginate<'a> for ListBucketsRequest {
    type Item = Bucket;

    fn extract_items(response: ListBucketsResponse) -> Vec<Self::Item> {
        response.items
    }

    fn into_request(self, response: &ListBucketsResponse) -> Option<Self> {
        if response.next_page_token.is_empty() {
            None
        } else {
            Some(ListBucketsRequest {
                page_token: response.next_page_token.clone(),
                ..self
            })
        }
    }
}

impl Query for InsertBucketRequest {
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.take().request_query();

        push_if!(self, query, project);

        push_enum!(self, query, PredefinedBucketAcl, predefined_acl);
        push_enum!(
            self,
            query,
            PredefinedObjectAcl,
            predefined_default_object_acl
        );
        push_enum!(self, query, Projection, projection);

        query
    }
}

impl Request for InsertBucketRequest {
    const REQUEST_METHOD: Method = Method::POST;

    type Response = Bucket;

    fn request_path(&self, base_url: Url) -> Result<Url> {
        Ok(base_url.join("b")?)
    }
}

impl From<Bucket> for InsertBucketRequest {
    fn from(value: Bucket) -> Self {
        InsertBucketRequest {
            bucket: Some(value),
            ..Default::default()
        }
    }
}

impl Query for DeleteBucketRequest {
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        push_if_opt!(self, query, if_metageneration_match);
        push_if_opt!(self, query, if_metageneration_not_match);

        query
    }
}

impl Request for DeleteBucketRequest {
    const REQUEST_METHOD: Method = Method::DELETE;

    type Response = ();

    fn request_path(&self, base_url: Url) -> Result<Url> {
        base_url.bucket(&self.bucket)
    }
}

impl From<Bucket> for DeleteBucketRequest {
    fn from(value: Bucket) -> Self {
        DeleteBucketRequest {
            bucket: value.name,
            if_metageneration_match: Some(value.metageneration),
            ..Default::default()
        }
    }
}

impl FromStr for DeleteBucketRequest {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        let bucket = s.parse::<Bucket>()?;

        Ok(bucket.into())
    }
}

impl Query for GetBucketRequest {
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        push_if_opt!(self, query, if_metageneration_match);
        push_if_opt!(self, query, if_metageneration_not_match);

        push_enum!(self, query, Projection, projection);

        query
    }
}

impl Request for GetBucketRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = Bucket;

    fn request_path(&self, base_url: Url) -> Result<Url> {
        base_url.bucket(&self.bucket)
    }
}

/// Convert gs://bucket/prefix Urls to a GetBucketRequest
impl TryInto<GetBucketRequest> for Url {
    type Error = crate::Error;

    fn try_into(self) -> Result<GetBucketRequest> {
        let bucket: Bucket = self.try_into()?;
        Ok(bucket.into())
    }
}

impl FromStr for GetBucketRequest {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        let bucket = s.parse::<Bucket>()?;

        Ok(bucket.into())
    }
}

impl From<Bucket> for GetBucketRequest {
    fn from(value: Bucket) -> Self {
        GetBucketRequest {
            bucket: value.name,
            if_metageneration_match: Some(value.metageneration),
            ..Default::default()
        }
    }
}

impl Query for UpdateBucketRequest {
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        push_if_opt!(self, query, if_metageneration_match);
        push_if_opt!(self, query, if_metageneration_not_match);

        push_enum!(self, query, PredefinedBucketAcl, predefined_acl);
        push_enum!(
            self,
            query,
            PredefinedObjectAcl,
            predefined_default_object_acl
        );
        push_enum!(self, query, Projection, projection);

        query
    }
}

impl Request for UpdateBucketRequest {
    const REQUEST_METHOD: Method = Method::PUT;

    type Response = Bucket;

    fn request_path(&self, base_url: Url) -> Result<Url> {
        base_url.bucket(&self.bucket)
    }
}

impl From<Bucket> for UpdateBucketRequest {
    fn from(value: Bucket) -> Self {
        let if_metageneration_match = Some(value.metageneration);

        UpdateBucketRequest {
            bucket: value.name.clone(),
            metadata: Some(value),
            if_metageneration_match,
            ..Default::default()
        }
    }
}

impl Client {
    #[doc = " Creates a new bucket."]
    #[tracing::instrument]
    pub async fn insert_bucket(
        &self,
        request: impl Into<InsertBucketRequest> + Debug,
    ) -> Result<Bucket> {
        let mut request = request.into();

        let bucket = request.bucket.take();

        self.invoke_json(request, bucket).await
    }

    #[doc = " Retrieves a list of buckets for a given project."]
    #[tracing::instrument]
    pub async fn list_buckets(
        &self,
        request: impl Into<ListBucketsRequest> + Debug,
    ) -> Result<ListBucketsResponse> {
        let request = request.into();

        self.invoke(request).await
    }

    #[doc = " Retrieves a list of buckets for a given project."]
    #[tracing::instrument]
    pub async fn list_buckets_stream<'a>(
        &'a self,
        request: impl Into<ListBucketsRequest> + Debug,
    ) -> Pin<Box<dyn Stream<Item = Result<Bucket>> + 'a>> {
        self.paginate(request.into())
    }

    #[doc = " Retrieves a list of buckets for a given project."]
    #[tracing::instrument]
    pub async fn list_buckets_vec(
        &self,
        request: impl Into<ListBucketsRequest> + Debug,
    ) -> Result<Vec<Bucket>> {
        self.list_buckets_stream(request)
            .await
            .try_collect()
            .instrument(tracing::trace_span!("try_collect"))
            .await
    }

    #[doc = " Returns metadata for the specified bucket."]
    #[tracing::instrument]
    pub async fn get_bucket(
        &self,
        request: impl Into<GetBucketRequest> + Debug,
    ) -> crate::Result<Bucket> {
        let request = request.into();

        self.invoke(request).await
    }

    #[doc = " Updates a bucket. Changes to the bucket will be readable immediately after"]
    #[doc = " writing, but configuration changes may take time to propagate."]
    pub async fn patch_bucket(
        &mut self,
        _request: impl Into<PatchBucketRequest>,
    ) -> Result<Bucket> {
        unimplemented!()
    }

    #[doc = " Updates a bucket. Equivalent to PatchBucket, but always replaces all"]
    #[doc = " mutatable fields of the bucket with new values, reverting all"]
    #[doc = " unspecified fields to their default values."]
    #[doc = " Like PatchBucket, Changes to the bucket will be readable immediately after"]
    #[doc = " writing, but configuration changes may take time to propagate."]
    #[tracing::instrument]
    pub async fn update_bucket(
        &self,
        request: impl Into<UpdateBucketRequest> + Debug,
    ) -> crate::Result<Bucket> {
        let mut request = request.into();

        let metadata = request.metadata.take();

        self.invoke_json(request, metadata).await
    }

    #[doc = " Permanently deletes an empty bucket."]
    #[tracing::instrument]
    pub async fn delete_bucket(
        &self,
        request: impl Into<DeleteBucketRequest> + Debug,
    ) -> crate::Result<()> {
        let request = request.into();

        self.invoke(request).await
    }
}
