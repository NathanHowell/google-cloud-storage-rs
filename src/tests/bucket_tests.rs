use crate::google::storage::v1::DeleteBucketRequest;
use crate::request::Request;
use crate::storage::v1::{Bucket, GetBucketRequest, ListBucketsRequest, ListBucketsResponse};

#[test]
fn delete_bucket_url() {
    let bucket = "gs://bucket".parse::<DeleteBucketRequest>().unwrap();

    let url = bucket
        .request_path(
            "https://storage.googleapis.com/storage/v1/"
                .parse()
                .unwrap(),
        )
        .unwrap();

    assert_eq!(
        url.as_str(),
        "https://storage.googleapis.com/storage/v1/b/bucket"
    );
}

#[test]
fn list_buckets_url() {
    let bucket = ListBucketsRequest::default();

    let url = bucket
        .request_path(
            "https://storage.googleapis.com/storage/v1/"
                .parse()
                .unwrap(),
        )
        .unwrap();

    assert_eq!(url.as_str(), "https://storage.googleapis.com/storage/v1/b");
}

#[test]
fn get_bucket_url() {
    let bucket = "gs://bucket".parse::<GetBucketRequest>().unwrap();

    let url = bucket
        .request_path(
            "https://storage.googleapis.com/storage/v1/"
                .parse()
                .unwrap(),
        )
        .unwrap();

    assert_eq!(
        url.as_str(),
        "https://storage.googleapis.com/storage/v1/b/bucket"
    );
}

#[test]
fn valid_bucket() {
    let res = serde_json::from_str::<Bucket>(include_str!("valid_bucket.json")).unwrap();

    assert_eq!(res.metageneration, 1);
    assert_eq!(res.name, "new-bucket");
    assert_eq!(res.project_number, 115258717311);
}

#[test]
fn valid_bucket_list() {
    let res = serde_json::from_str::<ListBucketsResponse>(include_str!("valid_bucket_list.json"))
        .unwrap();
    assert_eq!(res.items.len(), 2);
    let bucket = res.items.get(0).unwrap();
    assert_eq!(bucket.name, "new-bucket");

    let bucket = res.items.get(1).unwrap();
    assert_eq!(bucket.name, "old-website");
}
