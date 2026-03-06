/// Prometeo Web — Dashboard interattiva.
///
/// Avvia il server su http://localhost:3000

#[tokio::main]
async fn main() {
    let port: u16 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    prometeo::web::server::run(port).await;
}
