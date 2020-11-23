use crate::google::storage::v1::ListBucketsResponse;
use crate::storage::v1::Bucket;

#[test]
fn valid_bucket() {
    let res = serde_json::from_str::<Bucket>(include_str!("valid_bucket.json")).unwrap();

    assert_eq!(res.metageneration, 1);
    assert_eq!(res.name, "new-bucket");
    assert_eq!(res.project_number, 115258717310);
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
