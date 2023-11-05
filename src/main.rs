#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    ticket_toy_server::start_server().await
}
