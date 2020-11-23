use std::fs;
use std::process::Command;

fn main() {
    let mut config = prost_build::Config::new();

    config.type_attribute(".", r#"#[derive(serde::Deserialize)]"#);
    config.type_attribute(".", r#"#[derive(serde::Serialize)]"#);
    config.type_attribute(".", r#"#[serde(rename_all = "camelCase")]"#);

    config.field_attribute("in", r#"#[serde(rename = "in")]"#);
    config.field_attribute("type", r#"#[serde(rename = "type")]"#);

    for field in &["Bucket.project_number", "Bucket.metageneration"] {
        config.field_attribute(field, r#"#[serde(with = "crate::serde::into_string")]"#);
    }

    for path in &[
        "created_before",
        "creation_time",
        "effective_time",
        "expiration",
        "locked_time",
        "retention_expiration_time",
        "time_created",
        "time_deleted",
        "time_storage_class_updated",
        "updated",
    ] {
        config.field_attribute(
            path,
            r#"#[serde(with = "crate::serde::optional_timestamp")]"#,
        );
    }

    for path in &["fields", "update_mask"] {
        config.field_attribute(
            path,
            r#"#[serde(with = "crate::serde::optional_field_mask")]"#,
        );
    }

    let protos = &[
        "protos/google/iam/v1/policy.proto",
        "protos/google/storage/v1/storage.proto",
        "protos/google/storage/v1/storage_resources.proto",
    ];

    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("descriptor.pbf");
    match Command::new("protoc")
        .envs(std::env::vars())
        .arg("--descriptor_set_out")
        .arg(output.as_os_str())
        .arg("--include_imports")
        .arg("--proto_path")
        .arg("protos/")
        .args(protos)
        .status()
        .unwrap()
        .code()
    {
        None => std::process::exit(1),
        Some(code) if code != 0 => std::process::exit(code),
        _ => {}
    }

    let descriptor = fs::read(output).unwrap();

    use prost::Message;
    let descriptor = prost_types::FileDescriptorSet::decode(&*descriptor).unwrap();
    for file in descriptor.file {
        let package = file.package.unwrap();
        let mut messages = file
            .message_type
            .into_iter()
            .map(|x| (package.clone(), x))
            .collect::<Vec<_>>();
        while let Some((package, message)) = messages.pop() {
            let path = format!("{}.{}", package, message.name.unwrap());

            messages.extend(message.nested_type.into_iter().map(|x| (path.clone(), x)));

            config.type_attribute(&path, r#"#[serde(default)]"#);
        }
    }

    for proto in protos {
        println!("cargo:rerun-if-changed={}", proto)
    }

    config.compile_protos(protos, &["protos/"]).unwrap();
}
