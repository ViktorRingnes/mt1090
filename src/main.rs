mod api;

use axum::{Router, routing::post};

fn routes() -> Router {
    Router::new()
        .route("/LagreFilB2B", post(api::lagre_fil_b2b_handler))
        .route("/HentUNCForFiler", post(api::hent_unc_for_filer_handler))
        .route("/Ping", post(api::ping_handler))
}

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/MT1090_v1", routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
