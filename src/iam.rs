use crate::client::bucket_url;
use crate::google::iam::v1::{Policy, TestIamPermissionsResponse};
use crate::google::storage::v1::{
    GetIamPolicyRequest, SetIamPolicyRequest, TestIamPermissionsRequest,
};
use crate::query::Query;
use crate::request::Request;
use crate::{Client, Result};
use reqwest::Method;
use std::fmt::Debug;
use url::Url;

pub(crate) const FULL_CONTROL: &'static str =
    "https://www.googleapis.com/auth/devstorage.full_control";

fn iam_url<'a, R, F>(base_url: &Url, iam_request: Option<&'a R>, resource: F) -> Result<Url>
where
    F: FnOnce(&'a R) -> &'a str,
{
    let request = iam_request.ok_or(crate::Error::Other {
        source: "Expected iam_request field".into(),
        #[cfg(feature = "backtrace")]
        backtrace: std::backtrace::Backtrace::capture(),
    })?;

    let resource = resource(request);

    Ok(bucket_url(base_url, resource)?.join("iam")?)
}

impl Query for GetIamPolicyRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        if let Some(requested_policy_version) = self
            .iam_request
            .as_ref()
            .map(|r| r.options.as_ref())
            .flatten()
            .map(|o| o.requested_policy_version)
        {
            query.push((
                "optionsRequestedPolicyVersion",
                requested_policy_version.to_string(),
            ));
        }

        query
    }
}

impl Request for GetIamPolicyRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = Policy;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        iam_url(base_url, self.iam_request.as_ref(), |r| &r.resource)
    }
}

impl Query for SetIamPolicyRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.common_request_params.request_query()
    }
}

impl Request for SetIamPolicyRequest {
    const REQUEST_METHOD: Method = Method::PUT;

    type Response = Policy;

    fn scope(&self) -> &'static str {
        FULL_CONTROL
    }

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        iam_url(base_url, self.iam_request.as_ref(), |r| &r.resource)
    }
}

impl Query for TestIamPermissionsRequest {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = self.common_request_params.request_query();

        query.extend(
            self.iam_request
                .as_ref()
                .map(|request| {
                    request
                        .permissions
                        .iter()
                        .map(|v| ("permissions", v.clone()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
        );

        query
    }
}

impl Request for TestIamPermissionsRequest {
    const REQUEST_METHOD: Method = Method::GET;

    type Response = TestIamPermissionsResponse;

    fn request_path(&self, base_url: &Url) -> Result<Url> {
        Ok(
            iam_url(base_url, self.iam_request.as_ref(), |r| &r.resource)?
                .join("testPermissions")?,
        )
    }
}

impl Client {
    #[doc = " Gets the IAM policy for the specified bucket."]
    #[tracing::instrument]
    pub async fn get_bucket_iam_policy(
        &self,
        request: impl Into<GetIamPolicyRequest> + Debug,
    ) -> crate::Result<Policy> {
        let request = request.into();

        self.invoke(&request).await
    }

    #[doc = " Updates an IAM policy for the specified bucket."]
    #[tracing::instrument]
    pub async fn set_bucket_iam_policy(
        &self,
        request: impl Into<SetIamPolicyRequest> + Debug,
    ) -> crate::Result<Policy> {
        let request = request.into();

        let policy = match request.iam_request {
            Some(ref k) => k.policy.as_ref(),
            None => None,
        };

        self.invoke_json(&request, &policy).await
    }

    #[doc = " Tests a set of permissions on the given bucket to see which, if"]
    #[doc = " any, are held by the caller."]
    #[tracing::instrument]
    pub async fn test_bucket_iam_permissions(
        &self,
        request: impl Into<TestIamPermissionsRequest> + Debug,
    ) -> crate::Result<TestIamPermissionsResponse> {
        let request = request.into();

        self.invoke(&request).await
    }
}
