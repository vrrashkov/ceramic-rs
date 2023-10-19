use crate::did_provider::DidProvider;

use libipld::codec::Encode;
use libipld::{self};
use libipld_cbor::DagCborCodec;
use multibase::Base;
use serde_json::{json, Map, Value};
#[derive(Debug, Clone)]
pub struct Did<P: DidProvider> {
    pub provider: P,
    pub signature: Option<String>,
}
pub struct AuthParam {
    pub nonce: String,
    pub aud: String,
    pub paths: Vec<String>,
}
impl<P: DidProvider> Did<P> {
    /// Create a client using the specified server.
    pub fn new(provider: P) -> Did<P> {
        Did {
            provider,
            signature: None,
        }
    }

    pub fn authenticate(&self) {
        // check if did is valid
    }

    pub fn sign(&mut self, content: String, time: u64) {
        let signature = self.provider.sign(content, time, None, None);

        dbg!(&signature);
        self.signature = Some(signature);
    }

    pub fn sign_dagcbor(&self, content: String) -> Vec<u8> {
        let did = self.provider.get_did();

        let content_wrapper = json!({
            "header": {
                "controllers": ["did:key:z6MkeyNNgYK9fJf5CQEiikfRUcwZyw1JZwpG1VifJfCMkZ6M"],
                "family":  "1",
                "tags": ["2"]
            },
            "data": content
        })
        .to_string();

        // let jose = Jose::decode(DagJoseCodec, &mut Cursor::new(&data)).unwrap();
        let mut buf = Vec::new();
        let _ = content_wrapper.encode(DagCborCodec, &mut buf);

        let data = match hex::decode(&buf) {
            Ok(val) => val,
            Err(err) => panic!("{:?}", err),
        };

        buf
    }
}

pub fn encode_did(public_key: &[u8]) -> String {
    dbg!(public_key.len());
    let mut bytes: Vec<u8> = Vec::with_capacity(public_key.len() + 2);
    bytes.push(0xed);
    bytes.push(0x01);
    bytes.extend_from_slice(public_key);

    let encode_base58 = multibase::encode(Base::Base58Btc, bytes);
    return format!("did:key:{}", encode_base58);
}

pub fn to_stable_object(json_str: String) -> Map<String, Value> {
    let parsed: Value = serde_json::from_str(json_str.as_str()).unwrap();
    let map: Map<String, Value> = parsed.as_object().unwrap().clone();

    map
}
