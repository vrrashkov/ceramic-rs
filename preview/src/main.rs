use std::ops::{Deref, DerefMut};

use anyhow::Result;
use ceramic_rs_did::{did::Did, did_provider::Ed25519Provider};
use ceramic_rs_http_client::{
    self,
    client::Client,
    endpoint::{CommitBuilder, StreamBuilder},
};
use did_key::{generate, Ed25519KeyPair, Fingerprint};

#[tokio::main]
async fn main() -> Result<()> {
    let host = "http://localhost:7007";
    let config = None;
    let client = Client::new(host, config).unwrap();

    // let seed_data = "f9d07dc385aa624a26c35bca21e34567ff8c76d7a7212cd633384899443848de";
    stream_create(client).await?;
    Ok(())
}

// async fn did_provider(client: Client, seed_data: &str) -> Result<()> {
//     let base_seed = base16::decode(seed_data)?;
//     let provider = Ed25519Provider::new(Some(base_seed));
//     let did = Did::<Ed25519Provider>::new(provider);
//     did.authenticate();

//     let test_content = r#"{"name": "John", "age": "28"}"#;

//     let _stream_result = client
//         .stream()
//         .create(vec!["did:key:z6MkeyNNgYK9fJf5CQEiikfRUcwZyw1JZwpG1VifJfCMkZ6M"], test_content.to_string())
//         .await?;

//     // dbg!(did.provider);
//     Ok(())
// }

async fn authenticate_ceramic(seed: Option<&[u8]>) -> Result<()> {
    let key = generate::<Ed25519KeyPair>(seed);
    let key_fingerprint = key.fingerprint();

    // let key_resolved = resolve(&format!("did:key:{}", key_fingerprint)).unwrap();
    println!("key: {:?}", key_fingerprint);

    Ok(())
}

async fn commit_create(client: Client) -> Result<()> {
    println!("{:?}", client);

    // TEST DATA
    let stream_id = String::from("k2t6wyfsu4pfzkdhr4wz5spahs2ivpy61l51nmjizm55vtuez0em64ianphk1d");
    let controllers = vec!["did:key:z6MkeyNNgYK9fJf5CQEiikfRUcwZyw1JZwpG1VifJfCMkZ6M".to_string()];
    let test_content = r#"{"name": "John", "age": "28"}"#;
    // BUILDER
    let mut builder = CommitBuilder::new();
    builder
        .controllers(controllers)
        .content(serde_json::from_str(test_content).unwrap())
        .id(stream_id);
    // REQUEST
    let stream_result = client.builder(Some(builder)).create().await?;
    println!("{:#?}", stream_result);

    Ok(())
}

async fn stream_create(client: Client) -> Result<()> {
    println!("{:?}", client);

    // TEST DATA
    let controllers = vec!["did:key:z6MkeyNNgYK9fJf5CQEiikfRUcwZyw1JZwpG1VifJfCMkZ6M".to_string()];
    let test_content = r#"{"name": "John", "age": "28"}"#;
    // BUILDER
    let mut builder = StreamBuilder::new();
    builder
        .controllers(controllers)
        .content(serde_json::from_str(test_content).unwrap());
    // REQUEST
    let stream_result = client.builder(Some(builder)).create().await?;
    println!("{:#?}", stream_result);

    Ok(())
}

async fn stream_get(client: Client) -> Result<()> {
    println!("{:?}", client);

    // TEST DATA
    let uid = "k3y52l7qbv1frxovklo9of82f95q5yw7e6oxyu86pkdx1sctv5hqnnixavnk6siyo";
    // REQUEST
    let stream_result = client.builder::<StreamBuilder>(None).get(uid).await?;
    println!("{:#?}", stream_result);

    Ok(())
}
