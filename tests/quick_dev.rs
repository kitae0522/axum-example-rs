#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://127.0.0.1:3000")?;
    client.do_get("/ping").await?.print().await?;
    Ok(())
}