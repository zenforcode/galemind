use axum::{body::Body, http::{Request, StatusCode}};
use tokio::task;
use reqwest::Client;
use rest_server::model::new_model_router; 
#[tokio::test]
async fn test_model_endpoints() {
    // Build our app router
    let app = new_model_router();

    // Spawn server in background using hyper's Server::bind
    // Use a random free port with port 0
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 0));
    let server = hyper::Server::bind(&addr).serve(app.into_make_service());

    let addr = server.local_addr();
    let handle = task::spawn(server);

    // Create reqwest client
    let client = Client::new();

    // Test GET /models/{model_name}/ready
    let url = format!("http://{}/models/test_model/ready", addr);
    let res = client.get(&url).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
    assert_eq!(body, "Model: test_model, Ready!");

    // Test GET /models/{model_name}/versions/{model_version}/ready
    let url = format!("http://{}/models/test_model/versions/v1/ready", addr);
    let res = client.get(&url).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.text().await.unwrap();
    assert_eq!(body, "Model: test_model, Version: v1, Ready!");

    // Test POST /models/{model_name}/infer - expecting 501 Not Implemented
    let url = format!("http://{}/models/test_model/infer", addr);
    let res = client.post(&url)
        .json(&serde_json::json!({
            "inputs": [{
                "name": "input1",
                "shape": [1],
                "datatype": "FP32",
                "data": [1.0]
            }]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_IMPLEMENTED);

    // Stop the server
    handle.abort();
}
