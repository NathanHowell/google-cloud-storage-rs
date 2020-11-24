mod util;

use google_cloud_storage::storage::v1::{Bucket, InsertBucketRequest};
use google_cloud_storage::Client;
use httptest::{matchers::*, responders::*, Expectation, Server};
use url::Url;

#[tokio::test]
async fn insert_bucket() -> Result<(), Box<dyn std::error::Error>> {
    util::init();

    let server = Server::run();

    server.expect(
        Expectation::matching(request::method_path("POST", "/storage/v1/b"))
            .respond_with(status_code(200).body(include_str!("../src/tests/valid_bucket.json"))),
    );

    let base_url = Url::parse(server.url_str("/storage/v1/").as_str())?;

    let client = Client::builder().base_url(base_url).build()?;

    let bucket = client
        .insert_bucket(InsertBucketRequest {
            bucket: Some(Bucket {
                name: "test".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;

    assert_eq!(bucket.name, "new-bucket");

    Ok(())
}
