use std::collections::HashMap;

use hyperliquid_info_client::{InfoClient, BaseUrl};



#[tokio::test]
async fn test_all_mids() {
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap();
    let mids = info_client.all_mids(None).await.unwrap();
    assert!(!mids.is_empty(), "Expected non-empty mids");
    dbg!(&mids);
}

#[tokio::test]
async fn test_all_mids_dex_xyz() {
    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap();
    let mids = info_client.all_mids(Some("xyz".into())).await.unwrap();
    assert!(!mids.is_empty(), "Expected non-empty mids for dex xyz");
    dbg!(&mids);
}

#[tokio::test]
async fn test_fetch_meta() {
    let info_client = InfoClient::new(None, None).await.unwrap();

    let (meta, a) = info_client.meta_and_asset_contexts(Some("xyz".into())).await.unwrap();

    let mut decimals: HashMap<String, u32> = HashMap::default();

    for t in meta.universe.iter() {
        decimals.insert(t.name.clone(), t.sz_decimals);
    }

    dbg!(&decimals);
}

#[tokio::test]
async fn test_fetch_info() {
    use hyperliquid_info_client::BaseUrl;

    let info_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap();
    let info = info_client
        .user_state(
            "0x".parse().unwrap(),
            Some("xyz".into()),
        )
        .await.unwrap();

    dbg!(&info.cross_maintenance_margin_used);
    dbg!(&info.time);
    dbg!(info.total_isolated_margin_used());
    dbg!(info);
}

#[test]
fn test_total_isolated_margin_used() {
    use hyperliquid_info_client::response_structs::*;

    let make_position = |coin: &str, leverage_type: &str, margin: &str| AssetPosition {
        position: PositionData {
            coin: coin.to_string(),
            cum_funding: CumulativeFunding {
                all_time: "0".to_string(),
                since_change: "0".to_string(),
                since_open: "0".to_string(),
            },
            entry_px: None,
            leverage: Leverage {
                type_string: leverage_type.to_string(),
                value: 10,
                raw_usd: None,
            },
            liquidation_px: None,
            margin_used: margin.to_string(),
            max_leverage: 50,
            position_value: "0".to_string(),
            return_on_equity: "0".to_string(),
            szi: "0".to_string(),
            unrealized_pnl: "0".to_string(),
        },
        type_string: "oneWay".to_string(),
    };

    let state = UserStateResponse {
        asset_positions: vec![
            make_position("BTC", "cross", "100.5"),
            make_position("ETH", "isolated", "50.25"),
            make_position("SOL", "isolated", "25.75"),
        ],
        cross_margin_summary: MarginSummary {
            account_value: "1000".to_string(),
            total_margin_used: "100".to_string(),
            total_ntl_pos: "500".to_string(),
            total_raw_usd: "1000".to_string(),
        },
        cross_maintenance_margin_used: "50".to_string(),
        margin_summary: MarginSummary {
            account_value: "1000".to_string(),
            total_margin_used: "176".to_string(),
            total_ntl_pos: "500".to_string(),
            total_raw_usd: "1000".to_string(),
        },
        time: 1700000000000,
        withdrawable: "500".to_string(),
    };

    let total = state.total_isolated_margin_used();
    assert!((total - 76.0).abs() < f64::EPSILON, "Expected 76.0, got {total}");
}
