use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct HelloRequest {
    name: String,
}

#[derive(Serialize)]
struct HelloResponse {
    greeting: String,
}

async fn handle_hello(mut req: tide::Request<()>) -> tide::Result<String> {
    let hello_req: HelloRequest = req.body_json().await?;
    let hello_resp = HelloResponse { greeting: format!("Hello, {}!", hello_req.name), };
    Ok(serde_json::to_string(&hello_resp)?)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/api/hello").post(handle_hello);
    app.listen("0.0.0.0:9090").await?;
    Ok(())
}
