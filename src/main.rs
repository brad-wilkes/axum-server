use axum::{Server, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/", get(root));

    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));

    println!("listening on {}", address);
    Server::bind(&address).serve(routes.into_make_service()).await.unwrap();

}
pub async fn root() -> impl IntoResponse {
    "Hello, from the axum server!"
}