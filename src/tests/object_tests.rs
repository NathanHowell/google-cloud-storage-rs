use crate::google::storage::v1::{ListObjectsRequest, ListObjectsResponse};
use crate::request::Request;
use crate::storage::v1::GetObjectRequest;
use prost_types::Timestamp;

#[test]
fn valid_list_objects_url() {
    let bucket = "gs://bucket/object".parse::<ListObjectsRequest>().unwrap();

    let url = bucket
        .request_path(
            &"https://storage.googleapis.com/storage/v1/"
                .parse()
                .unwrap(),
        )
        .unwrap();

    assert_eq!(
        url.as_str(),
        "https://storage.googleapis.com/storage/v1/b/bucket/o"
    );
}

#[test]
fn valid_get_object_url() {
    let bucket = "gs://bucket/object".parse::<GetObjectRequest>().unwrap();

    let url = bucket
        .request_path(
            &"https://storage.googleapis.com/storage/v1/"
                .parse()
                .unwrap(),
        )
        .unwrap();

    assert_eq!(
        url.as_str(),
        "https://storage.googleapis.com/storage/v1/b/bucket/o/object"
    );
}

#[test]
fn valid_objects_list() {
    let res = serde_json::from_str::<ListObjectsResponse>(include_str!("valid_objects_list.json"))
        .unwrap();
    assert_eq!(res.items.len(), 2);

    let object = res.items.get(0).unwrap();
    assert_eq!(object.name, "BingSiteAuth.xml");
    assert_eq!(object.bucket, "old-website");
    assert_eq!(object.crc32c, Some(1714892481));
    assert_eq!(object.id, "old-website/BingSiteAuth.xml/1500357863879418");
    assert_eq!(object.name, "BingSiteAuth.xml");
    assert_eq!(object.generation, 1500357863879418);
    assert_eq!(object.metageneration, 3);
    assert_eq!(object.content_type, "text/xml");
    assert_eq!(object.storage_class, "MULTI_REGIONAL");
    assert_eq!(object.size, 85);
    assert_eq!(object.md5_hash, "7EST5TcVullac1DmfdqZGA==");
    assert_eq!(object.cache_control, "public, max-age=3600");
    assert_eq!(object.etag, "CPrljMyUktUCEAM=");

    assert_eq!(
        object.time_created,
        Some(Timestamp {
            seconds: 1500357863,
            nanos: 761_000000,
        })
    );

    assert_eq!(
        object.updated,
        Some(Timestamp {
            seconds: 1500369731,
            nanos: 234_000000,
        })
    );

    assert_eq!(
        object.time_storage_class_updated,
        Some(Timestamp {
            seconds: 1500357863,
            nanos: 761_000000,
        })
    );

    let object = res.items.get(1).unwrap();
    assert_eq!(
        object.id,
        "old-website/assets/jwplayer/glow/controlbar/background.png/1502378886491736"
    );
    assert_eq!(
        object.name,
        "assets/jwplayer/glow/controlbar/background.png"
    );
    assert_eq!(object.bucket, "old-website");
    assert_eq!(object.generation, 1502378886491736);
    assert_eq!(object.metageneration, 1);
    assert_eq!(object.content_type, "image/png");
    assert_eq!(object.storage_class, "MULTI_REGIONAL");
    assert_eq!(object.size, 141);
    assert_eq!(object.md5_hash, "uqEEEiB/FM3BCrHyCzr05A==");
    assert_eq!(object.crc32c, Some(2403284288));
    assert_eq!(object.etag, "CNjc4779zNUCEAE=");

    assert_eq!(
        object.time_created,
        Some(Timestamp {
            seconds: 1502378886,
            nanos: 445_000000,
        })
    );

    assert_eq!(
        object.updated,
        Some(Timestamp {
            seconds: 1502378886,
            nanos: 445_000000,
        })
    );

    assert_eq!(
        object.time_storage_class_updated,
        Some(Timestamp {
            seconds: 1502378886,
            nanos: 445_000000,
        })
    );

    assert_eq!(
        object.metadata,
        [(
            "goog-reserved-file-mtime".to_string(),
            "1502378875".to_string()
        )]
        .iter()
        .cloned()
        .collect()
    );
}
