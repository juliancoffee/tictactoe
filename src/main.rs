mod domain;
mod lobby;
mod server;
mod tic_tac_toe;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server::run_server().await;
    Ok(())
}
