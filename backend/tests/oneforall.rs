use httpc_test::new_client;
use serde_json::json;

#[tokio::test]
async fn oneforall() -> httpc_test::Result<()> {
    let hc = new_client("http://localhost:8080")?;

    // Test /hello GET
    hc.do_get("/hello").await?.print().await?;

    // Test /echo POST
    hc.do_post("/echo", json!({ "message": "test echo" })).await?.print().await?;

    // Test /login POST (success)
    hc.do_post("/login", json!({ "username": "admin", "password": "password" })).await?.print().await?;

    // Test /login POST (failure)
    hc.do_post("/login", json!({ "username": "wrong", "password": "creds" })).await?.print().await?;

    Ok(())
}