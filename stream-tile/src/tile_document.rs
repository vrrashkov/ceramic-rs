use ceramic_rs_http_client::{self, client::Client};

#[derive(Debug, Clone)]
pub struct TileDocument {
    pub client: Client,
}
