use blitz_rs::server::test_server::TestServer;

#[tokio::main]
async fn main() {
    let server = TestServer::start().await;

    println!("🚀 Test server running at:");
    println!("   {}", server.url("/"));
    println!("   {}", server.url("/slow"));
    println!("   {}", server.url("/delay/500"));
    println!("   {}", server.url("/json"));
    println!("   {}", server.url("/echo"));

    println!("\nPress Ctrl+C to stop.");

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl+c");

    println!("\n🛑 Shutting down server...");

    server.shutdown().await;
}
