use std::{borrow::BorrowMut, ops::Deref};

use jsonpath_rust::{JsonPathFinder, JsonPathQuery};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::client;

pub struct RequestEndpoint<'c, T> {
    pub setup: &'c client::Client,
    pub builder: Option<T>,
}
pub trait PayloadBuilder {
    fn json(&self) -> serde_json::Value;
    fn endpoint(&self) -> &String;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamData {
    pub stream_type: u32,
    pub genesis: BuilderDataValue,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitData {
    pub stream_id: String,
    pub commit: CommitDataValue,
}

#[derive(Clone)]
pub struct StreamBuilder {
    pub data: StreamData,
    pub endpoint: String,
}
#[derive(Clone)]
pub struct CommitBuilder {
    pub data: CommitData,
    pub endpoint: String,
}
impl PayloadBuilder for StreamBuilder {
    fn json(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
    fn endpoint(&self) -> &String {
        &self.endpoint
    }
}
impl PayloadBuilder for CommitBuilder {
    fn json(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
    fn endpoint(&self) -> &String {
        &self.endpoint
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitDataValue {
    pub value: BuilderDataValue,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuilderDataValue {
    pub header: StreamDataHeader,
    pub content: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamDataHeader {
    pub controllers: Vec<String>,
}

impl StreamBuilder {
    pub fn new() -> StreamBuilder {
        let data = StreamData {
            stream_type: 0,
            genesis: BuilderDataValue {
                header: StreamDataHeader {
                    controllers: vec![],
                },
                content: json!({}),
            },
        };

        StreamBuilder {
            data,
            endpoint: String::from("streams"),
        }
    }
    pub fn controllers(&mut self, data: Vec<String>) -> &mut Self {
        self.data.genesis.header.controllers = data;
        self
    }

    pub fn content(&mut self, data: serde_json::Value) -> &mut Self {
        self.data.genesis.content = data;
        self
    }

    pub fn stream_type(&mut self, data: u32) -> &mut Self {
        self.data.stream_type = data;
        self
    }
}

impl CommitBuilder {
    pub fn new() -> CommitBuilder {
        let data = CommitData {
            stream_id: String::new(),
            commit: CommitDataValue {
                value: BuilderDataValue {
                    header: StreamDataHeader {
                        controllers: vec![],
                    },
                    content: json!({}),
                },
            },
        };

        CommitBuilder {
            data,
            endpoint: String::from("commits"),
        }
    }
    pub fn controllers(&mut self, data: Vec<String>) -> &mut Self {
        self.data.commit.value.header.controllers = data;
        self
    }

    pub fn content(&mut self, data: serde_json::Value) -> &mut Self {
        self.data.commit.value.content = data;
        self
    }

    pub fn id(&mut self, data: String) -> &mut Self {
        self.data.stream_id = data;
        self
    }
}
