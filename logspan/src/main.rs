#[tracing::instrument]
async fn hello_world() {
    println!("Hello");
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tracing_subscriber::fmt::format::FmtSpan;
    
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_thread_ids(true)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        .with_target(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    tracing::info!("Hello, world!");
    tracing::error!("Hello, world!");

    hello_world().await;
    
    return Ok(());
}
