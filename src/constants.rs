#![allow(non_upper_case_globals)]

pub(crate) const delimiter: &str = "delimiter";
pub(crate) const destination_kms_key_name: &str = "destinationKmsKeyName";
pub(crate) const destination_predefined_acl: &str = "destinationPredefinedAcl";
pub(crate) const generation: &str = "generation";
pub(crate) const if_generation_match: &str = "ifGenerationMatch";
pub(crate) const if_generation_not_match: &str = "ifGenerationNotMatch";
pub(crate) const if_metageneration_match: &str = "ifMetagenerationMatch";
pub(crate) const if_metageneration_not_match: &str = "ifMetagenerationNotMatch";
pub(crate) const if_source_generation_match: &str = "ifSourceGenerationMatch";
pub(crate) const if_source_generation_not_match: &str = "ifSourceGenerationNotMatch";
pub(crate) const if_source_metageneration_match: &str = "ifSourceMetagenerationMatch";
pub(crate) const if_source_metageneration_not_match: &str = "ifSourceMetagenerationNotMatch";
pub(crate) const include_trailing_delimiter: &str = "includeTrailingDelimiter";
pub(crate) const kms_key_name: &str = "kmsKeyName";
pub(crate) const max_bytes_rewritten_per_call: &str = "maxBytesRewrittenPerCall";
pub(crate) const max_results: &str = "maxResults";
pub(crate) const name: &str = "name";
pub(crate) const page_token: &str = "pageToken";
pub(crate) const prefix: &str = "prefix";
pub(crate) const project: &str = "project";
pub(crate) const quota_user: &str = "quotaUser";
pub(crate) const rewrite_token: &str = "rewriteToken";
pub(crate) const service_account_email: &str = "serviceAccountEmail";
pub(crate) const show_deleted_keys: &str = "showDeletedKeys";
pub(crate) const source_generation: &str = "sourceGeneration";
pub(crate) const versions: &str = "versions";

#[macro_export]
macro_rules! push_if {
    ($self:ident, $query:ident, $x:ident) => {{
        use $crate::query::PushIf;
        $query.push_if($crate::constants::$x, &mut $self.$x);
    }};
}

#[macro_export]
macro_rules! push_if_opt {
    ($self:ident, $query:ident, $x:ident) => {{
        use $crate::query::PushIf;
        $query.push_if_opt($crate::constants::$x, &mut $self.$x);
    }};
}

#[macro_export]
macro_rules! push_enum {
    ($self:ident, $query:ident, $enum:ident, $x:ident) => {{
        $query.extend($enum::from_i32(mem::take(&mut $self.$x)).request_query())
    }};
}
