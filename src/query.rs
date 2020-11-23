use crate::google::storage::v1::common_enums::{
    PredefinedBucketAcl, PredefinedObjectAcl, Projection,
};
use crate::google::storage::v1::CommonRequestParams;

pub(crate) trait Query {
    fn request_query(&self) -> Vec<(&'static str, String)>;
}

impl<T: Query> Query for Option<T> {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        self.as_ref().map(|q| q.request_query()).unwrap_or_default()
    }
}

impl Query for CommonRequestParams {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if !self.quota_user.is_empty() {
            query.push(("quotaUser", self.quota_user.clone()));
        }
        if let Some(ref fields) = self.fields {
            query.push(("fields", fields.paths.join(",")));
        }

        query
    }
}

impl Query for PredefinedBucketAcl {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        use PredefinedBucketAcl::*;

        if let Some(predefined_acl) = match self {
            Unspecified => None,
            BucketAclAuthenticatedRead => Some("authenticatedRead"),
            BucketAclPrivate => Some("private"),
            BucketAclProjectPrivate => Some("projectPrivate"),
            BucketAclPublicRead => Some("publicRead"),
            BucketAclPublicReadWrite => Some("publicReadWrite"),
        } {
            vec![("predefinedAcl", predefined_acl.to_string())]
        } else {
            vec![]
        }
    }
}

impl Query for PredefinedObjectAcl {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        use PredefinedObjectAcl::*;
        if let Some(predefined_default_object_acl) = match self {
            Unspecified => None,
            ObjectAclAuthenticatedRead => Some("authenticatedRead"),
            ObjectAclBucketOwnerFullControl => Some("bucketOwnerFullControl"),
            ObjectAclBucketOwnerRead => Some("bucketOwnerRead"),
            ObjectAclPrivate => Some("private"),
            ObjectAclProjectPrivate => Some("projectPrivate"),
            ObjectAclPublicRead => Some("publicRead"),
        } {
            vec![(
                "predefinedDefaultObjectAcl",
                predefined_default_object_acl.to_string(),
            )]
        } else {
            vec![]
        }
    }
}

impl Query for Projection {
    fn request_query(&self) -> Vec<(&'static str, String)> {
        use Projection::*;
        if let Some(projection) = match self {
            Unspecified => None,
            NoAcl => Some("noAcl"),
            Full => Some("full"),
        } {
            vec![("projection", projection.to_string())]
        } else {
            vec![]
        }
    }
}
