pub mod req;
pub mod errors;
pub mod response_structs;

use std::collections::HashMap;
pub use errors::Error;
use crate::req::HttpClient;
use reqwest::Client;
use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use crate::response_structs::UserStateResponse;

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub static MAINNET_API_URL: &str = "https://api.hyperliquid.xyz";
pub static TESTNET_API_URL: &str = "https://api.hyperliquid-testnet.xyz";
pub static LOCAL_API_URL: &str = "http://localhost:3001";

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum InfoRequest {
    #[serde(rename = "clearinghouseState")]
    UserState {
        user: Address,
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
    Meta,
    SpotMeta,
    SpotMetaAndAssetCtxs,
    AllMids,
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
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandleSnapshotRequest {
    coin: String,
    interval: String,
    start_time: u64,
    end_time: u64,
}

#[derive(Copy, Clone)]
pub enum BaseUrl {
    Localhost,
    Testnet,
    Mainnet,
}

impl BaseUrl {
    pub(crate) fn get_url(&self) -> String {
        match self {
            BaseUrl::Localhost => LOCAL_API_URL.to_string(),
            BaseUrl::Mainnet => MAINNET_API_URL.to_string(),
            BaseUrl::Testnet => TESTNET_API_URL.to_string(),
        }
    }
}

pub struct InfoClient {
    pub http_client: HttpClient,
}


impl InfoClient {
    pub async fn new(client: Option<Client>, base_url: Option<BaseUrl>) -> Result<InfoClient> {
        let client = client.unwrap_or_default();
        let base_url = base_url.unwrap_or(BaseUrl::Mainnet).get_url();

        Ok(InfoClient {
            http_client: HttpClient { client, base_url },
        })
    }

    pub async fn all_mids(&self) -> Result<HashMap<String, String>> {
        let input = InfoRequest::AllMids;
        self.send_info_request(input).await
    }

    pub async fn user_state(&self, address: Address) -> Result<UserStateResponse> {
        let input = InfoRequest::UserState { user: address };
        self.send_info_request(input).await
    }

    async fn send_info_request<T: for<'a> Deserialize<'a>>(
        &self,
        info_request: InfoRequest,
    ) -> Result<T> {
        let data =
            serde_json::to_string(&info_request).map_err(|e| Error::JsonParse(e.to_string()))?;

        let return_data = self.http_client.post("/info", data).await?;
        serde_json::from_str(&return_data).map_err(|e| Error::JsonParse(e.to_string()))
    }
}


