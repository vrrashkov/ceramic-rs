use crate::{
    endpoint::{self, PayloadBuilder},
    API_PATH,
};

use anyhow::{anyhow, Context, Result};
use serde_json::json;

impl<'c, T: PayloadBuilder> endpoint::RequestEndpoint<'c, T> {
    pub async fn get(&self, uid: &str) -> Result<serde_json::Value> {
        let url = if let Some(val) = &self.builder {
            format!(
                "{}/{}/{}/{}",
                self.setup.host,
                API_PATH,
                &val.endpoint(),
                uid
            )
        } else {
            return Err(anyhow!("Please provide a builder"));
        };

        let request = self
            .setup
            .client
            .get(url)
            .send()
            .await
            .context("Failed to get request")?
            .text()
            .await
            .context("Failed to get body")?;

        Ok(json!(request))
    }

    pub async fn create(&self) -> Result<serde_json::Value> {
        let request = if let Some(val) = &self.builder {
            let url = &format!("{}/{}/{}", self.setup.host, API_PATH, &val.endpoint());
            let builder_data = serde_json::to_string(&val.json()).unwrap();

            dbg!(&builder_data);
            self.setup.client.post(url).json(&builder_data)
        } else {
            return Err(anyhow!("Please provide a builder"));
        };

        let request_result = request
            .send()
            .await
            .context("Failed to get request")?
            .text()
            .await
            .context("Failed to get body")?;

        Ok(json!(request_result))
    }

    // pub async fn create_tile<P: DidProvider>(
    //     &self,
    //     mut did: Did<P>,
    //     genesis_content: String,
    // ) -> Result<serde_json::Value> {
    //     did.sign(genesis_content, 10);

    //     // let request = &self.stream_post(controllers:did.signature.unwrap()).await?;
    //     Ok(json!(request))
    // }
}
