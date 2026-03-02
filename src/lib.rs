pub mod req;
pub mod response_structs;

use std::collections::HashMap;
use anyhow::Result;
use crate::req::HttpClient;
use reqwest::Client;
use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use crate::response_structs::{UserStateResponse, Meta, AssetContext};

pub static MAINNET_API_URL: &str = "https://api.hyperliquid.xyz";

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum InfoRequest {
    #[serde(rename = "clearinghouseState")]
    UserState {
        user: Address,
        #[serde(skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    #[serde(rename = "batchClearinghouseStates")]
    UserStates {
        users: Vec<Address>,
    },
    #[serde(rename = "spotClearinghouseState")]
    UserTokenBalances {
        user: Address,
    },
    UserFees {
        user: Address,
    },
    OpenOrders {
        user: Address,
    },
    OrderStatus {
        user: Address,
        oid: u64,
    },
    Meta {
        #[serde(skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    SpotMeta,
    SpotMetaAndAssetCtxs,
    AllMids {
        #[serde(skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    UserFills {
        user: Address,
    },
    #[serde(rename_all = "camelCase")]
    FundingHistory {
        coin: String,
        start_time: u64,
        end_time: Option<u64>,
    },
    #[serde(rename_all = "camelCase")]
    UserFunding {
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    },
    L2Book {
        coin: String,
    },
    RecentTrades {
        coin: String,
    },
    #[serde(rename_all = "camelCase")]
    CandleSnapshot {
        req: CandleSnapshotRequest,
    },
    Referral {
        user: Address,
    },
    MetaAndAssetCtxs {
        #[serde(skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandleSnapshotRequest {
    coin: String,
    interval: String,
    start_time: u64,
    end_time: u64,
}

pub struct InfoClient {
    pub http_client: HttpClient,
}

impl InfoClient {
    pub async fn new(client: Option<Client>) -> Result<InfoClient> {
        let client = client.unwrap_or_default();

        Ok(InfoClient {
            http_client: HttpClient {
                client,
                base_url: MAINNET_API_URL.to_string(),
            },
        })
    }

    pub async fn all_mids(&self, dex: Option<String>) -> Result<HashMap<String, String>> {
        let input = InfoRequest::AllMids { dex };
        self.send_info_request(input).await
    }

    pub async fn user_state(&self, address: Address, dex: Option<String>) -> Result<UserStateResponse> {
        let input = InfoRequest::UserState { user: address, dex };
        self.send_info_request(input).await
    }

    async fn send_info_request<T: for<'a> Deserialize<'a>>(
        &self,
        info_request: InfoRequest,
    ) -> Result<T> {
        let data = serde_json::to_string(&info_request)?;
        let return_data = self.http_client.post("/info", data).await?;
        Ok(serde_json::from_str(&return_data)?)
    }

    pub async fn meta(&self, dex: Option<String>) -> Result<Meta> {
        let input = InfoRequest::Meta { dex };
        self.send_info_request(input).await
    }

    pub async fn meta_and_asset_contexts(&self, dex: Option<String>) -> Result<(Meta, Vec<AssetContext>)> {
        let input = InfoRequest::MetaAndAssetCtxs { dex };
        self.send_info_request(input).await
    }
}


