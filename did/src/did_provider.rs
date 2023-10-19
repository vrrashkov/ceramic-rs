use crate::did;
use josekit::jwk::alg::ed::{EdCurve, EdKeyPair};
use josekit::{
    jws::{alg::eddsa::EddsaJwsAlgorithm::Eddsa, JwsHeader},
    jwt::{self, JwtPayload},
};
use jwt_simple::prelude::Audiences;
use std::process::exit;
pub trait DidProvider {
    fn get_did(&self) -> &String;
    fn sign(
        &self,
        payload: String,
        time: u64,
        nonce: Option<String>,
        aud: Option<Audiences>,
    ) -> String;
}

#[derive(Clone)]
pub struct Ed25519Provider {
    key_pair: EdKeyPair,
    did: String,
}

impl DidProvider for Ed25519Provider {
    fn get_did(&self) -> &String {
        &self.did
    }
    // time in minutes
    fn sign(
        &self,
        payload: String,
        time: u64,
        nonce: Option<String>,
        aud: Option<Audiences>,
    ) -> String {
        let mut header = JwsHeader::new();
        header.set_token_type("JWT");

        let mut payload_jwt = JwtPayload::new();
        payload_jwt.set_subject(payload);

        let signer = Eddsa
            .signer_from_der(&self.key_pair.to_der_private_key())
            .unwrap();
        let jwt = jwt::encode_with_signer(&payload_jwt, &header, &signer).unwrap();

        jwt
    }
}

impl Ed25519Provider {
    pub fn new(existing_seed: Option<Vec<u8>>) -> Ed25519Provider {
        let key_pair: EdKeyPair = Eddsa
            .generate_key_pair(EdCurve::Ed25519)
            .unwrap_or_else(|e| {
                eprintln!("Failed to generate the keypair: {}", e);
                exit(1)
            });

        let pk = key_pair.to_der_public_key();
        let encode_did = did::encode_did(pk.as_slice());

        Ed25519Provider {
            key_pair,
            did: encode_did,
        }
    }
}
