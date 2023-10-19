use multibase::Base;
use rand::Rng;

pub fn random_string() -> String {
    let random_bytes = rand::thread_rng().gen::<[u8; 16]>();
    let encode = multibase::encode(Base::Base64, random_bytes);
    dbg!(&encode);

    encode
}
