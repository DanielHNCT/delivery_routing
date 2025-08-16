mod api_client;
mod cli;

use cli::run_interactive_cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_interactive_cli().await
}
