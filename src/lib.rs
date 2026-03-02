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

#[derive(Deserialize)]
struct PerpDex {
    name: String,
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

    /// Builds a map from symbol to exchange asset index.
    ///
    /// Standard perps use their position in the universe: `"BTC" => 0`, `"ETH" => 1`, etc.
    /// HIP-3 perps use `100_000 + dex_index * 10_000 + asset_index`: `"xyz:TSLA" => 110001`, etc.
    ///
    /// Reference: <https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/asset-ids>
    pub async fn asset_map(&self) -> Result<HashMap<String, usize>> {
        let mut map = HashMap::new();

        // Standard perps
        let meta = self.meta(None).await?;
        for (i, asset) in meta.universe.iter().enumerate() {
            map.insert(asset.name.clone(), i);
        }

        // HIP-3 dexes
        let data = serde_json::to_string(&serde_json::json!({"type": "perpDexs"}))?;
        let resp = self.http_client.post("/info", data).await?;
        let dexes: Vec<Option<PerpDex>> = serde_json::from_str(&resp)?;

        for (dex_index, dex) in dexes.iter().enumerate() {
            let Some(dex) = dex else { continue };
            let dex_meta = self.meta(Some(dex.name.clone())).await?;
            for (i, asset) in dex_meta.universe.iter().enumerate() {
                let index = 100_000 + dex_index * 10_000 + i;
                map.insert(asset.name.clone(), index);
            }
        }

        Ok(map)
    }
}


