use hyperliquid_info_client::InfoClient;



#[tokio::test]
async fn test_fetch_meta() {
    let info_client = InfoClient::new(None, None).await.unwrap();

    let (meta, a) = info_client.meta_and_asset_contexts().await.unwrap();

    let mut index = 0;
    for (i, t) in meta.universe.iter().enumerate() {
        if t.name.contains("LINEA") {
            dbg!(&t);
            index = i;
            break;
        }
    }

    let c = &a[index];
    dbg!(c);
}