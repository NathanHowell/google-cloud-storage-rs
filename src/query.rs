use crate::google::storage::v1::common_enums::{
    PredefinedBucketAcl, PredefinedObjectAcl, Projection,
};
use crate::google::storage::v1::CommonRequestParams;
use crate::push_if;
use std::mem;

pub(crate) trait Query {
    fn request_query(&mut self) -> Vec<(&'static str, String)>;
}

impl<T: Query> Query for Option<T> {
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
        self.take()
            .as_mut()
            .map(|q| q.request_query())
            .unwrap_or_default()
    }
}

impl Query for CommonRequestParams {
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();

        push_if!(self, query, quota_user);

        if let Some(ref fields) = self.fields.take() {
            query.push(("fields", fields.paths.join(",")));
        }

        query
    }
}

impl Query for PredefinedBucketAcl {
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
        use PredefinedBucketAcl::*;

        if let Some(predefined_acl) = match mem::take(self) {
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
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
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
            *self = Unspecified;
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
    fn request_query(&mut self) -> Vec<(&'static str, String)> {
        use Projection::*;
        if let Some(projection) = match self {
            Unspecified => None,
            NoAcl => Some("noAcl"),
            Full => Some("full"),
        } {
            *self = Unspecified;
            vec![("projection", projection.to_string())]
        } else {
            vec![]
        }
    }
}

pub(crate) trait PushIf<T> {
    fn push_if(&mut self, key: &'static str, value: &mut T);
    fn push_if_opt(&mut self, key: &'static str, value: &mut Option<T>);
}

impl<T: Default + PartialEq + ToString> PushIf<T> for Vec<(&'static str, String)> {
    fn push_if(&mut self, key: &'static str, value: &mut T) {
        if value != &Default::default() {
            self.push((key, value.to_string()));
        }
    }

    fn push_if_opt(&mut self, key: &'static str, value: &mut Option<T>) {
        match value.take().as_mut() {
            Some(value) => self.push_if(key, value),
            None => {}
        }
    }
}
